use crate::early_console_write;

const UART_BASE: *mut u8 = 0x1fe0_01e0 as *mut u8;

pub(crate) fn write_usize_dec(mut value: usize) {
    let mut buf = [0u8; 20];
    let mut pos = buf.len();

    if value == 0 {
        early_console_write("0");
        return;
    }

    while value != 0 {
        pos -= 1;
        buf[pos] = b'0' + (value % 10) as u8;
        value /= 10;
    }

    write_bytes(&buf[pos..]);
}

pub(crate) fn write_usize_hex(mut value: usize) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut buf = [0u8; 18];
    buf[0] = b'0';
    buf[1] = b'x';
    let mut pos = buf.len();

    if value == 0 {
        early_console_write("0x0");
        return;
    }

    while value != 0 {
        pos -= 1;
        buf[pos] = HEX[value & 0xf];
        value >>= 4;
    }

    write_bytes(&buf[..2]);
    write_bytes(&buf[pos..]);
}

pub(crate) fn write_bytes(bytes: &[u8]) {
    for byte in bytes {
        unsafe {
            core::ptr::write_volatile(UART_BASE, *byte);
        }
    }
}
