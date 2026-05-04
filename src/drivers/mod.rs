pub mod uart;
pub mod virtio_blk;
pub mod virtio_net;

pub fn init() {
    uart::init();
    virtio_blk::init();
    virtio_net::init();

    crate::println!("[drivers] init");
}
