use super::boundary::BoundaryMode;
use super::readiness::{HardwareReadiness, ReadinessReason};
use super::trap::TrapVector;
use crate::core::mm::{
    BootFrameAllocator, KernelGlobalMappings, KernelLayout, PageTableRoot, UserAddressSpace,
    UserAddressSpaceLoadPlan, UserMemoryBlocker,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KernelMmuRequest {
    layout: KernelLayout,
    kernel_globals: KernelGlobalMappings,
    trap_vector: Option<TrapVector>,
}

impl KernelMmuRequest {
    pub const fn new(
        layout: KernelLayout,
        kernel_globals: KernelGlobalMappings,
        trap_vector: Option<TrapVector>,
    ) -> Self {
        Self {
            layout,
            kernel_globals,
            trap_vector,
        }
    }

    pub const fn layout(self) -> KernelLayout {
        self.layout
    }

    pub const fn kernel_globals(self) -> KernelGlobalMappings {
        self.kernel_globals
    }

    pub const fn trap_vector(self) -> Option<TrapVector> {
        self.trap_vector
    }
}

#[derive(Clone, Copy)]
pub struct MmuServices {
    activate_kernel: fn(KernelMmuRequest, BoundaryMode) -> KernelMmuState,
    prepare_user: for<'a> fn(
        &mut BootFrameAllocator,
        UserAddressSpaceLoadPlan<'a>,
        BoundaryMode,
    ) -> UserMmuState,
}

impl MmuServices {
    pub const fn new(
        activate_kernel: fn(KernelMmuRequest, BoundaryMode) -> KernelMmuState,
        prepare_user: for<'a> fn(
            &mut BootFrameAllocator,
            UserAddressSpaceLoadPlan<'a>,
            BoundaryMode,
        ) -> UserMmuState,
    ) -> Self {
        Self {
            activate_kernel,
            prepare_user,
        }
    }

    pub fn activate_kernel(self, request: KernelMmuRequest, mode: BoundaryMode) -> KernelMmuState {
        (self.activate_kernel)(request, mode)
    }

    pub fn prepare_user(
        self,
        frames: &mut BootFrameAllocator,
        load: UserAddressSpaceLoadPlan<'_>,
        mode: BoundaryMode,
    ) -> UserMmuState {
        (self.prepare_user)(frames, load, mode)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KernelMmuState {
    Planned(PageTableRoot),
    Prepared(PageTableRoot),
    Applied(PageTableRoot),
    NotReady(KernelMmuBlocker),
    Unsupported(KernelMmuUnsupported),
}

impl KernelMmuState {
    pub const fn root(self) -> Option<PageTableRoot> {
        match self {
            Self::Planned(root) | Self::Prepared(root) | Self::Applied(root) => Some(root),
            Self::NotReady(_) | Self::Unsupported(_) => None,
        }
    }

    pub const fn readiness(self) -> HardwareReadiness {
        match self {
            Self::Applied(_) => HardwareReadiness::Ready,
            Self::Planned(_) | Self::Prepared(_) | Self::NotReady(_) => {
                HardwareReadiness::NotReady(ReadinessReason::PageTableRootMissing)
            }
            Self::Unsupported(_) => {
                HardwareReadiness::Unsupported(ReadinessReason::HardwareExecutionNotVerified)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KernelMmuBlocker {
    CurrentInstructionUnmapped,
    CurrentStackUnmapped,
    KernelImageOutsideIdentityWindow,
    PageTableRootInvalid,
    TrapVectorMissing,
    TrapVectorUnmapped,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KernelMmuUnsupported {
    HardwareExecutionNotVerified,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UserMmuState {
    Planned(crate::core::mm::UserAddressSpacePlan),
    Prepared(UserAddressSpace),
    Applied(UserAddressSpace),
    NotReady(UserMmuBlocker),
    Unsupported(UserMmuUnsupported),
}

impl UserMmuState {
    pub const fn address_space(self) -> Option<UserAddressSpace> {
        match self {
            Self::Prepared(address_space) | Self::Applied(address_space) => Some(address_space),
            Self::Planned(_) | Self::NotReady(_) | Self::Unsupported(_) => None,
        }
    }

    pub const fn readiness(self) -> HardwareReadiness {
        match self {
            Self::Applied(_) => HardwareReadiness::Ready,
            Self::Planned(_) | Self::Prepared(_) | Self::NotReady(_) => {
                HardwareReadiness::NotReady(ReadinessReason::UserAddressSpaceMissing)
            }
            Self::Unsupported(_) => {
                HardwareReadiness::Unsupported(ReadinessReason::HardwareExecutionNotVerified)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UserMmuBlocker {
    FrameAllocator(crate::core::mm::FrameAllocError),
    KernelMappingsMissing,
    PageTableRootMissing,
    Permissions(UserMemoryBlocker),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UserMmuUnsupported {
    HardwareExecutionNotVerified,
}
