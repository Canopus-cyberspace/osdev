pub mod mutex;
pub mod wait_queue;

pub fn init() {
    mutex::init();
    wait_queue::init();

    crate::println!("[sync] init");
}
