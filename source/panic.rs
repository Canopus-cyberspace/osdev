use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    write_fatal(b"fatal panic\n");
    halt_fatal()
}

#[cfg(target_arch = "riscv64")]
fn write_fatal(bytes: &[u8]) -> usize {
    crate::arch::riscv64::console::write_fatal(bytes)
}

#[cfg(target_arch = "loongarch64")]
fn write_fatal(bytes: &[u8]) -> usize {
    crate::arch::loongarch64::console::write_fatal(bytes)
}

#[cfg(not(any(target_arch = "riscv64", target_arch = "loongarch64")))]
fn write_fatal(_bytes: &[u8]) -> usize {
    0
}

#[cfg(target_arch = "riscv64")]
fn halt_fatal() -> ! {
    crate::arch::riscv64::halt::halt(crate::arch::contract::HaltReason::Fatal(
        crate::arch::contract::FatalReason::Panic,
    ))
}

#[cfg(target_arch = "loongarch64")]
fn halt_fatal() -> ! {
    crate::arch::loongarch64::halt::halt(crate::arch::contract::HaltReason::Fatal(
        crate::arch::contract::FatalReason::Panic,
    ))
}

#[cfg(not(any(target_arch = "riscv64", target_arch = "loongarch64")))]
fn halt_fatal() -> ! {
    loop {
        core::hint::spin_loop();
    }
}
