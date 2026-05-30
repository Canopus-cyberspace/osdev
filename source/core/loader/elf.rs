use crate::core::mm::{
    KernelGlobalMappings, MappingFlags, UserAddressSpace, UserAddressSpaceLoadPlan,
    UserAddressSpacePlan, UserEntryAddress, UserLoadSegment, UserLoadSegmentSet, UserMemoryBlocker,
    UserMemoryRegion, UserPageInit, PAGE_SIZE,
};
use crate::core::task::{UserEntrySpec, UserRegisterImage};

const ELF_MAGIC: &[u8; 4] = b"\x7fELF";
const ELF_CLASS_64: u8 = 2;
const ELF_DATA_LITTLE: u8 = 1;
const ELF_VERSION_CURRENT: u32 = 1;
const ELF_HEADER_SIZE: usize = 64;
const PROGRAM_HEADER_SIZE: usize = 56;
const ET_EXEC: u16 = 2;
const ET_DYN: u16 = 3;
const PT_LOAD: u32 = 1;
const PT_INTERP: u32 = 3;
const PF_X: u32 = 1;
const PF_W: u32 = 2;
const PF_R: u32 = 4;
const USER_STACK_TOP: usize = 1usize << 37;
const MAX_LOAD_SEGMENTS: usize = 8;
const MAX_ARG_STRINGS: usize = 8;
const MAX_ENV_STRINGS: usize = 8;
const MAX_AUX_ENTRIES: usize = 16;
const AT_NULL: usize = 0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExecutableAbi {
    elf_machine: u16,
    os_abi: Option<u8>,
}

impl ExecutableAbi {
    pub const fn new(elf_machine: u16) -> Self {
        Self {
            elf_machine,
            os_abi: None,
        }
    }

    pub const fn with_os_abi(elf_machine: u16, os_abi: u8) -> Self {
        Self {
            elf_machine,
            os_abi: Some(os_abi),
        }
    }

    pub const fn elf_machine(self) -> u16 {
        self.elf_machine
    }

    pub const fn os_abi(self) -> Option<u8> {
        self.os_abi
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExecutableImage<'a> {
    bytes: &'a [u8],
}

impl<'a> ExecutableImage<'a> {
    pub const fn new(bytes: &'a [u8]) -> Result<Self, LoaderBlocker> {
        if bytes.is_empty() {
            Err(LoaderBlocker::LegitimatePayloadMissing)
        } else {
            Ok(Self { bytes })
        }
    }

    pub const fn bytes(self) -> &'a [u8] {
        self.bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExecutableSegmentPlan {
    virtual_start: usize,
    virtual_end: usize,
    file_offset: usize,
    file_size: usize,
    memory_size: usize,
    permissions: MappingFlags,
}

impl ExecutableSegmentPlan {
    pub const fn virtual_start(self) -> usize {
        self.virtual_start
    }

    pub const fn virtual_end(self) -> usize {
        self.virtual_end
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
        self.permissions
    }

    pub const fn page_start(self) -> usize {
        align_down(self.virtual_start, PAGE_SIZE)
    }

    pub const fn page_end(self) -> usize {
        align_up_saturated(self.virtual_end, PAGE_SIZE)
    }

    pub const fn contains_entry(self, entry: usize) -> bool {
        self.virtual_start <= entry && entry < self.virtual_end && self.permissions.is_executable()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExecutableSegmentPlanSet {
    entries: [Option<ExecutableSegmentPlan>; MAX_LOAD_SEGMENTS],
    len: usize,
}

impl ExecutableSegmentPlanSet {
    pub const fn empty() -> Self {
        Self {
            entries: [None; MAX_LOAD_SEGMENTS],
            len: 0,
        }
    }

    pub fn push(&mut self, segment: ExecutableSegmentPlan) -> Result<(), LoaderBlocker> {
        if self.len >= MAX_LOAD_SEGMENTS {
            return Err(LoaderBlocker::TooManyLoadSegments);
        }

        let mut index = 0usize;
        while index < self.len {
            if let Some(existing) = self.entries[index] {
                if ranges_overlap(
                    existing.virtual_start(),
                    existing.virtual_end(),
                    segment.virtual_start(),
                    segment.virtual_end(),
                ) {
                    return Err(LoaderBlocker::OverlappingSegments);
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

    pub const fn get(self, index: usize) -> Option<ExecutableSegmentPlan> {
        if index < self.len {
            self.entries[index]
        } else {
            None
        }
    }

    pub const fn entry_segment(self, entry: usize) -> Option<ExecutableSegmentPlan> {
        let mut index = 0usize;
        while index < self.len {
            match self.entries[index] {
                Some(segment) if segment.contains_entry(entry) => return Some(segment),
                _ => {}
            }
            index += 1;
        }

        None
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuxEntry {
    kind: usize,
    value: usize,
}

impl AuxEntry {
    pub const fn new(kind: usize, value: usize) -> Self {
        Self { kind, value }
    }

    pub const fn kind(self) -> usize {
        self.kind
    }

    pub const fn value(self) -> usize {
        self.value
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuxVectorPlan {
    entries: [Option<AuxEntry>; MAX_AUX_ENTRIES],
    len: usize,
}

impl AuxVectorPlan {
    pub const fn empty() -> Self {
        Self {
            entries: [None; MAX_AUX_ENTRIES],
            len: 0,
        }
    }

    pub fn from_entries(entries: &[AuxEntry]) -> Result<Self, LoaderBlocker> {
        if entries.len() > MAX_AUX_ENTRIES {
            return Err(LoaderBlocker::StackLayoutOverflow);
        }

        let mut plan = Self::empty();
        let mut index = 0usize;
        while index < entries.len() {
            plan.entries[index] = Some(entries[index]);
            index += 1;
        }
        plan.len = entries.len();
        Ok(plan)
    }

    pub const fn len(self) -> usize {
        self.len
    }

    pub const fn get(self, index: usize) -> Option<AuxEntry> {
        if index < self.len {
            self.entries[index]
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExecutableLoadPlan<'a> {
    abi: ExecutableAbi,
    load: UserAddressSpaceLoadPlan<'a>,
    segments: ExecutableSegmentPlanSet,
    auxv: AuxVectorPlan,
    entry: UserEntrySpec,
}

impl<'a> ExecutableLoadPlan<'a> {
    pub const fn abi(self) -> ExecutableAbi {
        self.abi
    }

    pub const fn address_space_load(self) -> UserAddressSpaceLoadPlan<'a> {
        self.load
    }

    pub const fn segments(self) -> ExecutableSegmentPlanSet {
        self.segments
    }

    pub const fn auxv(self) -> AuxVectorPlan {
        self.auxv
    }

    pub const fn entry(self) -> UserEntrySpec {
        self.entry
    }

    pub fn complete(
        self,
        address_space: UserAddressSpace,
    ) -> Result<LoadedUserImage, LoaderBlocker> {
        LoadedUserImage::new(address_space, self.entry, self.segments)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LoadedUserImage {
    address_space: UserAddressSpace,
    entry: UserEntrySpec,
    segments: ExecutableSegmentPlanSet,
}

impl LoadedUserImage {
    pub fn new(
        address_space: UserAddressSpace,
        entry: UserEntrySpec,
        segments: ExecutableSegmentPlanSet,
    ) -> Result<Self, LoaderBlocker> {
        let layout = address_space.plan();
        if layout.entry().value() != entry.entry_pc()
            || layout.initial_stack_pointer() != entry.stack_pointer()
        {
            Err(LoaderBlocker::EntrySpecMismatch)
        } else {
            Ok(Self {
                address_space,
                entry,
                segments,
            })
        }
    }

    pub const fn address_space(self) -> UserAddressSpace {
        self.address_space
    }

    pub const fn entry(self) -> UserEntrySpec {
        self.entry
    }

    pub const fn segments(self) -> ExecutableSegmentPlanSet {
        self.segments
    }
}

pub type ExecutableEntryPlan<'a> = ExecutableLoadPlan<'a>;

pub fn prepare_executable_entry<'a>(
    source: Option<&'a [u8]>,
    kernel_globals: KernelGlobalMappings,
    abi: ExecutableAbi,
) -> Result<ExecutableLoadPlan<'a>, LoaderBlocker> {
    prepare_executable_image(source, kernel_globals, abi, &[], &[], &[])
}

pub fn prepare_executable_image<'a>(
    source: Option<&'a [u8]>,
    kernel_globals: KernelGlobalMappings,
    abi: ExecutableAbi,
    argv: &[&[u8]],
    envp: &[&[u8]],
    auxv: &[AuxEntry],
) -> Result<ExecutableLoadPlan<'a>, LoaderBlocker> {
    let bytes = match source {
        Some(bytes) if !bytes.is_empty() => bytes,
        Some(_) | None => return Err(LoaderBlocker::LegitimatePayloadMissing),
    };
    let image = ExecutableImage::new(bytes)?;
    let header = ElfHeader::parse(image.bytes(), abi)?;
    let segments = load_segments(image.bytes(), header, kernel_globals)?;
    let entry_segment = match segments.entry_segment(header.entry) {
        Some(segment) => segment,
        None => return Err(LoaderBlocker::EntrySegmentMissing),
    };
    let entry = UserEntryAddress::new(header.entry).map_err(LoaderBlocker::UserMemory)?;
    let (stack_page, stack_pointer, argv_pointer, aux_plan) =
        materialize_user_stack(argv, envp, auxv)?;
    let user_segments = user_load_segments(image.bytes(), segments)?;

    let text = UserMemoryRegion::new(
        entry_segment.page_start(),
        entry_segment.page_end(),
        MappingFlags::USER_TEXT,
    )
    .map_err(LoaderBlocker::UserMemory)?;
    let stack = UserMemoryRegion::new(
        USER_STACK_TOP - PAGE_SIZE,
        USER_STACK_TOP,
        MappingFlags::USER_STACK,
    )
    .map_err(LoaderBlocker::UserMemory)?;
    let address_space =
        UserAddressSpacePlan::new(kernel_globals, text, stack, entry, stack_pointer)
            .map_err(LoaderBlocker::UserMemory)?;
    let load =
        UserAddressSpaceLoadPlan::new(address_space, image.bytes(), user_segments, stack_page)
            .map_err(LoaderBlocker::UserMemory)?;
    let registers = UserRegisterImage::new(argv.len(), argv_pointer);
    let entry = UserEntrySpec::new(entry, stack_pointer, registers);

    Ok(ExecutableLoadPlan {
        abi,
        load,
        segments,
        auxv: aux_plan,
        entry,
    })
}

fn load_segments(
    bytes: &[u8],
    header: ElfHeader,
    kernel_globals: KernelGlobalMappings,
) -> Result<ExecutableSegmentPlanSet, LoaderBlocker> {
    let table_size = header
        .program_header_entry_size
        .checked_mul(header.program_header_count)
        .ok_or(LoaderBlocker::MalformedExecutable)?;
    let table_end = header
        .program_header_offset
        .checked_add(table_size)
        .ok_or(LoaderBlocker::MalformedExecutable)?;
    if table_end > bytes.len() {
        return Err(LoaderBlocker::MalformedExecutable);
    }

    let mut segments = ExecutableSegmentPlanSet::empty();
    let mut index = 0usize;
    while index < header.program_header_count {
        let offset = header.program_header_offset + index * header.program_header_entry_size;
        let program = ProgramHeader::parse(bytes, offset)?;
        match program.kind {
            PT_LOAD => {
                let segment = segment_from_program(bytes, program, kernel_globals)?;
                segments.push(segment)?;
            }
            PT_INTERP => return Err(LoaderBlocker::UnsupportedExecutable),
            _ => {}
        }
        index += 1;
    }

    if segments.len() == 0 {
        Err(LoaderBlocker::EntrySegmentMissing)
    } else {
        Ok(segments)
    }
}

fn segment_from_program(
    bytes: &[u8],
    program: ProgramHeader,
    kernel_globals: KernelGlobalMappings,
) -> Result<ExecutableSegmentPlan, LoaderBlocker> {
    if program.memory_size == 0 || program.file_size > program.memory_size {
        return Err(LoaderBlocker::MalformedExecutable);
    }
    let virtual_end = program
        .virtual_address
        .checked_add(program.memory_size)
        .ok_or(LoaderBlocker::AddressOverflow)?;
    let file_end = program
        .file_offset
        .checked_add(program.file_size)
        .ok_or(LoaderBlocker::MalformedExecutable)?;
    if file_end > bytes.len() {
        return Err(LoaderBlocker::MalformedExecutable);
    }
    validate_segment_alignment(program)?;
    let permissions = segment_permissions(program.flags)?;
    let page_start = align_down(program.virtual_address, PAGE_SIZE);
    let page_end = align_up(virtual_end, PAGE_SIZE)?;
    UserMemoryRegion::new(page_start, page_end, permissions).map_err(LoaderBlocker::UserMemory)?;
    if range_overlaps_kernel_globals(page_start, page_end, kernel_globals) {
        return Err(LoaderBlocker::KernelRangeViolation);
    }

    Ok(ExecutableSegmentPlan {
        virtual_start: program.virtual_address,
        virtual_end,
        file_offset: program.file_offset,
        file_size: program.file_size,
        memory_size: program.memory_size,
        permissions,
    })
}

fn user_load_segments(
    bytes: &[u8],
    segments: ExecutableSegmentPlanSet,
) -> Result<UserLoadSegmentSet, LoaderBlocker> {
    let mut user_segments = UserLoadSegmentSet::empty();
    let mut segment_index = 0usize;
    while segment_index < segments.len() {
        let segment = match segments.get(segment_index) {
            Some(segment) => segment,
            None => return Err(LoaderBlocker::MalformedExecutable),
        };
        let user_segment = UserLoadSegment::new(
            segment.virtual_start(),
            segment.file_offset(),
            segment.file_size(),
            segment.memory_size(),
            segment.permissions(),
            bytes.len(),
        )
        .map_err(LoaderBlocker::UserMemory)?;
        user_segments
            .push(user_segment)
            .map_err(LoaderBlocker::UserMemory)?;
        segment_index += 1;
    }

    Ok(user_segments)
}

fn materialize_user_stack(
    argv: &[&[u8]],
    envp: &[&[u8]],
    auxv: &[AuxEntry],
) -> Result<(UserPageInit, usize, usize, AuxVectorPlan), LoaderBlocker> {
    if argv.len() > MAX_ARG_STRINGS || envp.len() > MAX_ENV_STRINGS {
        return Err(LoaderBlocker::StackLayoutOverflow);
    }
    let aux_plan = AuxVectorPlan::from_entries(auxv)?;
    let stack_start = USER_STACK_TOP - PAGE_SIZE;
    let mut bytes = [0u8; PAGE_SIZE];
    let mut cursor = PAGE_SIZE;
    let mut argv_ptrs = [0usize; MAX_ARG_STRINGS];
    let mut envp_ptrs = [0usize; MAX_ENV_STRINGS];

    let mut index = 0usize;
    while index < argv.len() {
        argv_ptrs[index] = push_stack_string(&mut bytes, &mut cursor, stack_start, argv[index])?;
        index += 1;
    }
    index = 0;
    while index < envp.len() {
        envp_ptrs[index] = push_stack_string(&mut bytes, &mut cursor, stack_start, envp[index])?;
        index += 1;
    }

    cursor = align_down(cursor, 16);
    let word_count = 1 + argv.len() + 1 + envp.len() + 1 + 2 * (auxv.len() + 1);
    let final_cursor = cursor
        .checked_sub(word_count * core::mem::size_of::<usize>())
        .ok_or(LoaderBlocker::StackLayoutOverflow)?;
    let final_sp = stack_start
        .checked_add(final_cursor)
        .ok_or(LoaderBlocker::AddressOverflow)?;
    if final_sp % 16 != 0 {
        cursor = cursor
            .checked_sub(final_sp % 16)
            .ok_or(LoaderBlocker::StackLayoutOverflow)?;
    }

    push_usize(&mut bytes, &mut cursor, 0)?;
    push_usize(&mut bytes, &mut cursor, AT_NULL)?;
    index = auxv.len();
    while index > 0 {
        index -= 1;
        push_usize(&mut bytes, &mut cursor, auxv[index].value())?;
        push_usize(&mut bytes, &mut cursor, auxv[index].kind())?;
    }
    push_usize(&mut bytes, &mut cursor, 0)?;
    index = envp.len();
    while index > 0 {
        index -= 1;
        push_usize(&mut bytes, &mut cursor, envp_ptrs[index])?;
    }
    push_usize(&mut bytes, &mut cursor, 0)?;
    index = argv.len();
    while index > 0 {
        index -= 1;
        push_usize(&mut bytes, &mut cursor, argv_ptrs[index])?;
    }
    let argv_pointer = stack_start
        .checked_add(cursor)
        .ok_or(LoaderBlocker::AddressOverflow)?;
    push_usize(&mut bytes, &mut cursor, argv.len())?;
    let stack_pointer = stack_start
        .checked_add(cursor)
        .ok_or(LoaderBlocker::AddressOverflow)?;
    if stack_pointer % 16 != 0 {
        return Err(LoaderBlocker::StackAlignmentInvalid);
    }

    let region = UserMemoryRegion::new(stack_start, USER_STACK_TOP, MappingFlags::USER_STACK)
        .map_err(LoaderBlocker::UserMemory)?;
    Ok((
        UserPageInit::new(region, bytes).map_err(LoaderBlocker::UserMemory)?,
        stack_pointer,
        argv_pointer,
        aux_plan,
    ))
}

fn push_stack_string(
    page: &mut [u8; PAGE_SIZE],
    cursor: &mut usize,
    stack_start: usize,
    value: &[u8],
) -> Result<usize, LoaderBlocker> {
    let mut index = 0usize;
    while index < value.len() {
        if value[index] == 0 {
            return Err(LoaderBlocker::StackStringContainsNul);
        }
        index += 1;
    }
    let len = value
        .len()
        .checked_add(1)
        .ok_or(LoaderBlocker::StackLayoutOverflow)?;
    *cursor = cursor
        .checked_sub(len)
        .ok_or(LoaderBlocker::StackLayoutOverflow)?;
    let end = *cursor + value.len();
    page[*cursor..end].copy_from_slice(value);
    page[end] = 0;
    stack_start
        .checked_add(*cursor)
        .ok_or(LoaderBlocker::AddressOverflow)
}

fn push_usize(
    page: &mut [u8; PAGE_SIZE],
    cursor: &mut usize,
    value: usize,
) -> Result<(), LoaderBlocker> {
    let width = core::mem::size_of::<usize>();
    *cursor = cursor
        .checked_sub(width)
        .ok_or(LoaderBlocker::StackLayoutOverflow)?;
    page[*cursor..*cursor + width].copy_from_slice(&value.to_ne_bytes());
    Ok(())
}

#[derive(Clone, Copy)]
struct ElfHeader {
    entry: usize,
    program_header_offset: usize,
    program_header_entry_size: usize,
    program_header_count: usize,
}

impl ElfHeader {
    fn parse(bytes: &[u8], abi: ExecutableAbi) -> Result<Self, LoaderBlocker> {
        if bytes.len() < ELF_HEADER_SIZE || &bytes[0..4] != ELF_MAGIC {
            return Err(LoaderBlocker::MalformedExecutable);
        }
        if bytes[4] != ELF_CLASS_64 || bytes[5] != ELF_DATA_LITTLE {
            return Err(LoaderBlocker::UnsupportedExecutable);
        }
        if bytes[6] != ELF_VERSION_CURRENT as u8 {
            return Err(LoaderBlocker::UnsupportedExecutable);
        }
        if let Some(os_abi) = abi.os_abi() {
            if bytes[7] != os_abi {
                return Err(LoaderBlocker::UnsupportedExecutable);
            }
        }
        let kind = read_u16(bytes, 16)?;
        if kind != ET_EXEC && kind != ET_DYN {
            return Err(LoaderBlocker::UnsupportedExecutable);
        }
        if read_u16(bytes, 18)? != abi.elf_machine() {
            return Err(LoaderBlocker::UnsupportedExecutable);
        }
        if read_u32(bytes, 20)? != ELF_VERSION_CURRENT {
            return Err(LoaderBlocker::UnsupportedExecutable);
        }
        let entry = read_u64(bytes, 24)? as usize;
        UserEntryAddress::new(entry).map_err(LoaderBlocker::UserMemory)?;
        let program_header_offset = read_u64(bytes, 32)? as usize;
        let header_size = read_u16(bytes, 52)? as usize;
        let program_header_entry_size = read_u16(bytes, 54)? as usize;
        let program_header_count = read_u16(bytes, 56)? as usize;
        if header_size != ELF_HEADER_SIZE
            || program_header_entry_size != PROGRAM_HEADER_SIZE
            || program_header_count == 0
        {
            return Err(LoaderBlocker::MalformedExecutable);
        }

        Ok(Self {
            entry,
            program_header_offset,
            program_header_entry_size,
            program_header_count,
        })
    }
}

#[derive(Clone, Copy)]
struct ProgramHeader {
    kind: u32,
    flags: u32,
    file_offset: usize,
    virtual_address: usize,
    file_size: usize,
    memory_size: usize,
    align: usize,
}

impl ProgramHeader {
    fn parse(bytes: &[u8], offset: usize) -> Result<Self, LoaderBlocker> {
        Ok(Self {
            kind: read_u32(bytes, offset)?,
            flags: read_u32(bytes, offset + 4)?,
            file_offset: read_u64(bytes, offset + 8)? as usize,
            virtual_address: read_u64(bytes, offset + 16)? as usize,
            file_size: read_u64(bytes, offset + 32)? as usize,
            memory_size: read_u64(bytes, offset + 40)? as usize,
            align: read_u64(bytes, offset + 48)? as usize,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoaderBlocker {
    AddressOverflow,
    EntrySegmentMissing,
    EntrySpecMismatch,
    KernelRangeViolation,
    LegitimatePayloadMissing,
    MalformedExecutable,
    OverlappingSegments,
    SegmentAlignmentInvalid,
    SegmentTooLarge,
    StackAlignmentInvalid,
    StackLayoutOverflow,
    StackStringContainsNul,
    TooManyLoadSegments,
    UnsupportedExecutable,
    UnsupportedSegmentPermissions,
    UserMemory(UserMemoryBlocker),
}

fn validate_segment_alignment(program: ProgramHeader) -> Result<(), LoaderBlocker> {
    if program.align == 0 || program.align == 1 {
        return Ok(());
    }
    if !program.align.is_power_of_two() {
        return Err(LoaderBlocker::SegmentAlignmentInvalid);
    }
    if program.virtual_address % program.align != program.file_offset % program.align {
        Err(LoaderBlocker::SegmentAlignmentInvalid)
    } else {
        Ok(())
    }
}

fn segment_permissions(flags: u32) -> Result<MappingFlags, LoaderBlocker> {
    let read = flags & PF_R != 0;
    let write = flags & PF_W != 0;
    let execute = flags & PF_X != 0;
    if !read || (write && execute) {
        return Err(LoaderBlocker::UnsupportedSegmentPermissions);
    }
    Ok(MappingFlags::user(read, write, execute))
}

fn read_u16(bytes: &[u8], offset: usize) -> Result<u16, LoaderBlocker> {
    let data = read_array::<2>(bytes, offset)?;
    Ok(u16::from_le_bytes(data))
}

fn read_u32(bytes: &[u8], offset: usize) -> Result<u32, LoaderBlocker> {
    let data = read_array::<4>(bytes, offset)?;
    Ok(u32::from_le_bytes(data))
}

fn read_u64(bytes: &[u8], offset: usize) -> Result<u64, LoaderBlocker> {
    let data = read_array::<8>(bytes, offset)?;
    Ok(u64::from_le_bytes(data))
}

fn read_array<const N: usize>(bytes: &[u8], offset: usize) -> Result<[u8; N], LoaderBlocker> {
    let end = offset
        .checked_add(N)
        .ok_or(LoaderBlocker::MalformedExecutable)?;
    if end > bytes.len() {
        return Err(LoaderBlocker::MalformedExecutable);
    }

    let mut data = [0u8; N];
    data.copy_from_slice(&bytes[offset..end]);
    Ok(data)
}

const fn align_down(value: usize, align: usize) -> usize {
    value / align * align
}

const fn align_up_saturated(value: usize, align: usize) -> usize {
    if value % align == 0 {
        value
    } else {
        align_down(value, align) + align
    }
}

fn align_up(value: usize, align: usize) -> Result<usize, LoaderBlocker> {
    if value % align == 0 {
        Ok(value)
    } else {
        align_down(value, align)
            .checked_add(align)
            .ok_or(LoaderBlocker::AddressOverflow)
    }
}

const fn ranges_overlap(
    first_start: usize,
    first_end: usize,
    second_start: usize,
    second_end: usize,
) -> bool {
    first_start < second_end && second_start < first_end
}

const fn range_overlaps_kernel_globals(
    start: usize,
    end: usize,
    kernel_globals: KernelGlobalMappings,
) -> bool {
    let mapping = kernel_globals.image_mapping();
    let kernel_start = mapping.virt_start();
    let kernel_end = mapping.virt_start() + mapping.byte_len();

    start < kernel_end && kernel_start < end
}
