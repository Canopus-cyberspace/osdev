use crate::arch::contract::{HardwareReadiness, ReadinessReason};
use crate::core::block::{BlockIoError, BlockReadCompletion, BlockReadRequest, BlockSector};

pub const fn readiness() -> HardwareReadiness {
    HardwareReadiness::Unsupported(ReadinessReason::HardwareExecutionNotVerified)
}

pub fn read_sector(request: BlockReadRequest, _out: &mut BlockSector) -> BlockReadCompletion {
    BlockReadCompletion::failed(request, BlockIoError::UnsupportedTransport)
}
