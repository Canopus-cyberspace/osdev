#![allow(dead_code)]

use core::arch::{asm, global_asm};
use core::sync::atomic::{AtomicBool, Ordering};

use crate::config::PAGE_SIZE;
use crate::loader::init_image::{load_init_image_to_page, LoadedInitImage};

const USER_STACK_TOP: usize = 0x4002_0000;
const USER_STACK_PAGES: usize = 4;
const USER_STACK_SIZE: usize = USER_STACK_PAGES * PAGE_SIZE;

const PTE_V: usize = 1 << 0;
const PTE_R: usize = 1 << 1;
const PTE_W: usize = 1 << 2;
const PTE_X: usize = 1 << 3;
const PTE_U: usize = 1 << 4;
const PTE_A: usize = 1 << 6;
const PTE_D: usize = 1 << 7;

const SATP_MODE_SV39: usize = 8usize << 60;
const SSTATUS_SPP: usize = 1 << 8;
const SSTATUS_SPIE: usize = 1 << 5;
const SSTATUS_SUM: usize = 1 << 18;

const KERNEL_LEAF: usize = PTE_V | PTE_R | PTE_W | PTE_X | PTE_A | PTE_D;
const USER_TEXT_FLAGS: usize = PTE_V | PTE_R | PTE_X | PTE_U | PTE_A | PTE_D;
const USER_STACK_FLAGS: usize = PTE_V | PTE_R | PTE_W | PTE_U | PTE_A | PTE_D;

#[repr(C, align(4096))]
struct PageTable512([usize; 512]);

#[repr(C, align(4096))]
struct UserStack([u8; USER_STACK_SIZE]);

#[repr(C)]
pub struct TrapContext {
    pub regs: [usize; 32],
    pub sstatus: usize,
    pub sepc: usize,
}

static mut ROOT_TABLE: PageTable512 = PageTable512([0; 512]);
static mut USER_L1_TABLE: PageTable512 = PageTable512([0; 512]);
static mut USER_L0_TABLE: PageTable512 = PageTable512([0; 512]);
static mut USER_STACK: UserStack = UserStack([0; USER_STACK_SIZE]);
static mut INIT_CONTEXT: TrapContext = TrapContext { regs: [0; 32], sstatus: 0, sepc: 0 };
static EXIT_SEEN: AtomicBool = AtomicBool::new(false);

extern "C" {
    fn __sv39_init_v50b_alltraps();
    fn __sv39_init_v50b_restore(cx: *const TrapContext) -> !;
}

global_asm!(r#"
    .section .text
    .balign 4
    .globl __sv39_init_v50b_alltraps
    .globl __sv39_init_v50b_restore

__sv39_init_v50b_alltraps:
    csrrw sp, sscratch, sp
    addi sp, sp, -272

    sd x1,   8(sp)
    csrr t0, sscratch
    sd t0,  16(sp)
    sd x3,  24(sp)
    sd x4,  32(sp)
    sd x5,  40(sp)
    sd x6,  48(sp)
    sd x7,  56(sp)
    sd x8,  64(sp)
    sd x9,  72(sp)
    sd x10, 80(sp)
    sd x11, 88(sp)
    sd x12, 96(sp)
    sd x13, 104(sp)
    sd x14, 112(sp)
    sd x15, 120(sp)
    sd x16, 128(sp)
    sd x17, 136(sp)
    sd x18, 144(sp)
    sd x19, 152(sp)
    sd x20, 160(sp)
    sd x21, 168(sp)
    sd x22, 176(sp)
    sd x23, 184(sp)
    sd x24, 192(sp)
    sd x25, 200(sp)
    sd x26, 208(sp)
    sd x27, 216(sp)
    sd x28, 224(sp)
    sd x29, 232(sp)
    sd x30, 240(sp)
    sd x31, 248(sp)

    csrr t0, sstatus
    sd t0, 256(sp)
    csrr t0, sepc
    sd t0, 264(sp)

    mv a0, sp
    call rust_sv39_init_v50b_trap_handler

    mv a0, sp
    j __sv39_init_v50b_restore

__sv39_init_v50b_restore:
    mv sp, a0

    la t0, external_init_trap_stack_top
    csrw sscratch, t0

    ld t0, 256(sp)
    csrw sstatus, t0
    ld t0, 264(sp)
    csrw sepc, t0

    ld x1,   8(sp)
    ld x3,  24(sp)
    ld x4,  32(sp)
    ld x5,  40(sp)
    ld x6,  48(sp)
    ld x7,  56(sp)
    ld x8,  64(sp)
    ld x9,  72(sp)
    ld x10, 80(sp)
    ld x11, 88(sp)
    ld x12, 96(sp)
    ld x13, 104(sp)
    ld x14, 112(sp)
    ld x15, 120(sp)
    ld x16, 128(sp)
    ld x17, 136(sp)
    ld x18, 144(sp)
    ld x19, 152(sp)
    ld x20, 160(sp)
    ld x21, 168(sp)
    ld x22, 176(sp)
    ld x23, 184(sp)
    ld x24, 192(sp)
    ld x25, 200(sp)
    ld x26, 208(sp)
    ld x27, 216(sp)
    ld x28, 224(sp)
    ld x29, 232(sp)
    ld x30, 240(sp)
    ld x31, 248(sp)
    ld x2, 16(sp)
    sret

    .section .trap_stack, "aw", @nobits
    .align 12
external_init_trap_stack:
    .space 4096 * 16
external_init_trap_stack_top:
"#);

pub fn run_external_init_elf_smoke() -> ! {
    crate::println!("[external-init-v50b] begin");
    crate::println!("[external-init-v50b] robust trap path with sscratch restore");

    let loaded = load_init_image_to_page()
        .expect("[external-init-v50b] load external init.elf failed");

    crate::println!("[external-init-v50b] elf entry = {:#x}", loaded.entry);
    crate::println!("[external-init-v50b] elf vaddr  = {:#x}", loaded.vaddr);
    crate::println!("[external-init-v50b] load pa    = {:#x}", loaded.load_pa);
    crate::println!("[external-init-v50b] pages      = {}", loaded.page_count);

    unsafe {
        build_page_table(loaded);
        install_trap_entry();
        activate_page_table();

        crate::println!("[external-init-v50b] after satp");
        crate::println!("[external-init-v50b] read satp = {:#x}", read_satp());

        enter_user(loaded.entry);
    }
}

unsafe fn build_page_table(loaded: LoadedInitImage) {
    ROOT_TABLE.0 = [0; 512];
    USER_L1_TABLE.0 = [0; 512];
    USER_L0_TABLE.0 = [0; 512];
    USER_STACK.0.fill(0);

    // Low 1GiB identity map keeps UART MMIO usable at 0x1000_0000.
    ROOT_TABLE.0[0] = leaf_1g_pte(0x0000_0000, KERNEL_LEAF);

    // Kernel/OpenSBI/QEMU RAM identity map.
    ROOT_TABLE.0[2] = leaf_1g_pte(0x8000_0000, KERNEL_LEAF);

    ROOT_TABLE.0[vpn2(loaded.vaddr)] = table_pte(core::ptr::addr_of!(USER_L1_TABLE) as usize);
    USER_L1_TABLE.0[vpn1(loaded.vaddr)] = table_pte(core::ptr::addr_of!(USER_L0_TABLE) as usize);

    let mut page = 0;
    while page < loaded.page_count {
        let va = loaded.vaddr + page * PAGE_SIZE;
        let pa = loaded.load_pa + page * PAGE_SIZE;
        map_user_4k(va, pa, USER_TEXT_FLAGS);
        page += 1;
    }

    let stack_pa = core::ptr::addr_of!(USER_STACK) as usize;
    let stack_base_va = USER_STACK_TOP - USER_STACK_SIZE;
    let mut i = 0;
    while i < USER_STACK_PAGES {
        let va = stack_base_va + i * PAGE_SIZE;
        let pa = stack_pa + i * PAGE_SIZE;
        map_user_4k(va, pa, USER_STACK_FLAGS);
        i += 1;
    }

    crate::println!("[external-init-v50b] user text mapped {:#x} pages {}", loaded.vaddr, loaded.page_count);
    crate::println!("[external-init-v50b] user stack mapped {:#x}..{:#x}", stack_base_va, USER_STACK_TOP);
    crate::println!("[external-init-v50b] root pa = {:#x}", root_pa());
}

unsafe fn map_user_4k(va: usize, pa: usize, flags: usize) {
    assert_eq!(va % PAGE_SIZE, 0);
    assert_eq!(pa % PAGE_SIZE, 0);
    USER_L0_TABLE.0[vpn0(va)] = leaf_4k_pte(pa, flags);
}

unsafe fn install_trap_entry() {
    let entry_raw = __sv39_init_v50b_alltraps as *const () as usize;
    let entry = entry_raw & !0x3usize;

    crate::println!("[external-init-v53f] trap entry raw = {:#x}", entry_raw);
    crate::println!("[external-init-v53f] trap entry aligned = {:#x}", entry);

    if entry_raw != entry {
        crate::println!("[external-init-v53f] corrected stvec low bits");
    }

    asm!("csrw stvec, {}", in(reg) entry);
    crate::println!("[external-init-v50b] stvec = {:#x}", entry);
}

unsafe fn activate_page_table() {
    let satp = SATP_MODE_SV39 | (root_pa() / PAGE_SIZE);
    crate::println!("[external-init-v50b] satp = {:#x}", satp);
    asm!("csrw satp, {}", in(reg) satp);
    asm!("sfence.vma zero, zero");
}

unsafe fn enter_user(entry: usize) -> ! {
    let cx = core::ptr::addr_of_mut!(INIT_CONTEXT);
    (*cx).regs = [0; 32];
    (*cx).regs[2] = USER_STACK_TOP;
    (*cx).sstatus = user_sstatus();
    (*cx).sepc = entry;

    crate::println!("[external-init-v50b] enter user sepc = {:#x}", (*cx).sepc);
    crate::println!("[external-init-v50b] enter user sp   = {:#x}", (*cx).regs[2]);

    __sv39_init_v50b_restore(cx);
}

#[no_mangle]
pub extern "C" fn rust_sv39_init_v50b_trap_handler(cx: &mut TrapContext) {
    let scause = read_scause();
    let stval = read_stval();

    crate::println!("[external-init-v50b] trap scause = {:#x}", scause);
    crate::println!("[external-init-v50b] trap sepc   = {:#x}", cx.sepc);
    crate::println!("[external-init-v50b] trap stval  = {:#x}", stval);

    if scause == 8 {
        cx.sepc += 4;
        handle_syscall(cx);
    } else {
        crate::println!("[external-init-v50b] unexpected trap");
        loop { unsafe { asm!("wfi"); } }
    }
}

fn handle_syscall(cx: &mut TrapContext) {
    let id = cx.regs[17];
    let a0 = cx.regs[10];
    let a1 = cx.regs[11];
    let a2 = cx.regs[12];

    crate::println!("[external-init-v50b] syscall id = {}", id);

    match id {
        64 => {
            let written = sys_write_user(a0, a1, a2);
            cx.regs[10] = written as usize;
        }
        93 => {
            crate::println!("[external-init-v50b] exit code = {}", a0);
            EXIT_SEEN.store(true, Ordering::SeqCst);
            crate::println!("[external-init-v50b] smoke passed");
            crate::println!("[external-init-v50b] kernel idle after external init ELF smoke");
            loop { unsafe { asm!("wfi"); } }
        }
        172 => cx.regs[10] = 1,
        173 => cx.regs[10] = 0,
        _ => cx.regs[10] = (-38isize) as usize,
    }
}

fn sys_write_user(fd: usize, user_ptr: usize, len: usize) -> isize {
    if fd != 1 && fd != 2 {
        return -9;
    }
    if len == 0 {
        return 0;
    }

    with_sum_enabled(|| {
        for i in 0..len {
            let ch = unsafe { core::ptr::read_volatile((user_ptr + i) as *const u8) };
            crate::sbi::console_putchar(ch as usize);
        }
    });

    len as isize
}

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

fn user_sstatus() -> usize {
    let mut value = read_sstatus();
    value &= !SSTATUS_SPP;
    value |= SSTATUS_SPIE;
    value
}

fn read_sstatus() -> usize {
    let value: usize;
    unsafe { asm!("csrr {}, sstatus", out(reg) value); }
    value
}

fn read_scause() -> usize {
    let value: usize;
    unsafe { asm!("csrr {}, scause", out(reg) value); }
    value
}

fn read_stval() -> usize {
    let value: usize;
    unsafe { asm!("csrr {}, stval", out(reg) value); }
    value
}

fn read_satp() -> usize {
    let value: usize;
    unsafe { asm!("csrr {}, satp", out(reg) value); }
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
