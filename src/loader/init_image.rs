//! v48: external user/init ELF image scaffold.

const INIT_ELF: &[u8] = include_bytes!("../../user/init.elf");

#[derive(Copy, Clone, Debug)]
pub struct InitElfInfo {
    pub entry: usize,
    pub phoff: usize,
    pub phnum: usize,
    pub load_vaddr: usize,
    pub load_offset: usize,
    pub load_filesz: usize,
    pub load_memsz: usize,
    pub load_flags: u32,
}

pub fn init() {
    crate::println!("[loader::init_image] scaffold init v48");
}

pub fn image() -> &'static [u8] {
    INIT_ELF
}

pub fn self_test() {
    crate::println!("[init-elf-v48] external init ELF scaffold begin");
    let info = parse_init_elf(INIT_ELF).expect("[init-elf-v48] parse failed");
    crate::println!("[init-elf-v48] entry = {:#x}", info.entry);
    crate::println!("[init-elf-v48] phoff = {:#x}", info.phoff);
    crate::println!("[init-elf-v48] phnum = {}", info.phnum);
    crate::println!("[init-elf-v48] load vaddr = {:#x}", info.load_vaddr);
    crate::println!("[init-elf-v48] load offset = {:#x}", info.load_offset);
    crate::println!("[init-elf-v48] load filesz = {:#x}", info.load_filesz);
    crate::println!("[init-elf-v48] load memsz = {:#x}", info.load_memsz);
    crate::println!("[init-elf-v48] load flags = {:#x}", info.load_flags);
    assert_eq!(info.entry, 0x4000_0000);
    assert_eq!(info.load_vaddr, 0x4000_0000);
    assert!(info.load_filesz > 0);
    assert!(info.load_memsz >= info.load_filesz);
    assert!(info.load_flags & 0x1 != 0);
    assert!(info.load_flags & 0x4 != 0);
    crate::println!("[init-elf-v48] self-test passed");
}

pub fn parse_init_elf(data: &[u8]) -> Result<InitElfInfo, &'static str> {
    if data.len() < 64 { return Err("ELF too small"); }
    if &data[0..4] != b"\x7fELF" { return Err("bad magic"); }
    if data[4] != 2 || data[5] != 1 || data[6] != 1 { return Err("unsupported ident"); }
    let e_type = read_u16(data, 16)?;
    let e_machine = read_u16(data, 18)?;
    let entry = read_u64(data, 24)? as usize;
    let phoff = read_u64(data, 32)? as usize;
    let ehsize = read_u16(data, 52)? as usize;
    let phentsize = read_u16(data, 54)? as usize;
    let phnum = read_u16(data, 56)? as usize;
    if e_type != 2 { return Err("not ET_EXEC"); }
    if e_machine != 243 { return Err("not RISC-V"); }
    if ehsize != 64 || phentsize != 56 || phnum == 0 { return Err("bad header sizes"); }
    for idx in 0..phnum {
        let base = phoff.checked_add(idx * phentsize).ok_or("ph overflow")?;
        if base + 56 > data.len() { return Err("ph out of range"); }
        let p_type = read_u32(data, base)?;
        let p_flags = read_u32(data, base + 4)?;
        let p_offset = read_u64(data, base + 8)? as usize;
        let p_vaddr = read_u64(data, base + 16)? as usize;
        let p_filesz = read_u64(data, base + 32)? as usize;
        let p_memsz = read_u64(data, base + 40)? as usize;
        if p_type == 1 {
            if p_offset.checked_add(p_filesz).ok_or("segment overflow")? > data.len() { return Err("segment out of range"); }
            if p_memsz < p_filesz { return Err("memsz < filesz"); }
            return Ok(InitElfInfo { entry, phoff, phnum, load_vaddr: p_vaddr, load_offset: p_offset, load_filesz: p_filesz, load_memsz: p_memsz, load_flags: p_flags });
        }
    }
    Err("missing PT_LOAD")
}

fn read_u16(data: &[u8], offset: usize) -> Result<u16, &'static str> {
    let bytes = data.get(offset..offset + 2).ok_or("u16 out of range")?;
    Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}
fn read_u32(data: &[u8], offset: usize) -> Result<u32, &'static str> {
    let bytes = data.get(offset..offset + 4).ok_or("u32 out of range")?;
    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}
fn read_u64(data: &[u8], offset: usize) -> Result<u64, &'static str> {
    let bytes = data.get(offset..offset + 8).ok_or("u64 out of range")?;
    Ok(u64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]))
}
