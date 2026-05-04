#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(static_mut_refs)]

use core::arch::global_asm;

global_asm!(include_str!("../arch/riscv64/boot.S"));

mod config;
mod console;
mod drivers;
mod fs;
mod futex;
mod lang_items;
mod loader;
mod mm;
mod net;
mod sbi;
mod signal;
mod sync;
mod syscall;
mod task;
mod timer;
mod trap;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    clear_bss();

    crate::println!("UESTC-Kernel booting...");
    crate::println!("[arch] riscv64");
    crate::println!("[stage] U-mode syscall matrix v30b");

    mm::init();
    mm::test();

    fs::init();
    loader::init();
    sync::init();
    signal::init();
    futex::init();
    timer::init();
    drivers::init();
    net::init();
    syscall::init();
    task::init();

    trap::init();

    if mm::sv39_smoke::ENABLE_KERNEL_SV39_SMOKE {
        mm::sv39_smoke::run_kernel_sv39_activation_smoke();
    }

    task::run_first_user_task();
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
