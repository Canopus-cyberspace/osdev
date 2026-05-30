use crate::arch::contract::HaltReason;

pub fn halt(_reason: HaltReason) -> ! {
    loop {
        wait_for_interrupt();
    }
}

#[cfg(target_arch = "loongarch64")]
#[inline(always)]
fn wait_for_interrupt() {
    unsafe {
        core::arch::asm!("idle 0", options(nomem, nostack));
    }
}

#[cfg(not(target_arch = "loongarch64"))]
#[inline(always)]
fn wait_for_interrupt() {
    core::hint::spin_loop();
}
