pub mod address_space;
pub mod cow;
pub mod frame_allocator;
pub mod kernel_space;
pub mod kernel_builder;
pub mod page_table;
pub mod page_table_build;
pub mod sv39;
pub mod sv39_preflight;
pub mod user_buffer;
pub mod user_space;
pub mod vm_area;
pub mod user_builder;

pub use frame_allocator::{frame_alloc, frame_dealloc};

pub fn init() {
    frame_allocator::init();
    address_space::init();
    page_table_build::init();
    vm_area::init();
    cow::init();
    sv39::init();
    sv39_preflight::init();
    user_space::init();
    user_builder::init();
    kernel_space::init();
    kernel_builder::init();

    crate::println!("[mm] init");
}

pub fn test() {
    frame_allocator::test();
    page_table::test();
    page_table_build::test();
    sv39::test_scaffold();
    sv39_preflight::test();
    user_buffer::test_direct_user_copy();
    user_space::test();
    user_builder::test();
    kernel_space::test();
    kernel_builder::test();
}
