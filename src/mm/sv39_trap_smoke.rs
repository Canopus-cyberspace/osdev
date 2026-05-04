use core::arch::{asm, global_asm};

const PAGE_SIZE: usize = 4096;
const SATP_MODE_SV39: usize = 8usize << 60;

const PTE_V: usize = 1 << 0;
const PTE_R: usize = 1 << 1;
const PTE_W: usize = 1 << 2;
const PTE_X: usize = 1 << 3;
const PTE_A: usize = 1 << 6;
const PTE_D: usize = 1 << 7;

const LEAF_RWX: usize = PTE_V | PTE_R | PTE_W | PTE_X | PTE_A | PTE_D;

#[repr(C, align(4096))]
struct RootPageTable {
    entries: [usize; 512],
}

static mut V43B_ROOT: RootPageTable = RootPageTable { entries: [0; 512] };
static mut V43B_TRAP_SEEN: usize = 0;
static mut V43B_DATA_PROBE: usize = 0x1234_5678_9abc_def0;

global_asm!(r#"
    .section .text
    .globl __sv39_kernel_trap_entry_v43b
__sv39_kernel_trap_entry_v43b:
    addi sp, sp, -160
    sd ra,   0(sp)
    sd t0,   8(sp)
    sd t1,  16(sp)
    sd t2,  24(sp)
    sd a0,  32(sp)
    sd a1,  40(sp)
    sd a2,  48(sp)
    sd a3,  56(sp)
    sd a4,  64(sp)
    sd a5,  72(sp)
    sd a6,  80(sp)
    sd a7,  88(sp)

    csrr a0, scause
    csrr a1, sepc
    csrr a2, stval
    call sv39_kernel_trap_handler_v43b
    csrw sepc, a0

    ld ra,   0(sp)
    ld t0,   8(sp)
    ld t1,  16(sp)
    ld t2,  24(sp)
    ld a0,  32(sp)
    ld a1,  40(sp)
    ld a2,  48(sp)
    ld a3,  56(sp)
    ld a4,  64(sp)
    ld a5,  72(sp)
    ld a6,  80(sp)
    ld a7,  88(sp)
    addi sp, sp, 160
    sret
"#);

extern "C" {
    fn __sv39_kernel_trap_entry_v43b();
}

pub fn init() {
    crate::println!("[sv39-trap-v43b] init");
}

pub fn test() {
    crate::println!("[sv39-trap-v43b] scaffold test");
}

pub fn run_kernel_sv39_trap_smoke_v43b() -> ! {
    crate::println!("[sv39-trap-v43b] begin");
    crate::println!("[sv39-trap-v43b] building static identity table");

    unsafe {
        init_root_page_table();
    }

    let root_pa = core::ptr::addr_of!(V43B_ROOT) as usize;
    let root_ppn = root_pa / PAGE_SIZE;
    let satp = SATP_MODE_SV39 | root_ppn;

    crate::println!("[sv39-trap-v43b] root pa = {:#x}", root_pa);
    crate::println!("[sv39-trap-v43b] root ppn = {:#x}", root_ppn);
    crate::println!("[sv39-trap-v43b] satp = {:#x}", satp);

    unsafe {
        let trap_entry = __sv39_kernel_trap_entry_v43b as *const () as usize;
        asm!("csrw stvec, {}", in(reg) trap_entry);
        asm!("csrw satp, {}", in(reg) satp);
        asm!("sfence.vma", options(nostack, preserves_flags));
    }

    crate::println!("[sv39-trap-v43b] after satp");

    unsafe {
        V43B_DATA_PROBE ^= 0x1111_2222_3333_4444;
        crate::println!("[sv39-trap-v43b] data probe = {:#x}", V43B_DATA_PROBE);
    }

    crate::println!("[sv39-trap-v43b] trigger S-mode ecall");

    unsafe {
        asm!("ecall");
    }

    crate::println!("[sv39-trap-v43b] trap returned after ecall");

    let seen = unsafe { V43B_TRAP_SEEN };
    crate::println!("[sv39-trap-v43b] trap seen = {}", seen);

    if seen == 1 {
        crate::println!("[sv39-trap-v43b] kernel trap smoke passed");
    } else {
        crate::println!("[sv39-trap-v43b] kernel trap smoke failed");
    }

    crate::println!("[sv39-trap-v43b] kernel idle after isolated Sv39 trap smoke");
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

unsafe fn init_root_page_table() {
    let root = core::ptr::addr_of_mut!(V43B_ROOT);

    for i in 0..512 {
        (*root).entries[i] = 0;
    }

    // root[0] maps 0x00000000..0x3fffffff. This covers UART MMIO at 0x10000000.
    (*root).entries[0] = leaf_1g(0x0000_0000, LEAF_RWX);

    // root[2] maps 0x80000000..0xbfffffff. This covers kernel RAM on qemu virt.
    (*root).entries[2] = leaf_1g(0x8000_0000, LEAF_RWX);
}

const fn leaf_1g(pa: usize, flags: usize) -> usize {
    ((pa / PAGE_SIZE) << 10) | flags
}

#[no_mangle]
pub extern "C" fn sv39_kernel_trap_handler_v43b(scause: usize, sepc: usize, stval: usize) -> usize {
    crate::println!("[sv39-trap-v43b] kernel trap handler scause = {:#x}", scause);
    crate::println!("[sv39-trap-v43b] kernel trap handler sepc   = {:#x}", sepc);
    crate::println!("[sv39-trap-v43b] kernel trap handler stval  = {:#x}", stval);

    if scause == 9 {
        crate::println!("[sv39-trap-v43b] S-mode ecall observed");
        unsafe {
            V43B_TRAP_SEEN = 1;
        }
        sepc + 4
    } else {
        crate::println!("[sv39-trap-v43b] unexpected trap cause");
        sepc + 4
    }
}
