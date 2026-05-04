pub mod elf;

pub fn init() {
    elf::init();
    crate::println!("[loader] init");
}
