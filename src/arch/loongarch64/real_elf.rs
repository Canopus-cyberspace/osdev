#![allow(dead_code)]

use core::cmp::{max, min};

use crate::sdcard_ext4;

const PAGE_SIZE: usize = 4096;
const OFFICIAL_ELF_CAP: usize = 128 * 1024;
const USER_IMAGE_SIZE: usize = 128 * 1024;
const USER_STACK_SIZE: usize = 64 * 1024;
const USER_HEAP_SIZE: usize = 64 * 1024;
const USER_MMAP_SIZE: usize = 128 * 1024;

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

#[repr(C, align(16384))]
struct ElfBytes {
    bytes: [u8; OFFICIAL_ELF_CAP],
}

#[repr(C, align(16384))]
struct UserImage {
    bytes: [u8; USER_IMAGE_SIZE],
}

#[repr(C, align(4096))]
struct UserStack {
    bytes: [u8; USER_STACK_SIZE],
}

#[repr(C, align(4096))]
struct UserHeap {
    bytes: [u8; USER_HEAP_SIZE],
}

#[repr(C, align(4096))]
struct UserMmap {
    bytes: [u8; USER_MMAP_SIZE],
}

#[derive(Copy, Clone)]
struct UserRange {
    start: usize,
    end: usize,
}

impl UserRange {
    const fn empty() -> Self {
        Self { start: 0, end: 0 }
    }

    fn contains(&self, ptr: usize, len: usize) -> bool {
        match ptr.checked_add(len) {
            Some(limit) => ptr >= self.start && limit <= self.end,
            None => false,
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
static mut USER_RANGES: [UserRange; 4] = [
    UserRange::empty(),
    UserRange::empty(),
    UserRange::empty(),
    UserRange::empty(),
];
static mut USER_BRK: usize = 0;
static mut USER_MMAP_ACTIVE: bool = false;
static mut USER_RANGES_BACKUP: [UserRange; 4] = [
    UserRange::empty(),
    UserRange::empty(),
    UserRange::empty(),
    UserRange::empty(),
];
static mut USER_BRK_BACKUP: usize = 0;
static mut USER_MMAP_ACTIVE_BACKUP: bool = false;

pub fn load_basic_write() -> Result<RealElfLoad, &'static str> {
    load_basic_case("/musl/basic/write")
}

pub fn load_basic_case(path: &str) -> Result<RealElfLoad, &'static str> {
    unsafe {
        let elf = elf_buf_mut();
        let file = sdcard_ext4::load_path(path, elf)?;
        let header = parse_header(&elf[..file.size])?;
        let image = user_image_mut();
        zero_bytes(image);

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
                let dst = ph.vaddr - min_page;
                let dst_end = dst.checked_add(ph.filesz).ok_or("elf_dst")?;
                let src_end = ph.offset.checked_add(ph.filesz).ok_or("elf_src")?;
                if dst_end > USER_IMAGE_SIZE || src_end > file.size {
                    return Err("elf_segment_bounds");
                }
                copy_bytes(&mut image[dst..dst_end], &elf[ph.offset..src_end]);
            }
            i += 1;
        }

        let image_base = image.as_ptr() as usize;
        let load_bias = image_base.wrapping_sub(min_page);
        let entry = load_bias + header.entry;
        let phdr = load_bias + header.phoff;
        let stack_pointer = build_stack(entry, phdr, header, path.as_bytes())?;

        let stack = user_stack_mut();
        let stack_start = stack.as_ptr() as usize;
        let stack_end = stack_start + USER_STACK_SIZE;
        let heap_start = user_heap_mut().as_ptr() as usize;
        let mmap_start = user_mmap_mut().as_ptr() as usize;
        USER_RANGES = [
            UserRange {
                start: image_base,
                end: image_base + load_size,
            },
            UserRange {
                start: stack_start,
                end: stack_end,
            },
            UserRange {
                start: heap_start,
                end: heap_start,
            },
            UserRange {
                start: mmap_start,
                end: mmap_start,
            },
        ];
        USER_BRK = heap_start;
        USER_MMAP_ACTIVE = false;

        Ok(RealElfLoad {
            inode: file.inode,
            mode: file.mode,
            file_size: file.size,
            phnum: header.phnum,
            entry,
            stack_pointer,
            load_base: image_base,
            load_size,
            load_segments,
        })
    }
}

pub fn user_range_valid(ptr: usize, len: usize) -> bool {
    unsafe {
        let mut i = 0usize;
        while i < 4 {
            if USER_RANGES[i].contains(ptr, len) {
                return true;
            }
            i += 1;
        }
    }
    false
}

pub fn has_loaded_user_elf() -> bool {
    unsafe { USER_RANGES[0].end > USER_RANGES[0].start }
}

pub(crate) fn save_user_snapshot() {
    unsafe {
        copy_bytes(user_image_backup_mut(), user_image_mut());
        copy_bytes(user_stack_backup_mut(), user_stack_mut());
        copy_bytes(user_heap_backup_mut(), user_heap_mut());
        copy_bytes(user_mmap_backup_mut(), user_mmap_mut());
        USER_RANGES_BACKUP = USER_RANGES;
        USER_BRK_BACKUP = USER_BRK;
        USER_MMAP_ACTIVE_BACKUP = USER_MMAP_ACTIVE;
    }
}

pub(crate) fn restore_user_snapshot() {
    unsafe {
        copy_bytes(user_image_mut(), user_image_backup_mut());
        copy_bytes(user_stack_mut(), user_stack_backup_mut());
        copy_bytes(user_heap_mut(), user_heap_backup_mut());
        copy_bytes(user_mmap_mut(), user_mmap_backup_mut());
        USER_RANGES = USER_RANGES_BACKUP;
        USER_BRK = USER_BRK_BACKUP;
        USER_MMAP_ACTIVE = USER_MMAP_ACTIVE_BACKUP;
    }
}

pub fn sys_brk(addr: usize) -> isize {
    unsafe {
        let heap_start = core::ptr::addr_of!(USER_HEAP.bytes) as usize;
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
            USER_RANGES[2] = UserRange {
                start: heap_start,
                end: USER_BRK,
            };
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
        let mmap_start = core::ptr::addr_of!(USER_MMAP.bytes) as usize;
        let aligned_len = align_up(len, PAGE_SIZE);
        if USER_MMAP_ACTIVE || aligned_len > USER_MMAP_SIZE {
            return -12;
        }
        let mmap = user_mmap_mut();
        zero_bytes(mmap);
        if let Some(src) = file_bytes {
            let take = min(src.len(), len);
            copy_bytes(&mut mmap[..take], &src[..take]);
        }
        USER_MMAP_ACTIVE = true;
        USER_RANGES[3] = UserRange {
            start: mmap_start,
            end: mmap_start + aligned_len,
        };
        mmap_start as isize
    }
}

pub fn sys_munmap(addr: usize, len: usize) -> isize {
    if len == 0 {
        return -22;
    }
    unsafe {
        let mmap_start = core::ptr::addr_of!(USER_MMAP.bytes) as usize;
        let aligned_len = align_up(len, PAGE_SIZE);
        if USER_MMAP_ACTIVE
            && addr == mmap_start
            && aligned_len <= USER_MMAP_SIZE
            && USER_RANGES[3].contains(addr, len)
        {
            USER_MMAP_ACTIVE = false;
            USER_RANGES[3] = UserRange {
                start: mmap_start,
                end: mmap_start,
            };
            0
        } else {
            -22
        }
    }
}

pub fn write_user_usize_pair(ptr: usize, first: usize, second: usize) -> Result<(), &'static str> {
    if !user_range_valid(ptr, 16) {
        return Err("user_pair_range");
    }
    unsafe {
        core::ptr::write_volatile(ptr as *mut usize, first);
        core::ptr::write_volatile((ptr + 8) as *mut usize, second);
    }
    Ok(())
}

pub fn write_user_usize(ptr: usize, value: usize) -> Result<(), &'static str> {
    if !user_range_valid(ptr, 8) {
        return Err("user_usize_range");
    }
    unsafe {
        core::ptr::write_volatile(ptr as *mut usize, value);
    }
    Ok(())
}

pub fn copy_to_user(ptr: usize, src: &[u8]) -> Result<(), &'static str> {
    if !user_range_valid(ptr, src.len()) {
        return Err("user_copy_to_range");
    }
    unsafe {
        let dst = core::slice::from_raw_parts_mut(ptr as *mut u8, src.len());
        copy_bytes(dst, src);
    }
    Ok(())
}

pub fn copy_from_user(ptr: usize, dst: &mut [u8]) -> Result<(), &'static str> {
    if !user_range_valid(ptr, dst.len()) {
        return Err("user_copy_from_range");
    }
    unsafe {
        let src = core::slice::from_raw_parts(ptr as *const u8, dst.len());
        copy_bytes(dst, src);
    }
    Ok(())
}

pub fn read_user_cstr(ptr: usize, out: &mut [u8]) -> Result<usize, &'static str> {
    if out.is_empty() {
        return Err("user_cstr_empty");
    }
    let mut i = 0usize;
    while i + 1 < out.len() {
        if !user_range_valid(ptr + i, 1) {
            return Err("user_cstr_range");
        }
        let byte = unsafe { core::ptr::read_volatile((ptr + i) as *const u8) };
        if byte == 0 {
            out[i] = 0;
            return Ok(i);
        }
        out[i] = byte;
        i += 1;
    }
    Err("user_cstr_long")
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

unsafe fn build_stack(
    entry: usize,
    phdr: usize,
    header: ElfHeader,
    argv0: &[u8],
) -> Result<usize, &'static str> {
    let stack = user_stack_mut();
    zero_bytes(stack);
    let stack_base = stack.as_ptr() as usize;
    let mut sp = USER_STACK_SIZE;
    let home_ptr = stack_copy_down(stack, stack_base, &mut sp, b"HOME=/musl")?;
    let path_ptr = stack_copy_down(stack, stack_base, &mut sp, b"PATH=/musl:/bin:/usr/bin:.")?;
    let argv0_ptr = stack_copy_down(stack, stack_base, &mut sp, argv0)?;
    sp &= !15usize;

    let words = 1 + 1 + 1 + 2 + 1 + 14;
    let bytes = words * core::mem::size_of::<usize>();
    if sp < bytes {
        return Err("user_stack_bounds");
    }
    sp -= bytes;
    let mut pos = sp;
    stack_write_usize(stack, pos, 1)?;
    pos += 8;
    stack_write_usize(stack, pos, argv0_ptr)?;
    pos += 8;
    stack_write_usize(stack, pos, 0)?;
    pos += 8;
    stack_write_usize(stack, pos, path_ptr)?;
    pos += 8;
    stack_write_usize(stack, pos, home_ptr)?;
    pos += 8;
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
    Ok(stack_base + sp)
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
