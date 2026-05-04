pub mod context;
pub mod process;
pub mod scheduler;
pub mod thread;

pub fn init() {
    process::init();
    thread::init();
    scheduler::init();

    crate::println!("[task] init");
}

pub fn run_first_user_task() -> ! {
    crate::println!("[task] run first user task");
    crate::println!("[task] U-mode path disabled in stable skeleton");
    crate::println!("[task] running kernel syscall self-test instead");

    kernel_syscall_self_test();

    crate::println!("[task] skeleton reached stable idle loop");

    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

fn kernel_syscall_self_test() {
    crate::println!("[test] syscall write");

    let msg = b"hello from full mechanism skeleton syscall write\n";
    let ret = crate::syscall::syscall(
        64,
        [1, msg.as_ptr() as usize, msg.len(), 0, 0, 0],
    );

    crate::println!("[test] write returned {}", ret);

    crate::println!("[test] syscall getpid");

    let pid = crate::syscall::syscall(172, [0, 0, 0, 0, 0, 0]);

    crate::println!("[test] getpid returned {}", pid);

    if pid == 1 {
        let ok = b"getpid returned 1\n";
        let _ = crate::syscall::syscall(
            64,
            [1, ok.as_ptr() as usize, ok.len(), 0, 0, 0],
        );
    }

    crate::println!("[test] syscall unsupported");

    let unsupported = crate::syscall::syscall(9999, [0, 0, 0, 0, 0, 0]);

    crate::println!("[test] unsupported returned {}", unsupported);
}
