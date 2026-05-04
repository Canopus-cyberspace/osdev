use core::arch::{asm, global_asm};
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crate::mm::sv39;

const PTE_V: usize = 1 << 0;
const PTE_R: usize = 1 << 1;
const PTE_W: usize = 1 << 2;
const PTE_X: usize = 1 << 3;
const PTE_A: usize = 1 << 6;
const PTE_D: usize = 1 << 7;

const LEAF_RWX: usize = PTE_V | PTE_R | PTE_W | PTE_X | PTE_A | PTE_D;

static TRAP_SEEN: AtomicBool = AtomicBool::new(false);
static TRAP_SCAUSE: AtomicUsize = AtomicUsize::new(usize::MAX);
static TRAP_SEPC: AtomicUsize = AtomicUsize::new(0);
static TRAP_STVAL: AtomicUsize = AtomicUsize::new(0);
static DATA_PROBE: AtomicUsize = AtomicUsize::new(0x3257_45aa_98f9_ab4);

#[repr(C, align(4096))]
struct PageTable512([usize; 512]);

static mut ROOT_TABLE: PageTable512 = PageTable512([0; 512]);

extern "C" {
    fn __sv39_v43e_trap_entry();
}

global_asm!(r#"
    .section .text
    .globl __sv39_v43e_trap_entry
__sv39_v43e_trap_entry:
    addi sp, sp, -64
    sd ra, 0(sp)
    sd a0, 8(sp)
    sd a1, 16(sp)
    sd a2, 24(sp)

    csrr a0, scause
    csrr a1, sepc
    csrr a2, stval
    call rust_sv39_v43e_trap_handler

    ld ra, 0(sp)
    ld a0, 8(sp)
    ld a1, 16(sp)
    ld a2, 24(sp)
    addi sp, sp, 64
    sret
"#);

pub fn init() {
    crate::println!("[mm::sv39_smoke] init v43e");
}

pub fn test() {
    test_scaffold();
}

pub fn test_scaffold() {
    crate::println!("[mm::sv39_smoke] scaffold test v43e");
}

pub fn run_kernel_sv39_smoke() -> ! {
    run_kernel_sv39_trap_ebreak_smoke()
}

pub fn run_kernel_sv39_activation_smoke() -> ! {
    run_kernel_sv39_trap_ebreak_smoke()
}

pub fn run_kernel_sv39_trap_ebreak_smoke() -> ! {
    crate::println!("[sv39-trap-v43e] begin");
    crate::println!("[sv39-trap-v43e] building stable 1GiB identity map");

    build_static_identity_page_table();

    let root_pa = root_table_pa();
    let root_ppn = root_pa / sv39::PAGE_SIZE;
    let satp = sv39::make_satp(root_ppn);

    crate::println!("[sv39-trap-v43e] root pa = {:#x}", root_pa);
    crate::println!("[sv39-trap-v43e] root ppn = {:#x}", root_ppn);
    crate::println!("[sv39-trap-v43e] satp = {:#x}", satp);

    install_trap_entry();

    unsafe {
        sv39::activate_satp_unchecked(satp);
    }

    crate::println!("[sv39-trap-v43e] after satp");
    crate::println!("[sv39-trap-v43e] read satp = {:#x}", sv39::read_satp());

    let old = DATA_PROBE.load(Ordering::SeqCst);
    DATA_PROBE.store(old ^ 0x55aa_55aa_55aa_55aa, Ordering::SeqCst);
    crate::println!("[sv39-trap-v43e] data probe = {:#x}", DATA_PROBE.load(Ordering::SeqCst));

    TRAP_SEEN.store(false, Ordering::SeqCst);
    TRAP_SCAUSE.store(usize::MAX, Ordering::SeqCst);
    TRAP_SEPC.store(0, Ordering::SeqCst);
    TRAP_STVAL.store(0, Ordering::SeqCst);

    crate::println!("[sv39-trap-v43e] trigger 32-bit ebreak");

    unsafe {
        asm!(".4byte 0x00100073");
    }

    crate::println!("[sv39-trap-v43e] trap returned after ebreak");
    crate::println!("[sv39-trap-v43e] trap seen = {}", TRAP_SEEN.load(Ordering::SeqCst) as usize);
    crate::println!("[sv39-trap-v43e] seen scause = {:#x}", TRAP_SCAUSE.load(Ordering::SeqCst));
    crate::println!("[sv39-trap-v43e] seen sepc = {:#x}", TRAP_SEPC.load(Ordering::SeqCst));
    crate::println!("[sv39-trap-v43e] seen stval = {:#x}", TRAP_STVAL.load(Ordering::SeqCst));

    if TRAP_SEEN.load(Ordering::SeqCst) && TRAP_SCAUSE.load(Ordering::SeqCst) == 0x3 {
        crate::println!("[sv39-trap-v43e] kernel trap smoke passed");
    } else {
        crate::println!("[sv39-trap-v43e] kernel trap smoke failed");
    }

    crate::println!("[sv39-trap-v43e] kernel idle after isolated Sv39 trap smoke");

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

fn build_static_identity_page_table() {
    unsafe {
        ROOT_TABLE.0 = [0; 512];

        // VA 0x0000_0000..0x3fff_ffff -> PA 0x0000_0000..0x3fff_ffff
        // Covers UART MMIO at 0x1000_0000.
        ROOT_TABLE.0[0] = leaf_1g_pte(0x0000_0000, LEAF_RWX);

        // VA 0x8000_0000..0xbfff_ffff -> PA 0x8000_0000..0xbfff_ffff
        // Covers OpenSBI/kernel/QEMU RAM area used by this project.
        ROOT_TABLE.0[2] = leaf_1g_pte(0x8000_0000, LEAF_RWX);
    }
}

const fn leaf_1g_pte(pa: usize, flags: usize) -> usize {
    ((pa >> 12) << 10) | flags
}

fn root_table_pa() -> usize {
    core::ptr::addr_of!(ROOT_TABLE) as usize
}

fn install_trap_entry() {
    let entry = __sv39_v43e_trap_entry as usize;
    unsafe {
        asm!("csrw stvec, {}", in(reg) entry);
    }

    crate::println!("[sv39-trap-v43e] stvec = {:#x}", entry);
}

#[no_mangle]
pub extern "C" fn rust_sv39_v43e_trap_handler(scause: usize, sepc: usize, stval: usize) {
    crate::println!("[sv39-trap-v43e] kernel trap handler scause = {:#x}", scause);
    crate::println!("[sv39-trap-v43e] kernel trap handler sepc   = {:#x}", sepc);
    crate::println!("[sv39-trap-v43e] kernel trap handler stval  = {:#x}", stval);

    TRAP_SEEN.store(true, Ordering::SeqCst);
    TRAP_SCAUSE.store(scause, Ordering::SeqCst);
    TRAP_SEPC.store(sepc, Ordering::SeqCst);
    TRAP_STVAL.store(stval, Ordering::SeqCst);

    let next_sepc = sepc + 4;
    unsafe {
        asm!("csrw sepc, {}", in(reg) next_sepc);
    }
}
