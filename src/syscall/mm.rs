pub fn sys_brk() -> isize {
    crate::config::ENOSYS
}

pub fn sys_mmap() -> isize {
    crate::config::ENOSYS
}

pub fn sys_munmap() -> isize {
    crate::config::ENOSYS
}
