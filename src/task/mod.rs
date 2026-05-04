pub mod context;
pub mod process;
pub mod scheduler;
pub mod thread;
pub mod umode;

pub fn init() {
    process::init();
    thread::init();
    scheduler::init();

    crate::println!("[task] init");
}

pub fn run_first_user_task() -> ! {
    crate::println!("[task] run kernel Sv39 trap smoke v43d");
    crate::println!("[task] U-mode disabled for isolated kernel Sv39 trap smoke");
    crate::mm::sv39_smoke::run_kernel_sv39_smoke();
}
