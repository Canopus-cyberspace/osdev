pub const SV39_MODE: usize = 8;
pub const SATP_MODE_SHIFT: usize = 60;
pub const SATP_PPN_MASK: usize = (1usize << 44) - 1;
pub const ENABLE_SV39_ACTIVATION_TEST: bool = false;

pub fn init() {
    crate::println!("[mm::sv39] activation scaffold init v36e");
}

pub fn test_scaffold() {
    crate::println!("[mm::sv39] activation scaffold test begin v36e");

    let root_ppn = 0x80234usize;
    let satp = make_satp(root_ppn);

    assert_eq!(satp_mode(satp), SV39_MODE);
    assert_eq!(satp_ppn(satp), root_ppn);

    let current = read_satp();
    crate::println!("[mm::sv39] current satp = {:#x}", current);
    crate::println!("[mm::sv39] candidate satp = {:#x}", satp);

    if ENABLE_SV39_ACTIVATION_TEST {
        crate::println!("[mm::sv39] activation test enabled unexpectedly");
        unsafe {
            activate_satp_unchecked(satp);
        }
    } else {
        crate::println!("[mm::sv39] activation disabled as expected");
    }

    crate::println!("[mm::sv39] activation scaffold test passed v36e");
}

#[inline]
pub const fn make_satp(root_ppn: usize) -> usize {
    (SV39_MODE << SATP_MODE_SHIFT) | (root_ppn & SATP_PPN_MASK)
}

#[inline]
pub const fn satp_mode(satp: usize) -> usize {
    satp >> SATP_MODE_SHIFT
}

#[inline]
pub const fn satp_ppn(satp: usize) -> usize {
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
/// This function changes the active address space.  It must only be called
/// after the kernel has a complete identity/high-half mapping and a valid trap
/// path in the target page table.  v36e deliberately does not call this.
#[inline]
pub unsafe fn activate_satp_unchecked(satp: usize) {
    core::arch::asm!("csrw satp, {}", in(reg) satp);
    sfence_vma();
}
