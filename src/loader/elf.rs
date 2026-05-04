const EI_NIDENT: usize = 16;
const ELF_HEADER_SIZE: usize = 64;
const PROGRAM_HEADER_SIZE: usize = 56;
const EM_RISCV: u16 = 243;
const PT_LOAD: u32 = 1;

#[derive(Copy, Clone)]
pub struct ElfHeader {
    pub entry: usize,
    pub phoff: usize,
    pub phentsize: usize,
    pub phnum: usize,
}

#[derive(Copy, Clone)]
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

pub fn parse_header(image: &[u8]) -> Result<ElfHeader, &'static str> {
    if image.len() < ELF_HEADER_SIZE {
        return Err("ELF image too small");
    }
    if &image[0..4] != b"\x7fELF" {
        return Err("bad ELF magic");
    }
    if image[4] != 2 {
        return Err("ELF is not 64-bit");
    }
    if image[5] != 1 {
        return Err("ELF is not little-endian");
    }
    if image[EI_NIDENT - 10] == 0xff {
        return Err("reserved ident rejected");
    }
    let machine = read_u16(image, 18)?;
    if machine != EM_RISCV {
        return Err("ELF is not RISC-V");
    }
    let phoff = read_u64(image, 32)? as usize;
    let phentsize = read_u16(image, 54)? as usize;
    let phnum = read_u16(image, 56)? as usize;
    if phentsize != PROGRAM_HEADER_SIZE {
        return Err("unexpected program header size");
    }
    if phnum == 0 {
        return Err("ELF has no program headers");
    }
    if phoff.checked_add(phentsize * phnum).map_or(true, |end| end > image.len()) {
        return Err("program header table out of range");
    }
    Ok(ElfHeader {
        entry: read_u64(image, 24)? as usize,
        phoff,
        phentsize,
        phnum,
    })
}

pub fn parse_program_header(image: &[u8], header: ElfHeader, index: usize) -> Result<ProgramHeader, &'static str> {
    if index >= header.phnum {
        return Err("program header index out of range");
    }
    let base = header.phoff + index * header.phentsize;
    let ph = ProgramHeader {
        p_type: read_u32(image, base)?,
        flags: read_u32(image, base + 4)?,
        offset: read_u64(image, base + 8)? as usize,
        vaddr: read_u64(image, base + 16)? as usize,
        paddr: read_u64(image, base + 24)? as usize,
        filesz: read_u64(image, base + 32)? as usize,
        memsz: read_u64(image, base + 40)? as usize,
        align: read_u64(image, base + 48)? as usize,
    };
    if ph.p_type == PT_LOAD {
        if ph.filesz > ph.memsz {
            return Err("PT_LOAD filesz > memsz");
        }
        if ph.offset.checked_add(ph.filesz).map_or(true, |end| end > image.len()) {
            return Err("PT_LOAD file range out of image");
        }
    }
    Ok(ph)
}

pub fn first_load_segment(image: &[u8]) -> Result<(ElfHeader, ProgramHeader), &'static str> {
    let header = parse_header(image)?;
    let mut i = 0;
    while i < header.phnum {
        let ph = parse_program_header(image, header, i)?;
        if ph.p_type == PT_LOAD {
            return Ok((header, ph));
        }
        i += 1;
    }
    Err("no PT_LOAD segment")
}

pub const fn is_load(ph: &ProgramHeader) -> bool {
    ph.p_type == PT_LOAD
}

pub fn self_test() {
    crate::println!("[elf-loader-v49c] self-test begin");
    let mut image = [0u8; ELF_HEADER_SIZE + PROGRAM_HEADER_SIZE + 16];
    image[0] = 0x7f;
    image[1] = b'E';
    image[2] = b'L';
    image[3] = b'F';
    image[4] = 2;
    image[5] = 1;
    image[6] = 1;
    put_u16(&mut image, 16, 2);
    put_u16(&mut image, 18, EM_RISCV);
    put_u32(&mut image, 20, 1);
    put_u64(&mut image, 24, 0x4000_0000);
    put_u64(&mut image, 32, ELF_HEADER_SIZE as u64);
    put_u16(&mut image, 52, ELF_HEADER_SIZE as u16);
    put_u16(&mut image, 54, PROGRAM_HEADER_SIZE as u16);
    put_u16(&mut image, 56, 1);

    let ph = ELF_HEADER_SIZE;
    let payload = ELF_HEADER_SIZE + PROGRAM_HEADER_SIZE;
    put_u32(&mut image, ph, PT_LOAD);
    put_u32(&mut image, ph + 4, 5);
    put_u64(&mut image, ph + 8, payload as u64);
    put_u64(&mut image, ph + 16, 0x4000_0000);
    put_u64(&mut image, ph + 24, 0x4000_0000);
    put_u64(&mut image, ph + 32, 16);
    put_u64(&mut image, ph + 40, 16);
    put_u64(&mut image, ph + 48, 0x1000);

    let (hdr, load) = first_load_segment(&image).expect("[elf-loader-v49c] synthetic ELF parse failed");
    assert_eq!(hdr.entry, 0x4000_0000);
    assert!(is_load(&load));
    assert_eq!(load.vaddr, 0x4000_0000);
    assert_eq!(load.filesz, 16);
    crate::println!("[elf-loader-v49c] entry = {:#x}", hdr.entry);
    crate::println!("[elf-loader-v49c] first PT_LOAD vaddr = {:#x}", load.vaddr);
    crate::println!("[elf-loader-v49c] self-test passed");
}

fn read_u16(image: &[u8], off: usize) -> Result<u16, &'static str> {
    if off + 2 > image.len() { return Err("read_u16 out of range"); }
    Ok(u16::from_le_bytes([image[off], image[off + 1]]))
}

fn read_u32(image: &[u8], off: usize) -> Result<u32, &'static str> {
    if off + 4 > image.len() { return Err("read_u32 out of range"); }
    Ok(u32::from_le_bytes([image[off], image[off + 1], image[off + 2], image[off + 3]]))
}

fn read_u64(image: &[u8], off: usize) -> Result<u64, &'static str> {
    if off + 8 > image.len() { return Err("read_u64 out of range"); }
    Ok(u64::from_le_bytes([
        image[off], image[off + 1], image[off + 2], image[off + 3],
        image[off + 4], image[off + 5], image[off + 6], image[off + 7],
    ]))
}

fn put_u16(image: &mut [u8], off: usize, value: u16) {
    image[off..off + 2].copy_from_slice(&value.to_le_bytes());
}
fn put_u32(image: &mut [u8], off: usize, value: u32) {
    image[off..off + 4].copy_from_slice(&value.to_le_bytes());
}
fn put_u64(image: &mut [u8], off: usize, value: u64) {
    image[off..off + 8].copy_from_slice(&value.to_le_bytes());
}
