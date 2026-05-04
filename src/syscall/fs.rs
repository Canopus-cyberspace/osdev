use crate::mm::user_buffer::{copy_from_user, UserBuffer, USER_COPY_CHUNK_SIZE};

pub fn sys_write(fd: usize, buf: usize, len: usize) -> isize {
    if fd != 1 && fd != 2 {
        return -1;
    }

    let user_buffer = match UserBuffer::new(buf, len) {
        Ok(buffer) => buffer,
        Err(err) => return err.as_errno(),
    };

    if user_buffer.is_empty() {
        return 0;
    }

    let mut written = 0usize;
    let mut current_ptr = user_buffer.ptr();
    let mut remaining = user_buffer.len();
    let mut chunk = [0u8; USER_COPY_CHUNK_SIZE];

    while remaining > 0 {
        let current_len = core::cmp::min(remaining, USER_COPY_CHUNK_SIZE);

        let copied = match copy_from_user(current_ptr, current_len, &mut chunk[..current_len]) {
            Ok(size) => size,
            Err(err) => return err.as_errno(),
        };

        for &ch in &chunk[..copied] {
            crate::sbi::console_putchar(ch as usize);
        }

        written += copied;
        current_ptr += copied;
        remaining -= copied;
    }

    written as isize
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
