#![no_std]
#![no_main]
#![deny(warnings)]

#[cfg(target_arch = "loongarch64")]
core::arch::global_asm!(
    r#"
    .section .text.entry, "ax"
    .globl _start
_start:
    la.local $sp, __loongarch64_boot_stack_top
    bl loongarch64_rust_entry
1:
    idle 0
    b 1b

    .section .bss.stack, "aw", @nobits
    .align 12
    .globl __loongarch64_boot_stack
__loongarch64_boot_stack:
    .space 2097152
    .globl __loongarch64_boot_stack_top
__loongarch64_boot_stack_top:
"#
);

#[no_mangle]
pub extern "C" fn loongarch64_rust_entry(arg0: usize, arg1: usize) -> ! {
    let boot = uestc_kernel_source::arch::loongarch64::boot::early_boot_info(arg0, arg1);
    let bsp = uestc_kernel_source::arch::loongarch64::boot::bsp_services(boot);

    uestc_kernel_source::kernel::boot::kernel_start(bsp)
}
