pub fn init() {
    crate::println!("[signal] stub init");
}

pub fn deliver_signal_stub() -> isize {
    crate::config::ENOSYS
}
