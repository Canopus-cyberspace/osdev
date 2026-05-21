#![allow(dead_code)]

use core::arch::global_asm;

use crate::console::{write_usize_dec, write_usize_hex};
use crate::early_console_write;
use crate::syscall;
use crate::user;
use crate::user_mmu;

const LOONGARCH_ECODE_SYS: usize = 11;
const LOONGARCH_ESTAT_ECODE_SHIFT: usize = 16;
const LOONGARCH_ESTAT_ECODE_MASK: usize = 0x3f;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LoongArchTrapFrame {
    pub regs: [usize; 32],
    pub era: usize,
    pub estat: usize,
    pub badv: usize,
    pub prmd: usize,
    trap_stack_top: usize,
}

#[repr(C)]
pub(crate) struct KernelReturnState {
    words: [usize; 12],
}

impl KernelReturnState {
    pub(crate) const fn empty() -> Self {
        Self { words: [0; 12] }
    }
}

global_asm!(
    r#"
    .section .text
    .balign 4
    .globl __loongarch64_trap_entry
__loongarch64_trap_entry:
    csrwr $sp, 0x30
    csrwr $r12, 0x31
    csrwr $r13, 0x32
    la.local $r13, loongarch64_trap_stack_cursor
    ld.d $sp, $r13, 0
    li.d $r12, -16384
    add.d $r12, $sp, $r12
    st.d $r12, $r13, 0
    addi.d $sp, $sp, -304
    addi.d $r12, $sp, 304
    st.d $r12, $sp, 288

    st.d $r1,  $sp, 8
    st.d $r2,  $sp, 16
    csrrd $r12, 0x30
    st.d $r12, $sp, 24

    st.d $r4,  $sp, 32
    st.d $r5,  $sp, 40
    st.d $r6,  $sp, 48
    st.d $r7,  $sp, 56
    st.d $r8,  $sp, 64
    st.d $r9,  $sp, 72
    st.d $r10, $sp, 80
    st.d $r11, $sp, 88
    csrrd $r12, 0x31
    st.d $r12, $sp, 96
    csrrd $r13, 0x32
    st.d $r13, $sp, 104
    st.d $r14, $sp, 112
    st.d $r15, $sp, 120
    st.d $r16, $sp, 128
    st.d $r17, $sp, 136
    st.d $r18, $sp, 144
    st.d $r19, $sp, 152
    st.d $r20, $sp, 160
    st.d $r21, $sp, 168
    st.d $r22, $sp, 176
    st.d $r23, $sp, 184
    st.d $r24, $sp, 192
    st.d $r25, $sp, 200
    st.d $r26, $sp, 208
    st.d $r27, $sp, 216
    st.d $r28, $sp, 224
    st.d $r29, $sp, 232
    st.d $r30, $sp, 240
    st.d $r31, $sp, 248

    csrrd $r12, 0x6
    st.d $r12, $sp, 256
    csrrd $r12, 0x5
    st.d $r12, $sp, 264
    csrrd $r12, 0x7
    st.d $r12, $sp, 272
    csrrd $r12, 0x1
    st.d $r12, $sp, 280

    or $a0, $sp, $zero
    bl loongarch64_trap_handler

    ld.d $r12, $sp, 288
    la.local $r13, loongarch64_trap_stack_cursor
    st.d $r12, $r13, 0

    ld.d $r12, $sp, 280
    csrwr $r12, 0x1
    ld.d $r12, $sp, 256
    csrwr $r12, 0x6

    ld.d $r1,  $sp, 8
    ld.d $r2,  $sp, 16
    ld.d $r4,  $sp, 32
    ld.d $r5,  $sp, 40
    ld.d $r6,  $sp, 48
    ld.d $r7,  $sp, 56
    ld.d $r8,  $sp, 64
    ld.d $r9,  $sp, 72
    ld.d $r10, $sp, 80
    ld.d $r11, $sp, 88
    ld.d $r12, $sp, 96
    ld.d $r13, $sp, 104
    ld.d $r14, $sp, 112
    ld.d $r15, $sp, 120
    ld.d $r16, $sp, 128
    ld.d $r17, $sp, 136
    ld.d $r18, $sp, 144
    ld.d $r19, $sp, 152
    ld.d $r20, $sp, 160
    ld.d $r21, $sp, 168
    ld.d $r22, $sp, 176
    ld.d $r23, $sp, 184
    ld.d $r24, $sp, 192
    ld.d $r25, $sp, 200
    ld.d $r26, $sp, 208
    ld.d $r27, $sp, 216
    ld.d $r28, $sp, 224
    ld.d $r29, $sp, 232
    ld.d $r30, $sp, 240
    ld.d $r31, $sp, 248
    ld.d $sp,  $sp, 24
    ertn

    .globl loongarch64_install_trap_vector
loongarch64_install_trap_vector:
    la.local $a0, __loongarch64_trap_entry
    csrwr $a0, 0xc
    ret

    .globl loongarch64_trigger_syscall_probe
loongarch64_trigger_syscall_probe:
    li.d $a0, 1
    la.local $a1, loongarch64_syscall_probe_msg
    li.d $a2, 43
    li.d $a7, 64
    syscall 0
    li.d $a0, 7
    li.d $a7, 93
    syscall 0
    ret

    .section .rodata
loongarch64_syscall_probe_msg:
    .ascii "[loongarch64-user] write via syscall probe\n"

    .section .text
    .balign 4
    .globl loongarch64_enter_user_smoke
loongarch64_enter_user_smoke:
    la.local $t0, loongarch64_kernel_sp_slot
    st.d $sp, $t0, 0
    la.local $t0, loongarch64_kernel_ra_slot
    st.d $ra, $t0, 0
    la.local $sp, loongarch64_user_stack_top
    la.local $t0, loongarch64_user_program
    csrwr $t0, 0x6
    li.d $t0, 0x3
    csrwr $t0, 0x1
    ertn

    .globl loongarch64_enter_user_entry
loongarch64_enter_user_entry:
    la.local $t0, loongarch64_kernel_sp_slot
    st.d $sp, $t0, 0
    la.local $t0, loongarch64_kernel_ra_slot
    st.d $ra, $t0, 0
    la.local $t0, loongarch64_kernel_saved_slot
    st.d $r22, $t0, 0
    st.d $r23, $t0, 8
    st.d $r24, $t0, 16
    st.d $r25, $t0, 24
    st.d $r26, $t0, 32
    st.d $r27, $t0, 40
    st.d $r28, $t0, 48
    st.d $r29, $t0, 56
    st.d $r30, $t0, 64
    st.d $r31, $t0, 72
    or $sp, $a1, $zero
    csrwr $a0, 0x6
    li.d $t0, 0x3
    csrwr $t0, 0x1
    ertn

    .globl loongarch64_enter_user_frame
loongarch64_enter_user_frame:
    la.local $t0, loongarch64_kernel_sp_slot
    st.d $sp, $t0, 0
    la.local $t0, loongarch64_kernel_ra_slot
    st.d $ra, $t0, 0
    la.local $t0, loongarch64_kernel_saved_slot
    st.d $r22, $t0, 0
    st.d $r23, $t0, 8
    st.d $r24, $t0, 16
    st.d $r25, $t0, 24
    st.d $r26, $t0, 32
    st.d $r27, $t0, 40
    st.d $r28, $t0, 48
    st.d $r29, $t0, 56
    st.d $r30, $t0, 64
    st.d $r31, $t0, 72

    or $r20, $a0, $zero
    ld.d $t0, $r20, 256
    csrwr $t0, 0x6
    li.d $t0, 0x3
    csrwr $t0, 0x1

    ld.d $r1,  $r20, 8
    ld.d $r2,  $r20, 16
    ld.d $r4,  $r20, 32
    ld.d $r5,  $r20, 40
    ld.d $r6,  $r20, 48
    ld.d $r7,  $r20, 56
    ld.d $r8,  $r20, 64
    ld.d $r9,  $r20, 72
    ld.d $r10, $r20, 80
    ld.d $r11, $r20, 88
    ld.d $r12, $r20, 96
    ld.d $r13, $r20, 104
    ld.d $r14, $r20, 112
    ld.d $r15, $r20, 120
    ld.d $r16, $r20, 128
    ld.d $r17, $r20, 136
    ld.d $r18, $r20, 144
    ld.d $r19, $r20, 152
    ld.d $r22, $r20, 176
    ld.d $r23, $r20, 184
    ld.d $r24, $r20, 192
    ld.d $r25, $r20, 200
    ld.d $r26, $r20, 208
    ld.d $r27, $r20, 216
    ld.d $r28, $r20, 224
    ld.d $r29, $r20, 232
    ld.d $r30, $r20, 240
    ld.d $r31, $r20, 248
    ld.d $r21, $r20, 168
    ld.d $sp,  $r20, 24
    ld.d $r20, $r20, 160
    ertn

    .globl loongarch64_user_exit_return
loongarch64_user_exit_return:
    la.local $t0, loongarch64_kernel_saved_slot
    ld.d $r22, $t0, 0
    ld.d $r23, $t0, 8
    ld.d $r24, $t0, 16
    ld.d $r25, $t0, 24
    ld.d $r26, $t0, 32
    ld.d $r27, $t0, 40
    ld.d $r28, $t0, 48
    ld.d $r29, $t0, 56
    ld.d $r30, $t0, 64
    ld.d $r31, $t0, 72
    la.local $t0, loongarch64_kernel_sp_slot
    ld.d $sp, $t0, 0
    la.local $t0, loongarch64_kernel_ra_slot
    ld.d $ra, $t0, 0
    ret

    .globl loongarch64_save_kernel_return_state
loongarch64_save_kernel_return_state:
    la.local $t0, loongarch64_kernel_sp_slot
    ld.d $t1, $t0, 0
    st.d $t1, $a0, 0
    la.local $t0, loongarch64_kernel_ra_slot
    ld.d $t1, $t0, 0
    st.d $t1, $a0, 8
    la.local $t0, loongarch64_kernel_saved_slot
    ld.d $t1, $t0, 0
    st.d $t1, $a0, 16
    ld.d $t1, $t0, 8
    st.d $t1, $a0, 24
    ld.d $t1, $t0, 16
    st.d $t1, $a0, 32
    ld.d $t1, $t0, 24
    st.d $t1, $a0, 40
    ld.d $t1, $t0, 32
    st.d $t1, $a0, 48
    ld.d $t1, $t0, 40
    st.d $t1, $a0, 56
    ld.d $t1, $t0, 48
    st.d $t1, $a0, 64
    ld.d $t1, $t0, 56
    st.d $t1, $a0, 72
    ld.d $t1, $t0, 64
    st.d $t1, $a0, 80
    ld.d $t1, $t0, 72
    st.d $t1, $a0, 88
    ret

    .globl loongarch64_restore_kernel_return_state
loongarch64_restore_kernel_return_state:
    la.local $t0, loongarch64_kernel_sp_slot
    ld.d $t1, $a0, 0
    st.d $t1, $t0, 0
    la.local $t0, loongarch64_kernel_ra_slot
    ld.d $t1, $a0, 8
    st.d $t1, $t0, 0
    la.local $t0, loongarch64_kernel_saved_slot
    ld.d $t1, $a0, 16
    st.d $t1, $t0, 0
    ld.d $t1, $a0, 24
    st.d $t1, $t0, 8
    ld.d $t1, $a0, 32
    st.d $t1, $t0, 16
    ld.d $t1, $a0, 40
    st.d $t1, $t0, 24
    ld.d $t1, $a0, 48
    st.d $t1, $t0, 32
    ld.d $t1, $a0, 56
    st.d $t1, $t0, 40
    ld.d $t1, $a0, 64
    st.d $t1, $t0, 48
    ld.d $t1, $a0, 72
    st.d $t1, $t0, 56
    ld.d $t1, $a0, 80
    st.d $t1, $t0, 64
    ld.d $t1, $a0, 88
    st.d $t1, $t0, 72
    ret

    .section .bss, "aw", @nobits
    .align 3
loongarch64_kernel_sp_slot:
    .space 8
loongarch64_kernel_ra_slot:
    .space 8
loongarch64_kernel_saved_slot:
    .space 80

    .section .trap_stack, "aw", @nobits
    .align 12
loongarch64_trap_stack:
    .space 65536
    .globl loongarch64_trap_stack_top
loongarch64_trap_stack_top:

    .section .data, "aw"
    .align 3
loongarch64_trap_stack_cursor:
    .dword loongarch64_trap_stack_top

    .section .user.text, "ax"
    .balign 4
    .globl loongarch64_user_region_start
loongarch64_user_region_start:
    .globl loongarch64_user_program
loongarch64_user_program:
    li.d $a0, 1
    la.local $a1, loongarch64_user_msg
    li.d $a2, 38
    li.d $a7, 64
    syscall 0
    li.d $a0, 0
    li.d $a7, 93
    syscall 0
1:
    b 1b

    .section .user.rodata, "a"
loongarch64_user_msg:
    .ascii "[loongarch64-user] PLV3 write syscall\n"

    .section .user.bss, "aw", @nobits
    .align 12
loongarch64_user_stack:
    .space 16384
    .globl loongarch64_user_stack_top
loongarch64_user_stack_top:
    .globl loongarch64_user_region_end
loongarch64_user_region_end:
"#
);

extern "C" {
    fn loongarch64_install_trap_vector();
    fn loongarch64_trigger_syscall_probe();
    fn loongarch64_enter_user_smoke();
    fn loongarch64_enter_user_entry(entry: usize, stack_pointer: usize);
    fn loongarch64_enter_user_frame(frame: *const LoongArchTrapFrame);
    fn loongarch64_user_exit_return();
    fn loongarch64_save_kernel_return_state(state: *mut KernelReturnState);
    fn loongarch64_restore_kernel_return_state(state: *const KernelReturnState);
}

pub fn install_trap_vector() {
    unsafe {
        loongarch64_install_trap_vector();
    }
    early_console_write("[loongarch64] trap vector installed\n");
}

pub fn run_syscall_probe() {
    early_console_write("[loongarch64] syscall probe begin\n");
    unsafe {
        loongarch64_trigger_syscall_probe();
    }
    early_console_write("[loongarch64] syscall probe returned\n");
}

pub(crate) fn enter_user_entry(entry: usize, stack_pointer: usize) {
    unsafe {
        loongarch64_enter_user_entry(entry, stack_pointer);
    }
}

pub(crate) fn enter_user_frame(frame: &LoongArchTrapFrame) {
    unsafe {
        loongarch64_enter_user_frame(frame as *const LoongArchTrapFrame);
    }
}

pub(crate) fn save_kernel_return_state(state: &mut KernelReturnState) {
    unsafe {
        loongarch64_save_kernel_return_state(state as *mut KernelReturnState);
    }
}

pub(crate) fn restore_kernel_return_state(state: &KernelReturnState) {
    unsafe {
        loongarch64_restore_kernel_return_state(state as *const KernelReturnState);
    }
}

pub(crate) fn user_exit_return_addr() -> usize {
    loongarch64_user_exit_return as *const () as usize
}

pub fn run_user_mode_smoke() {
    early_console_write(
        "[loongarch64] fallback embedded PLV3 smoke\n",
    );
    early_console_write("[loongarch64] user address space ready\n");
    early_console_write("[loongarch64] entering user mode\n");
    unsafe {
        loongarch64_enter_user_smoke();
    }
    early_console_write("[loongarch64] user mode returned\n");
}

#[no_mangle]
extern "C" fn loongarch64_trap_handler(frame: &mut LoongArchTrapFrame) {
    let ecode = (frame.estat >> LOONGARCH_ESTAT_ECODE_SHIFT) & LOONGARCH_ESTAT_ECODE_MASK;
    let quiet_group = user::is_any_group_active();
    let quiet_real_write = ecode == LOONGARCH_ECODE_SYS && syscall::is_quiet_real_write(frame);
    if !quiet_group && !quiet_real_write {
        early_console_write("[loongarch64-trap] ecode=");
        write_usize_dec(ecode);
        early_console_write(" prev_plv=");
        write_usize_dec(frame.prmd & 0x3);
        early_console_write(" era=");
        write_usize_hex(frame.era);
        early_console_write("\n");
    }

    if ecode == LOONGARCH_ECODE_SYS {
        syscall::handle_syscall(frame);
    } else {
        if (frame.prmd & 0x3) == 3 {
            user::record_user_fault(ecode, frame.era, frame.badv);
            frame.era = user_exit_return_addr();
            frame.prmd &= !0x3;
        } else {
            if !quiet_group {
                early_console_write("[loongarch64-trap] non-syscall trap\n");
            }
            frame.era = frame.era.wrapping_add(4);
        }
    }

    if (frame.prmd & 0x3) == 3 {
        user_mmu::sync_user_entry();
    }
}
