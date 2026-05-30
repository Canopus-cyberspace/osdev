//! Shared task/process gateway.

pub mod process;
pub mod single;
pub mod user_entry;

pub use process::{ExecCommitBlocker, ExitCode, ExitState, Pid, Process, ProcessState};
pub use single::{
    single_exit_state, single_heap_base, single_mmap_cursor, single_pid, single_process_exited,
    single_program_break, single_record_exit, single_set_mmap_cursor, single_set_program_break,
    single_set_tid_address, single_set_user_memory_state,
};
pub use user_entry::{PendingUserEntry, UserEntrySpec, UserRegisterImage};
