#![allow(dead_code)]

const EI_NIDENT: usize = 16;
const ELF_MAGIC: &[u8; 4] = b"\x7fELF";
const ELFCLASS64: u8 = 2;
const ELFDATA2LSB: u8 = 1;
const EV_CURRENT: u8 = 1;
const ET_EXEC: u16 = 2;
const EM_RISCV: u16 = 243;
const PT_LOAD: u32 = 1;
const ELF64_EHDR_SIZE: usize = 64;
const ELF64_PHDR_SIZE: usize = 56;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ElfError {
    TooSmall,
    BadMagic,
    UnsupportedClass,
    UnsupportedEndian,
    UnsupportedVersion,
    UnsupportedType,
    UnsupportedMachine,
    BadHeaderSize,
    BadProgramHeaderSize,
    ProgramHeaderOutOfRange,
    NoLoadSegment,
    LoadSegmentOutOfRange,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ElfHeader {
    pub entry: usize,
    pub phoff: usize,
    pub phentsize: usize,
    pub phnum: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ProgramHeader {
    pub p_type: u32,
    pub flags: u32,
    pub offset: usize,
    pub vaddr: usize,
    pub paddr: usize,
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

    pub const fn end_vaddr(&self) -> usize {
        self.vaddr + self.memsz
    }
}

pub fn init() {
    crate::println!("[loader::elf] init v46f");
}

pub fn parse_header(image: &[u8]) -> Result<ElfHeader, ElfError> {
    if image.len() < ELF64_EHDR_SIZE {
        return Err(ElfError::TooSmall);
    }

    if &image[0..4] != ELF_MAGIC {
        return Err(ElfError::BadMagic);
    }
    if image[4] != ELFCLASS64 {
        return Err(ElfError::UnsupportedClass);
    }
    if image[5] != ELFDATA2LSB {
        return Err(ElfError::UnsupportedEndian);
    }
    if image[6] != EV_CURRENT {
        return Err(ElfError::UnsupportedVersion);
    }

    let e_type = read_u16(image, 16);
    let e_machine = read_u16(image, 18);
    let e_version = read_u32(image, 20);
    let entry = read_u64(image, 24) as usize;
    let phoff = read_u64(image, 32) as usize;
    let ehsize = read_u16(image, 52) as usize;
    let phentsize = read_u16(image, 54) as usize;
    let phnum = read_u16(image, 56) as usize;

    if e_type != ET_EXEC {
        return Err(ElfError::UnsupportedType);
    }
    if e_machine != EM_RISCV {
        return Err(ElfError::UnsupportedMachine);
    }
    if e_version != 1 {
        return Err(ElfError::UnsupportedVersion);
    }
    if ehsize != ELF64_EHDR_SIZE {
        return Err(ElfError::BadHeaderSize);
    }
    if phentsize != ELF64_PHDR_SIZE {
        return Err(ElfError::BadProgramHeaderSize);
    }

    let ph_table_end = phoff.checked_add(phentsize.saturating_mul(phnum)).ok_or(ElfError::ProgramHeaderOutOfRange)?;
    if ph_table_end > image.len() {
        return Err(ElfError::ProgramHeaderOutOfRange);
    }

    Ok(ElfHeader { entry, phoff, phentsize, phnum })
}

pub fn program_header(image: &[u8], header: &ElfHeader, index: usize) -> Result<ProgramHeader, ElfError> {
    if index >= header.phnum {
        return Err(ElfError::ProgramHeaderOutOfRange);
    }

    let offset = header.phoff + index * header.phentsize;
    let end = offset + ELF64_PHDR_SIZE;
    if end > image.len() {
        return Err(ElfError::ProgramHeaderOutOfRange);
    }

    let p_type = read_u32(image, offset);
    let flags = read_u32(image, offset + 4);
    let p_offset = read_u64(image, offset + 8) as usize;
    let vaddr = read_u64(image, offset + 16) as usize;
    let paddr = read_u64(image, offset + 24) as usize;
    let filesz = read_u64(image, offset + 32) as usize;
    let memsz = read_u64(image, offset + 40) as usize;
    let align = read_u64(image, offset + 48) as usize;

    if p_type == PT_LOAD {
        let data_end = p_offset.checked_add(filesz).ok_or(ElfError::LoadSegmentOutOfRange)?;
        if filesz > memsz || data_end > image.len() {
            return Err(ElfError::LoadSegmentOutOfRange);
        }
    }

    Ok(ProgramHeader {
        p_type,
        flags,
        offset: p_offset,
        vaddr,
        paddr,
        filesz,
        memsz,
        align,
    })
}

pub fn first_load_segment(image: &[u8]) -> Result<(ElfHeader, ProgramHeader), ElfError> {
    let header = parse_header(image)?;

    for index in 0..header.phnum {
        let ph = program_header(image, &header, index)?;
        if ph.is_load() {
            return Ok((header, ph));
        }
    }

    Err(ElfError::NoLoadSegment)
}

pub fn self_test() {
    crate::println!("[elf-loader-v46f] self-test begin");

    let image = synthetic_static_elf();
    let (header, load) = first_load_segment(&image).expect("[elf-loader-v46f] parse synthetic ELF failed");

    crate::println!("[elf-loader-v46f] entry = {:#x}", header.entry);
    crate::println!("[elf-loader-v46f] phnum = {}", header.phnum);
    crate::println!("[elf-loader-v46f] load offset = {:#x}", load.offset);
    crate::println!("[elf-loader-v46f] load vaddr = {:#x}", load.vaddr);
    crate::println!("[elf-loader-v46f] load filesz = {:#x}", load.filesz);
    crate::println!("[elf-loader-v46f] load memsz = {:#x}", load.memsz);

    assert_eq!(header.entry, 0x4000_0000);
    assert_eq!(header.phnum, 1);
    assert!(load.is_load());
    assert!(load.readable());
    assert!(!load.writable());
    assert!(load.executable());
    assert_eq!(load.offset, 0x80);
    assert_eq!(load.vaddr, 0x4000_0000);
    assert_eq!(load.filesz, 4);
    assert_eq!(load.memsz, 4);

    crate::println!("[elf-loader-v46f] PT_LOAD parse ok");
    crate::println!("[elf-loader-v46f] self-test passed");
}

fn synthetic_static_elf() -> [u8; 256] {
    let mut image = [0u8; 256];

    image[0..4].copy_from_slice(ELF_MAGIC);
    image[4] = ELFCLASS64;
    image[5] = ELFDATA2LSB;
    image[6] = EV_CURRENT;
    image[EI_NIDENT - 1] = 0;

    write_u16(&mut image, 16, ET_EXEC);
    write_u16(&mut image, 18, EM_RISCV);
    write_u32(&mut image, 20, 1);
    write_u64(&mut image, 24, 0x4000_0000);
    write_u64(&mut image, 32, ELF64_EHDR_SIZE as u64);
    write_u64(&mut image, 40, 0);
    write_u32(&mut image, 48, 0);
    write_u16(&mut image, 52, ELF64_EHDR_SIZE as u16);
    write_u16(&mut image, 54, ELF64_PHDR_SIZE as u16);
    write_u16(&mut image, 56, 1);
    write_u16(&mut image, 58, 0);
    write_u16(&mut image, 60, 0);
    write_u16(&mut image, 62, 0);

    let ph = ELF64_EHDR_SIZE;
    write_u32(&mut image, ph, PT_LOAD);
    write_u32(&mut image, ph + 4, 5); // PF_R | PF_X
    write_u64(&mut image, ph + 8, 0x80);
    write_u64(&mut image, ph + 16, 0x4000_0000);
    write_u64(&mut image, ph + 24, 0x4000_0000);
    write_u64(&mut image, ph + 32, 4);
    write_u64(&mut image, ph + 40, 4);
    write_u64(&mut image, ph + 48, 0x1000);

    image[0x80..0x84].copy_from_slice(&[0x13, 0x00, 0x00, 0x00]); // nop-like addi x0,x0,0

    image
}

fn read_u16(image: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([image[offset], image[offset + 1]])
}

fn read_u32(image: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([image[offset], image[offset + 1], image[offset + 2], image[offset + 3]])
}

fn read_u64(image: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes([
        image[offset],
        image[offset + 1],
        image[offset + 2],
        image[offset + 3],
        image[offset + 4],
        image[offset + 5],
        image[offset + 6],
        image[offset + 7],
    ])
}

fn write_u16(image: &mut [u8], offset: usize, value: u16) {
    image[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

fn write_u32(image: &mut [u8], offset: usize, value: u32) {
    image[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn write_u64(image: &mut [u8], offset: usize, value: u64) {
    image[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}
