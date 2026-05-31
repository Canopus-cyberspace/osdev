use core::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering};

use super::process::{ExitCode, ExitState, Pid};

const INIT_PID: usize = 1;
const FIRST_CHILD_PID: usize = 2;
const CHILD_SLOT_COUNT: usize = 16;

macro_rules! atomic_bool_slots {
    () => {
        [
            AtomicBool::new(false),
            AtomicBool::new(false),
            AtomicBool::new(false),
            AtomicBool::new(false),
            AtomicBool::new(false),
            AtomicBool::new(false),
            AtomicBool::new(false),
            AtomicBool::new(false),
            AtomicBool::new(false),
            AtomicBool::new(false),
            AtomicBool::new(false),
            AtomicBool::new(false),
            AtomicBool::new(false),
            AtomicBool::new(false),
            AtomicBool::new(false),
            AtomicBool::new(false),
        ]
    };
}

macro_rules! atomic_i32_slots {
    () => {
        [
            AtomicI32::new(0),
            AtomicI32::new(0),
            AtomicI32::new(0),
            AtomicI32::new(0),
            AtomicI32::new(0),
            AtomicI32::new(0),
            AtomicI32::new(0),
            AtomicI32::new(0),
            AtomicI32::new(0),
            AtomicI32::new(0),
            AtomicI32::new(0),
            AtomicI32::new(0),
            AtomicI32::new(0),
            AtomicI32::new(0),
            AtomicI32::new(0),
            AtomicI32::new(0),
        ]
    };
}

macro_rules! atomic_usize_slots {
    () => {
        [
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
        ]
    };
}

static CURRENT_PID: AtomicUsize = AtomicUsize::new(INIT_PID);
static NEXT_PID: AtomicUsize = AtomicUsize::new(FIRST_CHILD_PID);
static EXITED: AtomicBool = AtomicBool::new(false);
static EXIT_PID: AtomicUsize = AtomicUsize::new(0);
static EXIT_CODE: AtomicI32 = AtomicI32::new(0);
static CHILD_PID: [AtomicUsize; CHILD_SLOT_COUNT] = atomic_usize_slots!();
static CHILD_PARENT_PID: [AtomicUsize; CHILD_SLOT_COUNT] = atomic_usize_slots!();
static CHILD_EXITED: [AtomicBool; CHILD_SLOT_COUNT] = atomic_bool_slots!();
static CHILD_EXIT_CODE: [AtomicI32; CHILD_SLOT_COUNT] = atomic_i32_slots!();
static CLEAR_CHILD_TID: AtomicUsize = AtomicUsize::new(0);
static ROBUST_LIST_HEAD: AtomicUsize = AtomicUsize::new(0);
static ROBUST_LIST_LEN: AtomicUsize = AtomicUsize::new(0);
static HEAP_BASE: AtomicUsize = AtomicUsize::new(0);
static PROGRAM_BREAK: AtomicUsize = AtomicUsize::new(0);
static MMAP_CURSOR: AtomicUsize = AtomicUsize::new(0);

pub fn single_pid() -> Pid {
    Pid::new(CURRENT_PID.load(Ordering::Relaxed))
}

pub fn single_parent_pid() -> Option<Pid> {
    let current = CURRENT_PID.load(Ordering::Acquire);
    let slot = child_slot_by_pid(current)?;
    Some(Pid::new(CHILD_PARENT_PID[slot].load(Ordering::Acquire)))
}

pub fn single_record_exit(exit: ExitState) {
    let pid = exit.pid().value();
    if let Some(slot) = child_slot_by_pid(pid) {
        CHILD_EXIT_CODE[slot].store(exit.code().value(), Ordering::Release);
        CHILD_EXITED[slot].store(true, Ordering::Release);
        return;
    }

    EXIT_CODE.store(exit.code().value(), Ordering::Release);
    EXIT_PID.store(pid, Ordering::Release);
    EXITED.store(true, Ordering::Release);
}

pub fn single_process_exited() -> bool {
    EXITED.load(Ordering::Acquire)
}

pub fn single_exit_state() -> Option<ExitState> {
    if single_process_exited() {
        Some(ExitState::new(
            Pid::new(EXIT_PID.load(Ordering::Acquire)),
            ExitCode::new(EXIT_CODE.load(Ordering::Acquire)),
        ))
    } else {
        None
    }
}

pub fn single_begin_child() -> Option<Pid> {
    let slot = free_child_slot()?;

    let pid = NEXT_PID.fetch_add(1, Ordering::AcqRel);
    CHILD_PARENT_PID[slot].store(CURRENT_PID.load(Ordering::Acquire), Ordering::Release);
    CHILD_EXIT_CODE[slot].store(0, Ordering::Release);
    CHILD_EXITED[slot].store(false, Ordering::Release);
    CHILD_PID[slot].store(pid, Ordering::Release);
    Some(Pid::new(pid))
}

pub fn single_enter_child(pid: Pid) {
    CURRENT_PID.store(pid.value(), Ordering::Release);
}

pub fn single_enter_parent() {
    CURRENT_PID.store(INIT_PID, Ordering::Release);
}

pub fn single_enter_pid(pid: Pid) {
    CURRENT_PID.store(pid.value(), Ordering::Release);
}

pub fn single_is_active_child(pid: Pid) -> bool {
    child_slot_by_pid(pid.value()).is_some() && pid.value() != INIT_PID
}

pub fn single_signal_process(pid: Pid, signal: usize) -> bool {
    if pid.value() == CURRENT_PID.load(Ordering::Acquire) {
        return true;
    }
    let slot = match child_slot_by_pid(pid.value()) {
        Some(slot) => slot,
        None => return false,
    };
    if !CHILD_EXITED[slot].load(Ordering::Acquire) {
        CHILD_EXIT_CODE[slot].store(128 + signal as i32, Ordering::Release);
        CHILD_EXITED[slot].store(true, Ordering::Release);
    }
    true
}

pub fn single_wait_for_child(target: isize) -> Option<ExitState> {
    let parent_pid = CURRENT_PID.load(Ordering::Acquire);
    let slot = exited_child_slot(parent_pid, target)?;
    let child_pid = CHILD_PID[slot].load(Ordering::Acquire);

    CHILD_PID[slot].store(0, Ordering::Release);
    CHILD_PARENT_PID[slot].store(0, Ordering::Release);
    CHILD_EXITED[slot].store(false, Ordering::Release);
    Some(ExitState::new(
        Pid::new(child_pid),
        ExitCode::new(CHILD_EXIT_CODE[slot].load(Ordering::Acquire)),
    ))
}

pub fn single_set_tid_address(tidptr: usize) {
    CLEAR_CHILD_TID.store(tidptr, Ordering::Release);
}

pub fn single_set_robust_list(head: usize, len: usize) {
    ROBUST_LIST_HEAD.store(head, Ordering::Release);
    ROBUST_LIST_LEN.store(len, Ordering::Release);
}

pub fn single_set_user_memory_state(heap_base: usize, program_break: usize, mmap_cursor: usize) {
    HEAP_BASE.store(heap_base, Ordering::Release);
    PROGRAM_BREAK.store(program_break, Ordering::Release);
    MMAP_CURSOR.store(mmap_cursor, Ordering::Release);
}

pub fn single_heap_base() -> usize {
    HEAP_BASE.load(Ordering::Acquire)
}

pub fn single_program_break() -> usize {
    PROGRAM_BREAK.load(Ordering::Acquire)
}

pub fn single_set_program_break(program_break: usize) {
    PROGRAM_BREAK.store(program_break, Ordering::Release);
}

pub fn single_mmap_cursor() -> usize {
    MMAP_CURSOR.load(Ordering::Acquire)
}

pub fn single_set_mmap_cursor(cursor: usize) {
    MMAP_CURSOR.store(cursor, Ordering::Release);
}

fn free_child_slot() -> Option<usize> {
    let mut index = 0usize;
    while index < CHILD_SLOT_COUNT {
        if CHILD_PID[index].load(Ordering::Acquire) == 0 {
            return Some(index);
        }
        index += 1;
    }

    None
}

fn child_slot_by_pid(pid: usize) -> Option<usize> {
    let mut index = 0usize;
    while index < CHILD_SLOT_COUNT {
        if CHILD_PID[index].load(Ordering::Acquire) == pid {
            return Some(index);
        }
        index += 1;
    }

    None
}

fn exited_child_slot(parent_pid: usize, target: isize) -> Option<usize> {
    let mut index = 0usize;
    while index < CHILD_SLOT_COUNT {
        let child_pid = CHILD_PID[index].load(Ordering::Acquire);
        let parent_matches = CHILD_PARENT_PID[index].load(Ordering::Acquire) == parent_pid;
        let target_matches = target <= 0 || target as usize == child_pid;
        if child_pid != 0
            && parent_matches
            && target_matches
            && CHILD_EXITED[index].load(Ordering::Acquire)
        {
            return Some(index);
        }
        index += 1;
    }

    None
}
