use core::arch::asm;

use crate::mm::sv39_init_exec::TrapContext;

const SATP_MODE_SV39: usize = 8usize << 60;
const SSTATUS_SPP: usize = 1 << 8;
const SSTATUS_SPIE: usize = 1 << 5;

pub unsafe fn activate_page_table(root_pa: usize) {
    let satp = SATP_MODE_SV39 | (root_pa / crate::config::PAGE_SIZE);
    crate::println!("[external-init-v82] satp = {:#x}", satp);
    asm!("csrw satp, {}", in(reg) satp);
    asm!("sfence.vma zero, zero");
}

pub unsafe fn enter_user(cx: *mut TrapContext, entry: usize, user_stack_top: usize) -> ! {
    (*cx).regs = [0; 32];
    (*cx).regs[2] = user_stack_top;
    (*cx).sstatus = user_sstatus();
    (*cx).sepc = entry;

    crate::println!("[external-init-v82] enter user sepc = {:#x}", (*cx).sepc);
    crate::println!("[external-init-v82] enter user sp   = {:#x}", (*cx).regs[2]);

    crate::trap::riscv_asm::restore(cx)
}

pub fn user_sstatus() -> usize {
    let mut value = read_sstatus();
    value &= !SSTATUS_SPP;
    value |= SSTATUS_SPIE;
    value
}

fn read_sstatus() -> usize {
    let value: usize;
    unsafe {
        asm!("csrr {}, sstatus", out(reg) value);
    }
    value
}
