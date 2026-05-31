use crate::core::loader::LoadedUserImage;

use super::user_entry::PendingUserEntry;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Pid {
    value: usize,
}

impl Pid {
    pub const fn new(value: usize) -> Self {
        Self { value }
    }

    pub const fn value(self) -> usize {
        self.value
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExitCode {
    value: i32,
}

impl ExitCode {
    pub const fn new(value: i32) -> Self {
        Self { value }
    }

    pub const fn value(self) -> i32 {
        self.value
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExitState {
    pid: Pid,
    code: ExitCode,
}

impl ExitState {
    pub const fn new(pid: Pid, code: ExitCode) -> Self {
        Self { pid, code }
    }

    pub const fn pid(self) -> Pid {
        self.pid
    }

    pub const fn code(self) -> ExitCode {
        self.code
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcessState {
    Runnable,
    Exited(ExitState),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceLimitKind {
    Stack,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceLimit {
    current: u64,
    maximum: u64,
}

impl ResourceLimit {
    pub const fn new(current: u64, maximum: u64) -> Self {
        Self { current, maximum }
    }

    pub const fn current(self) -> u64 {
        self.current
    }

    pub const fn maximum(self) -> u64 {
        self.maximum
    }
}

pub const fn process_resource_limit(kind: ResourceLimitKind) -> ResourceLimit {
    match kind {
        ResourceLimitKind::Stack => ResourceLimit::new(32 * 1024, 32 * 1024),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Process {
    pid: Pid,
    parent_pid: Option<Pid>,
    state: ProcessState,
    pending_entry: Option<PendingUserEntry>,
    clear_child_tid: usize,
    robust_list_head: usize,
    robust_list_len: usize,
    heap_base: usize,
    program_break: usize,
}

impl Process {
    pub const fn new(pid: Pid) -> Self {
        Self {
            pid,
            parent_pid: None,
            state: ProcessState::Runnable,
            pending_entry: None,
            clear_child_tid: 0,
            robust_list_head: 0,
            robust_list_len: 0,
            heap_base: 0,
            program_break: 0,
        }
    }

    pub const fn pid(self) -> Pid {
        self.pid
    }

    pub const fn parent_pid(self) -> Option<Pid> {
        self.parent_pid
    }

    pub const fn state(self) -> ProcessState {
        self.state
    }

    pub const fn pending_entry(self) -> Option<PendingUserEntry> {
        self.pending_entry
    }

    pub fn exit(&mut self, code: ExitCode) -> ExitState {
        let exit = ExitState::new(self.pid, code);
        self.state = ProcessState::Exited(exit);
        self.pending_entry = None;
        exit
    }

    pub fn commit_exec(
        &mut self,
        image: LoadedUserImage,
    ) -> Result<PendingUserEntry, ExecCommitBlocker> {
        if matches!(self.state, ProcessState::Exited(_)) {
            return Err(ExecCommitBlocker::ProcessExited);
        }

        let entry = image.entry();
        let address_space = image.address_space();
        let layout = address_space.plan();
        if !address_space.contains_mapped_address(entry.entry_pc())
            || !address_space.covers_mapped_region(layout.stack())
        {
            return Err(ExecCommitBlocker::AddressSpaceNotReady);
        }

        let initial_break = address_space.initial_program_break();
        let pending = PendingUserEntry::new(address_space, entry.registers());
        self.pending_entry = Some(pending);
        self.state = ProcessState::Runnable;
        self.heap_base = initial_break;
        self.program_break = initial_break;
        Ok(pending)
    }

    pub fn take_pending_entry(self) -> Result<PendingUserEntry, ExecCommitBlocker> {
        match self.pending_entry {
            Some(pending) => Ok(pending),
            None => Err(ExecCommitBlocker::PendingEntryMissing),
        }
    }

    pub fn set_tid_address(&mut self, tidptr: usize) {
        self.clear_child_tid = tidptr;
    }

    pub const fn clear_child_tid(self) -> usize {
        self.clear_child_tid
    }

    pub fn set_robust_list(&mut self, head: usize, len: usize) {
        self.robust_list_head = head;
        self.robust_list_len = len;
    }

    pub const fn robust_list_head(self) -> usize {
        self.robust_list_head
    }

    pub const fn robust_list_len(self) -> usize {
        self.robust_list_len
    }

    pub const fn heap_base(self) -> usize {
        self.heap_base
    }

    pub const fn program_break(self) -> usize {
        self.program_break
    }

    pub fn set_program_break(&mut self, value: usize) {
        self.program_break = value;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExecCommitBlocker {
    AddressSpaceNotReady,
    PendingEntryMissing,
    ProcessExited,
}
