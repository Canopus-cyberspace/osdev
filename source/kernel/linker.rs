use crate::core::mm::{KernelImageRange, KernelImageRangeError, KernelLayout, KernelSectionRange};

unsafe extern "C" {
    static __kernel_start: u8;
    static __kernel_end: u8;
    static __text_start: u8;
    static __text_end: u8;
    static __rodata_start: u8;
    static __rodata_end: u8;
    static __data_start: u8;
    static __data_end: u8;
    static __bss_start: u8;
    static __bss_end: u8;
}

pub fn kernel_image_range() -> Result<KernelImageRange, KernelImageRangeError> {
    let start = core::ptr::addr_of!(__kernel_start) as usize;
    let end = core::ptr::addr_of!(__kernel_end) as usize;

    KernelImageRange::new(start, end)
}

pub fn kernel_layout() -> Result<KernelLayout, KernelImageRangeError> {
    let image = kernel_image_range()?;
    let text = KernelSectionRange::new(
        core::ptr::addr_of!(__text_start) as usize,
        core::ptr::addr_of!(__text_end) as usize,
    )?;
    let rodata = KernelSectionRange::new(
        core::ptr::addr_of!(__rodata_start) as usize,
        core::ptr::addr_of!(__rodata_end) as usize,
    )?;
    let data = KernelSectionRange::new(
        core::ptr::addr_of!(__data_start) as usize,
        core::ptr::addr_of!(__data_end) as usize,
    )?;
    let bss = KernelSectionRange::new(
        core::ptr::addr_of!(__bss_start) as usize,
        core::ptr::addr_of!(__bss_end) as usize,
    )?;

    KernelLayout::new(image, text, rodata, data, bss)
}
