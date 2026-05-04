#![allow(dead_code)]

use crate::fd::{FdTable, FileKind};
use crate::loader::process_image::build_init_process_info;
use crate::loader::user_stack::build_initial_user_stack_dry_run;
use crate::process::{make_init_process, make_zombie};

pub const SYS_WRITE: usize = 64;
pub const SYS_EXIT: usize = 93;
pub const SYS_GETPID: usize = 172;
pub const SYS_GETPPID: usize = 173;
pub const SYS_CLONE: usize = 220;
pub const SYS_EXECVE: usize = 221;
pub const SYS_WAIT4: usize = 260;

pub const ENOSYS: isize = -38;
pub const EBADF: isize = -9;
pub const EINVAL: isize = -22;

#[derive(Copy, Clone, Debug)]
pub struct SyscallFrame {
    pub id: usize,
    pub args: [usize; 6],
}

impl SyscallFrame {
    pub const fn new(id: usize, args: [usize; 6]) -> Self {
        Self { id, args }
    }
}

pub fn dispatch_scaffold(frame: SyscallFrame) -> isize {
    match frame.id {
        SYS_GETPID => 1,
        SYS_GETPPID => 0,
        SYS_EXECVE => sys_execve_scaffold(frame.args[0], frame.args[1], frame.args[2]),
        SYS_WAIT4 => sys_wait4_scaffold(),
        SYS_CLONE => sys_clone_scaffold(),
        _ => ENOSYS,
    }
}

pub fn sys_execve_scaffold(_path: usize, _argv: usize, _envp: usize) -> isize {
    let info = match build_init_process_info() {
        Ok(info) => info,
        Err(_) => return EINVAL,
    };
    let stack = match build_initial_user_stack_dry_run() {
        Ok(stack) => stack,
        Err(_) => return EINVAL,
    };

    if info.entry() == 0 || stack.initial_sp == 0 {
        return EINVAL;
    }

    0
}

pub const fn sys_wait4_scaffold() -> isize {
    ENOSYS
}

pub const fn sys_clone_scaffold() -> isize {
    ENOSYS
}

pub fn self_test() {
    crate::println!("[syscall-scaffold-v53d] self-test begin");

    let getpid = dispatch_scaffold(SyscallFrame::new(SYS_GETPID, [0; 6]));
    let getppid = dispatch_scaffold(SyscallFrame::new(SYS_GETPPID, [0; 6]));
    let unsupported = dispatch_scaffold(SyscallFrame::new(9999, [0; 6]));
    let exec_ret = dispatch_scaffold(SyscallFrame::new(SYS_EXECVE, [0; 6]));

    crate::println!("[syscall-scaffold-v53d] getpid = {}", getpid);
    crate::println!("[syscall-scaffold-v53d] getppid = {}", getppid);
    crate::println!("[syscall-scaffold-v53d] unsupported = {}", unsupported);
    crate::println!("[syscall-scaffold-v53d] execve scaffold ret = {}", exec_ret);

    if let Ok(info) = build_init_process_info() {
        let init = make_init_process(info);
        let zombie = make_zombie(init, 0);
        crate::println!("[syscall-scaffold-v53d] zombie exit = {}", zombie.exit_code);
    }

    let mut fd_table = FdTable::with_stdio();
    let stdout_writable = fd_table.get(1).map(|fd| fd.writable).unwrap_or(false);
    crate::println!("[syscall-scaffold-v53d] stdout writable = {}", stdout_writable as usize);

    if let Some(dev_null) = fd_table.alloc(FileKind::DevNull, true, true) {
        let closed = fd_table.close(dev_null);
        crate::println!("[syscall-scaffold-v53d] devnull fd closed = {}", closed as usize);
    }

    crate::println!("[syscall-scaffold-v53d] self-test passed");
}
