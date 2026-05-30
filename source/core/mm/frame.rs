pub const PAGE_SIZE: usize = 4096;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PhysFrame {
    start: usize,
}

impl PhysFrame {
    pub const fn new(start: usize) -> Result<Self, FrameAllocError> {
        if start % PAGE_SIZE == 0 {
            Ok(Self { start })
        } else {
            Err(FrameAllocError::UnalignedAddress)
        }
    }

    pub const fn start(self) -> usize {
        self.start
    }

    pub const fn end(self) -> usize {
        self.start + PAGE_SIZE
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PhysRange {
    start: usize,
    end: usize,
}

impl PhysRange {
    pub const fn new(start: usize, end: usize) -> Result<Self, FrameAllocError> {
        if start % PAGE_SIZE != 0 || end % PAGE_SIZE != 0 {
            Err(FrameAllocError::UnalignedAddress)
        } else if start >= end {
            Err(FrameAllocError::EmptyRange)
        } else {
            Ok(Self { start, end })
        }
    }

    pub const fn start(self) -> usize {
        self.start
    }

    pub const fn end(self) -> usize {
        self.end
    }

    pub const fn contains(self, address: usize) -> bool {
        self.start <= address && address < self.end
    }

    pub const fn contains_range(self, start: usize, end: usize) -> bool {
        self.start <= start && start < end && end <= self.end
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KernelImageRange {
    start: usize,
    end: usize,
}

impl KernelImageRange {
    pub const fn new(start: usize, end: usize) -> Result<Self, KernelImageRangeError> {
        if start >= end {
            Err(KernelImageRangeError::Empty)
        } else {
            Ok(Self { start, end })
        }
    }

    pub const fn start(self) -> usize {
        self.start
    }

    pub const fn end(self) -> usize {
        self.end
    }

    pub const fn contains(self, address: usize) -> bool {
        self.start <= address && address < self.end
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KernelImageRangeError {
    Empty,
    InvalidSectionOrder,
    SectionOutsideImage,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KernelSectionRange {
    start: usize,
    end: usize,
}

impl KernelSectionRange {
    pub const fn new(start: usize, end: usize) -> Result<Self, KernelImageRangeError> {
        if start > end {
            Err(KernelImageRangeError::InvalidSectionOrder)
        } else {
            Ok(Self { start, end })
        }
    }

    pub const fn start(self) -> usize {
        self.start
    }

    pub const fn end(self) -> usize {
        self.end
    }

    pub const fn contains(self, address: usize) -> bool {
        self.start <= address && address < self.end
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KernelLayout {
    image: KernelImageRange,
    text: KernelSectionRange,
    rodata: KernelSectionRange,
    data: KernelSectionRange,
    bss: KernelSectionRange,
}

impl KernelLayout {
    pub const fn new(
        image: KernelImageRange,
        text: KernelSectionRange,
        rodata: KernelSectionRange,
        data: KernelSectionRange,
        bss: KernelSectionRange,
    ) -> Result<Self, KernelImageRangeError> {
        if !range_contains(image, text)
            || !range_contains(image, rodata)
            || !range_contains(image, data)
            || !range_contains(image, bss)
        {
            Err(KernelImageRangeError::SectionOutsideImage)
        } else if text.end() > rodata.start()
            || rodata.end() > data.start()
            || data.end() > bss.start()
        {
            Err(KernelImageRangeError::InvalidSectionOrder)
        } else {
            Ok(Self {
                image,
                text,
                rodata,
                data,
                bss,
            })
        }
    }

    pub const fn image(self) -> KernelImageRange {
        self.image
    }

    pub const fn text(self) -> KernelSectionRange {
        self.text
    }

    pub const fn rodata(self) -> KernelSectionRange {
        self.rodata
    }

    pub const fn data(self) -> KernelSectionRange {
        self.data
    }

    pub const fn bss(self) -> KernelSectionRange {
        self.bss
    }
}

const fn range_contains(outer: KernelImageRange, inner: KernelSectionRange) -> bool {
    outer.start() <= inner.start() && inner.end() <= outer.end()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameAllocatorBlocker {
    MemoryMapMissing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameAllocError {
    EmptyRange,
    Exhausted,
    NotReady(FrameAllocatorBlocker),
    UnalignedAddress,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BootFrameAllocator {
    Ready { next: usize, end: usize },
    NotReady(FrameAllocatorBlocker),
}

impl BootFrameAllocator {
    pub const fn discovery_required() -> Self {
        Self::NotReady(FrameAllocatorBlocker::MemoryMapMissing)
    }

    pub const fn from_range(range: PhysRange) -> Self {
        Self::Ready {
            next: range.start(),
            end: range.end(),
        }
    }

    pub fn allocate(&mut self) -> Result<PhysFrame, FrameAllocError> {
        match self {
            Self::Ready { next, end } => {
                if *next >= *end {
                    Err(FrameAllocError::Exhausted)
                } else {
                    let frame = PhysFrame::new(*next)?;
                    *next += PAGE_SIZE;
                    Ok(frame)
                }
            }
            Self::NotReady(blocker) => Err(FrameAllocError::NotReady(*blocker)),
        }
    }
}
