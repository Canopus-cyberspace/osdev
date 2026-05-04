pub mod elf;
pub mod init_image;
pub mod process_image;

pub fn init() {
    crate::println!("[loader::elf] init v51");
    crate::println!("[loader::init_image] init v51");
    crate::println!("[loader::process_image] init v51");
    crate::println!("[loader] init");
}

pub fn self_test() {
    crate::println!("[loader-v51] self-test begin");
    elf::self_test();
    init_image::self_test();
    process_image::self_test();
    crate::println!("[loader-v51] self-test passed");
}
