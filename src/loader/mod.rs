pub mod elf;
pub mod init_image;

pub fn init() {
    init_image::init();
    crate::println!("[loader] init v47");
    elf::init();
}
