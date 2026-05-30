use crate::arch::contract::{BoundaryMode, HardwareReadiness, ReadinessReason, UserEntryState};
use crate::core::task::PendingUserEntry;

#[cfg(target_arch = "riscv64")]
unsafe extern "C" {
    fn __riscv64_user_trap_stack_top();
}

pub const fn readiness() -> HardwareReadiness {
    HardwareReadiness::NotReady(ReadinessReason::UserAddressSpaceMissing)
}

pub fn enter_user(pending: PendingUserEntry, mode: BoundaryMode) -> UserEntryState {
    match mode {
        BoundaryMode::Inspect => UserEntryState::Planned(pending),
        BoundaryMode::Prepare => UserEntryState::Prepared(pending),
        BoundaryMode::ApplyUnsafe => unsafe { apply_user_return(pending) },
    }
}

#[cfg(target_arch = "riscv64")]
unsafe fn apply_user_return(pending: PendingUserEntry) -> ! {
    let address_space = pending.address_space();
    let layout = address_space.plan();
    let registers = pending.registers();
    let sepc = layout.entry().value();
    let sp = layout.initial_stack_pointer();
    let a0 = registers.arg0();
    let a1 = registers.arg1();
    let trap_stack_top = __riscv64_user_trap_stack_top as *const () as usize;

    core::arch::asm!(
        "csrw sepc, {sepc}",
        "csrr t0, sstatus",
        "li t1, ~(1 << 8)",
        "and t0, t0, t1",
        "li t1, (1 << 5)",
        "or t0, t0, t1",
        "csrw sstatus, t0",
        "csrw sscratch, {trap_stack_top}",
        "mv sp, {sp}",
        "mv a0, {a0}",
        "mv a1, {a1}",
        "sret",
        sepc = in(reg) sepc,
        sp = in(reg) sp,
        a0 = in(reg) a0,
        a1 = in(reg) a1,
        trap_stack_top = in(reg) trap_stack_top,
        options(noreturn)
    );
}

#[cfg(not(target_arch = "riscv64"))]
unsafe fn apply_user_return(_pending: PendingUserEntry) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
