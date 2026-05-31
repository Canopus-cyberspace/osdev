use super::frame::{BootFrameAllocator, PAGE_SIZE};
use super::page_table::{
    HardwareRootBlocker, HardwareRootReadiness, KernelGlobalMappings, MappingFlags, PageTableRoot,
};

pub const MAX_USER_LOAD_SEGMENTS: usize = 8;
pub const MAX_USER_MAPPED_REGIONS: usize = MAX_USER_LOAD_SEGMENTS + 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AddressSpace {
    hardware_root: HardwareRootReadiness,
    kernel_globals: KernelGlobalMappings,
}

impl AddressSpace {
    pub fn prepare_kernel(
        frames: &mut BootFrameAllocator,
        kernel_globals: KernelGlobalMappings,
    ) -> Self {
        let hardware_root = match frames.allocate() {
            Ok(frame) => HardwareRootReadiness::Ready(PageTableRoot::new(frame)),
            Err(error) => {
                HardwareRootReadiness::NotReady(HardwareRootBlocker::FrameAllocator(error))
            }
        };

        Self {
            hardware_root,
            kernel_globals,
        }
    }

    pub const fn from_hardware_root(
        hardware_root: HardwareRootReadiness,
        kernel_globals: KernelGlobalMappings,
    ) -> Self {
        Self {
            hardware_root,
            kernel_globals,
        }
    }

    pub const fn hardware_root(self) -> HardwareRootReadiness {
        self.hardware_root
    }

    pub const fn kernel_globals(self) -> KernelGlobalMappings {
        self.kernel_globals
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UserMemoryRegion {
    start: usize,
    end: usize,
    permissions: MappingFlags,
}

impl UserMemoryRegion {
    pub const fn new(
        start: usize,
        end: usize,
        permissions: MappingFlags,
    ) -> Result<Self, UserMemoryBlocker> {
        if start % PAGE_SIZE != 0 || end % PAGE_SIZE != 0 {
            Err(UserMemoryBlocker::UnalignedRegion)
        } else if start >= end {
            Err(UserMemoryBlocker::EmptyRegion)
        } else if !is_user_range(start, end) {
            Err(UserMemoryBlocker::RegionOutsideUserRange)
        } else {
            Ok(Self {
                start,
                end,
                permissions,
            })
        }
    }

    pub const fn start(self) -> usize {
        self.start
    }

    pub const fn end(self) -> usize {
        self.end
    }

    pub const fn byte_len(self) -> usize {
        self.end - self.start
    }

    pub const fn permissions(self) -> MappingFlags {
        self.permissions
    }

    pub const fn contains(self, address: usize) -> bool {
        self.start <= address && address < self.end
    }

    pub const fn covers(self, other: Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    pub const fn overlaps(self, other: Self) -> bool {
        self.start < other.end && other.start < self.end
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UserEntryAddress {
    value: usize,
}

impl UserEntryAddress {
    pub const fn new(value: usize) -> Result<Self, UserMemoryBlocker> {
        if value % 2 != 0 {
            Err(UserMemoryBlocker::UnalignedEntry)
        } else if !is_user_address(value) {
            Err(UserMemoryBlocker::EntryOutsideUserRange)
        } else {
            Ok(Self { value })
        }
    }

    pub const fn value(self) -> usize {
        self.value
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UserAddressSpacePlan {
    kernel_globals: KernelGlobalMappings,
    text: UserMemoryRegion,
    stack: UserMemoryRegion,
    entry: UserEntryAddress,
    initial_stack_pointer: usize,
}

impl UserAddressSpacePlan {
    pub const fn new(
        kernel_globals: KernelGlobalMappings,
        text: UserMemoryRegion,
        stack: UserMemoryRegion,
        entry: UserEntryAddress,
        initial_stack_pointer: usize,
    ) -> Result<Self, UserMemoryBlocker> {
        if !text.permissions().is_user_accessible()
            || !text.permissions().is_readable()
            || !text.permissions().is_executable()
            || text.permissions().is_writable()
        {
            Err(UserMemoryBlocker::InvalidTextPermissions)
        } else if !stack.permissions().is_user_accessible()
            || !stack.permissions().is_readable()
            || !stack.permissions().is_writable()
            || stack.permissions().is_executable()
        {
            Err(UserMemoryBlocker::InvalidStackPermissions)
        } else if text.overlaps(stack) {
            Err(UserMemoryBlocker::OverlappingRegions)
        } else if region_overlaps_kernel_globals(text, kernel_globals)
            || region_overlaps_kernel_globals(stack, kernel_globals)
        {
            Err(UserMemoryBlocker::KernelRangeViolation)
        } else if !text.contains(entry.value()) {
            Err(UserMemoryBlocker::EntryOutsideText)
        } else if initial_stack_pointer < stack.start() || initial_stack_pointer > stack.end() {
            Err(UserMemoryBlocker::StackPointerOutsideStack)
        } else {
            Ok(Self {
                kernel_globals,
                text,
                stack,
                entry,
                initial_stack_pointer,
            })
        }
    }

    pub const fn kernel_globals(self) -> KernelGlobalMappings {
        self.kernel_globals
    }

    pub const fn text(self) -> UserMemoryRegion {
        self.text
    }

    pub const fn stack(self) -> UserMemoryRegion {
        self.stack
    }

    pub const fn entry(self) -> UserEntryAddress {
        self.entry
    }

    pub const fn initial_stack_pointer(self) -> usize {
        self.initial_stack_pointer
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UserPageInit {
    region: UserMemoryRegion,
    bytes: [u8; PAGE_SIZE],
}

impl UserPageInit {
    pub const fn new(
        region: UserMemoryRegion,
        bytes: [u8; PAGE_SIZE],
    ) -> Result<Self, UserMemoryBlocker> {
        if region.byte_len() != PAGE_SIZE {
            Err(UserMemoryBlocker::RegionNotSinglePage)
        } else {
            Ok(Self { region, bytes })
        }
    }

    pub const fn region(self) -> UserMemoryRegion {
        self.region
    }

    pub const fn bytes_ref(&self) -> &[u8; PAGE_SIZE] {
        &self.bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UserPageCopy {
    source_offset: usize,
    destination_offset: usize,
    len: usize,
}

impl UserPageCopy {
    pub const fn source_offset(self) -> usize {
        self.source_offset
    }

    pub const fn destination_offset(self) -> usize {
        self.destination_offset
    }

    pub const fn len(self) -> usize {
        self.len
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UserLoadSegment {
    region: UserMemoryRegion,
    virtual_start: usize,
    file_offset: usize,
    file_size: usize,
    memory_size: usize,
}

impl UserLoadSegment {
    pub fn new(
        virtual_start: usize,
        file_offset: usize,
        file_size: usize,
        memory_size: usize,
        permissions: MappingFlags,
        source_len: usize,
    ) -> Result<Self, UserMemoryBlocker> {
        if memory_size == 0 || file_size > memory_size {
            return Err(UserMemoryBlocker::InvalidSegmentSize);
        }
        if !permissions.is_user_accessible() || !permissions.is_readable() {
            return Err(UserMemoryBlocker::InvalidSegmentPermissions);
        }
        let virtual_end = virtual_start
            .checked_add(memory_size)
            .ok_or(UserMemoryBlocker::AddressOverflow)?;
        let file_end = file_offset
            .checked_add(file_size)
            .ok_or(UserMemoryBlocker::PayloadRangeOutsideSource)?;
        if file_end > source_len {
            return Err(UserMemoryBlocker::PayloadRangeOutsideSource);
        }

        let page_start = align_down(virtual_start, PAGE_SIZE);
        let page_end = align_up(virtual_end, PAGE_SIZE)?;
        let region = UserMemoryRegion::new(page_start, page_end, permissions)?;
        Ok(Self {
            region,
            virtual_start,
            file_offset,
            file_size,
            memory_size,
        })
    }

    pub const fn region(self) -> UserMemoryRegion {
        self.region
    }

    pub const fn virtual_start(self) -> usize {
        self.virtual_start
    }

    pub const fn file_offset(self) -> usize {
        self.file_offset
    }

    pub const fn file_size(self) -> usize {
        self.file_size
    }

    pub const fn memory_size(self) -> usize {
        self.memory_size
    }

    pub const fn permissions(self) -> MappingFlags {
        self.region.permissions()
    }

    pub fn page_copy(self, page_start: usize) -> Result<Option<UserPageCopy>, UserMemoryBlocker> {
        if page_start % PAGE_SIZE != 0 || !self.region.contains(page_start) {
            return Err(UserMemoryBlocker::RegionOutsideUserRange);
        }
        let page_end = page_start
            .checked_add(PAGE_SIZE)
            .ok_or(UserMemoryBlocker::AddressOverflow)?;
        let file_virtual_end = self
            .virtual_start
            .checked_add(self.file_size)
            .ok_or(UserMemoryBlocker::AddressOverflow)?;
        let copy_start = max_usize(page_start, self.virtual_start);
        let copy_end = min_usize(page_end, file_virtual_end);
        if copy_start >= copy_end {
            return Ok(None);
        }

        let source_offset = self
            .file_offset
            .checked_add(copy_start - self.virtual_start)
            .ok_or(UserMemoryBlocker::PayloadRangeOutsideSource)?;
        Ok(Some(UserPageCopy {
            source_offset,
            destination_offset: copy_start - page_start,
            len: copy_end - copy_start,
        }))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UserLoadSegmentSet {
    entries: [Option<UserLoadSegment>; MAX_USER_LOAD_SEGMENTS],
    len: usize,
}

impl UserLoadSegmentSet {
    pub const fn empty() -> Self {
        Self {
            entries: [None; MAX_USER_LOAD_SEGMENTS],
            len: 0,
        }
    }

    pub fn push(&mut self, segment: UserLoadSegment) -> Result<(), UserMemoryBlocker> {
        if self.len >= MAX_USER_LOAD_SEGMENTS {
            return Err(UserMemoryBlocker::TooManyUserSegments);
        }

        let mut index = 0usize;
        while index < self.len {
            if let Some(existing) = self.entries[index] {
                if existing.region().overlaps(segment.region()) {
                    return Err(UserMemoryBlocker::OverlappingRegions);
                }
            }
            index += 1;
        }

        self.entries[self.len] = Some(segment);
        self.len += 1;
        Ok(())
    }

    pub const fn len(self) -> usize {
        self.len
    }

    pub const fn get(self, index: usize) -> Option<UserLoadSegment> {
        if index < self.len {
            self.entries[index]
        } else {
            None
        }
    }

    pub const fn contains_address(self, address: usize) -> bool {
        let mut index = 0usize;
        while index < self.len {
            match self.entries[index] {
                Some(segment) if segment.region().contains(address) => return true,
                _ => {}
            }
            index += 1;
        }

        false
    }

    pub const fn overlaps_region(self, region: UserMemoryRegion) -> bool {
        let mut index = 0usize;
        while index < self.len {
            match self.entries[index] {
                Some(segment) if segment.region().overlaps(region) => return true,
                _ => {}
            }
            index += 1;
        }

        false
    }

    pub const fn overlaps_kernel_globals(self, kernel_globals: KernelGlobalMappings) -> bool {
        let mut index = 0usize;
        while index < self.len {
            match self.entries[index] {
                Some(segment)
                    if region_overlaps_kernel_globals(segment.region(), kernel_globals) =>
                {
                    return true;
                }
                _ => {}
            }
            index += 1;
        }

        false
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UserAddressSpaceLoadPlan<'a> {
    address_space: UserAddressSpacePlan,
    source: &'a [u8],
    segments: UserLoadSegmentSet,
    stack_init: UserPageInit,
}

impl<'a> UserAddressSpaceLoadPlan<'a> {
    pub fn new(
        address_space: UserAddressSpacePlan,
        source: &'a [u8],
        segments: UserLoadSegmentSet,
        stack_init: UserPageInit,
    ) -> Result<Self, UserMemoryBlocker> {
        if source.is_empty() || segments.len() == 0 {
            Err(UserMemoryBlocker::UserPayloadMissing)
        } else if segments.overlaps_kernel_globals(address_space.kernel_globals()) {
            Err(UserMemoryBlocker::KernelRangeViolation)
        } else if !segments.contains_address(address_space.entry().value()) {
            Err(UserMemoryBlocker::EntryOutsideText)
        } else if !address_space.stack().covers(stack_init.region())
            || stack_init.region().byte_len() != PAGE_SIZE
        {
            Err(UserMemoryBlocker::StackPointerOutsideStack)
        } else if segments.overlaps_region(address_space.stack()) {
            Err(UserMemoryBlocker::OverlappingRegions)
        } else {
            Ok(Self {
                address_space,
                source,
                segments,
                stack_init,
            })
        }
    }

    pub const fn address_space(&self) -> UserAddressSpacePlan {
        self.address_space
    }

    pub const fn source(&self) -> &'a [u8] {
        self.source
    }

    pub const fn segments(&self) -> UserLoadSegmentSet {
        self.segments
    }

    pub const fn stack_region(&self) -> UserMemoryRegion {
        self.address_space.stack()
    }

    pub const fn stack_init_region(&self) -> UserMemoryRegion {
        self.stack_init.region()
    }

    pub const fn stack_init_bytes(&self) -> &[u8; PAGE_SIZE] {
        self.stack_init.bytes_ref()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UserMappedRegionSet {
    entries: [Option<UserMemoryRegion>; MAX_USER_MAPPED_REGIONS],
    len: usize,
}

impl UserMappedRegionSet {
    pub const fn empty() -> Self {
        Self {
            entries: [None; MAX_USER_MAPPED_REGIONS],
            len: 0,
        }
    }

    pub fn push(&mut self, region: UserMemoryRegion) -> Result<(), UserMemoryBlocker> {
        if self.len >= MAX_USER_MAPPED_REGIONS {
            return Err(UserMemoryBlocker::TooManyMappedRegions);
        }

        let mut index = 0usize;
        while index < self.len {
            if let Some(existing) = self.entries[index] {
                if existing.overlaps(region) {
                    return Err(UserMemoryBlocker::OverlappingRegions);
                }
            }
            index += 1;
        }

        self.entries[self.len] = Some(region);
        self.len += 1;
        Ok(())
    }

    pub const fn len(self) -> usize {
        self.len
    }

    pub const fn get(self, index: usize) -> Option<UserMemoryRegion> {
        if index < self.len {
            self.entries[index]
        } else {
            None
        }
    }

    pub const fn contains_address(self, address: usize) -> bool {
        let mut index = 0usize;
        while index < self.len {
            match self.entries[index] {
                Some(region) if region.contains(address) => return true,
                _ => {}
            }
            index += 1;
        }

        false
    }

    pub const fn covers_region(self, region: UserMemoryRegion) -> bool {
        let mut index = 0usize;
        while index < self.len {
            match self.entries[index] {
                Some(mapped) if mapped.covers(region) => return true,
                _ => {}
            }
            index += 1;
        }

        false
    }

    pub const fn max_end_below(self, limit: usize) -> usize {
        let mut max_end = 0usize;
        let mut index = 0usize;
        while index < self.len {
            match self.entries[index] {
                Some(region) if region.end() <= limit && region.end() > max_end => {
                    max_end = region.end();
                }
                _ => {}
            }
            index += 1;
        }

        max_end
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UserAddressSpace {
    plan: UserAddressSpacePlan,
    hardware_root: PageTableRoot,
    mapped_regions: UserMappedRegionSet,
}

impl UserAddressSpace {
    pub const fn new(
        plan: UserAddressSpacePlan,
        hardware_root: PageTableRoot,
        mapped_regions: UserMappedRegionSet,
    ) -> Result<Self, UserMemoryBlocker> {
        if !mapped_regions.contains_address(plan.entry().value()) {
            Err(UserMemoryBlocker::EntryOutsideText)
        } else if !mapped_regions.covers_region(plan.stack()) {
            Err(UserMemoryBlocker::StackPointerOutsideStack)
        } else {
            Ok(Self {
                plan,
                hardware_root,
                mapped_regions,
            })
        }
    }

    pub const fn plan(self) -> UserAddressSpacePlan {
        self.plan
    }

    pub const fn hardware_root(self) -> PageTableRoot {
        self.hardware_root
    }

    pub const fn mapped_regions(self) -> UserMappedRegionSet {
        self.mapped_regions
    }

    pub const fn contains_mapped_address(self, address: usize) -> bool {
        self.mapped_regions.contains_address(address)
    }

    pub const fn covers_mapped_region(self, region: UserMemoryRegion) -> bool {
        self.mapped_regions.covers_region(region)
    }

    pub const fn initial_program_break(self) -> usize {
        self.mapped_regions.max_end_below(self.plan.stack().start())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UserMemoryBlocker {
    AddressOverflow,
    EmptyRegion,
    EntryOutsideText,
    EntryOutsideUserRange,
    FrameAllocator(super::frame::FrameAllocError),
    InvalidSegmentPermissions,
    InvalidSegmentSize,
    InvalidStackPermissions,
    InvalidTextPermissions,
    KernelMappingsMissing,
    KernelRangeViolation,
    OverlappingRegions,
    PageTableRootMissing,
    PayloadRangeOutsideSource,
    RegionOutsideUserRange,
    RegionNotSinglePage,
    StackPointerOutsideStack,
    TooManyMappedRegions,
    TooManyUserSegments,
    UnalignedEntry,
    UnalignedRegion,
    UserPayloadMissing,
}

const USER_TOP: usize = 1usize << 38;

const fn is_user_address(address: usize) -> bool {
    address < USER_TOP
}

const fn is_user_range(start: usize, end: usize) -> bool {
    start < end && end <= USER_TOP
}

const fn region_overlaps_kernel_globals(
    region: UserMemoryRegion,
    kernel_globals: KernelGlobalMappings,
) -> bool {
    let mapping = kernel_globals.image_mapping();
    let kernel_start = mapping.virt_start();
    let kernel_end = mapping.virt_start() + mapping.byte_len();

    region.start() < kernel_end && kernel_start < region.end()
}

const fn align_down(value: usize, align: usize) -> usize {
    value / align * align
}

fn align_up(value: usize, align: usize) -> Result<usize, UserMemoryBlocker> {
    if value % align == 0 {
        Ok(value)
    } else {
        align_down(value, align)
            .checked_add(align)
            .ok_or(UserMemoryBlocker::AddressOverflow)
    }
}

const fn min_usize(lhs: usize, rhs: usize) -> usize {
    if lhs < rhs {
        lhs
    } else {
        rhs
    }
}

const fn max_usize(lhs: usize, rhs: usize) -> usize {
    if lhs > rhs {
        lhs
    } else {
        rhs
    }
}
