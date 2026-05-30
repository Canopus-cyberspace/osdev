#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundaryMode {
    Inspect,
    Prepare,
    ApplyUnsafe,
}
