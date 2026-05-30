use crate::arch::contract::HaltReason;

pub fn halt(_reason: HaltReason) -> ! {
    loop {
        wait_for_interrupt();
    }
}

#[cfg(target_arch = "riscv64")]
#[inline(always)]
fn wait_for_interrupt() {
    unsafe {
        core::arch::asm!("wfi", options(nomem, nostack));
    }
}

#[cfg(not(target_arch = "riscv64"))]
#[inline(always)]
fn wait_for_interrupt() {
    core::hint::spin_loop();
}
