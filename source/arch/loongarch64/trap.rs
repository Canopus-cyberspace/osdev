use crate::arch::contract::{
    BoundaryMode, HardwareReadiness, ReadinessReason, TrapInstallState, TrapVectorUnsupported,
};

pub const fn readiness() -> HardwareReadiness {
    HardwareReadiness::Unsupported(ReadinessReason::HardwareExecutionNotVerified)
}

pub fn install_trap_vector(_mode: BoundaryMode) -> TrapInstallState {
    TrapInstallState::Unsupported(TrapVectorUnsupported::HardwareExecutionNotVerified)
}
