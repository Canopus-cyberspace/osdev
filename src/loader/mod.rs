pub mod elf;
pub mod init_image;

pub fn init() {
    crate::println!("[loader::elf] init v50b");
    crate::println!("[loader::init_image] init v50b");
    crate::println!("[loader] init");
}

pub fn self_test() {
    crate::println!("[loader-v50b] self-test begin");
    elf::self_test();
    init_image::self_test();
    crate::println!("[loader-v50b] self-test passed");
}
