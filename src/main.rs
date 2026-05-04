#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(static_mut_refs)]
#![allow(unused_unsafe)]

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
    crate::println!("[stage] kernel Sv39 trap smoke v43e");

    mm::sv39_smoke::run_kernel_sv39_trap_ebreak_smoke();
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
