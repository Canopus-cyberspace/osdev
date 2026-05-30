use crate::arch::contract::{
    Architecture, BootInitBlocker, BootInitPath, BspServices, BspSnapshot, EarlyBootInfo,
    FatalConsole, HardwareReadiness, MmuServices, TrapServices, UserEntryServices,
};
use crate::core::mm::{BootMemory, BootMemoryBlocker, KernelLayout, PhysRange, PAGE_SIZE};

const FDT_MAGIC: u32 = 0xd00d_feed;
const FDT_BEGIN_NODE: u32 = 1;
const FDT_END_NODE: u32 = 2;
const FDT_PROP: u32 = 3;
const FDT_NOP: u32 = 4;
const FDT_END: u32 = 9;
const FDT_HEADER_LEN: usize = 40;
const MAX_FDT_SIZE: usize = 2 * 1024 * 1024;

pub fn early_boot_info(hart_id: usize, device_tree: usize) -> EarlyBootInfo {
    EarlyBootInfo::new(Architecture::Riscv64, hart_id, hart_id, device_tree)
}

pub fn bsp_services(boot: EarlyBootInfo) -> BspServices {
    let snapshot = BspSnapshot::new(
        boot,
        HardwareReadiness::Ready,
        super::trap::readiness(),
        super::timer::readiness(),
        super::mmu::readiness(),
        super::user_entry::readiness(),
        super::block::readiness(),
    );

    BspServices::new(
        snapshot,
        FatalConsole::new(super::console::write_fatal),
        TrapServices::new(super::trap::install_trap_vector),
        MmuServices::new(
            super::mmu::activate_kernel_address_space,
            super::mmu::prepare_user_address_space,
        ),
        UserEntryServices::new(super::user_entry::enter_user),
        crate::arch::contract::BlockServices::new(super::block::read_sector),
        discover_boot_memory,
        discover_boot_init_path,
        super::halt::halt,
    )
}

pub fn discover_boot_memory(boot: EarlyBootInfo, layout: KernelLayout) -> BootMemory {
    let fdt = boot.firmware_arg1();
    if fdt == 0 {
        return BootMemory::NotReady(BootMemoryBlocker::FirmwarePointerMissing);
    }

    match unsafe { discover_fdt_memory(fdt, layout) } {
        Ok(range) => BootMemory::UsableRange(range),
        Err(blocker) => BootMemory::NotReady(blocker),
    }
}

pub fn discover_boot_init_path(boot: EarlyBootInfo) -> Result<BootInitPath, BootInitBlocker> {
    let fdt = boot.firmware_arg1();
    if fdt == 0 {
        return Err(BootInitBlocker::NoBootInitPath);
    }

    unsafe { discover_fdt_boot_init_path(fdt) }
}

unsafe fn discover_fdt_boot_init_path(fdt: usize) -> Result<BootInitPath, BootInitBlocker> {
    let header = FdtHeader::read(fdt).map_err(map_boot_init_blocker)?;
    let bytes = core::slice::from_raw_parts(fdt as *const u8, header.total_size);
    let struct_block = checked_block(bytes, header.struct_offset, header.struct_size)
        .map_err(map_boot_init_blocker)?;
    let strings = checked_block(bytes, header.strings_offset, header.strings_size)
        .map_err(map_boot_init_blocker)?;

    parse_chosen_bootargs(struct_block, strings)
}

fn parse_chosen_bootargs(
    struct_block: &[u8],
    strings: &[u8],
) -> Result<BootInitPath, BootInitBlocker> {
    let mut cursor = 0usize;
    let mut depth = 0usize;
    let mut chosen_depth = 0usize;

    loop {
        let token = read_be32_at(struct_block, cursor).map_err(map_boot_init_blocker)?;
        cursor += 4;

        match token {
            FDT_BEGIN_NODE => {
                let (name, next) =
                    read_node_name(struct_block, cursor).map_err(map_boot_init_blocker)?;
                depth = depth
                    .checked_add(1)
                    .ok_or(BootInitBlocker::FirmwareTableMalformed)?;
                cursor = align_cursor(next, struct_block.len()).map_err(map_boot_init_blocker)?;

                if depth == 2 && name == b"chosen" {
                    chosen_depth = depth;
                }
            }
            FDT_END_NODE => {
                if depth == 0 {
                    return Err(BootInitBlocker::FirmwareTableMalformed);
                }
                if chosen_depth == depth {
                    chosen_depth = 0;
                }
                depth -= 1;
            }
            FDT_PROP => {
                let len =
                    read_be32_at(struct_block, cursor).map_err(map_boot_init_blocker)? as usize;
                let name_offset =
                    read_be32_at(struct_block, cursor + 4).map_err(map_boot_init_blocker)? as usize;
                let value_start = cursor + 8;
                let value_end = value_start
                    .checked_add(len)
                    .ok_or(BootInitBlocker::FirmwareTableMalformed)?;
                if value_end > struct_block.len() {
                    return Err(BootInitBlocker::FirmwareTableMalformed);
                }
                cursor =
                    align_cursor(value_end, struct_block.len()).map_err(map_boot_init_blocker)?;

                let name = string_at(strings, name_offset).map_err(map_boot_init_blocker)?;
                if chosen_depth == depth && name == b"bootargs" {
                    return init_path_from_bootargs(&struct_block[value_start..value_end]);
                }
            }
            FDT_NOP => {}
            FDT_END => break,
            _ => return Err(BootInitBlocker::FirmwareTableMalformed),
        }
    }

    Err(BootInitBlocker::NoBootInitPath)
}

fn init_path_from_bootargs(bootargs: &[u8]) -> Result<BootInitPath, BootInitBlocker> {
    let mut init_path = None;
    let mut cursor = 0usize;
    while cursor < bootargs.len() {
        while cursor < bootargs.len() && is_bootarg_separator(bootargs[cursor]) {
            cursor += 1;
        }
        let start = cursor;
        while cursor < bootargs.len() && !is_bootarg_separator(bootargs[cursor]) {
            cursor += 1;
        }
        if start < cursor && bootargs[start..cursor].starts_with(b"init=") {
            init_path = Some(BootInitPath::new(&bootargs[start + 5..cursor])?);
            break;
        }
    }

    let mut path = match init_path {
        Some(path) => path,
        None => return Err(BootInitBlocker::NoBootInitPath),
    };

    cursor = 0;
    while cursor < bootargs.len() {
        while cursor < bootargs.len() && is_bootarg_separator(bootargs[cursor]) {
            cursor += 1;
        }
        let start = cursor;
        while cursor < bootargs.len() && !is_bootarg_separator(bootargs[cursor]) {
            cursor += 1;
        }
        if start < cursor && bootargs[start..cursor].starts_with(b"init.arg=") {
            path.push_arg(&bootargs[start + 9..cursor])?;
        }
    }

    Ok(path)
}

const fn is_bootarg_separator(byte: u8) -> bool {
    byte == 0 || byte == b' ' || byte == b'\t' || byte == b'\n' || byte == b'\r'
}

const fn map_boot_init_blocker(blocker: BootMemoryBlocker) -> BootInitBlocker {
    match blocker {
        BootMemoryBlocker::FirmwarePointerMissing => BootInitBlocker::NoBootInitPath,
        BootMemoryBlocker::DiscoveryRequired
        | BootMemoryBlocker::FirmwareTableMalformed
        | BootMemoryBlocker::KernelImageOutsideUsableMemory
        | BootMemoryBlocker::MemoryNodeMissing
        | BootMemoryBlocker::NoUsableFramesAfterKernel => BootInitBlocker::FirmwareTableMalformed,
    }
}

unsafe fn discover_fdt_memory(
    fdt: usize,
    layout: KernelLayout,
) -> Result<PhysRange, BootMemoryBlocker> {
    let header = FdtHeader::read(fdt)?;
    let bytes = core::slice::from_raw_parts(fdt as *const u8, header.total_size);
    let struct_block = checked_block(bytes, header.struct_offset, header.struct_size)?;
    let strings = checked_block(bytes, header.strings_offset, header.strings_size)?;

    parse_memory_range(struct_block, strings, layout)
}

fn parse_memory_range(
    struct_block: &[u8],
    strings: &[u8],
    layout: KernelLayout,
) -> Result<PhysRange, BootMemoryBlocker> {
    let mut cursor = 0;
    let mut depth = 0usize;
    let mut address_cells = 2usize;
    let mut size_cells = 1usize;
    let mut memory_node_depth = 0usize;
    let mut saw_memory_node = false;

    loop {
        let token = read_be32_at(struct_block, cursor)?;
        cursor += 4;

        match token {
            FDT_BEGIN_NODE => {
                let (name, next) = read_node_name(struct_block, cursor)?;
                depth = depth
                    .checked_add(1)
                    .ok_or(BootMemoryBlocker::FirmwareTableMalformed)?;
                cursor = align_cursor(next, struct_block.len())?;

                if depth == 2 && is_memory_node_name(name) {
                    saw_memory_node = true;
                    memory_node_depth = depth;
                }
            }
            FDT_END_NODE => {
                if depth == 0 {
                    return Err(BootMemoryBlocker::FirmwareTableMalformed);
                }

                if memory_node_depth == depth {
                    memory_node_depth = 0;
                }

                depth -= 1;
            }
            FDT_PROP => {
                let len = read_be32_at(struct_block, cursor)? as usize;
                let name_offset = read_be32_at(struct_block, cursor + 4)? as usize;
                let value_start = cursor + 8;
                let value_end = value_start
                    .checked_add(len)
                    .ok_or(BootMemoryBlocker::FirmwareTableMalformed)?;
                if value_end > struct_block.len() {
                    return Err(BootMemoryBlocker::FirmwareTableMalformed);
                }
                cursor = align_cursor(value_end, struct_block.len())?;

                let name = string_at(strings, name_offset)?;
                let value = &struct_block[value_start..value_end];

                if depth == 1 {
                    if name == b"#address-cells" {
                        address_cells = read_single_cell(value)?;
                    } else if name == b"#size-cells" {
                        size_cells = read_single_cell(value)?;
                    }
                } else if memory_node_depth == depth && name == b"reg" {
                    if let Some(range) =
                        usable_range_from_reg(value, address_cells, size_cells, layout)?
                    {
                        return Ok(range);
                    }
                }
            }
            FDT_NOP => {}
            FDT_END => {
                break;
            }
            _ => return Err(BootMemoryBlocker::FirmwareTableMalformed),
        }
    }

    if saw_memory_node {
        Err(BootMemoryBlocker::KernelImageOutsideUsableMemory)
    } else {
        Err(BootMemoryBlocker::MemoryNodeMissing)
    }
}

fn usable_range_from_reg(
    reg: &[u8],
    address_cells: usize,
    size_cells: usize,
    layout: KernelLayout,
) -> Result<Option<PhysRange>, BootMemoryBlocker> {
    if address_cells == 0 || address_cells > 2 || size_cells == 0 || size_cells > 2 {
        return Err(BootMemoryBlocker::FirmwareTableMalformed);
    }

    let entry_cells = address_cells
        .checked_add(size_cells)
        .ok_or(BootMemoryBlocker::FirmwareTableMalformed)?;
    let entry_bytes = entry_cells
        .checked_mul(4)
        .ok_or(BootMemoryBlocker::FirmwareTableMalformed)?;
    if entry_bytes == 0 || reg.len() % entry_bytes != 0 {
        return Err(BootMemoryBlocker::FirmwareTableMalformed);
    }

    let mut offset = 0usize;
    while offset < reg.len() {
        let base = read_cells(reg, offset, address_cells)?;
        let size = read_cells(reg, offset + address_cells * 4, size_cells)?;
        let end = base
            .checked_add(size)
            .ok_or(BootMemoryBlocker::FirmwareTableMalformed)?;
        let image = layout.image();

        if base <= image.start() && image.end() <= end {
            let first_free = align_up(image.end(), PAGE_SIZE)?;
            let last_free = align_down(end, PAGE_SIZE);
            if first_free < last_free {
                return Ok(Some(
                    PhysRange::new(first_free, last_free)
                        .map_err(|_| BootMemoryBlocker::FirmwareTableMalformed)?,
                ));
            }

            return Err(BootMemoryBlocker::NoUsableFramesAfterKernel);
        }

        offset += entry_bytes;
    }

    Ok(None)
}

fn read_single_cell(value: &[u8]) -> Result<usize, BootMemoryBlocker> {
    if value.len() != 4 {
        return Err(BootMemoryBlocker::FirmwareTableMalformed);
    }

    Ok(read_be32_at(value, 0)? as usize)
}

fn read_cells(data: &[u8], offset: usize, cells: usize) -> Result<usize, BootMemoryBlocker> {
    let mut value = 0usize;
    let mut cell = 0usize;
    while cell < cells {
        let part = read_be32_at(data, offset + cell * 4)? as usize;
        value = (value << 32) | part;
        cell += 1;
    }

    Ok(value)
}

fn read_node_name(data: &[u8], start: usize) -> Result<(&[u8], usize), BootMemoryBlocker> {
    if start >= data.len() {
        return Err(BootMemoryBlocker::FirmwareTableMalformed);
    }

    let mut end = start;
    while end < data.len() && data[end] != 0 {
        end += 1;
    }

    if end == data.len() {
        return Err(BootMemoryBlocker::FirmwareTableMalformed);
    }

    Ok((&data[start..end], end + 1))
}

fn string_at(strings: &[u8], offset: usize) -> Result<&[u8], BootMemoryBlocker> {
    if offset >= strings.len() {
        return Err(BootMemoryBlocker::FirmwareTableMalformed);
    }

    let mut end = offset;
    while end < strings.len() && strings[end] != 0 {
        end += 1;
    }

    if end == strings.len() {
        return Err(BootMemoryBlocker::FirmwareTableMalformed);
    }

    Ok(&strings[offset..end])
}

fn is_memory_node_name(name: &[u8]) -> bool {
    name == b"memory" || name.starts_with(b"memory@")
}

fn checked_block(bytes: &[u8], offset: usize, size: usize) -> Result<&[u8], BootMemoryBlocker> {
    let end = offset
        .checked_add(size)
        .ok_or(BootMemoryBlocker::FirmwareTableMalformed)?;
    if offset > bytes.len() || end > bytes.len() {
        return Err(BootMemoryBlocker::FirmwareTableMalformed);
    }

    Ok(&bytes[offset..end])
}

fn align_cursor(value: usize, limit: usize) -> Result<usize, BootMemoryBlocker> {
    let aligned = value
        .checked_add(3)
        .ok_or(BootMemoryBlocker::FirmwareTableMalformed)?
        & !3usize;
    if aligned > limit {
        Err(BootMemoryBlocker::FirmwareTableMalformed)
    } else {
        Ok(aligned)
    }
}

fn align_down(value: usize, align: usize) -> usize {
    value / align * align
}

fn align_up(value: usize, align: usize) -> Result<usize, BootMemoryBlocker> {
    if value % align == 0 {
        Ok(value)
    } else {
        align_down(value, align)
            .checked_add(align)
            .ok_or(BootMemoryBlocker::FirmwareTableMalformed)
    }
}

fn read_be32_at(data: &[u8], offset: usize) -> Result<u32, BootMemoryBlocker> {
    let end = offset
        .checked_add(4)
        .ok_or(BootMemoryBlocker::FirmwareTableMalformed)?;
    if end > data.len() {
        return Err(BootMemoryBlocker::FirmwareTableMalformed);
    }

    Ok(u32::from_be_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
    ]))
}

struct FdtHeader {
    total_size: usize,
    struct_offset: usize,
    strings_offset: usize,
    struct_size: usize,
    strings_size: usize,
}

impl FdtHeader {
    unsafe fn read(fdt: usize) -> Result<Self, BootMemoryBlocker> {
        if read_be32_ptr(fdt, 0)? != FDT_MAGIC {
            return Err(BootMemoryBlocker::FirmwareTableMalformed);
        }

        let total_size = read_be32_ptr(fdt, 4)? as usize;
        if !(FDT_HEADER_LEN..=MAX_FDT_SIZE).contains(&total_size) {
            return Err(BootMemoryBlocker::FirmwareTableMalformed);
        }

        let struct_offset = read_be32_ptr(fdt, 8)? as usize;
        let strings_offset = read_be32_ptr(fdt, 12)? as usize;
        let strings_size = read_be32_ptr(fdt, 32)? as usize;
        let struct_size = read_be32_ptr(fdt, 36)? as usize;

        Ok(Self {
            total_size,
            struct_offset,
            strings_offset,
            struct_size,
            strings_size,
        })
    }
}

unsafe fn read_be32_ptr(base: usize, offset: usize) -> Result<u32, BootMemoryBlocker> {
    let address = base
        .checked_add(offset)
        .ok_or(BootMemoryBlocker::FirmwareTableMalformed)?;
    let ptr = address as *const u8;

    Ok(u32::from_be_bytes([
        ptr.read(),
        ptr.add(1).read(),
        ptr.add(2).read(),
        ptr.add(3).read(),
    ]))
}
