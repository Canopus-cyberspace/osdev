#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;

    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("a0") arg0 => ret,
            in("a1") arg1,
            in("a2") arg2,
            in("a7") which,
        );
    }

    ret
}

pub fn console_putchar(ch: usize) {
    const SBI_CONSOLE_PUTCHAR: usize = 1;
    sbi_call(SBI_CONSOLE_PUTCHAR, ch, 0, 0);
}

#[allow(dead_code)]
pub fn shutdown() -> ! {
    const SBI_SHUTDOWN: usize = 8;
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);

    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}