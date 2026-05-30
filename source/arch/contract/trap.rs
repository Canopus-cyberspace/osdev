use super::boundary::BoundaryMode;
use super::readiness::{HardwareReadiness, ReadinessReason};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TrapVector {
    address: usize,
}

impl TrapVector {
    pub const fn new(address: usize) -> Result<Self, TrapVectorBlocker> {
        if address == 0 {
            Err(TrapVectorBlocker::MissingVectorAddress)
        } else if address % 4 != 0 {
            Err(TrapVectorBlocker::UnalignedVectorAddress)
        } else {
            Ok(Self { address })
        }
    }

    pub const fn address(self) -> usize {
        self.address
    }
}

#[derive(Clone, Copy)]
pub struct TrapServices {
    install: fn(BoundaryMode) -> TrapInstallState,
}

impl TrapServices {
    pub const fn new(install: fn(BoundaryMode) -> TrapInstallState) -> Self {
        Self { install }
    }

    pub fn install(self, mode: BoundaryMode) -> TrapInstallState {
        (self.install)(mode)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrapInstallState {
    Planned(TrapVector),
    Prepared(TrapVector),
    Applied(TrapVector),
    NotReady(TrapVectorBlocker),
    Unsupported(TrapVectorUnsupported),
}

impl TrapInstallState {
    pub const fn vector(self) -> Option<TrapVector> {
        match self {
            Self::Planned(vector) | Self::Prepared(vector) | Self::Applied(vector) => Some(vector),
            Self::NotReady(_) | Self::Unsupported(_) => None,
        }
    }

    pub const fn readiness(self) -> HardwareReadiness {
        match self {
            Self::Applied(_) => HardwareReadiness::Ready,
            Self::Planned(_) | Self::Prepared(_) | Self::NotReady(_) => {
                HardwareReadiness::NotReady(ReadinessReason::TrapVectorNotInstalled)
            }
            Self::Unsupported(_) => {
                HardwareReadiness::Unsupported(ReadinessReason::HardwareExecutionNotVerified)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrapVectorBlocker {
    MissingVectorAddress,
    UnalignedVectorAddress,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrapVectorUnsupported {
    HardwareExecutionNotVerified,
}
