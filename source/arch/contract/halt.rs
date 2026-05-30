#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FatalReason {
    InvalidKernelImageRange,
    Panic,
    Trap,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HaltReason {
    NoRunnableWork,
    Fatal(FatalReason),
}
