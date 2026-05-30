use core::ptr;
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::arch::contract::{
    BoundaryMode, HardwareReadiness, KernelMmuBlocker, KernelMmuRequest, KernelMmuState,
    ReadinessReason, UserMmuBlocker, UserMmuState,
};
use crate::core::mm::{
    BootFrameAllocator, FrameAllocError, MappingFlags, PageTableRoot, PhysFrame, UserAddressSpace,
    UserAddressSpaceLoadPlan, UserCopyError, UserLoadSegment, UserMapError, UserMappedRegionSet,
    UserMemoryBlocker, UserMemoryMapper, UserMemoryReader, UserMemoryRegion, UserMemoryWriter,
    PAGE_SIZE,
};

#[cfg(target_arch = "riscv64")]
const SV39_MODE: usize = 8usize << 60;
const SATP_MODE_SHIFT: usize = 60;
const SATP_PPN_MASK: usize = (1usize << 44) - 1;
const SV39_ROOT_ENTRIES: usize = 512;
const IDENTITY_GIB: usize = 1024 * 1024 * 1024;
const DEVICE_IDENTITY_BASE: usize = 0x0000_0000;
const CONSOLE_MMIO_BASE: usize = 0x1000_0000;
const BLOCK_MMIO_BASE: usize = 0x1000_1000;
const KERNEL_IDENTITY_BASE: usize = 0x8000_0000;
const KERNEL_IDENTITY_END: usize = KERNEL_IDENTITY_BASE + IDENTITY_GIB;
const USER_TOP: usize = 1usize << 38;

const PTE_V: usize = 1 << 0;
const PTE_R: usize = 1 << 1;
const PTE_W: usize = 1 << 2;
const PTE_X: usize = 1 << 3;
const PTE_U: usize = 1 << 4;
const PTE_G: usize = 1 << 5;
const PTE_A: usize = 1 << 6;
const PTE_D: usize = 1 << 7;

#[repr(align(4096))]
struct RootPageTable {
    entries: [usize; SV39_ROOT_ENTRIES],
}

static mut KERNEL_ROOT_TABLE: RootPageTable = RootPageTable {
    entries: [0; SV39_ROOT_ENTRIES],
};

static ACTIVE_USER_FRAME_ALLOCATOR: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActiveUserMemory;

impl UserMemoryReader for ActiveUserMemory {
    fn read_user(&self, address: usize, out: &mut [u8]) -> Result<(), UserCopyError> {
        unsafe { read_active_user(address, out) }
    }
}

impl UserMemoryWriter for ActiveUserMemory {
    fn write_user(&self, address: usize, input: &[u8]) -> Result<(), UserCopyError> {
        unsafe { write_active_user(address, input) }
    }
}

impl UserMemoryMapper for ActiveUserMemory {
    fn map_zeroed_user_pages(&self, start: usize, byte_len: usize) -> Result<(), UserMapError> {
        unsafe { map_active_zeroed_user_pages(start, byte_len) }
    }
}

pub const fn readiness() -> HardwareReadiness {
    HardwareReadiness::NotReady(ReadinessReason::PageTableRootMissing)
}

pub fn activate_kernel_address_space(
    request: KernelMmuRequest,
    mode: BoundaryMode,
) -> KernelMmuState {
    match validate_root(request) {
        Ok(root) => match mode {
            BoundaryMode::Inspect => KernelMmuState::Planned(root),
            BoundaryMode::Prepare => prepare_root(root),
            BoundaryMode::ApplyUnsafe => apply_root(root),
        },
        Err(blocker) => KernelMmuState::NotReady(blocker),
    }
}

pub fn prepare_user_address_space(
    frames: &mut BootFrameAllocator,
    load: UserAddressSpaceLoadPlan<'_>,
    mode: BoundaryMode,
) -> UserMmuState {
    match validate_user_load(&load) {
        Ok(()) => match mode {
            BoundaryMode::Inspect => UserMmuState::Planned(load.address_space()),
            BoundaryMode::Prepare => materialize_user_root(frames, load, false),
            BoundaryMode::ApplyUnsafe => materialize_user_root(frames, load, true),
        },
        Err(blocker) => UserMmuState::NotReady(blocker),
    }
}

fn validate_root(request: KernelMmuRequest) -> Result<PageTableRoot, KernelMmuBlocker> {
    let image = request.layout().image();
    if !kernel_identity_window_contains(image.start())
        || !kernel_identity_window_contains(image.end().saturating_sub(1))
    {
        return Err(KernelMmuBlocker::KernelImageOutsideIdentityWindow);
    }

    let trap_vector = match request.trap_vector() {
        Some(vector) => vector,
        None => return Err(KernelMmuBlocker::TrapVectorMissing),
    };

    if !request
        .kernel_globals()
        .contains_kernel_address(trap_vector.address())
    {
        return Err(KernelMmuBlocker::TrapVectorUnmapped);
    }

    if !request
        .kernel_globals()
        .contains_kernel_address(current_instruction())
    {
        return Err(KernelMmuBlocker::CurrentInstructionUnmapped);
    }

    if !request
        .kernel_globals()
        .contains_kernel_address(current_stack())
    {
        return Err(KernelMmuBlocker::CurrentStackUnmapped);
    }

    let root_address = root_table_address();
    let frame = PhysFrame::new(root_address).map_err(|_| KernelMmuBlocker::PageTableRootInvalid)?;

    Ok(PageTableRoot::new(frame))
}

fn prepare_root(root: PageTableRoot) -> KernelMmuState {
    unsafe {
        materialize_root_table();
    }

    KernelMmuState::Prepared(root)
}

fn validate_user_load(load: &UserAddressSpaceLoadPlan<'_>) -> Result<(), UserMmuBlocker> {
    let plan = load.address_space();
    if !kernel_identity_window_contains(plan.kernel_globals().image().start())
        || !kernel_identity_window_contains(plan.kernel_globals().image().end().saturating_sub(1))
    {
        return Err(UserMmuBlocker::KernelMappingsMissing);
    }

    validate_single_page_region(plan.stack())?;
    validate_single_page_region(load.stack_region())?;

    let segments = load.segments();
    let mut index = 0usize;
    while index < segments.len() {
        let segment = match segments.get(index) {
            Some(segment) => segment,
            None => {
                return Err(UserMmuBlocker::Permissions(
                    UserMemoryBlocker::UserPayloadMissing,
                ));
            }
        };
        validate_page_aligned_region(segment.region())?;
        index += 1;
    }

    Ok(())
}

fn validate_single_page_region(region: UserMemoryRegion) -> Result<(), UserMmuBlocker> {
    if region.byte_len() != PAGE_SIZE {
        Err(UserMmuBlocker::Permissions(
            UserMemoryBlocker::RegionNotSinglePage,
        ))
    } else {
        Ok(())
    }
}

fn validate_page_aligned_region(region: UserMemoryRegion) -> Result<(), UserMmuBlocker> {
    if region.byte_len() == 0 || region.start() % PAGE_SIZE != 0 || region.end() % PAGE_SIZE != 0 {
        Err(UserMmuBlocker::Permissions(
            UserMemoryBlocker::RegionNotSinglePage,
        ))
    } else {
        Ok(())
    }
}

fn materialize_user_root(
    frames: &mut BootFrameAllocator,
    load: UserAddressSpaceLoadPlan<'_>,
    apply: bool,
) -> UserMmuState {
    let plan = load.address_space();
    let root_frame = match alloc_frame(frames) {
        Ok(frame) => frame,
        Err(blocker) => return UserMmuState::NotReady(blocker),
    };

    unsafe {
        zero_frame(root_frame);
        if let Err(blocker) = map_kernel_globals(frames, root_frame) {
            return UserMmuState::NotReady(blocker);
        }
    }

    let mut mapped_regions = UserMappedRegionSet::empty();
    if let Err(blocker) =
        unsafe { materialize_load_segments(frames, root_frame, &load, &mut mapped_regions) }
    {
        return UserMmuState::NotReady(blocker);
    }
    if let Err(blocker) =
        unsafe { materialize_stack_page(frames, root_frame, &load, &mut mapped_regions) }
    {
        return UserMmuState::NotReady(blocker);
    }

    let root = PageTableRoot::new(root_frame);
    let address_space = match UserAddressSpace::new(plan, root, mapped_regions) {
        Ok(address_space) => address_space,
        Err(blocker) => {
            return UserMmuState::NotReady(UserMmuBlocker::Permissions(blocker));
        }
    };

    if apply {
        install_active_user_allocator(frames);
        apply_user_root(root);
        UserMmuState::Applied(address_space)
    } else {
        UserMmuState::Prepared(address_space)
    }
}

fn install_active_user_allocator(frames: &mut BootFrameAllocator) {
    ACTIVE_USER_FRAME_ALLOCATOR.store(
        frames as *mut BootFrameAllocator as usize,
        Ordering::Release,
    );
}

fn alloc_frame(frames: &mut BootFrameAllocator) -> Result<PhysFrame, UserMmuBlocker> {
    frames.allocate().map_err(UserMmuBlocker::FrameAllocator)
}

unsafe fn materialize_load_segments(
    frames: &mut BootFrameAllocator,
    root_frame: PhysFrame,
    load: &UserAddressSpaceLoadPlan<'_>,
    mapped_regions: &mut UserMappedRegionSet,
) -> Result<(), UserMmuBlocker> {
    let segments = load.segments();
    let mut segment_index = 0usize;
    while segment_index < segments.len() {
        let segment = match segments.get(segment_index) {
            Some(segment) => segment,
            None => {
                return Err(UserMmuBlocker::Permissions(
                    UserMemoryBlocker::UserPayloadMissing,
                ));
            }
        };
        materialize_segment_pages(frames, root_frame, load.source(), segment)?;
        mapped_regions
            .push(segment.region())
            .map_err(UserMmuBlocker::Permissions)?;
        segment_index += 1;
    }

    Ok(())
}

unsafe fn materialize_segment_pages(
    frames: &mut BootFrameAllocator,
    root_frame: PhysFrame,
    source: &[u8],
    segment: UserLoadSegment,
) -> Result<(), UserMmuBlocker> {
    let mut page_start = segment.region().start();
    while page_start < segment.region().end() {
        let data_frame = alloc_frame(frames)?;
        zero_frame(data_frame);
        if let Some(copy) = segment
            .page_copy(page_start)
            .map_err(UserMmuBlocker::Permissions)?
        {
            let source_end =
                copy.source_offset()
                    .checked_add(copy.len())
                    .ok_or(UserMmuBlocker::Permissions(
                        UserMemoryBlocker::PayloadRangeOutsideSource,
                    ))?;
            if source_end > source.len() {
                return Err(UserMmuBlocker::Permissions(
                    UserMemoryBlocker::PayloadRangeOutsideSource,
                ));
            }
            copy_source_bytes(
                data_frame,
                copy.destination_offset(),
                &source[copy.source_offset()..source_end],
            )?;
        }

        let page_region =
            UserMemoryRegion::new(page_start, page_start + PAGE_SIZE, segment.permissions())
                .map_err(UserMmuBlocker::Permissions)?;
        map_user_page_allocating(frames, root_frame, page_region, data_frame)?;
        page_start += PAGE_SIZE;
    }

    Ok(())
}

unsafe fn materialize_stack_page(
    frames: &mut BootFrameAllocator,
    root_frame: PhysFrame,
    load: &UserAddressSpaceLoadPlan<'_>,
    mapped_regions: &mut UserMappedRegionSet,
) -> Result<(), UserMmuBlocker> {
    let data_frame = alloc_frame(frames)?;
    zero_frame(data_frame);
    copy_page_bytes(data_frame, load.stack_bytes());
    map_user_page_allocating(frames, root_frame, load.stack_region(), data_frame)?;
    mapped_regions
        .push(load.stack_region())
        .map_err(UserMmuBlocker::Permissions)
}

unsafe fn map_kernel_globals(
    frames: &mut BootFrameAllocator,
    root_frame: PhysFrame,
) -> Result<(), UserMmuBlocker> {
    let root = root_frame.start() as *mut usize;

    root.add(root_index(KERNEL_IDENTITY_BASE)).write(leaf_pte(
        KERNEL_IDENTITY_BASE,
        PTE_R | PTE_W | PTE_X | PTE_G,
    ));
    map_kernel_page_allocating(
        frames,
        root_frame,
        CONSOLE_MMIO_BASE,
        CONSOLE_MMIO_BASE,
        PTE_R | PTE_W | PTE_G,
    )?;
    map_kernel_page_allocating(
        frames,
        root_frame,
        BLOCK_MMIO_BASE,
        BLOCK_MMIO_BASE,
        PTE_R | PTE_W | PTE_G,
    )
}

unsafe fn map_kernel_page_allocating(
    frames: &mut BootFrameAllocator,
    root_frame: PhysFrame,
    virtual_address: usize,
    physical_address: usize,
    flags: usize,
) -> Result<(), UserMmuBlocker> {
    let root = root_frame.start() as *mut usize;
    let l1 = ensure_table(frames, root.add(root_index(virtual_address)))?;
    let l0 = ensure_table(frames, l1.add(level1_index(virtual_address)))?;
    let entry = l0.add(level0_index(virtual_address));
    if entry.read() & PTE_V != 0 {
        return Err(UserMmuBlocker::Permissions(
            UserMemoryBlocker::OverlappingRegions,
        ));
    }
    entry.write(leaf_pte(physical_address, flags));
    Ok(())
}

unsafe fn map_user_page_allocating(
    frames: &mut BootFrameAllocator,
    root_frame: PhysFrame,
    region: UserMemoryRegion,
    data_frame: PhysFrame,
) -> Result<(), UserMmuBlocker> {
    let root = root_frame.start() as *mut usize;
    let virtual_address = region.start();
    let l1 = ensure_table(frames, root.add(root_index(virtual_address)))?;
    let l0 = ensure_table(frames, l1.add(level1_index(virtual_address)))?;
    let entry = l0.add(level0_index(virtual_address));
    if entry.read() & PTE_V != 0 {
        return Err(UserMmuBlocker::Permissions(
            UserMemoryBlocker::OverlappingRegions,
        ));
    }

    entry.write(leaf_pte(
        data_frame.start(),
        user_pte_flags(region.permissions()),
    ));
    Ok(())
}

unsafe fn ensure_table(
    frames: &mut BootFrameAllocator,
    entry: *mut usize,
) -> Result<*mut usize, UserMmuBlocker> {
    let pte = entry.read();
    if pte & PTE_V == 0 {
        let frame = alloc_frame(frames)?;
        zero_frame(frame);
        entry.write(table_pte(frame.start()));
        Ok(frame.start() as *mut usize)
    } else if pte_is_leaf(pte) {
        Err(UserMmuBlocker::Permissions(
            UserMemoryBlocker::KernelMappingsMissing,
        ))
    } else {
        Ok(pte_phys(pte) as *mut usize)
    }
}

unsafe fn zero_frame(frame: PhysFrame) {
    let words = frame.start() as *mut usize;
    for index in 0..SV39_ROOT_ENTRIES {
        words.add(index).write(0);
    }
}

unsafe fn copy_page_bytes(frame: PhysFrame, bytes: &[u8; PAGE_SIZE]) {
    let target = frame.start() as *mut u8;
    let mut index = 0usize;
    while index < PAGE_SIZE {
        target.add(index).write(bytes[index]);
        index += 1;
    }
}

unsafe fn copy_source_bytes(
    frame: PhysFrame,
    destination_offset: usize,
    bytes: &[u8],
) -> Result<(), UserMmuBlocker> {
    let end = destination_offset
        .checked_add(bytes.len())
        .ok_or(UserMmuBlocker::Permissions(
            UserMemoryBlocker::PayloadRangeOutsideSource,
        ))?;
    if end > PAGE_SIZE {
        return Err(UserMmuBlocker::Permissions(
            UserMemoryBlocker::PayloadRangeOutsideSource,
        ));
    }

    let target = (frame.start() + destination_offset) as *mut u8;
    let mut index = 0usize;
    while index < bytes.len() {
        target.add(index).write(bytes[index]);
        index += 1;
    }
    Ok(())
}

unsafe fn read_active_user(address: usize, out: &mut [u8]) -> Result<(), UserCopyError> {
    if out.is_empty() {
        return Ok(());
    }
    let end = address
        .checked_add(out.len())
        .ok_or(UserCopyError::AddressOverflow)?;
    if address >= USER_TOP || end > USER_TOP {
        return Err(UserCopyError::InvalidUserRange);
    }

    let mut copied = 0usize;
    while copied < out.len() {
        let virtual_address = address + copied;
        let phys = translate_active_user_read(virtual_address)?;
        let page_remaining = PAGE_SIZE - (virtual_address % PAGE_SIZE);
        let amount = min_usize(page_remaining, out.len() - copied);
        let source = phys as *const u8;
        let mut index = 0usize;
        while index < amount {
            out[copied + index] = source.add(index).read_volatile();
            index += 1;
        }
        copied += amount;
    }

    Ok(())
}

unsafe fn write_active_user(address: usize, input: &[u8]) -> Result<(), UserCopyError> {
    if input.is_empty() {
        return Ok(());
    }
    let end = address
        .checked_add(input.len())
        .ok_or(UserCopyError::AddressOverflow)?;
    if address >= USER_TOP || end > USER_TOP {
        return Err(UserCopyError::InvalidUserRange);
    }

    let mut copied = 0usize;
    while copied < input.len() {
        let virtual_address = address + copied;
        let phys = translate_active_user_write(virtual_address)?;
        let page_remaining = PAGE_SIZE - (virtual_address % PAGE_SIZE);
        let amount = min_usize(page_remaining, input.len() - copied);
        let target = phys as *mut u8;
        let mut index = 0usize;
        while index < amount {
            target.add(index).write_volatile(input[copied + index]);
            index += 1;
        }
        copied += amount;
    }

    Ok(())
}

unsafe fn translate_active_user_read(address: usize) -> Result<usize, UserCopyError> {
    let satp = read_satp();
    if satp >> SATP_MODE_SHIFT != 8 {
        return Err(UserCopyError::Unsupported);
    }

    let mut table = ((satp & SATP_PPN_MASK) << 12) as *const usize;
    let mut level = 2usize;
    loop {
        let entry = table.add(level_index(address, level)).read_volatile();
        if entry & PTE_V == 0 || (entry & PTE_W != 0 && entry & PTE_R == 0) {
            return Err(UserCopyError::NotMapped);
        }

        if pte_is_leaf(entry) {
            if entry & PTE_U == 0 || entry & PTE_R == 0 {
                return Err(UserCopyError::PermissionDenied);
            }
            let offset_mask = (1usize << (12 + level * 9)) - 1;
            return Ok(pte_phys(entry) + (address & offset_mask));
        }

        if level == 0 {
            return Err(UserCopyError::NotMapped);
        }

        table = pte_phys(entry) as *const usize;
        level -= 1;
    }
}

unsafe fn translate_active_user_write(address: usize) -> Result<usize, UserCopyError> {
    let satp = read_satp();
    if satp >> SATP_MODE_SHIFT != 8 {
        return Err(UserCopyError::Unsupported);
    }

    let mut table = ((satp & SATP_PPN_MASK) << 12) as *const usize;
    let mut level = 2usize;
    loop {
        let entry = table.add(level_index(address, level)).read_volatile();
        if entry & PTE_V == 0 || (entry & PTE_W != 0 && entry & PTE_R == 0) {
            return Err(UserCopyError::NotMapped);
        }

        if pte_is_leaf(entry) {
            if entry & PTE_U == 0 || entry & PTE_W == 0 {
                return Err(UserCopyError::PermissionDenied);
            }
            let offset_mask = (1usize << (12 + level * 9)) - 1;
            return Ok(pte_phys(entry) + (address & offset_mask));
        }

        if level == 0 {
            return Err(UserCopyError::NotMapped);
        }

        table = pte_phys(entry) as *const usize;
        level -= 1;
    }
}

unsafe fn map_active_zeroed_user_pages(start: usize, byte_len: usize) -> Result<(), UserMapError> {
    if byte_len == 0 || start % PAGE_SIZE != 0 || byte_len % PAGE_SIZE != 0 {
        return Err(UserMapError::InvalidRange);
    }
    let end = start
        .checked_add(byte_len)
        .ok_or(UserMapError::AddressOverflow)?;
    if start >= USER_TOP || end > USER_TOP {
        return Err(UserMapError::InvalidRange);
    }

    let satp = read_satp();
    if satp >> SATP_MODE_SHIFT != 8 {
        return Err(UserMapError::NotReady);
    }
    let root_start = (satp & SATP_PPN_MASK) << 12;
    let root_frame = PhysFrame::new(root_start).map_err(|_| UserMapError::NotReady)?;
    let frames_ptr = ACTIVE_USER_FRAME_ALLOCATOR.load(Ordering::Acquire) as *mut BootFrameAllocator;
    if frames_ptr.is_null() {
        return Err(UserMapError::NotReady);
    }

    let mut page = start;
    while page < end {
        if active_user_page_mapped(root_frame, page)? {
            return Err(UserMapError::AlreadyMapped);
        }
        page += PAGE_SIZE;
    }

    page = start;
    let frames = &mut *frames_ptr;
    while page < end {
        let data_frame = frames.allocate().map_err(frame_alloc_error_to_map)?;
        zero_frame(data_frame);
        let region = UserMemoryRegion::new(page, page + PAGE_SIZE, MappingFlags::USER_DATA)
            .map_err(user_memory_blocker_to_map)?;
        map_user_page_allocating(frames, root_frame, region, data_frame)
            .map_err(user_mmu_blocker_to_map)?;
        page += PAGE_SIZE;
    }

    sfence_vma();
    Ok(())
}

unsafe fn active_user_page_mapped(
    root_frame: PhysFrame,
    address: usize,
) -> Result<bool, UserMapError> {
    let mut table = root_frame.start() as *const usize;
    let mut level = 2usize;
    loop {
        let entry = table.add(level_index(address, level)).read_volatile();
        if entry & PTE_V == 0 {
            return Ok(false);
        }
        if pte_is_leaf(entry) {
            return Ok(true);
        }
        if level == 0 {
            return Ok(false);
        }

        table = pte_phys(entry) as *const usize;
        level -= 1;
    }
}

const fn frame_alloc_error_to_map(error: FrameAllocError) -> UserMapError {
    match error {
        FrameAllocError::Exhausted => UserMapError::FrameExhausted,
        FrameAllocError::EmptyRange
        | FrameAllocError::NotReady(_)
        | FrameAllocError::UnalignedAddress => UserMapError::NotReady,
    }
}

const fn user_memory_blocker_to_map(blocker: UserMemoryBlocker) -> UserMapError {
    match blocker {
        UserMemoryBlocker::AddressOverflow => UserMapError::AddressOverflow,
        UserMemoryBlocker::FrameAllocator(error) => frame_alloc_error_to_map(error),
        UserMemoryBlocker::OverlappingRegions => UserMapError::AlreadyMapped,
        UserMemoryBlocker::EmptyRegion
        | UserMemoryBlocker::EntryOutsideText
        | UserMemoryBlocker::EntryOutsideUserRange
        | UserMemoryBlocker::InvalidSegmentPermissions
        | UserMemoryBlocker::InvalidSegmentSize
        | UserMemoryBlocker::InvalidStackPermissions
        | UserMemoryBlocker::InvalidTextPermissions
        | UserMemoryBlocker::KernelMappingsMissing
        | UserMemoryBlocker::KernelRangeViolation
        | UserMemoryBlocker::PageTableRootMissing
        | UserMemoryBlocker::PayloadRangeOutsideSource
        | UserMemoryBlocker::RegionOutsideUserRange
        | UserMemoryBlocker::RegionNotSinglePage
        | UserMemoryBlocker::StackPointerOutsideStack
        | UserMemoryBlocker::TooManyMappedRegions
        | UserMemoryBlocker::TooManyUserSegments
        | UserMemoryBlocker::UnalignedEntry
        | UserMemoryBlocker::UnalignedRegion
        | UserMemoryBlocker::UserPayloadMissing => UserMapError::InvalidRange,
    }
}

const fn user_mmu_blocker_to_map(blocker: UserMmuBlocker) -> UserMapError {
    match blocker {
        UserMmuBlocker::FrameAllocator(error) => frame_alloc_error_to_map(error),
        UserMmuBlocker::KernelMappingsMissing | UserMmuBlocker::PageTableRootMissing => {
            UserMapError::NotReady
        }
        UserMmuBlocker::Permissions(blocker) => user_memory_blocker_to_map(blocker),
    }
}

#[cfg(target_arch = "riscv64")]
fn apply_root(root: PageTableRoot) -> KernelMmuState {
    unsafe {
        materialize_root_table();
        write_satp(SV39_MODE | (root.frame().start() >> 12));
        sfence_vma();
    }

    KernelMmuState::Applied(root)
}

#[cfg(not(target_arch = "riscv64"))]
fn apply_root(_root: PageTableRoot) -> KernelMmuState {
    KernelMmuState::Unsupported(
        crate::arch::contract::KernelMmuUnsupported::HardwareExecutionNotVerified,
    )
}

unsafe fn materialize_root_table() {
    let entries = ptr::addr_of_mut!(KERNEL_ROOT_TABLE.entries) as *mut usize;

    for index in 0..SV39_ROOT_ENTRIES {
        entries.add(index).write(0);
    }

    entries
        .add(root_index(DEVICE_IDENTITY_BASE))
        .write(leaf_pte(DEVICE_IDENTITY_BASE, PTE_R | PTE_W | PTE_G));
    entries
        .add(root_index(KERNEL_IDENTITY_BASE))
        .write(leaf_pte(
            KERNEL_IDENTITY_BASE,
            PTE_R | PTE_W | PTE_X | PTE_G,
        ));
}

fn root_table_address() -> usize {
    ptr::addr_of!(KERNEL_ROOT_TABLE) as usize
}

const fn table_pte(phys: usize) -> usize {
    ((phys >> 12) << 10) | PTE_V
}

const fn leaf_pte(phys: usize, flags: usize) -> usize {
    ((phys >> 12) << 10) | flags | PTE_V | PTE_A | PTE_D
}

const fn pte_phys(pte: usize) -> usize {
    ((pte >> 10) << 12) & !0xfff
}

const fn pte_is_leaf(pte: usize) -> bool {
    pte & (PTE_R | PTE_W | PTE_X) != 0
}

const fn root_index(virt: usize) -> usize {
    (virt >> 30) & 0x1ff
}

const fn level1_index(virt: usize) -> usize {
    (virt >> 21) & 0x1ff
}

const fn level0_index(virt: usize) -> usize {
    (virt >> 12) & 0x1ff
}

const fn level_index(virt: usize, level: usize) -> usize {
    match level {
        2 => root_index(virt),
        1 => level1_index(virt),
        _ => level0_index(virt),
    }
}

const fn min_usize(lhs: usize, rhs: usize) -> usize {
    if lhs < rhs {
        lhs
    } else {
        rhs
    }
}

fn user_pte_flags(flags: MappingFlags) -> usize {
    let mut pte = PTE_U;
    if flags.is_readable() {
        pte |= PTE_R;
    }
    if flags.is_writable() {
        pte |= PTE_W;
    }
    if flags.is_executable() {
        pte |= PTE_X;
    }
    pte
}

const fn kernel_identity_window_contains(address: usize) -> bool {
    KERNEL_IDENTITY_BASE <= address && address < KERNEL_IDENTITY_END
}

#[cfg(target_arch = "riscv64")]
fn apply_user_root(root: PageTableRoot) {
    unsafe {
        write_satp(SV39_MODE | (root.frame().start() >> 12));
        sfence_vma();
    }
}

#[cfg(not(target_arch = "riscv64"))]
fn apply_user_root(_root: PageTableRoot) {}

#[cfg(target_arch = "riscv64")]
fn current_instruction() -> usize {
    let pc: usize;

    unsafe {
        core::arch::asm!("auipc {pc}, 0", pc = out(reg) pc, options(nomem, nostack));
    }

    pc
}

#[cfg(not(target_arch = "riscv64"))]
fn current_instruction() -> usize {
    current_instruction as *const () as usize
}

#[cfg(target_arch = "riscv64")]
fn current_stack() -> usize {
    let sp: usize;

    unsafe {
        core::arch::asm!("mv {sp}, sp", sp = out(reg) sp, options(nomem, nostack));
    }

    sp
}

#[cfg(not(target_arch = "riscv64"))]
fn current_stack() -> usize {
    root_table_address()
}

#[cfg(target_arch = "riscv64")]
unsafe fn write_satp(value: usize) {
    core::arch::asm!("csrw satp, {value}", value = in(reg) value, options(nostack));
}

#[cfg(target_arch = "riscv64")]
unsafe fn read_satp() -> usize {
    let value: usize;
    core::arch::asm!("csrr {value}, satp", value = out(reg) value, options(nostack));
    value
}

#[cfg(not(target_arch = "riscv64"))]
unsafe fn read_satp() -> usize {
    0
}

#[cfg(target_arch = "riscv64")]
unsafe fn sfence_vma() {
    core::arch::asm!("sfence.vma x0, x0", options(nostack));
}

#[cfg(not(target_arch = "riscv64"))]
unsafe fn sfence_vma() {}
