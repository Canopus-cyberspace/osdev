pub mod address_space;
pub mod cow;
pub mod frame_allocator;
pub mod page_table;
pub mod vm_area;

pub use frame_allocator::{frame_alloc, frame_dealloc};

pub fn init() {
    frame_allocator::init();
    address_space::init();
    vm_area::init();
    cow::init();

    crate::println!("[mm] init");
}

pub fn test() {
    frame_allocator::test();
    page_table::test();
}
