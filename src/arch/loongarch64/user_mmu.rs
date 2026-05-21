use core::arch::global_asm;

const CSR_CRMD_DA: usize = 1 << 3;
const CSR_CRMD_PG: usize = 1 << 4;
const CSR_CRMD_DATF_MASK: usize = 0b11 << 5;
const CSR_CRMD_DATM_MASK: usize = 0b11 << 7;
const CSR_CRMD_DIRECT_MASK: usize =
    CSR_CRMD_DA | CSR_CRMD_PG | CSR_CRMD_DATF_MASK | CSR_CRMD_DATM_MASK;
const CSR_CRMD_PAGED_CACHED: usize = CSR_CRMD_PG | (0b01 << 5) | (0b01 << 7);

const TLBIDX_PS_SHIFT: usize = 24;
const HUGE_PAGE_SHIFT: usize = 21;
const HUGE_PAGE_SIZE: usize = 1 << HUGE_PAGE_SHIFT;
const HUGE_PAIR_SIZE: usize = HUGE_PAGE_SIZE * 2;
const TLBELO_V: usize = 1 << 0;
const TLBELO_D: usize = 1 << 1;
const TLBELO_PLV3: usize = 3 << 2;
const TLBELO_MAT_CC: usize = 1 << 4;
const TLBELO_G: usize = 1 << 6;
const TLBELO_USER_RW: usize = TLBELO_V | TLBELO_D | TLBELO_PLV3 | TLBELO_MAT_CC | TLBELO_G;

static mut SAVED_CRMD: usize = 0;
static mut SAVED_DMW0: usize = 0;
static mut ACTIVE: bool = false;

global_asm!(
    r#"
    .section .text
    .balign 4
    .globl loongarch64_mmu_read_crmd
loongarch64_mmu_read_crmd:
    csrrd $a0, 0x0
    ret

    .globl loongarch64_mmu_write_crmd
loongarch64_mmu_write_crmd:
    csrwr $a0, 0x0
    ibar 0
    dbar 0
    ret

    .globl loongarch64_mmu_read_dmw0
loongarch64_mmu_read_dmw0:
    csrrd $a0, 0x180
    ret

    .globl loongarch64_mmu_write_dmw0
loongarch64_mmu_write_dmw0:
    csrwr $a0, 0x180
    ibar 0
    dbar 0
    ret

    .globl loongarch64_mmu_flush_tlb_all
loongarch64_mmu_flush_tlb_all:
    invtlb 0x0, $zero, $zero
    ibar 0
    dbar 0
    ret

    .globl loongarch64_mmu_tlbfill
loongarch64_mmu_tlbfill:
    csrwr $a0, 0x10
    csrwr $a1, 0x11
    csrwr $a2, 0x12
    csrwr $a3, 0x13
    tlbfill
    ibar 0
    dbar 0
    ret
"#
);

extern "C" {
    fn loongarch64_mmu_read_crmd() -> usize;
    fn loongarch64_mmu_write_crmd(value: usize);
    fn loongarch64_mmu_read_dmw0() -> usize;
    fn loongarch64_mmu_write_dmw0(value: usize);
    fn loongarch64_mmu_flush_tlb_all();
    fn loongarch64_mmu_tlbfill(tlbidx: usize, tlbehi: usize, tlbelo0: usize, tlbelo1: usize);
}

pub(crate) fn begin_mapping_install() {
    unsafe {
        if !ACTIVE {
            SAVED_CRMD = loongarch64_mmu_read_crmd();
            SAVED_DMW0 = loongarch64_mmu_read_dmw0();
            ACTIVE = true;
        }
        // PLV0-only low-segment direct map keeps kernel/trap/MMIO accesses alive
        // after CRMD switches from direct-address mode to mapped-address mode.
        loongarch64_mmu_write_dmw0((0 << 60) | (0b01 << 4) | 0b0001);
        loongarch64_mmu_flush_tlb_all();
    }
}

pub(crate) fn map_huge_range(
    user_start: usize,
    host_start: usize,
    len: usize,
) -> Result<(), &'static str> {
    if len == 0 {
        return Ok(());
    }
    let user_base = align_down(user_start, HUGE_PAGE_SIZE);
    let user_offset = user_start - user_base;
    if host_start < user_offset {
        return Err("mmu_host_underflow");
    }
    let host_base = host_start - user_offset;
    if (host_base & (HUGE_PAGE_SIZE - 1)) != 0 {
        return Err("mmu_host_align");
    }
    let total = align_up(user_offset.checked_add(len).ok_or("mmu_len")?, HUGE_PAGE_SIZE);
    let mut offset = 0usize;
    while offset < total {
        let va = user_base.checked_add(offset).ok_or("mmu_va")?;
        let pa = host_base.checked_add(offset).ok_or("mmu_pa")?;
        let even_len = total - offset;
        let map_odd = even_len > HUGE_PAGE_SIZE;
        install_huge_pair(va, pa, map_odd);
        offset += if map_odd { HUGE_PAIR_SIZE } else { HUGE_PAGE_SIZE };
    }
    Ok(())
}

pub(crate) fn activate_paged_mode() {
    unsafe {
        let current = loongarch64_mmu_read_crmd();
        let next = (current & !CSR_CRMD_DIRECT_MASK) | CSR_CRMD_PAGED_CACHED;
        loongarch64_mmu_write_crmd(next);
    }
}

pub(crate) fn deactivate_paged_mode() {
    unsafe {
        if ACTIVE {
            loongarch64_mmu_write_crmd(SAVED_CRMD);
            loongarch64_mmu_write_dmw0(SAVED_DMW0);
            loongarch64_mmu_flush_tlb_all();
            ACTIVE = false;
        }
    }
}

fn install_huge_pair(va: usize, pa: usize, map_odd: bool) {
    let pair_va = align_down(va, HUGE_PAIR_SIZE);
    let page_odd = ((va - pair_va) / HUGE_PAGE_SIZE) != 0;
    let tlbidx = HUGE_PAGE_SHIFT << TLBIDX_PS_SHIFT;
    let tlbehi = pair_va;
    let first = (pa & !(HUGE_PAGE_SIZE - 1)) | TLBELO_USER_RW;
    let second = ((pa + HUGE_PAGE_SIZE) & !(HUGE_PAGE_SIZE - 1)) | TLBELO_USER_RW;
    let even_entry = if page_odd {
        0
    } else {
        first
    };
    let odd_entry = if page_odd {
        first
    } else if map_odd {
        second
    } else {
        0
    };
    unsafe {
        loongarch64_mmu_tlbfill(tlbidx, tlbehi, even_entry, odd_entry);
    }
}

fn align_down(value: usize, align: usize) -> usize {
    value & !(align - 1)
}

fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}
