//! v47: static ELF parser scaffold plus linked user-image metadata path.
//!
//! This stage intentionally does not load an external file yet. It introduces
//! the same metadata shape that a future `execve` loader will return, then
//! validates it against the currently linked `.user` image used by the Sv39
//! U-mode smoke test.

const EI_NIDENT: usize = 16;
const ELF_MAGIC: &[u8; 4] = b"\x7fELF";
const ELFCLASS64: u8 = 2;
const ELFDATA2LSB: u8 = 1;
const ET_EXEC: u16 = 2;
const EM_RISCV: u16 = 243;
const PT_LOAD: u32 = 1;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ElfHeader {
    pub entry: usize,
    pub phoff: usize,
    pub phentsize: usize,
    pub phnum: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LoadSegment {
    pub offset: usize,
    pub vaddr: usize,
    pub filesz: usize,
    pub memsz: usize,
    pub flags: u32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LinkedUserImage {
    pub image_start_pa: usize,
    pub image_end_pa: usize,
    pub entry_pa: usize,
    pub entry_offset: usize,
    pub len: usize,
}

pub fn init() {
    crate::println!("[loader::elf] init v47");
}

pub fn self_test() {
    self_test_v47();
}

pub fn self_test_v47() {
    crate::println!("[elf-loader-v47] synthetic ELF self-test begin");

    let mut image = [0u8; 256];
    write_synthetic_elf64(&mut image);

    let header = parse_header(&image).expect("[elf-loader-v47] parse header failed");
    assert_eq!(header.entry, 0x4000_0000);
    assert_eq!(header.phoff, 64);
    assert_eq!(header.phnum, 1);

    let segment = first_load_segment(&image, header)
        .expect("[elf-loader-v47] missing PT_LOAD segment");
    assert_eq!(segment.offset, 0x1000);
    assert_eq!(segment.vaddr, 0x4000_0000);
    assert_eq!(segment.filesz, 0x80);
    assert_eq!(segment.memsz, 0x100);
    assert_eq!(segment.flags, 5);

    crate::println!("[elf-loader-v47] entry = {:#x}", header.entry);
    crate::println!("[elf-loader-v47] load vaddr = {:#x}", segment.vaddr);
    crate::println!("[elf-loader-v47] synthetic ELF self-test passed");
}

pub fn linked_user_image_v47() -> LinkedUserImage {
    extern "C" {
        fn suser();
        fn euser();
        fn __user_v45_start();
    }

    let image_start_pa = suser as *const () as usize;
    let image_end_pa = euser as *const () as usize;
    let entry_pa = __user_v45_start as *const () as usize;

    assert!(image_start_pa < image_end_pa);
    assert!(entry_pa >= image_start_pa);
    assert!(entry_pa < image_end_pa);

    LinkedUserImage {
        image_start_pa,
        image_end_pa,
        entry_pa,
        entry_offset: entry_pa - image_start_pa,
        len: image_end_pa - image_start_pa,
    }
}

pub fn linked_user_image_self_test_v47() {
    crate::println!("[elf-loader-v47] linked user image metadata begin");

    let image = linked_user_image_v47();

    crate::println!("[elf-loader-v47] linked user pa = {:#x}..{:#x}", image.image_start_pa, image.image_end_pa);
    crate::println!("[elf-loader-v47] linked user entry pa = {:#x}", image.entry_pa);
    crate::println!("[elf-loader-v47] linked user entry offset = {:#x}", image.entry_offset);
    crate::println!("[elf-loader-v47] linked user len = {}", image.len);

    assert!(image.len > 0);
    assert!(image.len <= 0x4000);
    assert_eq!(image.image_start_pa & 0xfff, 0);

    crate::println!("[elf-loader-v47] linked user image metadata passed");
}

pub fn parse_header(image: &[u8]) -> Result<ElfHeader, &'static str> {
    if image.len() < 64 {
        return Err("ELF image too small");
    }

    if &image[0..4] != ELF_MAGIC {
        return Err("bad ELF magic");
    }
    if image[4] != ELFCLASS64 {
        return Err("not ELF64");
    }
    if image[5] != ELFDATA2LSB {
        return Err("not little endian");
    }

    let ident_size = EI_NIDENT;
    let ty = read_u16(image, ident_size)?;
    let machine = read_u16(image, ident_size + 2)?;
    if ty != ET_EXEC {
        return Err("not ET_EXEC");
    }
    if machine != EM_RISCV {
        return Err("not RISC-V");
    }

    let entry = read_u64(image, 24)? as usize;
    let phoff = read_u64(image, 32)? as usize;
    let phentsize = read_u16(image, 54)? as usize;
    let phnum = read_u16(image, 56)? as usize;

    if phentsize < 56 {
        return Err("program header too small");
    }
    if phnum == 0 {
        return Err("no program headers");
    }

    Ok(ElfHeader { entry, phoff, phentsize, phnum })
}

pub fn first_load_segment(image: &[u8], header: ElfHeader) -> Result<LoadSegment, &'static str> {
    for i in 0..header.phnum {
        let off = header.phoff + i * header.phentsize;
        if off + 56 > image.len() {
            return Err("program header out of range");
        }

        let p_type = read_u32(image, off)?;
        if p_type != PT_LOAD {
            continue;
        }

        let flags = read_u32(image, off + 4)?;
        let offset = read_u64(image, off + 8)? as usize;
        let vaddr = read_u64(image, off + 16)? as usize;
        let filesz = read_u64(image, off + 32)? as usize;
        let memsz = read_u64(image, off + 40)? as usize;

        if filesz > memsz {
            return Err("filesz > memsz");
        }

        return Ok(LoadSegment { offset, vaddr, filesz, memsz, flags });
    }

    Err("no PT_LOAD segment")
}

fn read_u16(image: &[u8], off: usize) -> Result<u16, &'static str> {
    if off + 2 > image.len() {
        return Err("u16 out of range");
    }
    Ok(u16::from_le_bytes([image[off], image[off + 1]]))
}

fn read_u32(image: &[u8], off: usize) -> Result<u32, &'static str> {
    if off + 4 > image.len() {
        return Err("u32 out of range");
    }
    Ok(u32::from_le_bytes([image[off], image[off + 1], image[off + 2], image[off + 3]]))
}

fn read_u64(image: &[u8], off: usize) -> Result<u64, &'static str> {
    if off + 8 > image.len() {
        return Err("u64 out of range");
    }
    Ok(u64::from_le_bytes([
        image[off], image[off + 1], image[off + 2], image[off + 3],
        image[off + 4], image[off + 5], image[off + 6], image[off + 7],
    ]))
}

fn write_synthetic_elf64(image: &mut [u8; 256]) {
    image[0..4].copy_from_slice(ELF_MAGIC);
    image[4] = ELFCLASS64;
    image[5] = ELFDATA2LSB;
    image[6] = 1;

    write_u16(image, 16, ET_EXEC);
    write_u16(image, 18, EM_RISCV);
    write_u32(image, 20, 1);
    write_u64(image, 24, 0x4000_0000);
    write_u64(image, 32, 64);
    write_u64(image, 40, 0);
    write_u32(image, 48, 0);
    write_u16(image, 52, 64);
    write_u16(image, 54, 56);
    write_u16(image, 56, 1);
    write_u16(image, 58, 0);
    write_u16(image, 60, 0);
    write_u16(image, 62, 0);

    let ph = 64;
    write_u32(image, ph, PT_LOAD);
    write_u32(image, ph + 4, 5);
    write_u64(image, ph + 8, 0x1000);
    write_u64(image, ph + 16, 0x4000_0000);
    write_u64(image, ph + 24, 0x4000_0000);
    write_u64(image, ph + 32, 0x80);
    write_u64(image, ph + 40, 0x100);
    write_u64(image, ph + 48, 0x1000);
}

fn write_u16(image: &mut [u8], off: usize, value: u16) {
    image[off..off + 2].copy_from_slice(&value.to_le_bytes());
}

fn write_u32(image: &mut [u8], off: usize, value: u32) {
    image[off..off + 4].copy_from_slice(&value.to_le_bytes());
}

fn write_u64(image: &mut [u8], off: usize, value: u64) {
    image[off..off + 8].copy_from_slice(&value.to_le_bytes());
}
