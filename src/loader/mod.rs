pub mod elf;
pub mod init_image;
pub mod process_image;
pub mod user_stack;

pub fn init() {
    crate::println!("[loader::elf] init v52");
    crate::println!("[loader::init_image] init v52");
    crate::println!("[loader::process_image] init v52");
    crate::println!("[loader::user_stack] init v52");
    crate::println!("[loader] init");
}

pub fn self_test() {
    crate::println!("[loader-v52] self-test begin");
    elf::self_test();
    init_image::self_test();
    process_image::self_test();
    user_stack::self_test();
    crate::println!("[loader-v52] self-test passed");
}
