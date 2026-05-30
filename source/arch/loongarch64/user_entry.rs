use crate::arch::contract::{
    BoundaryMode, HardwareReadiness, ReadinessReason, UserEntryState, UserEntryUnsupported,
};
use crate::core::task::PendingUserEntry;

pub const fn readiness() -> HardwareReadiness {
    HardwareReadiness::NotReady(ReadinessReason::UserAddressSpaceMissing)
}

pub fn enter_user(_pending: PendingUserEntry, _mode: BoundaryMode) -> UserEntryState {
    UserEntryState::Unsupported(UserEntryUnsupported::HardwareExecutionNotVerified)
}
