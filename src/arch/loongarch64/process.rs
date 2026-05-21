use crate::fd_table;
use crate::real_elf;
use crate::trap::{self, KernelReturnState, LoongArchTrapFrame};
use crate::user_mem;

const MAX_EXITED_CHILDREN: usize = 8;
const ROOT_PID: usize = 1;
const ROOT_PPID: usize = 1;

const ECHILD: isize = -10;
const EFAULT: isize = -14;
const EINVAL: isize = -22;

#[derive(Copy, Clone)]
struct ExitedChild {
    pid: usize,
    code: usize,
    active: bool,
}

const EMPTY_CHILD: ExitedChild = ExitedChild {
    pid: 0,
    code: 0,
    active: false,
};

static mut CURRENT_PID: usize = ROOT_PID;
static mut CURRENT_PPID: usize = ROOT_PPID;
static mut NEXT_PID: usize = 2;
static mut IN_CHILD: bool = false;
static mut SAVED_PARENT_PID: usize = ROOT_PID;
static mut SAVED_PARENT_PPID: usize = ROOT_PPID;
static mut ACTIVE_CHILD_PID: usize = 0;
static mut EXITED_CHILDREN: [ExitedChild; MAX_EXITED_CHILDREN] = [EMPTY_CHILD; MAX_EXITED_CHILDREN];

pub(crate) fn reset_case_process_state() {
    unsafe {
        CURRENT_PID = ROOT_PID;
        CURRENT_PPID = ROOT_PPID;
        NEXT_PID = 2;
        IN_CHILD = false;
        SAVED_PARENT_PID = ROOT_PID;
        SAVED_PARENT_PPID = ROOT_PPID;
        ACTIVE_CHILD_PID = 0;
        let mut i = 0usize;
        while i < MAX_EXITED_CHILDREN {
            EXITED_CHILDREN[i] = EMPTY_CHILD;
            i += 1;
        }
    }
}

pub(crate) fn current_pid() -> usize {
    unsafe { CURRENT_PID }
}

pub(crate) fn current_ppid() -> usize {
    unsafe { CURRENT_PPID }
}

pub(crate) fn sys_clone(
    frame: &mut LoongArchTrapFrame,
    flags: usize,
    child_stack: usize,
) -> Result<usize, isize> {
    if (flags & !0xffusize) != 0 {
        return Err(EINVAL);
    }
    let mut child_frame = *frame;
    let child_pid = unsafe {
        if IN_CHILD {
            return Err(EINVAL);
        }
        let pid = NEXT_PID;
        NEXT_PID = NEXT_PID.saturating_add(1);
        SAVED_PARENT_PID = CURRENT_PID;
        SAVED_PARENT_PPID = CURRENT_PPID;
        ACTIVE_CHILD_PID = pid;
        pid
    };

    real_elf::save_user_snapshot();
    fd_table::save_fd_snapshot();
    child_frame.regs[4] = 0;
    if child_stack != 0 {
        child_frame.regs[3] = child_stack;
    }
    child_frame.era = child_frame.era.wrapping_add(4);

    let mut kernel_return = KernelReturnState::empty();
    trap::save_kernel_return_state(&mut kernel_return);
    unsafe {
        CURRENT_PID = child_pid;
        CURRENT_PPID = SAVED_PARENT_PID;
        IN_CHILD = true;
    }
    trap::enter_user_frame(&child_frame);
    trap::restore_kernel_return_state(&kernel_return);
    real_elf::restore_user_snapshot();
    fd_table::restore_fd_snapshot_after_child();
    unsafe {
        CURRENT_PID = SAVED_PARENT_PID;
        CURRENT_PPID = SAVED_PARENT_PPID;
        IN_CHILD = false;
        ACTIVE_CHILD_PID = 0;
    }
    Ok(child_pid)
}

pub(crate) fn exit_current_and_maybe_restore_parent(
    frame: &mut LoongArchTrapFrame,
    code: usize,
) -> bool {
    unsafe {
        if !IN_CHILD {
            return false;
        }
        let child_pid = ACTIVE_CHILD_PID;
        push_exited_child(child_pid, code);
        frame.era = trap::user_exit_return_addr();
        frame.prmd &= !0x3;
        true
    }
}

pub(crate) fn sys_wait4(pid_raw: usize, status_ptr: usize, options: usize) -> isize {
    if options != 0 {
        return EINVAL;
    }
    let pid = pid_raw as isize;
    unsafe {
        let mut i = 0usize;
        while i < MAX_EXITED_CHILDREN {
            let child = EXITED_CHILDREN[i];
            if child.active && child_matches(pid, child.pid) {
                if status_ptr != 0 {
                    let mut status = [0u8; 4];
                    write_le32(&mut status, child.code as u32);
                    if user_mem::copy_to_user(status_ptr, &status).is_err() {
                        return EFAULT;
                    }
                }
                remove_exited_child(i);
                return child.pid as isize;
            }
            i += 1;
        }
    }
    ECHILD
}

pub(crate) fn exec_current(frame: &mut LoongArchTrapFrame, path: &str) -> isize {
    match real_elf::load_basic_case(path) {
        Ok(load) => {
            frame.era = load.entry;
            frame.regs[3] = load.stack_pointer;
            frame.regs[4] = 0;
            0
        }
        Err(_) => -2,
    }
}

fn child_matches(wait_pid: isize, child_pid: usize) -> bool {
    wait_pid == -1 || wait_pid == 0 || wait_pid as usize == child_pid
}

unsafe fn push_exited_child(pid: usize, code: usize) {
    let mut i = 0usize;
    while i < MAX_EXITED_CHILDREN {
        if !EXITED_CHILDREN[i].active {
            EXITED_CHILDREN[i] = ExitedChild {
                pid,
                code,
                active: true,
            };
            return;
        }
        i += 1;
    }
    EXITED_CHILDREN[MAX_EXITED_CHILDREN - 1] = ExitedChild {
        pid,
        code,
        active: true,
    };
}

unsafe fn remove_exited_child(index: usize) {
    let mut i = index;
    while i + 1 < MAX_EXITED_CHILDREN {
        EXITED_CHILDREN[i] = EXITED_CHILDREN[i + 1];
        i += 1;
    }
    EXITED_CHILDREN[MAX_EXITED_CHILDREN - 1] = EMPTY_CHILD;
}

fn write_le32(dst: &mut [u8], value: u32) {
    let bytes = value.to_le_bytes();
    dst[0] = bytes[0];
    dst[1] = bytes[1];
    dst[2] = bytes[2];
    dst[3] = bytes[3];
}
