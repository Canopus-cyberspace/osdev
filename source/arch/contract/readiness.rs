#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HardwareReadiness {
    Ready,
    NotReady(ReadinessReason),
    Unsupported(ReadinessReason),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReadinessReason {
    HardwareExecutionNotVerified,
    BlockProviderMissing,
    PageTableRootMissing,
    TrapVectorNotInstalled,
    TimerSourceNotBound,
    UserAddressSpaceMissing,
}
