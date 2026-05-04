use crate::mm::user_buffer::{copy_from_user, UserBuffer};

const WRITE_CHUNK_SIZE: usize = 256;

pub fn sys_write(fd: usize, buf: usize, len: usize) -> isize {
    if fd != 1 && fd != 2 {
        return -1;
    }

    let user_buffer = match UserBuffer::new(buf, len) {
        Ok(buffer) => buffer,
        Err(err) => {
            crate::println!("[sys_write] invalid user buffer: {}", err);
            return -1;
        }
    };

    if user_buffer.is_empty() {
        return 0;
    }

    let mut copied = 0usize;
    let mut scratch = [0u8; WRITE_CHUNK_SIZE];

    while copied < user_buffer.len() {
        let remain = user_buffer.len() - copied;
        let chunk_len = core::cmp::min(remain, scratch.len());
        let src = user_buffer.ptr() + copied;
        let dst = &mut scratch[..chunk_len];

        if let Err(err) = copy_from_user(src, dst) {
            crate::println!("[sys_write] copy_from_user failed: {}", err);
            return -1;
        }

        for &ch in dst.iter() {
            crate::sbi::console_putchar(ch as usize);
        }

        copied += chunk_len;
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
