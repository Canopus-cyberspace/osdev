use crate::arch::contract::{
    BoundaryMode, HardwareReadiness, KernelMmuRequest, KernelMmuState, KernelMmuUnsupported,
    ReadinessReason, UserMmuState, UserMmuUnsupported,
};
use crate::core::mm::{BootFrameAllocator, UserAddressSpaceLoadPlan};

pub const fn readiness() -> HardwareReadiness {
    HardwareReadiness::Unsupported(ReadinessReason::HardwareExecutionNotVerified)
}

pub fn activate_kernel_address_space(
    _request: KernelMmuRequest,
    _mode: BoundaryMode,
) -> KernelMmuState {
    KernelMmuState::Unsupported(KernelMmuUnsupported::HardwareExecutionNotVerified)
}

pub fn prepare_user_address_space(
    _frames: &mut BootFrameAllocator,
    _load: UserAddressSpaceLoadPlan<'_>,
    _mode: BoundaryMode,
) -> UserMmuState {
    UserMmuState::Unsupported(UserMmuUnsupported::HardwareExecutionNotVerified)
}
