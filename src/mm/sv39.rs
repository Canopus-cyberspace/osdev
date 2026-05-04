pub const SATP_MODE_SV39: usize = 8;
pub const SATP_MODE_SHIFT: usize = 60;
pub const SATP_PPN_MASK: usize = (1usize << 44) - 1;

pub const ENABLE_SV39_ACTIVATION_TEST: bool = true;

pub fn init() {
    crate::println!("[mm::sv39] init v42");
}

pub fn test_scaffold() {
    let ppn = 0x80200usize;
    let satp = make_satp(ppn);

    assert_eq!(satp_mode(satp), SATP_MODE_SV39);
    assert_eq!(satp_ppn(satp), ppn);

    crate::println!("[mm::sv39] scaffold test passed v42");
}

#[inline]
pub fn make_satp(root_ppn: usize) -> usize {
    (SATP_MODE_SV39 << SATP_MODE_SHIFT) | (root_ppn & SATP_PPN_MASK)
}

#[inline]
pub fn satp_mode(satp: usize) -> usize {
    satp >> SATP_MODE_SHIFT
}

#[inline]
pub fn satp_ppn(satp: usize) -> usize {
    satp & SATP_PPN_MASK
}

#[inline]
pub fn read_satp() -> usize {
    let value: usize;
    unsafe {
        core::arch::asm!("csrr {}, satp", out(reg) value);
    }
    value
}

#[inline]
pub fn sfence_vma() {
    unsafe {
        core::arch::asm!("sfence.vma", options(nostack, preserves_flags));
    }
}

/// # Safety
/// Caller must ensure the page table identity-maps the currently executing
/// kernel code, data, stack, trap vector and UART MMIO before calling this.
#[inline]
pub unsafe fn activate_satp_unchecked(satp: usize) {
    core::arch::asm!("csrw satp, {}", in(reg) satp, options(nostack, preserves_flags));
    sfence_vma();
}
