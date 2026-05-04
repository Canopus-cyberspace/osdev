pub mod fs;
pub mod mm;
pub mod process;
pub mod time;

const SYS_WRITE: usize = 64;
const SYS_EXIT: usize = 93;
const SYS_GETPID: usize = 172;
const SYS_GETPPID: usize = 173;

pub fn init() {
    crate::println!("[syscall] init");
}

pub fn syscall(id: usize, args: [usize; 6]) -> isize {
    match id {
        SYS_WRITE => fs::sys_write(args[0], args[1], args[2]),
        SYS_EXIT => process::sys_exit(args[0] as i32),
        SYS_GETPID => process::sys_getpid(),
        SYS_GETPPID => process::sys_getppid(),
        _ => {
            crate::println!("[syscall] unsupported id = {}", id);
            crate::config::ENOSYS
        }
    }
}
