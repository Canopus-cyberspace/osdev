pub fn sys_write(fd: usize, buf: usize, len: usize) -> isize {
    if fd != 1 && fd != 2 {
        return -1;
    }

    let bytes = unsafe { core::slice::from_raw_parts(buf as *const u8, len) };

    for &ch in bytes {
        crate::sbi::console_putchar(ch as usize);
    }

    len as isize
}

pub fn sys_openat() -> isize {
    crate::config::ENOSYS
}

pub fn sys_read() -> isize {
    crate::config::ENOSYS
}

pub fn sys_close() -> isize {
    crate::config::ENOSYS
}

pub fn sys_getdents64() -> isize {
    crate::config::ENOSYS
}
