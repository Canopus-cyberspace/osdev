pub mod context;
pub mod process;
pub mod scheduler;
pub mod thread;
pub mod umode;

pub const ENABLE_KERNEL_SV39_SMOKE: bool = false;

pub fn init() {
    process::init();
    thread::init();
    scheduler::init();

    crate::println!("[task] init");
}

pub fn run_first_user_task() -> ! {
    crate::println!("[task] run first user task");

    if ENABLE_KERNEL_SV39_SMOKE {
        crate::println!("[task] running kernel Sv39 smoke path");
        crate::mm::sv39_smoke::run_kernel_sv39_smoke();
    }

    crate::println!("[task] running U-mode regression path");
    umode::run_umode_test();
}
