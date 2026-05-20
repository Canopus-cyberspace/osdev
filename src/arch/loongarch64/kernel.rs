#![no_std]
#![no_main]

use core::arch::global_asm;

#[path = "../../kernel_early.rs"]
mod kernel_early;
mod basic_runner;
mod console;
mod fd_table;
mod real_elf;
mod sdcard_ext4;
mod syscall;
mod trap;
mod user;
mod user_mem;
mod virtio_blk_pci;

global_asm!(
    r#"
    .section .text.entry, "ax"
    .globl _start
_start:
    la.local $sp, boot_stack_top
    bl loongarch64_early_entry
    li.d $t1, 0x100e001c
    li.w $t2, 0x34
    st.b $t2, $t1, 0
1:
    idle 0
    b 1b

    .section .bss, "aw", @nobits
    .align 12
boot_stack:
    .space 16384
boot_stack_top:
"#
);

#[no_mangle]
extern "C" fn loongarch64_early_entry() {
    early_console_write("[loongarch64] early boot ok\n");
    kernel_early::common_kernel_init(
        kernel_early::ArchInfo {
            name: "loongarch64",
        },
        early_console_write,
    );
    trap::install_trap_vector();
    basic_runner::run_loongarch_basic_musl_group();
}

pub(crate) fn early_console_write(s: &str) {
    const UART_BASE: *mut u8 = 0x1fe0_01e0 as *mut u8;

    for byte in s.as_bytes() {
        unsafe {
            core::ptr::write_volatile(UART_BASE, *byte);
        }
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
