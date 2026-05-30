use super::frame::{FrameAllocError, KernelImageRange, PhysFrame, PAGE_SIZE};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PageTableRoot {
    frame: PhysFrame,
}

impl PageTableRoot {
    pub const fn new(frame: PhysFrame) -> Self {
        Self { frame }
    }

    pub const fn frame(self) -> PhysFrame {
        self.frame
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KernelGlobalMappings {
    image: KernelImageRange,
    image_mapping: KernelGlobalMapping,
}

impl KernelGlobalMappings {
    pub const fn required(image: KernelImageRange) -> Self {
        let mapping = KernelGlobalMapping::identity(
            align_down(image.start(), PAGE_SIZE),
            align_up(image.end(), PAGE_SIZE),
            MappingFlags::KERNEL_IMAGE,
        );

        Self {
            image,
            image_mapping: mapping,
        }
    }

    pub const fn image(self) -> KernelImageRange {
        self.image
    }

    pub const fn image_mapping(self) -> KernelGlobalMapping {
        self.image_mapping
    }

    pub const fn contains_kernel_address(self, address: usize) -> bool {
        self.image_mapping.contains_virtual(address)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KernelGlobalMapping {
    virt_start: usize,
    phys_start: usize,
    byte_len: usize,
    flags: MappingFlags,
}

impl KernelGlobalMapping {
    pub const fn identity(virt_start: usize, virt_end: usize, flags: MappingFlags) -> Self {
        Self {
            virt_start,
            phys_start: virt_start,
            byte_len: virt_end - virt_start,
            flags,
        }
    }

    pub const fn virt_start(self) -> usize {
        self.virt_start
    }

    pub const fn phys_start(self) -> usize {
        self.phys_start
    }

    pub const fn byte_len(self) -> usize {
        self.byte_len
    }

    pub const fn flags(self) -> MappingFlags {
        self.flags
    }

    pub const fn contains_virtual(self, address: usize) -> bool {
        self.virt_start <= address && address < self.virt_start + self.byte_len
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MappingFlags {
    bits: u8,
}

impl MappingFlags {
    pub const READ: Self = Self { bits: 1 << 0 };
    pub const WRITE: Self = Self { bits: 1 << 1 };
    pub const EXECUTE: Self = Self { bits: 1 << 2 };
    pub const GLOBAL: Self = Self { bits: 1 << 3 };
    pub const USER: Self = Self { bits: 1 << 4 };
    pub const KERNEL_IMAGE: Self = Self {
        bits: Self::READ.bits | Self::WRITE.bits | Self::EXECUTE.bits | Self::GLOBAL.bits,
    };
    pub const USER_TEXT: Self = Self {
        bits: Self::READ.bits | Self::EXECUTE.bits | Self::USER.bits,
    };
    pub const USER_DATA: Self = Self {
        bits: Self::READ.bits | Self::WRITE.bits | Self::USER.bits,
    };
    pub const USER_STACK: Self = Self {
        bits: Self::READ.bits | Self::WRITE.bits | Self::USER.bits,
    };

    pub const fn user(read: bool, write: bool, execute: bool) -> Self {
        let mut bits = Self::USER.bits;
        if read {
            bits |= Self::READ.bits;
        }
        if write {
            bits |= Self::WRITE.bits;
        }
        if execute {
            bits |= Self::EXECUTE.bits;
        }

        Self { bits }
    }

    pub const fn bits(self) -> u8 {
        self.bits
    }

    pub const fn contains(self, required: Self) -> bool {
        self.bits & required.bits == required.bits
    }

    pub const fn is_user_accessible(self) -> bool {
        self.contains(Self::USER)
    }

    pub const fn is_writable(self) -> bool {
        self.contains(Self::WRITE)
    }

    pub const fn is_executable(self) -> bool {
        self.contains(Self::EXECUTE)
    }

    pub const fn is_readable(self) -> bool {
        self.contains(Self::READ)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HardwareRootBlocker {
    FrameAllocator(FrameAllocError),
    PageTableRootMissing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HardwareRootReadiness {
    Ready(PageTableRoot),
    NotReady(HardwareRootBlocker),
}

const fn align_down(value: usize, align: usize) -> usize {
    value / align * align
}

const fn align_up(value: usize, align: usize) -> usize {
    if value % align == 0 {
        value
    } else {
        align_down(value, align) + align
    }
}

impl HardwareRootReadiness {
    pub const fn is_ready(self) -> bool {
        matches!(self, Self::Ready(_))
    }
}
