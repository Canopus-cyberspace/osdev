pub mod context;
pub mod process;
pub mod scheduler;
pub mod thread;
pub mod umode;

pub const ENABLE_UMODE_TEST: bool = true;

pub fn init() {
    process::init();
    thread::init();
    scheduler::init();

    crate::println!("[task] init");
}

pub fn run_first_user_task() -> ! {
    crate::println!("[task] run first user task");

    if ENABLE_UMODE_TEST {
        umode::run_umode_smoke_test();
    }

    crate::println!("[task] U-mode disabled, running kernel syscall self-test instead");
    kernel_syscall_self_test();

    crate::println!("[task] skeleton reached stable idle loop");
    idle_loop();
}

fn kernel_syscall_self_test() {
    crate::println!("[test] syscall write");

    let msg = b"hello from full mechanism skeleton syscall write\n";
    let ret = crate::syscall::syscall(64, [1, msg.as_ptr() as usize, msg.len(), 0, 0, 0]);

    crate::println!("[test] write returned {}", ret);

    let pid = crate::syscall::syscall(172, [0, 0, 0, 0, 0, 0]);
    crate::println!("[test] getpid returned {}", pid);

    let unsupported = crate::syscall::syscall(9999, [0, 0, 0, 0, 0, 0]);
    crate::println!("[test] unsupported returned {}", unsupported);
}

pub fn idle_loop() -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}
