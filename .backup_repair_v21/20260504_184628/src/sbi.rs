pub fn console_putchar(ch: usize) {
    const UART0: *mut u8 = 0x1000_0000 as *mut u8;

    unsafe {
        core::ptr::write_volatile(UART0, ch as u8);
    }
}

#[allow(dead_code)]
pub fn shutdown() -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}
