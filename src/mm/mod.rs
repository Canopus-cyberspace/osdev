pub mod address_space;
pub mod cow;
pub mod frame_allocator;
pub mod kernel_space;
pub mod page_table;
pub mod sv39_preflight;
pub mod user_buffer;
pub mod user_space;
pub mod vm_area;

pub use frame_allocator::frame_alloc;
#[allow(unused_imports)]
pub use frame_allocator::frame_dealloc;

pub fn init() {
    frame_allocator::init();
    address_space::init();
    vm_area::init();
    cow::init();
    sv39_preflight::init();
    user_space::init();
    kernel_space::init();

    crate::println!("[mm] init");
}

pub fn test() {
    frame_allocator::test();
    page_table::test();
    sv39_preflight::test();
    user_buffer::test_direct_user_copy();
    user_space::test();
    kernel_space::test();
}
