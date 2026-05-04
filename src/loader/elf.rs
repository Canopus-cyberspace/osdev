#![allow(dead_code)]

pub const PT_LOAD: u32 = 1;
pub const EM_RISCV: u16 = 243;
pub const ET_EXEC: u16 = 2;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ElfError {
    TooSmall,
    BadMagic,
    NotElf64,
    NotLittleEndian,
    NotExecutable,
    NotRiscv,
    BadProgramHeader,
    NoLoadSegment,
    Range,
}

#[derive(Copy, Clone, Debug)]
pub struct ElfHeader {
    pub entry: usize,
    pub phoff: usize,
    pub phentsize: usize,
    pub phnum: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct ProgramHeader {
    pub p_type: u32,
    pub flags: u32,
    pub offset: usize,
    pub vaddr: usize,
    pub filesz: usize,
    pub memsz: usize,
    pub align: usize,
}

impl ProgramHeader {
    pub const fn is_load(&self) -> bool {
        self.p_type == PT_LOAD
    }

    pub const fn readable(&self) -> bool {
        self.flags & 4 != 0
    }

    pub const fn writable(&self) -> bool {
        self.flags & 2 != 0
    }

    pub const fn executable(&self) -> bool {
        self.flags & 1 != 0
    }
}

#[inline]
fn get_u16(data: &[u8], off: usize) -> Result<u16, ElfError> {
    if off + 2 > data.len() {
        return Err(ElfError::TooSmall);
    }
    Ok(u16::from_le_bytes([data[off], data[off + 1]]))
}

#[inline]
fn get_u32(data: &[u8], off: usize) -> Result<u32, ElfError> {
    if off + 4 > data.len() {
        return Err(ElfError::TooSmall);
    }
    Ok(u32::from_le_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]]))
}

#[inline]
fn get_u64(data: &[u8], off: usize) -> Result<u64, ElfError> {
    if off + 8 > data.len() {
        return Err(ElfError::TooSmall);
    }
    Ok(u64::from_le_bytes([
        data[off], data[off + 1], data[off + 2], data[off + 3],
        data[off + 4], data[off + 5], data[off + 6], data[off + 7],
    ]))
}

pub fn parse_header(data: &[u8]) -> Result<ElfHeader, ElfError> {
    if data.len() < 64 {
        return Err(ElfError::TooSmall);
    }
    if &data[0..4] != b"\x7fELF" {
        return Err(ElfError::BadMagic);
    }
    if data[4] != 2 {
        return Err(ElfError::NotElf64);
    }
    if data[5] != 1 {
        return Err(ElfError::NotLittleEndian);
    }

    let e_type = get_u16(data, 16)?;
    let e_machine = get_u16(data, 18)?;

    if e_type != ET_EXEC {
        return Err(ElfError::NotExecutable);
    }
    if e_machine != EM_RISCV {
        return Err(ElfError::NotRiscv);
    }

    Ok(ElfHeader {
        entry: get_u64(data, 24)? as usize,
        phoff: get_u64(data, 32)? as usize,
        phentsize: get_u16(data, 54)? as usize,
        phnum: get_u16(data, 56)? as usize,
    })
}

pub fn parse_program_header(data: &[u8], header: ElfHeader, index: usize) -> Result<ProgramHeader, ElfError> {
    if index >= header.phnum || header.phentsize < 56 {
        return Err(ElfError::BadProgramHeader);
    }

    let off = header
        .phoff
        .checked_add(index.checked_mul(header.phentsize).ok_or(ElfError::Range)?)
        .ok_or(ElfError::Range)?;

    if off + 56 > data.len() {
        return Err(ElfError::BadProgramHeader);
    }

    let p_type = get_u32(data, off)?;
    let flags = get_u32(data, off + 4)?;
    let offset = get_u64(data, off + 8)? as usize;
    let vaddr = get_u64(data, off + 16)? as usize;
    let filesz = get_u64(data, off + 32)? as usize;
    let memsz = get_u64(data, off + 40)? as usize;
    let align = get_u64(data, off + 48)? as usize;

    if filesz > memsz {
        return Err(ElfError::BadProgramHeader);
    }
    if offset.checked_add(filesz).ok_or(ElfError::Range)? > data.len() {
        return Err(ElfError::BadProgramHeader);
    }

    Ok(ProgramHeader {
        p_type,
        flags,
        offset,
        vaddr,
        filesz,
        memsz,
        align,
    })
}

pub fn first_load_segment(data: &[u8]) -> Result<(ElfHeader, ProgramHeader), ElfError> {
    let header = parse_header(data)?;
    let mut i = 0;

    while i < header.phnum {
        let ph = parse_program_header(data, header, i)?;
        if ph.is_load() {
            return Ok((header, ph));
        }
        i += 1;
    }

    Err(ElfError::NoLoadSegment)
}

pub fn self_test() {
    crate::println!("[elf-loader-v50b] parser self-test begin");
    let (header, ph) = first_load_segment(crate::loader::init_image::INIT_ELF)
        .expect("[elf-loader-v50b] parse external init ELF failed");

    crate::println!("[elf-loader-v50b] entry = {:#x}", header.entry);
    crate::println!(
        "[elf-loader-v50b] load  = off {:#x} va {:#x} filesz {:#x} memsz {:#x}",
        ph.offset, ph.vaddr, ph.filesz, ph.memsz
    );

    assert!(ph.readable());
    assert!(ph.executable());
    assert!(!ph.writable());

    crate::println!("[elf-loader-v50b] parser self-test passed");
}
