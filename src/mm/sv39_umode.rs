use core::arch::{asm, global_asm};
use core::sync::atomic::{AtomicBool, Ordering};

global_asm!(include_str!("../../arch/riscv64/user_v45.S"));

const PAGE_SIZE: usize = 4096;
const USER_BASE: usize = 0x4000_0000;
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
    fn __user_v45_start();
    fn __user_v45_end();
    fn __sv39_v45d_alltraps();
    fn __sv39_v45d_restore(cx: *const TrapContext) -> !;
}

global_asm!(r#"
    .section .text
    .globl __sv39_v45d_alltraps
    .globl __sv39_v45d_restore

__sv39_v45d_alltraps:
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
    call rust_sv39_v45d_trap_handler

    mv a0, sp
    j __sv39_v45d_restore

__sv39_v45d_restore:
    mv sp, a0

    la t0, trap_stack_top
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
trap_stack:
    .space 4096 * 16
trap_stack_top:
"#);

pub fn run_sv39_umode_smoke() -> ! {
    crate::println!("[sv39-umode-v45d] begin");
    crate::println!("[sv39-umode-v45d] user base avoids UART MMIO conflict");

    unsafe {
        build_page_table();
        install_trap_entry();
        activate_page_table();

        crate::println!("[sv39-umode-v45d] after satp");
        crate::println!("[sv39-umode-v45d] read satp = {:#x}", read_satp());

        enter_user();
    }
}

unsafe fn build_page_table() {
    ROOT_TABLE.0 = [0; 512];
    USER_L1_TABLE.0 = [0; 512];
    USER_L0_TABLE.0 = [0; 512];

    // 0x0000_0000..0x3fff_ffff identity: UART MMIO stays usable at 0x1000_0000.
    ROOT_TABLE.0[0] = leaf_1g_pte(0x0000_0000, KERNEL_LEAF);

    // 0x8000_0000..0xbfff_ffff identity: OpenSBI + kernel + RAM.
    ROOT_TABLE.0[2] = leaf_1g_pte(0x8000_0000, KERNEL_LEAF);

    ROOT_TABLE.0[1] = table_pte(table_ppn(core::ptr::addr_of!(USER_L1_TABLE) as usize));

    let user_vpn1 = vpn1(USER_BASE);
    USER_L1_TABLE.0[user_vpn1] = table_pte(table_ppn(core::ptr::addr_of!(USER_L0_TABLE) as usize));

    map_user_text();
    map_user_stack();

    crate::println!("[sv39-umode-v45d] root pa = {:#x}", root_pa());
    crate::println!("[sv39-umode-v45d] user va = {:#x}", user_entry_va());
    crate::println!("[sv39-umode-v45d] user stack top = {:#x}", USER_STACK_TOP);
}

unsafe fn map_user_text() {
    let start = __user_v45_start as *const () as usize;
    let end = __user_v45_end as *const () as usize;
    let size = end - start;
    let pages = (size + PAGE_SIZE - 1) / PAGE_SIZE;

    for i in 0..pages {
        let va = USER_BASE + i * PAGE_SIZE;
        let pa = align_down(start, PAGE_SIZE) + i * PAGE_SIZE;
        USER_L0_TABLE.0[vpn0(va)] = leaf_4k_pte(pa, USER_TEXT_FLAGS);
    }

    crate::println!("[sv39-umode-v45d] user text pa = {:#x}..{:#x}", start, end);
    crate::println!("[sv39-umode-v45d] user text pages = {}", pages);
}

unsafe fn map_user_stack() {
    let stack_pa = core::ptr::addr_of!(USER_STACK) as usize;
    let stack_base_va = USER_STACK_TOP - USER_STACK_SIZE;

    for i in 0..USER_STACK_PAGES {
        let va = stack_base_va + i * PAGE_SIZE;
        let pa = stack_pa + i * PAGE_SIZE;
        USER_L0_TABLE.0[vpn0(va)] = leaf_4k_pte(pa, USER_STACK_FLAGS);
    }

    crate::println!("[sv39-umode-v45d] user stack pa = {:#x}", stack_pa);
    crate::println!("[sv39-umode-v45d] user stack va = {:#x}..{:#x}", stack_base_va, USER_STACK_TOP);
}

unsafe fn install_trap_entry() {
    let entry = __sv39_v45d_alltraps as *const () as usize;
    asm!("csrw stvec, {}", in(reg) entry);
    crate::println!("[sv39-umode-v45d] stvec = {:#x}", entry);
}

unsafe fn activate_page_table() {
    let satp = SATP_MODE_SV39 | (root_pa() / PAGE_SIZE);
    crate::println!("[sv39-umode-v45d] satp = {:#x}", satp);
    asm!("csrw satp, {}", in(reg) satp);
    asm!("sfence.vma zero, zero");
}

unsafe fn enter_user() -> ! {
    let cx = core::ptr::addr_of_mut!(INIT_CONTEXT);
    (*cx).regs = [0; 32];
    (*cx).regs[2] = USER_STACK_TOP;
    (*cx).sstatus = user_sstatus();
    (*cx).sepc = user_entry_va();

    crate::println!("[sv39-umode-v45d] enter user sepc = {:#x}", (*cx).sepc);
    crate::println!("[sv39-umode-v45d] enter user sp   = {:#x}", (*cx).regs[2]);

    __sv39_v45d_restore(cx);
}

#[no_mangle]
pub extern "C" fn rust_sv39_v45d_trap_handler(cx: &mut TrapContext) {
    let scause = read_scause();
    let stval = read_stval();

    crate::println!("[sv39-umode-v45d] trap scause = {:#x}", scause);
    crate::println!("[sv39-umode-v45d] trap sepc   = {:#x}", cx.sepc);
    crate::println!("[sv39-umode-v45d] trap stval  = {:#x}", stval);

    if scause == 8 {
        cx.sepc += 4;
        handle_syscall(cx);
    } else {
        crate::println!("[sv39-umode-v45d] unexpected trap");
        loop { unsafe { asm!("wfi"); } }
    }
}

fn handle_syscall(cx: &mut TrapContext) {
    let id = cx.regs[17];
    let a0 = cx.regs[10];
    let a1 = cx.regs[11];
    let a2 = cx.regs[12];

    crate::println!("[sv39-umode-v45d] syscall id = {}", id);

    match id {
        64 => {
            let written = sys_write_user(a0, a1, a2);
            cx.regs[10] = written as usize;
        }
        93 => {
            crate::println!("[sv39-umode-v45d] exit code = {}", a0);
            EXIT_SEEN.store(true, Ordering::SeqCst);
            crate::println!("[sv39-umode-v45d] smoke passed");
            crate::println!("[sv39-umode-v45d] kernel idle after Sv39 U-mode smoke");
            loop { unsafe { asm!("wfi"); } }
        }
        172 => cx.regs[10] = 1,
        173 => cx.regs[10] = 0,
        _ => cx.regs[10] = (-38isize) as usize,
    }
}

fn sys_write_user(fd: usize, user_ptr: usize, len: usize) -> isize {
    if fd != 1 && fd != 2 {
        return -1;
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

fn user_entry_va() -> usize {
    let entry_pa = __user_v45_start as *const () as usize;
    let start_pa = __user_v45_start as *const () as usize;
    USER_BASE + (entry_pa - start_pa)
}

const fn align_down(value: usize, align: usize) -> usize {
    value & !(align - 1)
}

const fn table_ppn(pa: usize) -> usize {
    pa / PAGE_SIZE
}

const fn table_pte(ppn: usize) -> usize {
    (ppn << 10) | PTE_V
}

const fn leaf_1g_pte(pa: usize, flags: usize) -> usize {
    ((pa >> 12) << 10) | flags
}

const fn leaf_4k_pte(pa: usize, flags: usize) -> usize {
    ((pa >> 12) << 10) | flags
}

const fn vpn1(va: usize) -> usize {
    (va >> 21) & 0x1ff
}

const fn vpn0(va: usize) -> usize {
    (va >> 12) & 0x1ff
}
