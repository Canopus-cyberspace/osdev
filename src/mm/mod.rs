pub mod address_space;
pub mod cow;
pub mod frame_allocator;
pub mod page_table;
pub mod user_buffer;
pub mod user_space;
pub mod vm_area;
pub mod sv39_preflight;

pub use frame_allocator::{frame_alloc, frame_dealloc};

pub fn init() {
    frame_allocator::init();
    address_space::init();
    vm_area::init();
    cow::init();
    sv39_preflight::init();
    user_space::init();

    crate::println!("[mm] init");
}

pub fn test() {
    frame_allocator::test();
    page_table::test();
    sv39_preflight::test();
    user_buffer::test_direct_user_copy();
    user_space::test_user_address_space_metadata();
}
