#![no_std]
#![no_main]

use core::arch::global_asm;

global_asm!(include_str!("../arch/riscv64/boot.S"));

mod console;
mod lang_items;
mod mm;
mod pagetable;
mod sbi;
mod syscall;
mod trap;

static USER_IMAGE: &[u8] = include_bytes!("../user/user.bin");

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    clear_bss();

    crate::println!("UESTC-Kernel booting...");
    crate::println!("arch = riscv64");
    crate::println!("stage = user image with Sv39 test");

    mm::init();
    mm::test();
    pagetable::test();

    trap::init();

    let (user_entry, user_sp) = pagetable::init_kernel_space(USER_IMAGE);

    trap::enter_user(user_entry, user_sp);
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    unsafe {
        let start = sbss as *const () as usize;
        let end = ebss as *const () as usize;
        core::slice::from_raw_parts_mut(start as *mut u8, end - start).fill(0);
    }
}
