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
    crate::println!("[task] run first user task");
    umode::run_umode_test();
}
