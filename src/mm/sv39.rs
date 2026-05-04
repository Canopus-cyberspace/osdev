pub const SATP_MODE_SV39: usize = 8usize << 60;
pub const PAGE_SIZE: usize = 4096;

pub fn init() {
    crate::println!("[mm::sv39] init v43e safe scaffold");
}

pub fn test_scaffold() {
    let ppn = 0x8020fusize;
    let satp = make_satp(ppn);

    crate::println!("[mm::sv39] scaffold test v43e");
    crate::println!("[mm::sv39] sample ppn = {:#x}", ppn);
    crate::println!("[mm::sv39] sample satp = {:#x}", satp);

    assert_eq!(satp_mode(satp), 8);
    assert_eq!(satp_ppn(satp), ppn);
}

#[inline]
pub const fn make_satp(root_ppn: usize) -> usize {
    SATP_MODE_SV39 | (root_ppn & ((1usize << 44) - 1))
}

#[inline]
pub const fn satp_mode(satp: usize) -> usize {
    satp >> 60
}

#[inline]
pub const fn satp_ppn(satp: usize) -> usize {
    satp & ((1usize << 44) - 1)
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
pub unsafe fn activate_satp_unchecked(satp: usize) {
    core::arch::asm!("csrw satp, {}", in(reg) satp);
    sfence_vma();
}

#[inline]
pub fn sfence_vma() {
    unsafe {
        core::arch::asm!("sfence.vma zero, zero");
    }
}
