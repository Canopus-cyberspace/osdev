//! Shared executable-loader gateway.

pub mod elf;

pub use elf::{
    prepare_executable_entry, prepare_executable_image, AuxEntry, AuxVectorPlan, ExecutableAbi,
    ExecutableEntryPlan, ExecutableImage, ExecutableLoadPlan, ExecutableSegmentPlan,
    ExecutableSegmentPlanSet, LoadedUserImage, LoaderBlocker,
};
