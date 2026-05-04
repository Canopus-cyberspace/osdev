pub mod elf;
pub mod init_image;

pub fn init() {
    crate::println!("[loader] init v49c");
}

pub fn self_test() {
    crate::println!("[loader-v49c] self-test begin");
    elf::self_test();
    init_image::self_test();
    crate::println!("[loader-v49c] self-test passed");
}

pub fn test() {
    self_test();
}
