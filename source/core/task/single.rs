use core::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering};

use super::process::{ExitState, Pid};

static SINGLE_PID: AtomicUsize = AtomicUsize::new(1);
static EXITED: AtomicBool = AtomicBool::new(false);
static EXIT_PID: AtomicUsize = AtomicUsize::new(0);
static EXIT_CODE: AtomicI32 = AtomicI32::new(0);
static CLEAR_CHILD_TID: AtomicUsize = AtomicUsize::new(0);
static HEAP_BASE: AtomicUsize = AtomicUsize::new(0);
static PROGRAM_BREAK: AtomicUsize = AtomicUsize::new(0);
static MMAP_CURSOR: AtomicUsize = AtomicUsize::new(0);

pub fn single_pid() -> Pid {
    Pid::new(SINGLE_PID.load(Ordering::Relaxed))
}

pub fn single_record_exit(exit: ExitState) {
    EXIT_CODE.store(exit.code().value(), Ordering::Release);
    EXIT_PID.store(exit.pid().value(), Ordering::Release);
    EXITED.store(true, Ordering::Release);
}

pub fn single_process_exited() -> bool {
    EXITED.load(Ordering::Acquire)
}

pub fn single_exit_state() -> Option<ExitState> {
    if single_process_exited() {
        Some(ExitState::new(
            Pid::new(EXIT_PID.load(Ordering::Acquire)),
            super::process::ExitCode::new(EXIT_CODE.load(Ordering::Acquire)),
        ))
    } else {
        None
    }
}

pub fn single_set_tid_address(tidptr: usize) {
    CLEAR_CHILD_TID.store(tidptr, Ordering::Release);
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
