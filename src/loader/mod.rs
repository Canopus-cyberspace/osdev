pub mod elf;

pub fn init() {
    crate::println!("[loader] init v47");
    elf::init();
}
