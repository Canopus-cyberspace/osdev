#![allow(dead_code)]

use crate::config::PAGE_SIZE;
use crate::loader::elf::{first_load_segment, ElfError};

pub const INIT_ELF: &[u8] = include_bytes!("../../user/init.elf");
pub const MAX_INIT_LOAD_SIZE: usize = PAGE_SIZE * 4;

#[repr(align(4096))]
pub struct InitLoadPage(pub [u8; MAX_INIT_LOAD_SIZE]);

static mut INIT_LOAD_PAGE: InitLoadPage = InitLoadPage([0; MAX_INIT_LOAD_SIZE]);

#[derive(Copy, Clone, Debug)]
pub struct LoadedInitImage {
    pub entry: usize,
    pub vaddr: usize,
    pub memsz: usize,
    pub filesz: usize,
    pub load_pa: usize,
    pub page_count: usize,
}

pub fn load_init_image_to_page() -> Result<LoadedInitImage, ElfError> {
    let (header, ph) = first_load_segment(INIT_ELF)?;

    if ph.memsz > MAX_INIT_LOAD_SIZE {
        return Err(ElfError::Range);
    }

    unsafe {
        INIT_LOAD_PAGE.0.fill(0);

        let src = &INIT_ELF[ph.offset..ph.offset + ph.filesz];
        let dst = &mut INIT_LOAD_PAGE.0[..ph.filesz];

        let mut i = 0;
        while i < src.len() {
            dst[i] = src[i];
            i += 1;
        }

        let page_count = (ph.memsz + PAGE_SIZE - 1) / PAGE_SIZE;

        Ok(LoadedInitImage {
            entry: header.entry,
            vaddr: ph.vaddr,
            memsz: ph.memsz,
            filesz: ph.filesz,
            load_pa: core::ptr::addr_of!(INIT_LOAD_PAGE) as usize,
            page_count,
        })
    }
}

pub fn self_test() {
    crate::println!("[init-image-v50b] external init image self-test begin");

    let loaded = load_init_image_to_page()
        .expect("[init-image-v50b] load external init ELF failed");

    crate::println!("[init-image-v50b] entry   = {:#x}", loaded.entry);
    crate::println!("[init-image-v50b] vaddr   = {:#x}", loaded.vaddr);
    crate::println!("[init-image-v50b] memsz   = {:#x}", loaded.memsz);
    crate::println!("[init-image-v50b] filesz  = {:#x}", loaded.filesz);
    crate::println!("[init-image-v50b] load pa = {:#x}", loaded.load_pa);
    crate::println!("[init-image-v50b] pages   = {}", loaded.page_count);

    assert_eq!(loaded.entry, loaded.vaddr);
    assert!(loaded.filesz > 0);
    assert!(loaded.memsz >= loaded.filesz);
    assert!(loaded.page_count >= 1);

    crate::println!("[init-image-v50b] external init image self-test passed");
}
