//! Shared memory-management gateway.

pub mod address_space;
pub mod foundation;
pub mod frame;
pub mod page_table;
pub mod user_copy;

pub use address_space::{
    AddressSpace, UserAddressSpace, UserAddressSpaceLoadPlan, UserAddressSpacePlan,
    UserEntryAddress, UserLoadSegment, UserLoadSegmentSet, UserMappedRegionSet, UserMemoryBlocker,
    UserMemoryRegion, UserPageCopy, UserPageInit, MAX_USER_LOAD_SEGMENTS, MAX_USER_MAPPED_REGIONS,
};
pub use foundation::{BootMemory, BootMemoryBlocker, MemoryFoundation, MemorySummary};
pub use frame::{
    BootFrameAllocator, FrameAllocError, FrameAllocatorBlocker, KernelImageRange,
    KernelImageRangeError, KernelLayout, KernelSectionRange, PhysFrame, PhysRange, PAGE_SIZE,
};
pub use page_table::{
    HardwareRootBlocker, HardwareRootReadiness, KernelGlobalMapping, KernelGlobalMappings,
    MappingFlags, PageTableRoot,
};
pub use user_copy::{
    copy_from_user, copy_to_user, map_zeroed_user_pages, NoUserMemory, UserCopyError, UserMapError,
    UserMemoryMapper, UserMemoryReader, UserMemoryWriter,
};
