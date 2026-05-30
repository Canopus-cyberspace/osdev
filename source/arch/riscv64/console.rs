use core::ptr;

const CONSOLE_BASE: *mut u8 = 0x1000_0000 as *mut u8;

pub fn write_fatal(bytes: &[u8]) -> usize {
    for byte in bytes {
        unsafe {
            ptr::write_volatile(CONSOLE_BASE, *byte);
        }
    }

    bytes.len()
}
