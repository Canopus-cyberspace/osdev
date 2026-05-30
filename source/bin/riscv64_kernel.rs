#![no_std]
#![no_main]
#![deny(warnings)]

#[cfg(target_arch = "riscv64")]
core::arch::global_asm!(
    r#"
    .section .text.entry, "ax"
    .globl _start
_start:
    la sp, __riscv64_boot_stack_top
    call riscv64_rust_entry
1:
    wfi
    j 1b

    .section .bss.stack, "aw", @nobits
    .align 12
    .globl __riscv64_boot_stack
__riscv64_boot_stack:
	    .space 2097152
    .globl __riscv64_boot_stack_top
__riscv64_boot_stack_top:
"#
);

#[no_mangle]
pub extern "C" fn riscv64_rust_entry(hart_id: usize, device_tree: usize) -> ! {
    let boot = uestc_kernel_source::arch::riscv64::boot::early_boot_info(hart_id, device_tree);
    let bsp = uestc_kernel_source::arch::riscv64::boot::bsp_services(boot);

    uestc_kernel_source::kernel::boot::kernel_start(bsp)
}
