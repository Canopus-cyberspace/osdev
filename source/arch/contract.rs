//! Architecture-neutral BSP contracts.

pub mod block;
pub mod boot;
pub mod boundary;
pub mod console;
pub mod halt;
pub mod mmu;
pub mod readiness;
pub mod trap;
pub mod user_entry;

pub use block::BlockServices;
pub use boot::{
    Architecture, BootInitBlocker, BootInitPath, BspServices, BspSnapshot, EarlyBootInfo,
    BOOT_INIT_ARG_COUNT,
};
pub use boundary::BoundaryMode;
pub use console::FatalConsole;
pub use halt::{FatalReason, HaltReason};
pub use mmu::{
    KernelMmuBlocker, KernelMmuRequest, KernelMmuState, KernelMmuUnsupported, MmuServices,
    UserMmuBlocker, UserMmuState, UserMmuUnsupported,
};
pub use readiness::{HardwareReadiness, ReadinessReason};
pub use trap::{
    TrapInstallState, TrapServices, TrapVector, TrapVectorBlocker, TrapVectorUnsupported,
};
pub use user_entry::{UserEntryServices, UserEntryState, UserEntryUnsupported};
