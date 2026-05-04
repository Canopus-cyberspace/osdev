use crate::config::PAGE_SIZE;
use crate::loader::elf;

pub static INIT_ELF_IMAGE: &[u8] = include_bytes!("../../user/init.elf");

#[repr(align(4096))]
struct InitLoadPage([u8; PAGE_SIZE]);

static mut INIT_LOAD_PAGE: InitLoadPage = InitLoadPage([0; PAGE_SIZE]);

#[derive(Copy, Clone)]
pub struct LoadedInitImage {
    pub entry: usize,
    pub vaddr: usize,
    pub memsz: usize,
    pub filesz: usize,
    pub load_page_pa: usize,
}

pub fn load_to_single_page() -> Result<LoadedInitImage, &'static str> {
    let (header, ph) = elf::first_load_segment(INIT_ELF_IMAGE)?;
    if ph.memsz > PAGE_SIZE {
        return Err("init PT_LOAD does not fit into one page scaffold");
    }
    let src_start = ph.offset;
    let src_end = ph.offset + ph.filesz;
    unsafe {
        let page_ptr = core::ptr::addr_of_mut!(INIT_LOAD_PAGE.0) as *mut u8;
        core::slice::from_raw_parts_mut(page_ptr, PAGE_SIZE).fill(0);
        let dst = core::slice::from_raw_parts_mut(page_ptr, ph.filesz);
        dst.copy_from_slice(&INIT_ELF_IMAGE[src_start..src_end]);
    }
    Ok(LoadedInitImage {
        entry: header.entry,
        vaddr: ph.vaddr,
        memsz: ph.memsz,
        filesz: ph.filesz,
        load_page_pa: core::ptr::addr_of!(INIT_LOAD_PAGE) as usize,
    })
}

pub fn self_test() {
    crate::println!("[init-image-v49c] self-test begin");
    let loaded = load_to_single_page().expect("[init-image-v49c] load init image failed");
    assert!(loaded.entry >= loaded.vaddr);
    assert!(loaded.filesz <= loaded.memsz);
    assert!(loaded.memsz <= PAGE_SIZE);
    assert_eq!(loaded.load_page_pa % PAGE_SIZE, 0);
    crate::println!("[init-image-v49c] entry = {:#x}", loaded.entry);
    crate::println!("[init-image-v49c] load vaddr = {:#x}", loaded.vaddr);
    crate::println!("[init-image-v49c] load page pa = {:#x}", loaded.load_page_pa);
    crate::println!("[init-image-v49c] load-page self-test passed");
}
