pub mod devfs;
pub mod ext4;
pub mod fat32;
pub mod fd_table;
pub mod file;
pub mod official_basic_musl;
pub mod pipe;
pub mod procfs;
pub mod runtime;
pub mod tmpfs;
pub mod vfs;

pub fn init() {
    vfs::init();
    fd_table::init();
    devfs::init();
    procfs::init();
    runtime::init();
    tmpfs::init();
    fat32::init();
    ext4::init();
    pipe::init();

    crate::println!("[fs] init");
}
