const UART0: usize = 0x1000_0000;

pub fn console_putchar(ch: usize) {
    unsafe {
        core::ptr::write_volatile(UART0 as *mut u8, ch as u8);
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
