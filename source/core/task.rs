//! Shared task/process gateway.

pub mod process;
pub mod single;
pub mod user_entry;

pub use process::{
    process_resource_limit, ExecCommitBlocker, ExitCode, ExitState, ForkRequest, ForkRequestError,
    Pid, Process, ProcessState, ResourceLimitKind,
};
pub use single::{
    single_begin_child, single_enter_child, single_enter_parent, single_enter_pid,
    single_exit_state, single_heap_base, single_is_active_child, single_mmap_cursor,
    single_parent_pid, single_pid, single_process_exited, single_program_break, single_record_exit,
    single_set_mmap_cursor, single_set_program_break, single_set_robust_list,
    single_set_tid_address, single_set_user_memory_state, single_signal_process,
    single_wait_for_child,
};
pub use user_entry::{PendingUserEntry, UserEntrySpec, UserRegisterImage};
