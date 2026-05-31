//! Shared executable-loader gateway.

pub mod elf;

pub use elf::{
    executable_interpreter, prepare_executable_entry, prepare_executable_image, AuxEntry,
    AuxVectorPlan, ElfInterpreter, ExecutableAbi, ExecutableEntryPlan, ExecutableImage,
    ExecutableLoadPlan, ExecutableSegmentPlan, ExecutableSegmentPlanSet, LoadedUserImage,
    LoaderBlocker,
};
