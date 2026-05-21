#![allow(dead_code)]

use core::cmp::{max, min};

use crate::console::{write_usize_dec, write_usize_hex};
use crate::early_console_write;
use crate::sdcard_ext4;
use crate::user_mmu;

const PAGE_SIZE: usize = 4096;
const HUGE_PAGE_SIZE: usize = 2 * 1024 * 1024;
const OFFICIAL_ELF_CAP: usize = 3 * 1024 * 1024;
const USER_IMAGE_SIZE: usize = 4 * 1024 * 1024;
const USER_STACK_SIZE: usize = 2 * 1024 * 1024;
const USER_HEAP_SIZE: usize = 2 * 1024 * 1024;
const USER_MMAP_SIZE: usize = 2 * 1024 * 1024;
const MAX_USER_REGIONS: usize = 16;
const MAX_MMAP_SLOTS: usize = 8;
const FIXED_STACK_VA: usize = 0x1_3000_0000;
const FIXED_HEAP_VA: usize = 0x1_3400_0000;
const FIXED_MMAP_VA: usize = 0x1_3800_0000;
pub(crate) const EXEC_ARG_MAX: usize = 8;
pub(crate) const EXEC_ENV_MAX: usize = 8;
pub(crate) const EXEC_STRING_MAX: usize = 128;

const ELF_MAGIC: &[u8; 4] = b"\x7fELF";
const ET_EXEC: u16 = 2;
const ET_DYN: u16 = 3;
const EM_LOONGARCH: u16 = 258;
const PT_LOAD: u32 = 1;

const AT_NULL: usize = 0;
const AT_PHDR: usize = 3;
const AT_PHENT: usize = 4;
const AT_PHNUM: usize = 5;
const AT_PAGESZ: usize = 6;
const AT_BASE: usize = 7;
const AT_ENTRY: usize = 9;

const REGION_IMAGE: u8 = 1;
const REGION_STACK: u8 = 2;
const REGION_HEAP: u8 = 3;
const REGION_MMAP: u8 = 4;

#[repr(C, align(16384))]
struct ElfBytes {
    bytes: [u8; OFFICIAL_ELF_CAP],
}

#[repr(C, align(2097152))]
struct UserImage {
    bytes: [u8; USER_IMAGE_SIZE],
}

#[repr(C, align(2097152))]
struct UserStack {
    bytes: [u8; USER_STACK_SIZE],
}

#[repr(C, align(2097152))]
struct UserHeap {
    bytes: [u8; USER_HEAP_SIZE],
}

#[repr(C, align(2097152))]
struct UserMmap {
    bytes: [u8; USER_MMAP_SIZE],
}

#[derive(Copy, Clone)]
struct UserRegion {
    user_start: usize,
    host_start: usize,
    len: usize,
    kind: u8,
    active: bool,
}

impl UserRegion {
    const fn empty() -> Self {
        Self {
            user_start: 0,
            host_start: 0,
            len: 0,
            kind: 0,
            active: false,
        }
    }

    fn new(kind: u8, user_start: usize, host_start: usize, len: usize) -> Self {
        Self {
            user_start,
            host_start,
            len,
            kind,
            active: true,
        }
    }

    fn translate(&self, ptr: usize) -> Option<(usize, usize)> {
        if !self.active || self.len == 0 {
            return None;
        }
        if ptr >= self.user_start {
            let off = ptr - self.user_start;
            if off < self.len {
                return Some((self.host_start + off, self.len - off));
            }
        }
        if ptr >= self.host_start {
            let off = ptr - self.host_start;
            if off < self.len {
                return Some((self.host_start + off, self.len - off));
            }
        }
        None
    }

    fn contains(&self, ptr: usize, len: usize) -> bool {
        if len == 0 {
            return true;
        }
        match self.translate(ptr) {
            Some((_, available)) => len <= available,
            None => false,
        }
    }
}

#[derive(Copy, Clone)]
struct MmapSlot {
    offset: usize,
    len: usize,
    active: bool,
}

impl MmapSlot {
    const fn empty() -> Self {
        Self {
            offset: 0,
            len: 0,
            active: false,
        }
    }
}

pub struct RealElfLoad {
    pub inode: u32,
    pub mode: u16,
    pub file_size: usize,
    pub phnum: usize,
    pub entry: usize,
    pub stack_pointer: usize,
    pub load_base: usize,
    pub load_size: usize,
    pub load_segments: usize,
}

#[derive(Copy, Clone)]
pub(crate) struct ExecString {
    bytes: [u8; EXEC_STRING_MAX],
    len: usize,
}

impl ExecString {
    pub(crate) const fn empty() -> Self {
        Self {
            bytes: [0; EXEC_STRING_MAX],
            len: 0,
        }
    }

    pub(crate) fn set_from_slice(&mut self, src: &[u8]) -> Result<(), &'static str> {
        if src.len() >= EXEC_STRING_MAX {
            return Err("exec_string_long");
        }
        self.len = src.len();
        let mut i = 0usize;
        while i < src.len() {
            self.bytes[i] = src[i];
            i += 1;
        }
        self.bytes[self.len] = 0;
        while i + 1 < EXEC_STRING_MAX {
            i += 1;
            self.bytes[i] = 0;
        }
        Ok(())
    }

    fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

#[derive(Copy, Clone)]
struct ElfHeader {
    e_type: u16,
    entry: usize,
    phoff: usize,
    phentsize: usize,
    phnum: usize,
}

#[derive(Copy, Clone)]
struct ProgramHeader {
    p_type: u32,
    offset: usize,
    vaddr: usize,
    filesz: usize,
    memsz: usize,
}

static mut OFFICIAL_ELF: ElfBytes = ElfBytes {
    bytes: [0; OFFICIAL_ELF_CAP],
};
static mut USER_IMAGE: UserImage = UserImage {
    bytes: [0; USER_IMAGE_SIZE],
};
static mut USER_STACK: UserStack = UserStack {
    bytes: [0; USER_STACK_SIZE],
};
static mut USER_HEAP: UserHeap = UserHeap {
    bytes: [0; USER_HEAP_SIZE],
};
static mut USER_MMAP: UserMmap = UserMmap {
    bytes: [0; USER_MMAP_SIZE],
};
static mut USER_IMAGE_BACKUP: UserImage = UserImage {
    bytes: [0; USER_IMAGE_SIZE],
};
static mut USER_STACK_BACKUP: UserStack = UserStack {
    bytes: [0; USER_STACK_SIZE],
};
static mut USER_HEAP_BACKUP: UserHeap = UserHeap {
    bytes: [0; USER_HEAP_SIZE],
};
static mut USER_MMAP_BACKUP: UserMmap = UserMmap {
    bytes: [0; USER_MMAP_SIZE],
};
static mut USER_IMAGE_EXEC_BACKUP: UserImage = UserImage {
    bytes: [0; USER_IMAGE_SIZE],
};
static mut USER_STACK_EXEC_BACKUP: UserStack = UserStack {
    bytes: [0; USER_STACK_SIZE],
};
static mut USER_HEAP_EXEC_BACKUP: UserHeap = UserHeap {
    bytes: [0; USER_HEAP_SIZE],
};
static mut USER_MMAP_EXEC_BACKUP: UserMmap = UserMmap {
    bytes: [0; USER_MMAP_SIZE],
};
static mut USER_REGIONS: [UserRegion; MAX_USER_REGIONS] = [UserRegion::empty(); MAX_USER_REGIONS];
static mut USER_BRK: usize = 0;
static mut USER_HEAP_USER_START: usize = 0;
static mut USER_MMAP_USER_START: usize = 0;
static mut CURRENT_LOAD_USER_START: usize = 0;
static mut CURRENT_LOAD_HOST_START: usize = 0;
static mut CURRENT_LOAD_SIZE: usize = 0;
static mut CURRENT_ENTRY: usize = 0;
static mut USER_MMAP_SLOTS: [MmapSlot; MAX_MMAP_SLOTS] = [MmapSlot::empty(); MAX_MMAP_SLOTS];
static mut USER_MMAP_NEXT: usize = 0;
static mut USER_REGIONS_BACKUP: [UserRegion; MAX_USER_REGIONS] =
    [UserRegion::empty(); MAX_USER_REGIONS];
static mut USER_BRK_BACKUP: usize = 0;
static mut USER_HEAP_USER_START_BACKUP: usize = 0;
static mut USER_MMAP_USER_START_BACKUP: usize = 0;
static mut CURRENT_LOAD_USER_START_BACKUP: usize = 0;
static mut CURRENT_LOAD_HOST_START_BACKUP: usize = 0;
static mut CURRENT_LOAD_SIZE_BACKUP: usize = 0;
static mut CURRENT_ENTRY_BACKUP: usize = 0;
static mut USER_MMAP_SLOTS_BACKUP: [MmapSlot; MAX_MMAP_SLOTS] =
    [MmapSlot::empty(); MAX_MMAP_SLOTS];
static mut USER_MMAP_NEXT_BACKUP: usize = 0;
static mut USER_REGIONS_EXEC_BACKUP: [UserRegion; MAX_USER_REGIONS] =
    [UserRegion::empty(); MAX_USER_REGIONS];
static mut USER_BRK_EXEC_BACKUP: usize = 0;
static mut USER_HEAP_USER_START_EXEC_BACKUP: usize = 0;
static mut USER_MMAP_USER_START_EXEC_BACKUP: usize = 0;
static mut CURRENT_LOAD_USER_START_EXEC_BACKUP: usize = 0;
static mut CURRENT_LOAD_HOST_START_EXEC_BACKUP: usize = 0;
static mut CURRENT_LOAD_SIZE_EXEC_BACKUP: usize = 0;
static mut CURRENT_ENTRY_EXEC_BACKUP: usize = 0;
static mut USER_MMAP_SLOTS_EXEC_BACKUP: [MmapSlot; MAX_MMAP_SLOTS] =
    [MmapSlot::empty(); MAX_MMAP_SLOTS];
static mut USER_MMAP_NEXT_EXEC_BACKUP: usize = 0;

pub fn load_basic_write() -> Result<RealElfLoad, &'static str> {
    load_basic_case("/musl/basic/write")
}

pub fn load_basic_case(path: &str) -> Result<RealElfLoad, &'static str> {
    load_basic_case_with_args(path, &[], &[])
}

pub(crate) fn load_basic_case_with_args(
    path: &str,
    argv: &[ExecString],
    envp: &[ExecString],
) -> Result<RealElfLoad, &'static str> {
    load_user_elf_with_args(path, argv, envp)
}

pub(crate) fn load_user_elf_with_args(
    path: &str,
    argv: &[ExecString],
    envp: &[ExecString],
) -> Result<RealElfLoad, &'static str> {
    unsafe {
        let elf = elf_buf_mut();
        let file = sdcard_ext4::load_path(path, elf)?;
        let header = parse_header(&elf[..file.size])?;
        let image = user_image_mut();
        zero_bytes(image);
        zero_bytes(user_stack_mut());
        zero_bytes(user_heap_mut());
        zero_bytes(user_mmap_mut());
        reset_regions();
        reset_mmap_slots();

        let mut min_page = usize::MAX;
        let mut max_end = 0usize;
        let mut load_segments = 0usize;
        let mut i = 0usize;
        while i < header.phnum {
            let ph = parse_program_header(&elf[..file.size], header, i)?;
            if ph.p_type == PT_LOAD {
                if ph.filesz > ph.memsz {
                    return Err("elf_load_filesz");
                }
                min_page = min(min_page, ph.vaddr & !(PAGE_SIZE - 1));
                max_end = max(max_end, ph.vaddr.checked_add(ph.memsz).ok_or("elf_memsz")?);
                load_segments += 1;
            }
            i += 1;
        }
        if load_segments == 0 || min_page == usize::MAX || max_end <= min_page {
            return Err("elf_no_load");
        }
        let load_size = max_end - min_page;
        if load_size > USER_IMAGE_SIZE {
            return Err("elf_user_image_size");
        }
        if header.entry < min_page || header.entry >= max_end {
            return Err("elf_entry_bounds");
        }

        i = 0;
        while i < header.phnum {
            let ph = parse_program_header(&elf[..file.size], header, i)?;
            if ph.p_type == PT_LOAD {
                let seg_page = ph.vaddr & !(PAGE_SIZE - 1);
                let page_offset = ph.vaddr - seg_page;
                let dst = (seg_page - min_page)
                    .checked_add(page_offset)
                    .ok_or("elf_dst")?;
                let dst_end = dst.checked_add(ph.filesz).ok_or("elf_dst")?;
                let src_end = ph.offset.checked_add(ph.filesz).ok_or("elf_src")?;
                if dst_end > USER_IMAGE_SIZE || src_end > file.size {
                    return Err("elf_segment_bounds");
                }
                copy_bytes(&mut image[dst..dst_end], &elf[ph.offset..src_end]);
                let region_len = align_up(page_offset + ph.memsz, PAGE_SIZE);
                let user_start = if header.e_type == ET_EXEC {
                    seg_page
                } else {
                    image.as_ptr() as usize + (seg_page - min_page)
                };
                let host_start = image.as_ptr() as usize + (seg_page - min_page);
                add_region(REGION_IMAGE, user_start, host_start, region_len)?;
            }
            i += 1;
        }

        let image_base = image.as_ptr() as usize;
        let runtime_bias = image_base.wrapping_sub(min_page);
        let fixed_va = header.e_type == ET_EXEC;
        let entry = if fixed_va {
            header.entry
        } else {
            runtime_bias + header.entry
        };
        let phdr = if fixed_va {
            min_page + header.phoff
        } else {
            runtime_bias + header.phoff
        };
        let stack_user_start = if fixed_va {
            FIXED_STACK_VA
        } else {
            user_stack_mut().as_ptr() as usize
        };
        let stack_pointer =
            build_stack(entry, phdr, header, stack_user_start, path.as_bytes(), argv, envp)?;

        let stack = user_stack_mut();
        let stack_start = stack.as_ptr() as usize;
        let stack_user_end = stack_user_start + USER_STACK_SIZE;
        let heap_start = user_heap_mut().as_ptr() as usize;
        let heap_user_start = if fixed_va { FIXED_HEAP_VA } else { heap_start };
        let mmap_start = user_mmap_mut().as_ptr() as usize;
        let mmap_user_start = if fixed_va { FIXED_MMAP_VA } else { mmap_start };
        add_region(
            REGION_STACK,
            stack_user_start,
            stack_start,
            stack_user_end - stack_user_start,
        )?;
        add_region(REGION_HEAP, heap_user_start, heap_start, 0)?;
        USER_BRK = heap_user_start;
        USER_HEAP_USER_START = heap_user_start;
        USER_MMAP_USER_START = mmap_user_start;
        CURRENT_LOAD_USER_START = if fixed_va { min_page } else { image_base };
        CURRENT_LOAD_HOST_START = image_base;
        CURRENT_LOAD_SIZE = load_size;
        CURRENT_ENTRY = entry;

        Ok(RealElfLoad {
            inode: file.inode,
            mode: file.mode,
            file_size: file.size,
            phnum: header.phnum,
            entry,
            stack_pointer,
            load_base: if header.e_type == ET_EXEC {
                min_page
            } else {
                image_base
            },
            load_size,
            load_segments,
        })
    }
}

pub fn user_range_valid(ptr: usize, len: usize) -> bool {
    if len == 0 {
        return true;
    }
    unsafe {
        range_valid_current(ptr, len)
    }
}

pub fn has_loaded_user_elf() -> bool {
    unsafe {
        let mut i = 0usize;
        while i < MAX_USER_REGIONS {
            if USER_REGIONS[i].active && USER_REGIONS[i].kind == REGION_IMAGE {
                return true;
            }
            i += 1;
        }
        false
    }
}

pub(crate) fn activate_current_user_mmu() -> Result<(), &'static str> {
    unsafe {
        if CURRENT_LOAD_USER_START == 0 || CURRENT_LOAD_HOST_START == 0 || CURRENT_LOAD_SIZE == 0 {
            return Err("mmu_no_image");
        }
        user_mmu::begin_mapping_install();
        user_mmu::map_huge_range(
            CURRENT_LOAD_USER_START,
            CURRENT_LOAD_HOST_START,
            CURRENT_LOAD_SIZE,
        )?;
        user_mmu::map_huge_range(
            FIXED_STACK_VA,
            core::ptr::addr_of!(USER_STACK.bytes) as usize,
            USER_STACK_SIZE,
        )?;
        user_mmu::map_huge_range(
            FIXED_HEAP_VA,
            core::ptr::addr_of!(USER_HEAP.bytes) as usize,
            USER_HEAP_SIZE,
        )?;
        user_mmu::map_huge_range(
            FIXED_MMAP_VA,
            core::ptr::addr_of!(USER_MMAP.bytes) as usize,
            USER_MMAP_SIZE,
        )?;
        user_mmu::activate_paged_mode();
        Ok(())
    }
}

pub(crate) fn deactivate_current_user_mmu() {
    user_mmu::deactivate_paged_mode();
}

pub(crate) fn current_entry() -> usize {
    unsafe { CURRENT_ENTRY }
}

pub(crate) fn dump_user_regions(prefix: &str) {
    unsafe {
        early_console_write(prefix);
        early_console_write("entry=");
        write_usize_hex(CURRENT_ENTRY);
        early_console_write(" regions:\n");
        let mut i = 0usize;
        while i < MAX_USER_REGIONS {
            let r = USER_REGIONS[i];
            if r.active {
                early_console_write(prefix);
                early_console_write(" region kind=");
                write_usize_dec(r.kind as usize);
                early_console_write(" user=");
                write_usize_hex(r.user_start);
                early_console_write(" host=");
                write_usize_hex(r.host_start);
                early_console_write(" len=");
                write_usize_hex(r.len);
                early_console_write("\n");
            }
            i += 1;
        }
    }
}

pub(crate) fn save_user_snapshot() {
    unsafe {
        copy_bytes(user_image_backup_mut(), user_image_mut());
        copy_bytes(user_stack_backup_mut(), user_stack_mut());
        copy_bytes(user_heap_backup_mut(), user_heap_mut());
        copy_bytes(user_mmap_backup_mut(), user_mmap_mut());
        USER_REGIONS_BACKUP = USER_REGIONS;
        USER_BRK_BACKUP = USER_BRK;
        USER_HEAP_USER_START_BACKUP = USER_HEAP_USER_START;
        USER_MMAP_USER_START_BACKUP = USER_MMAP_USER_START;
        CURRENT_LOAD_USER_START_BACKUP = CURRENT_LOAD_USER_START;
        CURRENT_LOAD_HOST_START_BACKUP = CURRENT_LOAD_HOST_START;
        CURRENT_LOAD_SIZE_BACKUP = CURRENT_LOAD_SIZE;
        CURRENT_ENTRY_BACKUP = CURRENT_ENTRY;
        USER_MMAP_SLOTS_BACKUP = USER_MMAP_SLOTS;
        USER_MMAP_NEXT_BACKUP = USER_MMAP_NEXT;
    }
}

pub(crate) fn restore_user_snapshot() {
    unsafe {
        copy_bytes(user_image_mut(), user_image_backup_mut());
        copy_bytes(user_stack_mut(), user_stack_backup_mut());
        copy_bytes(user_heap_mut(), user_heap_backup_mut());
        copy_bytes(user_mmap_mut(), user_mmap_backup_mut());
        USER_REGIONS = USER_REGIONS_BACKUP;
        USER_BRK = USER_BRK_BACKUP;
        USER_HEAP_USER_START = USER_HEAP_USER_START_BACKUP;
        USER_MMAP_USER_START = USER_MMAP_USER_START_BACKUP;
        CURRENT_LOAD_USER_START = CURRENT_LOAD_USER_START_BACKUP;
        CURRENT_LOAD_HOST_START = CURRENT_LOAD_HOST_START_BACKUP;
        CURRENT_LOAD_SIZE = CURRENT_LOAD_SIZE_BACKUP;
        CURRENT_ENTRY = CURRENT_ENTRY_BACKUP;
        USER_MMAP_SLOTS = USER_MMAP_SLOTS_BACKUP;
        USER_MMAP_NEXT = USER_MMAP_NEXT_BACKUP;
    }
}

pub(crate) fn save_exec_snapshot() {
    unsafe {
        copy_bytes(user_image_exec_backup_mut(), user_image_mut());
        copy_bytes(user_stack_exec_backup_mut(), user_stack_mut());
        copy_bytes(user_heap_exec_backup_mut(), user_heap_mut());
        copy_bytes(user_mmap_exec_backup_mut(), user_mmap_mut());
        USER_REGIONS_EXEC_BACKUP = USER_REGIONS;
        USER_BRK_EXEC_BACKUP = USER_BRK;
        USER_HEAP_USER_START_EXEC_BACKUP = USER_HEAP_USER_START;
        USER_MMAP_USER_START_EXEC_BACKUP = USER_MMAP_USER_START;
        CURRENT_LOAD_USER_START_EXEC_BACKUP = CURRENT_LOAD_USER_START;
        CURRENT_LOAD_HOST_START_EXEC_BACKUP = CURRENT_LOAD_HOST_START;
        CURRENT_LOAD_SIZE_EXEC_BACKUP = CURRENT_LOAD_SIZE;
        CURRENT_ENTRY_EXEC_BACKUP = CURRENT_ENTRY;
        USER_MMAP_SLOTS_EXEC_BACKUP = USER_MMAP_SLOTS;
        USER_MMAP_NEXT_EXEC_BACKUP = USER_MMAP_NEXT;
    }
}

pub(crate) fn restore_exec_snapshot() {
    unsafe {
        copy_bytes(user_image_mut(), user_image_exec_backup_mut());
        copy_bytes(user_stack_mut(), user_stack_exec_backup_mut());
        copy_bytes(user_heap_mut(), user_heap_exec_backup_mut());
        copy_bytes(user_mmap_mut(), user_mmap_exec_backup_mut());
        USER_REGIONS = USER_REGIONS_EXEC_BACKUP;
        USER_BRK = USER_BRK_EXEC_BACKUP;
        USER_HEAP_USER_START = USER_HEAP_USER_START_EXEC_BACKUP;
        USER_MMAP_USER_START = USER_MMAP_USER_START_EXEC_BACKUP;
        CURRENT_LOAD_USER_START = CURRENT_LOAD_USER_START_EXEC_BACKUP;
        CURRENT_LOAD_HOST_START = CURRENT_LOAD_HOST_START_EXEC_BACKUP;
        CURRENT_LOAD_SIZE = CURRENT_LOAD_SIZE_EXEC_BACKUP;
        CURRENT_ENTRY = CURRENT_ENTRY_EXEC_BACKUP;
        USER_MMAP_SLOTS = USER_MMAP_SLOTS_EXEC_BACKUP;
        USER_MMAP_NEXT = USER_MMAP_NEXT_EXEC_BACKUP;
    }
}

pub fn sys_brk(addr: usize) -> isize {
    unsafe {
        let heap_host_start = core::ptr::addr_of!(USER_HEAP.bytes) as usize;
        let heap_start = if USER_HEAP_USER_START == 0 {
            heap_host_start
        } else {
            USER_HEAP_USER_START
        };
        let heap_end = heap_start + USER_HEAP_SIZE;
        if USER_BRK == 0 {
            USER_BRK = heap_start;
        }
        if addr == 0 {
            return USER_BRK as isize;
        }
        let mut requested = addr;
        if requested < heap_start || requested > heap_end {
            requested = (heap_start & !0xffff_ffffusize) | (addr & 0xffff_ffffusize);
        }
        if requested >= heap_start && requested <= heap_end {
            USER_BRK = requested;
            set_region(REGION_HEAP, heap_start, heap_host_start, USER_BRK - heap_start);
        }
        USER_BRK as isize
    }
}

pub fn sys_mmap(
    addr: usize,
    len: usize,
    prot: usize,
    flags: usize,
    file_bytes: Option<&[u8]>,
) -> isize {
    let _ = (addr, prot, flags);
    if len == 0 {
        return -22;
    }
    unsafe {
        let aligned_len = align_up(len, PAGE_SIZE);
        if aligned_len > USER_MMAP_SIZE {
            return -12;
        }
        let slot = match alloc_mmap_slot(aligned_len) {
            Some(slot) => slot,
            None => return -12,
        };
        let mmap_host_base = core::ptr::addr_of!(USER_MMAP.bytes) as usize;
        let mmap_user_base = if USER_MMAP_USER_START == 0 {
            mmap_host_base
        } else {
            USER_MMAP_USER_START
        };
        let mmap_start = mmap_user_base + USER_MMAP_SLOTS[slot].offset;
        let mmap_host_start = mmap_host_base + USER_MMAP_SLOTS[slot].offset;
        let mmap = user_mmap_mut();
        zero_bytes(&mut mmap[USER_MMAP_SLOTS[slot].offset..USER_MMAP_SLOTS[slot].offset + aligned_len]);
        if let Some(src) = file_bytes {
            let take = min(src.len(), len);
            let dst = USER_MMAP_SLOTS[slot].offset;
            copy_bytes(&mut mmap[dst..dst + take], &src[..take]);
        }
        if add_region(REGION_MMAP, mmap_start, mmap_host_start, aligned_len).is_err() {
            USER_MMAP_SLOTS[slot] = MmapSlot::empty();
            return -12;
        }
        mmap_start as isize
    }
}

pub fn sys_munmap(addr: usize, len: usize) -> isize {
    if len == 0 {
        return -22;
    }
    unsafe {
        let aligned_len = align_up(len, PAGE_SIZE);
        let mmap_host_base = core::ptr::addr_of!(USER_MMAP.bytes) as usize;
        let mmap_base = if USER_MMAP_USER_START == 0 {
            mmap_host_base
        } else {
            USER_MMAP_USER_START
        };
        let mut i = 0usize;
        while i < MAX_MMAP_SLOTS {
            if USER_MMAP_SLOTS[i].active
                && addr == mmap_base + USER_MMAP_SLOTS[i].offset
                && aligned_len == USER_MMAP_SLOTS[i].len
            {
                USER_MMAP_SLOTS[i] = MmapSlot::empty();
                remove_region(REGION_MMAP, addr, aligned_len);
                return 0;
            }
            i += 1;
        }
        -22
    }
}

pub fn write_user_usize_pair(ptr: usize, first: usize, second: usize) -> Result<(), &'static str> {
    let mut bytes = [0u8; 16];
    write_usize_le(&mut bytes, 0, first);
    write_usize_le(&mut bytes, 8, second);
    copy_to_user(ptr, &bytes)
}

pub fn write_user_usize(ptr: usize, value: usize) -> Result<(), &'static str> {
    let mut bytes = [0u8; 8];
    write_usize_le(&mut bytes, 0, value);
    copy_to_user(ptr, &bytes)
}

pub fn copy_to_user(ptr: usize, src: &[u8]) -> Result<(), &'static str> {
    copy_user_bytes(ptr, src, true)
}

pub fn copy_from_user(ptr: usize, dst: &mut [u8]) -> Result<(), &'static str> {
    copy_from_user_bytes(ptr, dst)
}

pub fn read_user_cstr(ptr: usize, out: &mut [u8]) -> Result<usize, &'static str> {
    if out.is_empty() {
        return Err("user_cstr_empty");
    }
    let mut i = 0usize;
    while i + 1 < out.len() {
        let mut byte = [0u8; 1];
        copy_from_user(ptr + i, &mut byte)?;
        if byte[0] == 0 {
            out[i] = 0;
            return Ok(i);
        }
        out[i] = byte[0];
        i += 1;
    }
    Err("user_cstr_long")
}

fn copy_user_bytes(ptr: usize, src: &[u8], to_user: bool) -> Result<(), &'static str> {
    if src.is_empty() {
        return Ok(());
    }
    let mut copied = 0usize;
    while copied < src.len() {
        let cur = ptr.checked_add(copied).ok_or("user_copy_overflow")?;
        let (host, avail) = translate_user_chunk(cur).ok_or("user_copy_range")?;
        let take = min(src.len() - copied, avail);
        unsafe {
            let dst = core::slice::from_raw_parts_mut(host as *mut u8, take);
            if to_user {
                copy_bytes(dst, &src[copied..copied + take]);
            }
        }
        copied += take;
    }
    Ok(())
}

fn copy_from_user_bytes(ptr: usize, dst: &mut [u8]) -> Result<(), &'static str> {
    if dst.is_empty() {
        return Ok(());
    }
    let mut copied = 0usize;
    while copied < dst.len() {
        let cur = ptr.checked_add(copied).ok_or("user_copy_overflow")?;
        let (host, avail) = translate_user_chunk(cur).ok_or("user_copy_range")?;
        let take = min(dst.len() - copied, avail);
        unsafe {
            let src = core::slice::from_raw_parts(host as *const u8, take);
            copy_bytes(&mut dst[copied..copied + take], src);
        }
        copied += take;
    }
    Ok(())
}

unsafe fn reset_regions() {
    let mut i = 0usize;
    while i < MAX_USER_REGIONS {
        USER_REGIONS[i] = UserRegion::empty();
        i += 1;
    }
}

unsafe fn reset_mmap_slots() {
    let mut i = 0usize;
    while i < MAX_MMAP_SLOTS {
        USER_MMAP_SLOTS[i] = MmapSlot::empty();
        i += 1;
    }
    USER_MMAP_NEXT = 0;
}

unsafe fn add_region(
    kind: u8,
    user_start: usize,
    host_start: usize,
    len: usize,
) -> Result<(), &'static str> {
    let mut i = 0usize;
    while i < MAX_USER_REGIONS {
        if !USER_REGIONS[i].active {
            USER_REGIONS[i] = UserRegion::new(kind, user_start, host_start, len);
            return Ok(());
        }
        i += 1;
    }
    Err("user_region_full")
}

unsafe fn set_region(kind: u8, user_start: usize, host_start: usize, len: usize) {
    let mut i = 0usize;
    while i < MAX_USER_REGIONS {
        if USER_REGIONS[i].active && USER_REGIONS[i].kind == kind {
            USER_REGIONS[i] = UserRegion::new(kind, user_start, host_start, len);
            return;
        }
        i += 1;
    }
    let _ = add_region(kind, user_start, host_start, len);
}

unsafe fn remove_region(kind: u8, user_start: usize, len: usize) {
    let mut i = 0usize;
    while i < MAX_USER_REGIONS {
        if USER_REGIONS[i].active
            && USER_REGIONS[i].kind == kind
            && USER_REGIONS[i].user_start == user_start
            && USER_REGIONS[i].len == len
        {
            USER_REGIONS[i] = UserRegion::empty();
            return;
        }
        i += 1;
    }
}

unsafe fn alloc_mmap_slot(len: usize) -> Option<usize> {
    let aligned_len = align_up(len, PAGE_SIZE);
    if aligned_len == 0 || aligned_len > USER_MMAP_SIZE {
        return None;
    }
    let mut i = 0usize;
    while i < MAX_MMAP_SLOTS {
        if !USER_MMAP_SLOTS[i].active {
            let mut offset = align_up(USER_MMAP_NEXT, PAGE_SIZE);
            if offset + aligned_len > USER_MMAP_SIZE {
                offset = 0;
            }
            if mmap_space_free(offset, aligned_len) {
                USER_MMAP_SLOTS[i] = MmapSlot {
                    offset,
                    len: aligned_len,
                    active: true,
                };
                USER_MMAP_NEXT = offset + aligned_len;
                return Some(i);
            }
            let mut probe = 0usize;
            while probe + aligned_len <= USER_MMAP_SIZE {
                if mmap_space_free(probe, aligned_len) {
                    USER_MMAP_SLOTS[i] = MmapSlot {
                        offset: probe,
                        len: aligned_len,
                        active: true,
                    };
                    USER_MMAP_NEXT = probe + aligned_len;
                    return Some(i);
                }
                probe += PAGE_SIZE;
            }
            return None;
        }
        i += 1;
    }
    None
}

unsafe fn mmap_space_free(offset: usize, len: usize) -> bool {
    let end = match offset.checked_add(len) {
        Some(end) => end,
        None => return false,
    };
    if end > USER_MMAP_SIZE {
        return false;
    }
    let mut i = 0usize;
    while i < MAX_MMAP_SLOTS {
        if USER_MMAP_SLOTS[i].active {
            let other_start = USER_MMAP_SLOTS[i].offset;
            let other_end = other_start + USER_MMAP_SLOTS[i].len;
            if offset < other_end && end > other_start {
                return false;
            }
        }
        i += 1;
    }
    true
}

unsafe fn range_valid_current(ptr: usize, len: usize) -> bool {
    if len == 0 {
        return true;
    }
    let mut checked = 0usize;
    while checked < len {
        let cur = match ptr.checked_add(checked) {
            Some(cur) => cur,
            None => return false,
        };
        let mut found = false;
        let mut i = 0usize;
        while i < MAX_USER_REGIONS {
            if let Some((_, avail)) = USER_REGIONS[i].translate(cur) {
                if avail == 0 {
                    return false;
                }
                checked += min(avail, len - checked);
                found = true;
                break;
            }
            i += 1;
        }
        if !found {
            return false;
        }
    }
    true
}

fn translate_user_chunk(ptr: usize) -> Option<(usize, usize)> {
    unsafe {
        let mut i = 0usize;
        while i < MAX_USER_REGIONS {
            if let Some(chunk) = USER_REGIONS[i].translate(ptr) {
                return Some(chunk);
            }
            i += 1;
        }
    }
    None
}

fn write_usize_le(dst: &mut [u8], off: usize, value: usize) {
    let bytes = (value as u64).to_le_bytes();
    let mut i = 0usize;
    while i < 8 {
        dst[off + i] = bytes[i];
        i += 1;
    }
}

unsafe fn elf_buf_mut() -> &'static mut [u8] {
    core::slice::from_raw_parts_mut(
        core::ptr::addr_of_mut!(OFFICIAL_ELF.bytes) as *mut u8,
        OFFICIAL_ELF_CAP,
    )
}

unsafe fn user_image_mut() -> &'static mut [u8] {
    core::slice::from_raw_parts_mut(
        core::ptr::addr_of_mut!(USER_IMAGE.bytes) as *mut u8,
        USER_IMAGE_SIZE,
    )
}

unsafe fn user_stack_mut() -> &'static mut [u8] {
    core::slice::from_raw_parts_mut(
        core::ptr::addr_of_mut!(USER_STACK.bytes) as *mut u8,
        USER_STACK_SIZE,
    )
}

unsafe fn user_heap_mut() -> &'static mut [u8] {
    core::slice::from_raw_parts_mut(
        core::ptr::addr_of_mut!(USER_HEAP.bytes) as *mut u8,
        USER_HEAP_SIZE,
    )
}

unsafe fn user_mmap_mut() -> &'static mut [u8] {
    core::slice::from_raw_parts_mut(
        core::ptr::addr_of_mut!(USER_MMAP.bytes) as *mut u8,
        USER_MMAP_SIZE,
    )
}

unsafe fn user_image_backup_mut() -> &'static mut [u8] {
    core::slice::from_raw_parts_mut(
        core::ptr::addr_of_mut!(USER_IMAGE_BACKUP.bytes) as *mut u8,
        USER_IMAGE_SIZE,
    )
}

unsafe fn user_stack_backup_mut() -> &'static mut [u8] {
    core::slice::from_raw_parts_mut(
        core::ptr::addr_of_mut!(USER_STACK_BACKUP.bytes) as *mut u8,
        USER_STACK_SIZE,
    )
}

unsafe fn user_heap_backup_mut() -> &'static mut [u8] {
    core::slice::from_raw_parts_mut(
        core::ptr::addr_of_mut!(USER_HEAP_BACKUP.bytes) as *mut u8,
        USER_HEAP_SIZE,
    )
}

unsafe fn user_mmap_backup_mut() -> &'static mut [u8] {
    core::slice::from_raw_parts_mut(
        core::ptr::addr_of_mut!(USER_MMAP_BACKUP.bytes) as *mut u8,
        USER_MMAP_SIZE,
    )
}

unsafe fn user_image_exec_backup_mut() -> &'static mut [u8] {
    core::slice::from_raw_parts_mut(
        core::ptr::addr_of_mut!(USER_IMAGE_EXEC_BACKUP.bytes) as *mut u8,
        USER_IMAGE_SIZE,
    )
}

unsafe fn user_stack_exec_backup_mut() -> &'static mut [u8] {
    core::slice::from_raw_parts_mut(
        core::ptr::addr_of_mut!(USER_STACK_EXEC_BACKUP.bytes) as *mut u8,
        USER_STACK_SIZE,
    )
}

unsafe fn user_heap_exec_backup_mut() -> &'static mut [u8] {
    core::slice::from_raw_parts_mut(
        core::ptr::addr_of_mut!(USER_HEAP_EXEC_BACKUP.bytes) as *mut u8,
        USER_HEAP_SIZE,
    )
}

unsafe fn user_mmap_exec_backup_mut() -> &'static mut [u8] {
    core::slice::from_raw_parts_mut(
        core::ptr::addr_of_mut!(USER_MMAP_EXEC_BACKUP.bytes) as *mut u8,
        USER_MMAP_SIZE,
    )
}

unsafe fn build_stack(
    entry: usize,
    phdr: usize,
    header: ElfHeader,
    stack_user_base: usize,
    argv0: &[u8],
    argv: &[ExecString],
    envp: &[ExecString],
) -> Result<usize, &'static str> {
    let stack = user_stack_mut();
    zero_bytes(stack);
    let mut sp = USER_STACK_SIZE;
    let mut env_ptrs = [0usize; EXEC_ENV_MAX + 2];
    let envc = if envp.is_empty() {
        env_ptrs[0] =
            stack_copy_down(stack, stack_user_base, &mut sp, b"PATH=/musl:/bin:/usr/bin:.")?;
        env_ptrs[1] = stack_copy_down(stack, stack_user_base, &mut sp, b"HOME=/musl")?;
        2
    } else {
        if envp.len() > EXEC_ENV_MAX {
            return Err("user_stack_envc");
        }
        let mut i = 0usize;
        while i < envp.len() {
            env_ptrs[i] = stack_copy_down(stack, stack_user_base, &mut sp, envp[i].as_bytes())?;
            i += 1;
        }
        envp.len()
    };
    let mut arg_ptrs = [0usize; EXEC_ARG_MAX + 1];
    let argc = if argv.is_empty() {
        arg_ptrs[0] = stack_copy_down(stack, stack_user_base, &mut sp, argv0)?;
        1
    } else {
        if argv.len() > EXEC_ARG_MAX {
            return Err("user_stack_argc");
        }
        let mut i = 0usize;
        while i < argv.len() {
            arg_ptrs[i] = stack_copy_down(stack, stack_user_base, &mut sp, argv[i].as_bytes())?;
            i += 1;
        }
        argv.len()
    };
    sp &= !15usize;

    let words = 1 + argc + 1 + envc + 1 + 14;
    let bytes = align_up(words * core::mem::size_of::<usize>(), 16);
    if sp < bytes {
        return Err("user_stack_bounds");
    }
    sp -= bytes;
    let mut pos = sp;
    stack_write_usize(stack, pos, argc)?;
    pos += 8;
    let mut i = 0usize;
    while i < argc {
        stack_write_usize(stack, pos, arg_ptrs[i])?;
        pos += 8;
        i += 1;
    }
    stack_write_usize(stack, pos, 0)?;
    pos += 8;
    i = 0;
    while i < envc {
        stack_write_usize(stack, pos, env_ptrs[i])?;
        pos += 8;
        i += 1;
    }
    stack_write_usize(stack, pos, 0)?;
    pos += 8;
    stack_write_usize(stack, pos, AT_PHDR)?;
    pos += 8;
    stack_write_usize(stack, pos, phdr)?;
    pos += 8;
    stack_write_usize(stack, pos, AT_PHENT)?;
    pos += 8;
    stack_write_usize(stack, pos, header.phentsize)?;
    pos += 8;
    stack_write_usize(stack, pos, AT_PHNUM)?;
    pos += 8;
    stack_write_usize(stack, pos, header.phnum)?;
    pos += 8;
    stack_write_usize(stack, pos, AT_PAGESZ)?;
    pos += 8;
    stack_write_usize(stack, pos, PAGE_SIZE)?;
    pos += 8;
    stack_write_usize(stack, pos, AT_ENTRY)?;
    pos += 8;
    stack_write_usize(stack, pos, entry)?;
    pos += 8;
    stack_write_usize(stack, pos, AT_BASE)?;
    pos += 8;
    stack_write_usize(stack, pos, 0)?;
    pos += 8;
    stack_write_usize(stack, pos, AT_NULL)?;
    pos += 8;
    stack_write_usize(stack, pos, 0)?;
    Ok(stack_user_base + sp)
}

fn stack_copy_down(
    stack: &mut [u8],
    stack_base: usize,
    sp: &mut usize,
    src: &[u8],
) -> Result<usize, &'static str> {
    let len = src.len().checked_add(1).ok_or("user_stack_string")?;
    if *sp < len {
        return Err("user_stack_string_bounds");
    }
    *sp -= len;
    let mut i = 0usize;
    while i < src.len() {
        stack[*sp + i] = src[i];
        i += 1;
    }
    stack[*sp + src.len()] = 0;
    Ok(stack_base + *sp)
}

fn stack_write_usize(stack: &mut [u8], off: usize, value: usize) -> Result<(), &'static str> {
    if off + 8 > stack.len() {
        return Err("user_stack_write");
    }
    let bytes = (value as u64).to_le_bytes();
    let mut i = 0usize;
    while i < 8 {
        stack[off + i] = bytes[i];
        i += 1;
    }
    Ok(())
}

fn parse_header(data: &[u8]) -> Result<ElfHeader, &'static str> {
    if data.len() < 64 {
        return Err("elf_header_short");
    }
    if &data[0..4] != ELF_MAGIC {
        return Err("elf_magic");
    }
    if data[4] != 2 || data[5] != 1 {
        return Err("elf_class_endian");
    }
    let e_type = read_u16(data, 16)?;
    let e_machine = read_u16(data, 18)?;
    if e_type != ET_EXEC && e_type != ET_DYN {
        return Err("elf_type");
    }
    if e_machine != EM_LOONGARCH {
        return Err("elf_machine");
    }
    let phoff = read_u64(data, 32)? as usize;
    let phentsize = read_u16(data, 54)? as usize;
    let phnum = read_u16(data, 56)? as usize;
    if phentsize < 56 || phnum == 0 || phnum > 16 {
        return Err("elf_phnum");
    }
    Ok(ElfHeader {
        e_type,
        entry: read_u64(data, 24)? as usize,
        phoff,
        phentsize,
        phnum,
    })
}

fn parse_program_header(
    data: &[u8],
    header: ElfHeader,
    index: usize,
) -> Result<ProgramHeader, &'static str> {
    if index >= header.phnum {
        return Err("elf_ph_index");
    }
    let off = header
        .phoff
        .checked_add(index.checked_mul(header.phentsize).ok_or("elf_ph_mul")?)
        .ok_or("elf_ph_off")?;
    if off + 56 > data.len() {
        return Err("elf_ph_bounds");
    }
    Ok(ProgramHeader {
        p_type: read_u32(data, off)?,
        offset: read_u64(data, off + 8)? as usize,
        vaddr: read_u64(data, off + 16)? as usize,
        filesz: read_u64(data, off + 32)? as usize,
        memsz: read_u64(data, off + 40)? as usize,
    })
}

fn read_u16(data: &[u8], off: usize) -> Result<u16, &'static str> {
    if off + 2 > data.len() {
        return Err("read_u16");
    }
    Ok(u16::from_le_bytes([data[off], data[off + 1]]))
}

fn read_u32(data: &[u8], off: usize) -> Result<u32, &'static str> {
    if off + 4 > data.len() {
        return Err("read_u32");
    }
    Ok(u32::from_le_bytes([
        data[off],
        data[off + 1],
        data[off + 2],
        data[off + 3],
    ]))
}

fn read_u64(data: &[u8], off: usize) -> Result<u64, &'static str> {
    if off + 8 > data.len() {
        return Err("read_u64");
    }
    Ok(u64::from_le_bytes([
        data[off],
        data[off + 1],
        data[off + 2],
        data[off + 3],
        data[off + 4],
        data[off + 5],
        data[off + 6],
        data[off + 7],
    ]))
}

fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}

fn copy_bytes(dst: &mut [u8], src: &[u8]) {
    let mut i = 0usize;
    while i < dst.len() {
        dst[i] = src[i];
        i += 1;
    }
}

fn zero_bytes(dst: &mut [u8]) {
    let mut i = 0usize;
    while i < dst.len() {
        dst[i] = 0;
        i += 1;
    }
}
