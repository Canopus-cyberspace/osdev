const SYS_WRITE: usize = 64;
const SYS_EXIT: usize = 93;
const SYS_GETPID: usize = 172;
const SYS_GETPPID: usize = 173;

pub fn syscall(id: usize, args: [usize; 6]) -> isize {
    match id {
        SYS_WRITE => sys_write(args[0], args[1], args[2]),
        SYS_EXIT => sys_exit(args[0] as i32),
        SYS_GETPID => sys_getpid(),
        SYS_GETPPID => sys_getppid(),
        _ => {
            crate::println!("[syscall] unsupported syscall id = {}", id);
            -38
        }
    }
}

fn sys_write(fd: usize, buf: usize, len: usize) -> isize {
    if fd != 1 && fd != 2 {
        return -1;
    }

    let bytes = unsafe {
        core::slice::from_raw_parts(buf as *const u8, len)
    };

    for &ch in bytes {
        crate::sbi::console_putchar(ch as usize);
    }

    len as isize
}

fn sys_exit(code: i32) -> ! {
    crate::println!("[syscall] exit code = {}", code);

    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

fn sys_getpid() -> isize {
    1
}

fn sys_getppid() -> isize {
    0
}
