const UART0: usize = 0x1000_0000;
const SBI_EXT_SRST: usize = 0x5352_5354;
const SBI_SRST_FID_RESET: usize = 0;
const SBI_SRST_TYPE_SHUTDOWN: usize = 0;
const SBI_SRST_REASON_NONE: usize = 0;
const QEMU_VIRT_TEST: usize = 0x0010_0000;
const QEMU_VIRT_TEST_PASS: u32 = 0x5555;

pub fn console_putchar(ch: usize) {
    unsafe {
        core::ptr::write_volatile(UART0 as *mut u8, ch as u8);
    }
}

fn sbi_system_reset(reset_type: usize, reason: usize) -> isize {
    let error: isize;
    let _value: usize;
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("a0") reset_type as isize => error,
            inlateout("a1") reason => _value,
            in("a6") SBI_SRST_FID_RESET,
            in("a7") SBI_EXT_SRST,
        );
    }
    error
}

pub fn shutdown_success() -> ! {
    let error = sbi_system_reset(SBI_SRST_TYPE_SHUTDOWN, SBI_SRST_REASON_NONE);
    crate::println!(
        "[official-qemu-v194] SBI shutdown returned error {}; using QEMU virt finisher fallback",
        error
    );
    unsafe {
        core::ptr::write_volatile(QEMU_VIRT_TEST as *mut u32, QEMU_VIRT_TEST_PASS);
    }
    shutdown()
}

#[allow(dead_code)]
pub fn shutdown() -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}
