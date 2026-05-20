fn with_sum_enabled<F: FnOnce()>(f: F) {
    let old = read_sstatus();
    unsafe {
        asm!("csrs sstatus, {}", in(reg) SSTATUS_SUM);
    }
    f();
    unsafe {
        asm!("csrw sstatus, {}", in(reg) old);
    }
}

fn with_sum_enabled_ret<R, F: FnOnce() -> R>(f: F) -> R {
    let old = read_sstatus();
    unsafe {
        asm!("csrs sstatus, {}", in(reg) SSTATUS_SUM);
    }
    let ret = f();
    unsafe {
        asm!("csrw sstatus, {}", in(reg) old);
    }
    ret
}

fn read_sstatus() -> usize {
    let value: usize;
    unsafe {
        asm!("csrr {}, sstatus", out(reg) value);
    }
    value
}

fn read_scause() -> usize {
    let value: usize;
    unsafe {
        asm!("csrr {}, scause", out(reg) value);
    }
    value
}

fn read_stval() -> usize {
    let value: usize;
    unsafe {
        asm!("csrr {}, stval", out(reg) value);
    }
    value
}

fn read_satp() -> usize {
    let value: usize;
    unsafe {
        asm!("csrr {}, satp", out(reg) value);
    }
    value
}

fn root_pa() -> usize {
    core::ptr::addr_of!(ROOT_TABLE) as usize
}
const fn table_pte(pa: usize) -> usize {
    ((pa >> 12) << 10) | PTE_V
}
const fn leaf_1g_pte(pa: usize, flags: usize) -> usize {
    ((pa >> 12) << 10) | flags
}
const fn leaf_2m_pte(pa: usize, flags: usize) -> usize {
    ((pa >> 12) << 10) | flags
}
const fn leaf_4k_pte(pa: usize, flags: usize) -> usize {
    ((pa >> 12) << 10) | flags
}
const fn vpn0(va: usize) -> usize {
    (va >> 12) & 0x1ff
}
const fn vpn1(va: usize) -> usize {
    (va >> 21) & 0x1ff
}
const fn vpn2(va: usize) -> usize {
    (va >> 30) & 0x1ff
}
