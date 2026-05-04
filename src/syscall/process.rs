pub fn sys_exit(code: i32) -> ! {
    crate::println!("[syscall] exit code = {}", code);
    crate::println!("[umode] v30b syscall matrix done");

    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

pub fn sys_getpid() -> isize {
    1
}

pub fn sys_getppid() -> isize {
    0
}

pub fn sys_fork() -> isize {
    crate::config::ENOSYS
}

pub fn sys_execve() -> isize {
    crate::config::ENOSYS
}

pub fn sys_wait4() -> isize {
    crate::config::ENOSYS
}
