pub fn init() {
    crate::println!("[futex] stub init");
}

pub fn futex_wait_stub() -> isize {
    crate::config::ENOSYS
}
