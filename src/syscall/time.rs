pub fn sys_clock_gettime() -> isize {
    crate::config::ENOSYS
}

pub fn sys_nanosleep() -> isize {
    crate::config::ENOSYS
}
