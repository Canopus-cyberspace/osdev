use super::address_space::AddressSpace;
use super::frame::{BootFrameAllocator, KernelImageRange, PhysRange};
use super::page_table::{HardwareRootBlocker, HardwareRootReadiness, KernelGlobalMappings};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BootMemory {
    NotReady(BootMemoryBlocker),
    UsableRange(PhysRange),
}

impl BootMemory {
    pub const fn discovery_required() -> Self {
        Self::NotReady(BootMemoryBlocker::DiscoveryRequired)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BootMemoryBlocker {
    DiscoveryRequired,
    FirmwarePointerMissing,
    FirmwareTableMalformed,
    KernelImageOutsideUsableMemory,
    MemoryNodeMissing,
    NoUsableFramesAfterKernel,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryFoundation {
    frames: BootFrameAllocator,
    kernel_globals: KernelGlobalMappings,
    kernel_address_space: AddressSpace,
}

impl MemoryFoundation {
    pub fn prepare(kernel_image: KernelImageRange, boot_memory: BootMemory) -> Self {
        let mut frames = boot_frame_allocator(boot_memory);
        let kernel_globals = KernelGlobalMappings::required(kernel_image);
        let kernel_address_space = AddressSpace::prepare_kernel(&mut frames, kernel_globals);

        Self {
            frames,
            kernel_globals,
            kernel_address_space,
        }
    }

    pub const fn from_kernel_mmu(
        kernel_globals: KernelGlobalMappings,
        hardware_root: Option<super::page_table::PageTableRoot>,
        boot_memory: BootMemory,
    ) -> Self {
        let frames = boot_frame_allocator(boot_memory);
        let root = match hardware_root {
            Some(root) => HardwareRootReadiness::Ready(root),
            None => HardwareRootReadiness::NotReady(HardwareRootBlocker::PageTableRootMissing),
        };
        let kernel_address_space = AddressSpace::from_hardware_root(root, kernel_globals);

        Self {
            frames,
            kernel_globals,
            kernel_address_space,
        }
    }

    pub const fn summary(self) -> MemorySummary {
        MemorySummary {
            hardware_root: self.kernel_address_space.hardware_root(),
        }
    }

    pub fn frames_mut(&mut self) -> &mut BootFrameAllocator {
        &mut self.frames
    }

    pub const fn kernel_globals(self) -> KernelGlobalMappings {
        self.kernel_globals
    }

    pub const fn kernel_address_space(self) -> AddressSpace {
        self.kernel_address_space
    }
}

const fn boot_frame_allocator(boot_memory: BootMemory) -> BootFrameAllocator {
    match boot_memory {
        BootMemory::NotReady(_) => BootFrameAllocator::discovery_required(),
        BootMemory::UsableRange(range) => BootFrameAllocator::from_range(range),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemorySummary {
    hardware_root: HardwareRootReadiness,
}

impl MemorySummary {
    pub const fn hardware_root(self) -> HardwareRootReadiness {
        self.hardware_root
    }
}
