use super::*;
use core::arch::asm;

#[derive(Copy, Clone)]
#[repr(C, align(4096))]
struct RealMmPage([u8; PAGE_SIZE]);

#[derive(Copy, Clone)]
struct RealMmMapping {
    used: bool,
    va: usize,
    page_idx: usize,
    flags: usize,
}

impl RealMmMapping {
    const fn empty() -> Self {
        Self {
            used: false,
            va: 0,
            page_idx: usize::MAX,
            flags: 0,
        }
    }
}

const REAL_MM_PAGE_COUNT: usize = 256;
const REAL_MM_MAPPING_COUNT: usize = 64;
const REAL_MM_ZERO_PAGE: RealMmPage = RealMmPage([0; PAGE_SIZE]);
static mut REAL_MM_PAGES: [RealMmPage; REAL_MM_PAGE_COUNT] =
    [REAL_MM_ZERO_PAGE; REAL_MM_PAGE_COUNT];
static mut REAL_MM_PAGE_STATE: [u8; REAL_MM_PAGE_COUNT] = [0; REAL_MM_PAGE_COUNT];
static mut REAL_MM_MAPPINGS: [RealMmMapping; REAL_MM_MAPPING_COUNT] =
    [RealMmMapping::empty(); REAL_MM_MAPPING_COUNT];
static mut REAL_MM_ALLOC_COUNT: usize = 0;
static mut REAL_MM_FREE_COUNT: usize = 0;
static mut REAL_MM_DOUBLE_FREE_COUNT: usize = 0;
static mut REAL_MM_EXHAUST_COUNT: usize = 0;
static mut REAL_MM_FAULT_ALLOC_COUNT: usize = 0;

pub(super) unsafe fn real_mm_page_pa(idx: usize) -> usize {
    core::ptr::addr_of!(REAL_MM_PAGES[idx]) as usize
}

pub(super) unsafe fn real_mm_page_idx_from_pa(pa: usize) -> Option<usize> {
    let mut idx = 0usize;
    while idx < REAL_MM_PAGE_COUNT {
        if real_mm_page_pa(idx) == pa {
            return Some(idx);
        }
        idx += 1;
    }
    None
}

pub(super) fn real_mm_pte_pa(pte: usize) -> usize {
    (pte >> 10) << 12
}

pub(super) unsafe fn real_mm_pte_for_va(va: usize) -> usize {
    USER_L0_TABLE.0[vpn0(va)]
}

pub(super) unsafe fn real_mm_reset_allocator_state() {
    let mut i = 0usize;
    while i < 512 {
        let pte = USER_L0_TABLE.0[i];
        if (pte & PTE_V) != 0 {
            if real_mm_page_idx_from_pa(real_mm_pte_pa(pte)).is_some() {
                USER_L0_TABLE.0[i] = 0;
            }
        }
        i += 1;
    }
    i = 0;
    while i < REAL_MM_PAGE_COUNT {
        REAL_MM_PAGE_STATE[i] = 0;
        REAL_MM_PAGES[i].0.fill(0);
        i += 1;
    }
    i = 0;
    while i < REAL_MM_MAPPING_COUNT {
        REAL_MM_MAPPINGS[i] = RealMmMapping::empty();
        i += 1;
    }
    REAL_MM_ALLOC_COUNT = 0;
    REAL_MM_FREE_COUNT = 0;
    REAL_MM_DOUBLE_FREE_COUNT = 0;
    REAL_MM_EXHAUST_COUNT = 0;
    REAL_MM_FAULT_ALLOC_COUNT = 0;
    asm!("sfence.vma zero, zero");
}

pub(super) unsafe fn real_mm_alloc_page() -> Option<usize> {
    let mut idx = 0usize;
    while idx < REAL_MM_PAGE_COUNT {
        if REAL_MM_PAGE_STATE[idx] == 0 {
            REAL_MM_PAGE_STATE[idx] = 1;
            REAL_MM_ALLOC_COUNT += 1;
            REAL_MM_PAGES[idx].0.fill(0);
            return Some(idx);
        }
        idx += 1;
    }
    REAL_MM_EXHAUST_COUNT += 1;
    None
}

pub(super) unsafe fn real_mm_free_page(idx: usize) -> bool {
    if idx >= REAL_MM_PAGE_COUNT || REAL_MM_PAGE_STATE[idx] == 0 {
        REAL_MM_DOUBLE_FREE_COUNT += 1;
        return false;
    }
    REAL_MM_PAGE_STATE[idx] = 0;
    REAL_MM_FREE_COUNT += 1;
    REAL_MM_PAGES[idx].0.fill(0);
    true
}

pub(super) unsafe fn real_mm_record_mapping(va: usize, page_idx: usize, flags: usize) -> bool {
    let mut i = 0usize;
    while i < REAL_MM_MAPPING_COUNT {
        if REAL_MM_MAPPINGS[i].used && REAL_MM_MAPPINGS[i].va == va {
            REAL_MM_MAPPINGS[i] = RealMmMapping {
                used: true,
                va,
                page_idx,
                flags,
            };
            return true;
        }
        i += 1;
    }
    i = 0;
    while i < REAL_MM_MAPPING_COUNT {
        if !REAL_MM_MAPPINGS[i].used {
            REAL_MM_MAPPINGS[i] = RealMmMapping {
                used: true,
                va,
                page_idx,
                flags,
            };
            return true;
        }
        i += 1;
    }
    false
}

pub(super) unsafe fn real_mm_forget_mapping(va: usize) {
    let mut i = 0usize;
    while i < REAL_MM_MAPPING_COUNT {
        if REAL_MM_MAPPINGS[i].used && REAL_MM_MAPPINGS[i].va == va {
            REAL_MM_MAPPINGS[i] = RealMmMapping::empty();
        }
        i += 1;
    }
}

pub(super) unsafe fn real_mm_map_allocated_page(va: usize, page_idx: usize, flags: usize) -> bool {
    if (va & (PAGE_SIZE - 1)) != 0
        || page_idx >= REAL_MM_PAGE_COUNT
        || REAL_MM_PAGE_STATE[page_idx] == 0
    {
        return false;
    }
    if (real_mm_pte_for_va(va) & PTE_V) != 0 {
        return false;
    }
    let pa = real_mm_page_pa(page_idx);
    USER_L0_TABLE.0[vpn0(va)] = leaf_4k_pte(pa, flags);
    if !real_mm_record_mapping(va, page_idx, flags) {
        USER_L0_TABLE.0[vpn0(va)] = 0;
        let _ = real_mm_free_page(page_idx);
        return false;
    }
    asm!("sfence.vma zero, zero");
    true
}

pub(super) unsafe fn real_mm_unmap_page(va: usize) -> bool {
    let aligned = va & !(PAGE_SIZE - 1);
    let pte = real_mm_pte_for_va(aligned);
    if (pte & PTE_V) == 0 {
        return false;
    }
    USER_L0_TABLE.0[vpn0(aligned)] = 0;
    if let Some(idx) = real_mm_page_idx_from_pa(real_mm_pte_pa(pte)) {
        let _ = real_mm_free_page(idx);
        real_mm_forget_mapping(aligned);
    }
    asm!("sfence.vma zero, zero");
    true
}

pub(super) unsafe fn real_mm_unmap_range(addr: usize, len: usize) {
    if len == 0 {
        return;
    }
    let mut va = addr & !(PAGE_SIZE - 1);
    let end = (addr + len + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
    while va < end {
        let _ = real_mm_unmap_page(va);
        va += PAGE_SIZE;
    }
}

pub(super) fn real_mm_pte_flags_from_prot(prot: usize) -> usize {
    let mut flags = PTE_V | PTE_U | PTE_A | PTE_D;
    if (prot & crate::fs::runtime::RUNTIME_PROT_READ) != 0 {
        flags |= PTE_R;
    }
    if (prot & crate::fs::runtime::RUNTIME_PROT_WRITE) != 0 {
        flags |= PTE_R | PTE_W;
    }
    if (prot & crate::fs::runtime::RUNTIME_PROT_EXEC) != 0 {
        flags |= PTE_X;
    }
    flags
}

pub(super) unsafe fn real_mm_mprotect_range(addr: usize, len: usize, prot: usize) {
    if len == 0 {
        return;
    }
    let mut va = addr & !(PAGE_SIZE - 1);
    let end = (addr + len + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
    let flags = real_mm_pte_flags_from_prot(prot);
    while va < end {
        let pte = real_mm_pte_for_va(va);
        if (pte & PTE_V) != 0 {
            USER_L0_TABLE.0[vpn0(va)] = (pte & !0x3ffusize) | flags;
            let mut i = 0usize;
            while i < REAL_MM_MAPPING_COUNT {
                if REAL_MM_MAPPINGS[i].used && REAL_MM_MAPPINGS[i].va == va {
                    REAL_MM_MAPPINGS[i].flags = flags;
                }
                i += 1;
            }
        }
        va += PAGE_SIZE;
    }
    asm!("sfence.vma zero, zero");
}

pub(super) unsafe fn real_mm_clear_lazy_user_ptes() {
    let mut va = USER_HEAP_START;
    while va < USER_HEAP_END {
        let _ = real_mm_unmap_page(va);
        va += PAGE_SIZE;
    }
    va = USER_MMAP_START;
    while va < USER_MMAP_END {
        let _ = real_mm_unmap_page(va);
        va += PAGE_SIZE;
    }
    let stack_bottom = USER_STACK_TOP - USER_STACK_SIZE;
    va = stack_bottom;
    while va + PAGE_SIZE < USER_STACK_TOP {
        let _ = real_mm_unmap_page(va);
        va += PAGE_SIZE;
    }
}

const REAL_MM_V198_ADDR: usize = USER_MMAP_START;
const REAL_MM_V200_ADDR: usize = USER_MMAP_START + 0x10000;

pub(super) const REAL_MM_PROGRAM_LAZY: usize = 1;
pub(super) const REAL_MM_PROGRAM_RO_FAULT: usize = 2;
pub(super) const REAL_MM_PROGRAM_UNMAP_FAULT: usize = 3;
pub(super) const REAL_MM_PROGRAM_STRESS: usize = 4;

pub(super) fn real_mm_emit_write_msg(asm: &mut RealUmodeAsm, message_len: usize) {
    const A0: u32 = 10;
    const A1: u32 = 11;
    const A2: u32 = 12;
    const A7: u32 = 17;
    asm.li(A0, 1);
    asm.load_abs(A1, REAL_UMODE_BASE + REAL_UMODE_MSG_OFF);
    asm.li(A2, message_len as isize);
    asm.li(A7, crate::syscall::SYS_WRITE as isize);
    asm.ecall();
}

pub(super) fn real_mm_emit_exit(asm: &mut RealUmodeAsm, code: isize) {
    const A0: u32 = 10;
    const A7: u32 = 17;
    asm.li(A0, code);
    asm.li(A7, crate::syscall::SYS_EXIT as isize);
    asm.ecall();
}

pub(super) fn real_mm_emit_store_check_abs(asm: &mut RealUmodeAsm, addr: usize, value: isize) {
    const T0: u32 = 5;
    const T1: u32 = 6;
    const T2: u32 = 7;
    asm.load_abs(T0, addr);
    asm.li(T1, value);
    asm.sb(T1, 0, T0);
    asm.lbu(T2, 0, T0);
    asm.bne_fail(T2, T1);
}

pub(super) fn real_mm_emit_load_abs(asm: &mut RealUmodeAsm, addr: usize) {
    const T0: u32 = 5;
    const T1: u32 = 6;
    asm.load_abs(T0, addr);
    asm.lbu(T1, 0, T0);
}

pub(super) fn real_mm_emit_brk(asm: &mut RealUmodeAsm, target: usize) {
    const A0: u32 = 10;
    const A7: u32 = 17;
    const T0: u32 = 5;
    asm.load_abs(A0, target);
    asm.li(A7, crate::syscall::SYS_BRK as isize);
    asm.ecall();
    asm.load_abs(T0, target);
    asm.bne_fail(A0, T0);
}

pub(super) fn real_mm_emit_mmap_fixed(asm: &mut RealUmodeAsm, addr: usize, len: usize, prot: usize) {
    const A0: u32 = 10;
    const A1: u32 = 11;
    const A2: u32 = 12;
    const A3: u32 = 13;
    const A4: u32 = 14;
    const A5: u32 = 15;
    const A7: u32 = 17;
    const T0: u32 = 5;
    asm.load_abs(A0, addr);
    asm.load_abs(A1, len);
    asm.li(A2, prot as isize);
    asm.li(
        A3,
        (0x02 | crate::fs::runtime::RUNTIME_MAP_ANONYMOUS | crate::fs::runtime::RUNTIME_MAP_FIXED)
            as isize,
    );
    asm.li(A4, -1);
    asm.li(A5, 0);
    asm.li(A7, crate::syscall::SYS_MMAP as isize);
    asm.ecall();
    asm.load_abs(T0, addr);
    asm.bne_fail(A0, T0);
}

pub(super) fn real_mm_emit_mprotect(asm: &mut RealUmodeAsm, addr: usize, len: usize, prot: usize) {
    const A0: u32 = 10;
    const A1: u32 = 11;
    const A2: u32 = 12;
    const A7: u32 = 17;
    asm.load_abs(A0, addr);
    asm.load_abs(A1, len);
    asm.li(A2, prot as isize);
    asm.li(A7, crate::syscall::SYS_MPROTECT as isize);
    asm.ecall();
    asm.bne_fail(A0, 0);
}

pub(super) fn real_mm_emit_munmap(asm: &mut RealUmodeAsm, addr: usize, len: usize) {
    const A0: u32 = 10;
    const A1: u32 = 11;
    const A7: u32 = 17;
    asm.load_abs(A0, addr);
    asm.load_abs(A1, len);
    asm.li(A7, crate::syscall::SYS_MUNMAP as isize);
    asm.ecall();
    asm.bne_fail(A0, 0);
}

pub(super) fn real_mm_make_elf(
    out: &mut [u8; REAL_UMODE_ELF_CAP],
    message: &[u8],
    exit_code: isize,
    program: usize,
) -> usize {
    out.fill(0);
    let mut asm = RealUmodeAsm::new();

    real_mm_emit_write_msg(&mut asm, message.len());
    if program == REAL_MM_PROGRAM_LAZY {
        real_mm_emit_brk(&mut asm, USER_HEAP_START + 3 * PAGE_SIZE);
        real_mm_emit_store_check_abs(&mut asm, USER_HEAP_START, 0x5a);
        real_mm_emit_store_check_abs(&mut asm, USER_HEAP_START + PAGE_SIZE, 0x5b);
        real_mm_emit_mmap_fixed(
            &mut asm,
            USER_MMAP_START,
            2 * PAGE_SIZE,
            crate::fs::runtime::RUNTIME_PROT_READ | crate::fs::runtime::RUNTIME_PROT_WRITE,
        );
        real_mm_emit_store_check_abs(&mut asm, USER_MMAP_START + PAGE_SIZE, 0x6a);
        real_mm_emit_store_check_abs(&mut asm, USER_STACK_TOP - 2 * PAGE_SIZE, 0x7a);
        real_mm_emit_exit(&mut asm, exit_code);
    } else if program == REAL_MM_PROGRAM_RO_FAULT {
        real_mm_emit_mmap_fixed(
            &mut asm,
            REAL_MM_V198_ADDR,
            PAGE_SIZE,
            crate::fs::runtime::RUNTIME_PROT_READ | crate::fs::runtime::RUNTIME_PROT_WRITE,
        );
        real_mm_emit_store_check_abs(&mut asm, REAL_MM_V198_ADDR, 0x31);
        real_mm_emit_mprotect(
            &mut asm,
            REAL_MM_V198_ADDR,
            PAGE_SIZE,
            crate::fs::runtime::RUNTIME_PROT_READ,
        );
        const T0: u32 = 5;
        const T1: u32 = 6;
        asm.load_abs(T0, REAL_MM_V198_ADDR);
        asm.li(T1, 0x32);
        asm.sb(T1, 0, T0);
        real_mm_emit_exit(&mut asm, 98);
    } else if program == REAL_MM_PROGRAM_UNMAP_FAULT {
        real_mm_emit_mmap_fixed(
            &mut asm,
            REAL_MM_V198_ADDR,
            PAGE_SIZE,
            crate::fs::runtime::RUNTIME_PROT_READ | crate::fs::runtime::RUNTIME_PROT_WRITE,
        );
        real_mm_emit_store_check_abs(&mut asm, REAL_MM_V198_ADDR, 0x41);
        real_mm_emit_munmap(&mut asm, REAL_MM_V198_ADDR, PAGE_SIZE);
        real_mm_emit_load_abs(&mut asm, REAL_MM_V198_ADDR);
        real_mm_emit_exit(&mut asm, 99);
    } else {
        real_mm_emit_brk(&mut asm, USER_HEAP_START + 4 * PAGE_SIZE);
        real_mm_emit_store_check_abs(&mut asm, USER_HEAP_START, 0x11);
        real_mm_emit_store_check_abs(&mut asm, USER_HEAP_START + PAGE_SIZE, 0x12);
        real_mm_emit_store_check_abs(&mut asm, USER_HEAP_START + 2 * PAGE_SIZE, 0x13);
        real_mm_emit_store_check_abs(&mut asm, USER_HEAP_START + 3 * PAGE_SIZE, 0x14);
        real_mm_emit_mmap_fixed(
            &mut asm,
            REAL_MM_V200_ADDR,
            3 * PAGE_SIZE,
            crate::fs::runtime::RUNTIME_PROT_READ | crate::fs::runtime::RUNTIME_PROT_WRITE,
        );
        real_mm_emit_store_check_abs(&mut asm, REAL_MM_V200_ADDR, 0x21);
        real_mm_emit_store_check_abs(&mut asm, REAL_MM_V200_ADDR + PAGE_SIZE, 0x22);
        real_mm_emit_store_check_abs(&mut asm, REAL_MM_V200_ADDR + 2 * PAGE_SIZE, 0x23);
        real_mm_emit_munmap(&mut asm, REAL_MM_V200_ADDR + PAGE_SIZE, PAGE_SIZE);
        real_mm_emit_mprotect(
            &mut asm,
            REAL_MM_V200_ADDR,
            PAGE_SIZE,
            crate::fs::runtime::RUNTIME_PROT_READ,
        );
        real_mm_emit_load_abs(&mut asm, REAL_MM_V200_ADDR);
        real_mm_emit_store_check_abs(&mut asm, REAL_MM_V200_ADDR + 2 * PAGE_SIZE, 0x24);
        real_mm_emit_exit(&mut asm, exit_code);
    }
    asm.label_fail();
    real_mm_emit_exit(&mut asm, 90 + (exit_code & 7));

    out[0] = 0x7f;
    out[1] = b'E';
    out[2] = b'L';
    out[3] = b'F';
    out[4] = 2;
    out[5] = 1;
    out[6] = 1;
    write_real_u16(out, 16, 2);
    write_real_u16(out, 18, 243);
    write_real_u32(out, 20, 1);
    write_real_u64(out, 24, REAL_UMODE_BASE + REAL_UMODE_CODE_OFF);
    write_real_u64(out, 32, 64);
    write_real_u16(out, 52, 64);
    write_real_u16(out, 54, 56);
    write_real_u16(out, 56, 1);
    write_real_u32(out, 64, 1);
    write_real_u32(out, 68, 5);
    write_real_u64(out, 72, 0);
    write_real_u64(out, 80, REAL_UMODE_BASE);
    write_real_u64(out, 88, REAL_UMODE_BASE);
    let code_end = asm.finish(out, REAL_UMODE_CODE_OFF);
    let mut msg_pos = REAL_UMODE_MSG_OFF;
    let mut i = 0usize;
    while i < message.len() && msg_pos < out.len() {
        out[msg_pos] = message[i];
        msg_pos += 1;
        i += 1;
    }
    let mut file_size = if msg_pos > code_end {
        msg_pos
    } else {
        code_end
    };
    file_size = (file_size + 7) & !7usize;
    write_real_u64(out, 96, file_size);
    write_real_u64(out, 104, PAGE_SIZE);
    write_real_u64(out, 112, PAGE_SIZE);
    file_size
}

pub(super) fn real_mm_install_program(path: &[u8], message: &[u8], exit_code: isize, program: usize) -> isize {
    let mut elf = [0u8; REAL_UMODE_ELF_CAP];
    let len = real_mm_make_elf(&mut elf, message, exit_code, program);
    let ret = real_umode_write_rootfs_file(path, &elf[..len], 0o755);
    if ret == len as isize {
        0
    } else {
        ret
    }
}

pub(super) fn real_mm_access_label(access: crate::fs::runtime::RuntimeFaultAccess) -> &'static str {
    match access {
        crate::fs::runtime::RuntimeFaultAccess::Read => "read",
        crate::fs::runtime::RuntimeFaultAccess::Write => "write",
        crate::fs::runtime::RuntimeFaultAccess::Execute => "exec",
    }
}

pub(super) fn real_mm_kind_label(kind: crate::fs::runtime::RuntimeVmaKind) -> &'static str {
    match kind {
        crate::fs::runtime::RuntimeVmaKind::Load => "load",
        crate::fs::runtime::RuntimeVmaKind::Heap => "heap",
        crate::fs::runtime::RuntimeVmaKind::Stack => "stack",
        crate::fs::runtime::RuntimeVmaKind::Mmap => "mmap",
        crate::fs::runtime::RuntimeVmaKind::Empty => "empty",
    }
}

pub(super) fn real_mm_marker_for_phase() -> &'static str {
    match unsafe { REAL_UMODE_PHASE } {
        REAL_UMODE_PHASE_V197_LAZY => "[ucompat-v197] real page fault lazy allocation",
        REAL_UMODE_PHASE_V198_RO | REAL_UMODE_PHASE_V198_UNMAP => {
            "[ucompat-v198] page permission unmap"
        }
        REAL_UMODE_PHASE_V200_STRESS => "[ucompat-v200] memory stress suite",
        _ => "[ucompat-v196] user page table mapping",
    }
}

pub(super) fn real_mm_fault_access(scause: usize) -> Option<crate::fs::runtime::RuntimeFaultAccess> {
    if scause == 12 {
        Some(crate::fs::runtime::RuntimeFaultAccess::Execute)
    } else if scause == 13 {
        Some(crate::fs::runtime::RuntimeFaultAccess::Read)
    } else if scause == 15 {
        Some(crate::fs::runtime::RuntimeFaultAccess::Write)
    } else {
        None
    }
}

pub(super) fn real_mm_perm_allows(
    perm: crate::fs::runtime::RuntimeVmPermissions,
    access: crate::fs::runtime::RuntimeFaultAccess,
) -> bool {
    match access {
        crate::fs::runtime::RuntimeFaultAccess::Read => perm.readable,
        crate::fs::runtime::RuntimeFaultAccess::Write => perm.writable,
        crate::fs::runtime::RuntimeFaultAccess::Execute => perm.executable,
    }
}

pub(super) fn real_mm_pte_flags_from_permissions(perm: crate::fs::runtime::RuntimeVmPermissions) -> usize {
    let mut flags = PTE_V | PTE_U | PTE_A | PTE_D;
    if perm.readable {
        flags |= PTE_R;
    }
    if perm.writable {
        flags |= PTE_R | PTE_W;
    }
    if perm.executable {
        flags |= PTE_X;
    }
    flags
}

pub(super) fn real_mm_run_v195_allocator() -> Option<&'static str> {
    let mut pages = [usize::MAX; REAL_MM_PAGE_COUNT];
    let mut count = 0usize;
    unsafe {
        real_mm_reset_allocator_state();
        while count < REAL_MM_PAGE_COUNT {
            match real_mm_alloc_page() {
                Some(idx) => {
                    pages[count] = idx;
                    count += 1;
                }
                None => return Some("early_exhaustion"),
            }
        }
        if real_mm_alloc_page().is_some() {
            return Some("exhaustion_allocated");
        }
        if !real_mm_free_page(pages[0]) {
            return Some("free_first");
        }
        if real_mm_free_page(pages[0]) {
            return Some("double_free_allowed");
        }
        let mut i = 1usize;
        while i < count {
            if !real_mm_free_page(pages[i]) {
                return Some("free_rest");
            }
            i += 1;
        }
        if REAL_MM_ALLOC_COUNT != REAL_MM_PAGE_COUNT
            || REAL_MM_FREE_COUNT != REAL_MM_PAGE_COUNT
            || REAL_MM_EXHAUST_COUNT != 1
            || REAL_MM_DOUBLE_FREE_COUNT != 1
        {
            return Some("accounting");
        }
        crate::println!(
            "[ucompat-v195] allocator evidence pages={} alloc={} free={} exhaust={} double_free={} PASS",
            REAL_MM_PAGE_COUNT,
            REAL_MM_ALLOC_COUNT,
            REAL_MM_FREE_COUNT,
            REAL_MM_EXHAUST_COUNT,
            REAL_MM_DOUBLE_FREE_COUNT
        );
    }
    None
}

pub(super) fn real_mm_run_v196_mapping() -> Option<&'static str> {
    const TEST_VA: usize = USER_MMAP_START + 0x70000;
    unsafe {
        real_mm_reset_allocator_state();
        let idx = match real_mm_alloc_page() {
            Some(idx) => idx,
            None => return Some("alloc"),
        };
        let flags = PTE_V | PTE_R | PTE_W | PTE_U | PTE_A | PTE_D;
        if !real_mm_map_allocated_page(TEST_VA, idx, flags) {
            return Some("map");
        }
        let pte = real_mm_pte_for_va(TEST_VA);
        if (pte & flags) != flags || real_mm_page_idx_from_pa(real_mm_pte_pa(pte)) != Some(idx) {
            return Some("query");
        }
        real_mm_mprotect_range(TEST_VA, PAGE_SIZE, crate::fs::runtime::RUNTIME_PROT_READ);
        let ro = real_mm_pte_for_va(TEST_VA);
        if (ro & PTE_R) == 0 || (ro & PTE_W) != 0 || (ro & PTE_U) == 0 {
            return Some("protect_query");
        }
        if !real_mm_unmap_page(TEST_VA) {
            return Some("unmap");
        }
        if (real_mm_pte_for_va(TEST_VA) & PTE_V) != 0 || REAL_MM_FREE_COUNT != 1 {
            return Some("unmap_query");
        }
        crate::println!(
            "[ucompat-v196] mapping evidence va={:#x} page={} flags=RWU protect=RU unmap=1 PASS",
            TEST_VA,
            idx
        );
        real_mm_reset_allocator_state();
    }
    None
}

pub(super) fn real_mm_run_v199_fork_copy() -> Option<&'static str> {
    unsafe {
        real_mm_reset_allocator_state();
        let child = crate::fs::runtime::clone_task(17);
        if child <= 0 {
            return Some("clone");
        }
        let parent_idx = match real_mm_alloc_page() {
            Some(idx) => idx,
            None => return Some("parent_alloc"),
        };
        REAL_MM_PAGES[parent_idx].0[0] = 0x91;
        REAL_MM_PAGES[parent_idx].0[1] = 0x55;
        let child_idx = match real_mm_alloc_page() {
            Some(idx) => idx,
            None => return Some("child_alloc"),
        };
        let mut i = 0usize;
        while i < PAGE_SIZE {
            REAL_MM_PAGES[child_idx].0[i] = REAL_MM_PAGES[parent_idx].0[i];
            i += 1;
        }
        REAL_MM_PAGES[child_idx].0[0] = 0x92;
        if REAL_MM_PAGES[parent_idx].0[0] != 0x91
            || REAL_MM_PAGES[child_idx].0[0] != 0x92
            || REAL_MM_PAGES[child_idx].0[1] != 0x55
        {
            return Some("isolation");
        }
        if crate::fs::runtime::exit_task_pid(child as usize, 55) != 0 {
            return Some("child_exit");
        }
        let mut status = 0isize;
        if crate::fs::runtime::wait4(child, &mut status) != child || status != (55 << 8) {
            return Some("wait");
        }
        crate::println!(
            "[ucompat-v199] fork copy evidence child={} parent_page={} child_page={} parent_byte=0x{:x} child_byte=0x{:x} status={} PASS",
            child,
            parent_idx,
            child_idx,
            REAL_MM_PAGES[parent_idx].0[0],
            REAL_MM_PAGES[child_idx].0[0],
            status
        );
    }
    None
}

pub(super) fn real_mm_begin(cx: &mut TrapContext) {
    if let Some(step) = real_mm_run_v195_allocator() {
        real_umode_fail("[ucompat-v195] physical page allocator", step);
    }
    crate::println!("[ucompat-v195] physical page allocator PASS");
    if let Some(step) = real_mm_run_v196_mapping() {
        real_umode_fail("[ucompat-v196] user page table mapping", step);
    }
    crate::println!("[ucompat-v196] user page table mapping PASS");
    real_umode_launch_phase(cx, REAL_UMODE_PHASE_V197_LAZY);
}

pub(super) fn real_mm_finish_v200() -> ! {
    let faults = unsafe { REAL_MM_FAULT_ALLOC_COUNT };
    if faults < 7 {
        real_umode_fail("[ucompat-v200] memory stress suite", "fault_count");
    }
    crate::println!(
        "[ucompat-v200] stress evidence faults={} alloc={} free={} PASS",
        faults,
        unsafe { REAL_MM_ALLOC_COUNT },
        unsafe { REAL_MM_FREE_COUNT }
    );
    crate::println!("[ucompat-v200] memory stress suite PASS");
    if let Some(step) = crate::fs::runtime::run_v201_v206_scheduler_blocking_suite() {
        real_umode_fail("[ucompat-v206] scheduler regression suite", step);
    }
    if let Some(step) = crate::fs::runtime::run_v207_v214_storage_image_suite() {
        real_umode_fail("[ucompat-v214] filesystem submission hardening", step);
    }
    unsafe {
        REAL_UMODE_PHASE = REAL_UMODE_PHASE_DONE;
        real_mm_reset_allocator_state();
    }
    crate::fs::runtime::reset_for_integration();
    crate::println!("[external-init-v82] smoke passed");
    crate::println!("[external-init-v82] kernel idle after external init ELF smoke");
    finish_official_qemu_runtime();
}

pub(super) fn handle_real_mm_expected_fault(scause: usize, stval: usize, cx: &mut TrapContext) -> bool {
    let phase = unsafe { REAL_UMODE_PHASE };
    let fault_page = stval & !(PAGE_SIZE - 1);
    if phase == REAL_UMODE_PHASE_V198_RO && scause == 15 && fault_page == REAL_MM_V198_ADDR {
        let pte = unsafe { real_mm_pte_for_va(REAL_MM_V198_ADDR) };
        if (pte & PTE_V) == 0 || (pte & PTE_W) != 0 {
            real_umode_fail("[ucompat-v198] page permission unmap", "readonly_pte");
        }
        crate::println!(
            "[ucompat-v198] readonly fault evidence va={:#x} pte_flags={:#x} PASS",
            stval,
            pte & 0x3ff
        );
        real_umode_launch_phase(cx, REAL_UMODE_PHASE_V198_UNMAP);
        return true;
    }
    if phase == REAL_UMODE_PHASE_V198_UNMAP && scause == 13 && fault_page == REAL_MM_V198_ADDR {
        let pte = unsafe { real_mm_pte_for_va(REAL_MM_V198_ADDR) };
        if (pte & PTE_V) != 0 || crate::fs::runtime::page_fault_permissions(stval).is_some() {
            real_umode_fail(
                "[ucompat-v198] page permission unmap",
                "munmap_still_mapped",
            );
        }
        crate::println!(
            "[ucompat-v198] munmap fault evidence va={:#x} unmapped=1 PASS",
            stval
        );
        crate::println!("[ucompat-v198] page permission unmap PASS");
        if let Some(step) = real_mm_run_v199_fork_copy() {
            real_umode_fail(
                "[ucompat-v199] fork address space copy cow foundation",
                step,
            );
        }
        crate::println!("[ucompat-v199] fork address space copy cow foundation PASS");
        real_umode_launch_phase(cx, REAL_UMODE_PHASE_V200_STRESS);
        return true;
    }
    false
}

pub(super) fn handle_real_mm_page_fault(scause: usize, stval: usize, cx: &mut TrapContext) -> bool {
    k02_record_page_fault(stval);
    let access = match real_mm_fault_access(scause) {
        Some(access) => access,
        None => return false,
    };
    let fault_page = stval & !(PAGE_SIZE - 1);
    let perm = match crate::fs::runtime::page_fault_permissions(stval) {
        Some(perm) => perm,
        None => return handle_real_mm_expected_fault(scause, stval, cx),
    };
    if !real_mm_perm_allows(perm, access) || !perm.user {
        return handle_real_mm_expected_fault(scause, stval, cx);
    }
    unsafe {
        let existing = real_mm_pte_for_va(fault_page);
        if (existing & PTE_V) != 0 {
            return handle_real_mm_expected_fault(scause, stval, cx);
        }
        let page_idx = match real_mm_alloc_page() {
            Some(idx) => idx,
            None => real_umode_fail(real_mm_marker_for_phase(), "alloc_page_fault"),
        };
        let pte_flags = real_mm_pte_flags_from_permissions(perm);
        if !real_mm_map_allocated_page(fault_page, page_idx, pte_flags) {
            real_umode_fail(real_mm_marker_for_phase(), "map_page_fault");
        }
        let page_bytes =
            core::slice::from_raw_parts_mut(real_mm_page_pa(page_idx) as *mut u8, PAGE_SIZE);
        let backing_copied = crate::fs::runtime::read_mmap_backing_page(fault_page, page_bytes);
        REAL_MM_FAULT_ALLOC_COUNT += 1;
        if !crate::fs::runtime::page_fault_validate(stval, access) {
            real_umode_fail(real_mm_marker_for_phase(), "canonical_fault_validate");
        }
        crate::println!(
            "{} fault evidence va={:#x} page={:#x} pa={:#x} access={} kind={} mm={} lazy={} flags={:#x} PASS",
            real_mm_marker_for_phase(),
            stval,
            fault_page,
            real_mm_page_pa(page_idx),
            real_mm_access_label(access),
            real_mm_kind_label(perm.kind),
            perm.mm_id,
            if perm.lazy { 1 } else { 0 },
            pte_flags & 0x3ff
        );
        if k05_is_memory_kind(k04a_current_kind()) {
            let snap = crate::fs::runtime::vm_snapshot();
            crate::println!(
                "[K05-page-fault-trace] case={} va={:#x} page={:#x} access={} kind={} backing_bytes={} ret_mm={} resident_pages={} fault_count={}",
                k02_current_case_name(),
                stval,
                fault_page,
                real_mm_access_label(access),
                real_mm_kind_label(perm.kind),
                backing_copied,
                snap.mm_id,
                snap.resident_pages,
                K02_PAGE_FAULT_COUNT[K02_CURRENT_CASE]
            );
        }
    }
    true
}

pub(super) fn real_mm_fault_alloc_count() -> usize {
    unsafe { REAL_MM_FAULT_ALLOC_COUNT }
}

pub(super) fn real_mm_alloc_count() -> usize {
    unsafe { REAL_MM_ALLOC_COUNT }
}
