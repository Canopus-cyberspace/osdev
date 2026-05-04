#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(static_mut_refs)]

use core::arch::global_asm;

global_asm!(include_str!("../arch/riscv64/boot.S"));

mod console;
mod lang_items;
mod mm;
mod sbi;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    clear_bss();

    crate::println!("UESTC-Kernel booting...");
    crate::println!("[arch] riscv64");
    crate::println!("[stage] Sv39 + U-mode ecall smoke v45g");
    crate::println!("[v45g] before run_sv39_umode_smoke");

    mm::sv39_umode::run_sv39_umode_smoke();
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
