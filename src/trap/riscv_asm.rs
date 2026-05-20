use core::arch::{asm, global_asm};

use crate::mm::sv39_init_exec::TrapContext;

global_asm!(
    r#"
    .section .text
    .balign 4
    .globl __sv39_init_v50b_alltraps
    .globl __sv39_init_v50b_restore

__sv39_init_v50b_alltraps:
    csrrw sp, sscratch, sp
    addi sp, sp, -272

    sd x1,   8(sp)
    csrr t0, sscratch
    sd t0,  16(sp)
    sd x3,  24(sp)
    sd x4,  32(sp)
    sd x5,  40(sp)
    sd x6,  48(sp)
    sd x7,  56(sp)
    sd x8,  64(sp)
    sd x9,  72(sp)
    sd x10, 80(sp)
    sd x11, 88(sp)
    sd x12, 96(sp)
    sd x13, 104(sp)
    sd x14, 112(sp)
    sd x15, 120(sp)
    sd x16, 128(sp)
    sd x17, 136(sp)
    sd x18, 144(sp)
    sd x19, 152(sp)
    sd x20, 160(sp)
    sd x21, 168(sp)
    sd x22, 176(sp)
    sd x23, 184(sp)
    sd x24, 192(sp)
    sd x25, 200(sp)
    sd x26, 208(sp)
    sd x27, 216(sp)
    sd x28, 224(sp)
    sd x29, 232(sp)
    sd x30, 240(sp)
    sd x31, 248(sp)

    csrr t0, sstatus
    sd t0, 256(sp)
    csrr t0, sepc
    sd t0, 264(sp)

    mv a0, sp
    call rust_sv39_init_v50b_trap_handler

    mv a0, sp
    j __sv39_init_v50b_restore

__sv39_init_v50b_restore:
    mv sp, a0

    la t0, external_init_trap_stack_top
    csrw sscratch, t0

    ld t0, 256(sp)
    csrw sstatus, t0
    ld t0, 264(sp)
    csrw sepc, t0

    ld x1,   8(sp)
    ld x3,  24(sp)
    ld x4,  32(sp)
    ld x5,  40(sp)
    ld x6,  48(sp)
    ld x7,  56(sp)
    ld x8,  64(sp)
    ld x9,  72(sp)
    ld x10, 80(sp)
    ld x11, 88(sp)
    ld x12, 96(sp)
    ld x13, 104(sp)
    ld x14, 112(sp)
    ld x15, 120(sp)
    ld x16, 128(sp)
    ld x17, 136(sp)
    ld x18, 144(sp)
    ld x19, 152(sp)
    ld x20, 160(sp)
    ld x21, 168(sp)
    ld x22, 176(sp)
    ld x23, 184(sp)
    ld x24, 192(sp)
    ld x25, 200(sp)
    ld x26, 208(sp)
    ld x27, 216(sp)
    ld x28, 224(sp)
    ld x29, 232(sp)
    ld x30, 240(sp)
    ld x31, 248(sp)
    ld x2, 16(sp)
    sret

    .section .trap_stack, "aw", @nobits
    .align 12
external_init_trap_stack:
    .space 4096 * 16
external_init_trap_stack_top:
"#
);

extern "C" {
    fn __sv39_init_v50b_alltraps();
    fn __sv39_init_v50b_restore(cx: *const TrapContext) -> !;
}

pub unsafe fn install_trap_entry() {
    let entry_raw = __sv39_init_v50b_alltraps as *const () as usize;
    let entry = entry_raw & !0x3usize;

    // UCOMPAT_V145E_SUPPRESSED_EXTERNAL_INIT_TRAP_LOG
    // UCOMPAT_V145E_SUPPRESSED_EXTERNAL_INIT_TRAP_LOG

    asm!("csrw stvec, {}", in(reg) entry);
    crate::println!("[external-init-v82] stvec = {:#x}", entry);
}

pub unsafe fn restore(cx: *const TrapContext) -> ! {
    __sv39_init_v50b_restore(cx)
}
