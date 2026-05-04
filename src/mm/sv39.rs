pub const SATP_MODE_SV39: usize = 8;
pub const ENABLE_SV39_ACTIVATION_TEST: bool = false;

pub fn init() {
    crate::println!("[mm::sv39] scaffold init v40d");
}

pub fn make_satp(root_ppn: usize) -> usize {
    (SATP_MODE_SV39 << 60) | root_ppn
}

pub fn satp_mode(satp: usize) -> usize {
    satp >> 60
}

pub fn satp_ppn(satp: usize) -> usize {
    satp & ((1usize << 44) - 1)
}

pub fn read_satp() -> usize {
    let value: usize;
    unsafe {
        core::arch::asm!("csrr {}, satp", out(reg) value);
    }
    value
}

pub fn sfence_vma() {
    unsafe {
        core::arch::asm!("sfence.vma", options(nostack, preserves_flags));
    }
}

pub unsafe fn activate_satp_unchecked(satp: usize) {
    core::arch::asm!("csrw satp, {}", in(reg) satp);
    sfence_vma();
}

pub fn test_scaffold() {
    crate::println!("[sv39-v40d] scaffold begin");

    let root_ppn = 0x80200usize;
    let satp = make_satp(root_ppn);

    assert_eq!(satp_mode(satp), SATP_MODE_SV39);
    assert_eq!(satp_ppn(satp), root_ppn);

    let current = read_satp();
    crate::println!("[sv39-v40d] current satp = {:#x}", current);
    crate::println!("[sv39-v40d] sample satp  = {:#x}", satp);

    if ENABLE_SV39_ACTIVATION_TEST {
        crate::println!("[sv39-v40d] activation test requested but intentionally not run in v40d");
    } else {
        crate::println!("[sv39-v40d] activation disabled");
    }

    crate::println!("[sv39-v40d] scaffold passed");
}
