use crate::arch::contract::{HardwareReadiness, ReadinessReason};

pub const fn readiness() -> HardwareReadiness {
    HardwareReadiness::NotReady(ReadinessReason::TimerSourceNotBound)
}
