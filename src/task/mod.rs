pub mod process;

pub fn init() {
    process::init();
    crate::println!("[task] init");
}
