use super::boundary::BoundaryMode;
use crate::core::task::PendingUserEntry;

#[derive(Clone, Copy)]
pub struct UserEntryServices {
    enter: fn(PendingUserEntry, BoundaryMode) -> UserEntryState,
}

impl UserEntryServices {
    pub const fn new(enter: fn(PendingUserEntry, BoundaryMode) -> UserEntryState) -> Self {
        Self { enter }
    }

    pub fn enter(self, pending: PendingUserEntry, mode: BoundaryMode) -> UserEntryState {
        (self.enter)(pending, mode)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UserEntryState {
    Planned(PendingUserEntry),
    Prepared(PendingUserEntry),
    Unsupported(UserEntryUnsupported),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UserEntryUnsupported {
    HardwareExecutionNotVerified,
}
