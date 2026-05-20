#[path = "vfs_legacy_file.rs"]
pub(crate) mod legacy_file;

#[path = "vfs_legacy_tree.rs"]
pub(crate) mod legacy_tree;

pub fn init() {
    crate::println!("[fs::vfs] stub init");
}
