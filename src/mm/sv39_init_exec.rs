#![allow(dead_code)]

use core::arch::{asm, global_asm};
use core::sync::atomic::{AtomicBool, Ordering};

use crate::config::PAGE_SIZE;
use crate::fd::{RuntimeFdKind, RuntimeReadTarget, RuntimeWriteTarget};
use crate::loader::init_image::{load_init_image_to_page, LoadedInitImage};
use crate::syscall::{RuntimeSyscallAction, RuntimeSyscallArgs};

const USER_STACK_TOP: usize = 0x4002_0000;
const USER_STACK_PAGES: usize = 4;
const USER_STACK_SIZE: usize = USER_STACK_PAGES * PAGE_SIZE;
const USER_HEAP_START: usize = 0x4003_0000;
const USER_HEAP_PAGES: usize = 4;
const USER_HEAP_SIZE: usize = USER_HEAP_PAGES * PAGE_SIZE;
const USER_HEAP_END: usize = USER_HEAP_START + USER_HEAP_SIZE;
const USER_MMAP_START: usize = 0x4004_0000;
const USER_MMAP_PAGES: usize = 4;
const USER_MMAP_SIZE: usize = USER_MMAP_PAGES * PAGE_SIZE;
const USER_MMAP_END: usize = USER_MMAP_START + USER_MMAP_SIZE;

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

#[repr(C, align(4096))]
struct UserHeap([u8; USER_HEAP_SIZE]);

#[repr(C, align(4096))]
struct UserMmapArea([u8; USER_MMAP_SIZE]);

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
static mut USER_HEAP: UserHeap = UserHeap([0; USER_HEAP_SIZE]);
static mut USER_MMAP_AREA: UserMmapArea = UserMmapArea([0; USER_MMAP_SIZE]);
static mut USER_BRK: usize = USER_HEAP_START;
static mut USER_MMAP_ACTIVE: bool = false;
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
    crate::println!("[external-init-v62] mprotect/madvise path enabled");

    let loaded = load_init_image_to_page()
        .expect("[external-init-v62] load external init.elf failed");

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
    USER_HEAP.0.fill(0);
    USER_MMAP_AREA.0.fill(0);
    USER_BRK = USER_HEAP_START;
    USER_MMAP_ACTIVE = false;

    ROOT_TABLE.0[0] = leaf_1g_pte(0x0000_0000, KERNEL_LEAF);
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
    let heap_pa = core::ptr::addr_of!(USER_HEAP) as usize;
    let mut hp = 0;
    while hp < USER_HEAP_PAGES {
        let va = USER_HEAP_START + hp * PAGE_SIZE;
        let pa = heap_pa + hp * PAGE_SIZE;
        map_user_4k(va, pa, USER_STACK_FLAGS);
        hp += 1;
    }

    crate::println!("[brk-v60] user heap mapped {:#x}..{:#x}", USER_HEAP_START, USER_HEAP_END);

    let mmap_pa = core::ptr::addr_of!(USER_MMAP_AREA) as usize;
    let mut mp = 0;
    while mp < USER_MMAP_PAGES {
        let va = USER_MMAP_START + mp * PAGE_SIZE;
        let pa = mmap_pa + mp * PAGE_SIZE;
        map_user_4k(va, pa, USER_STACK_FLAGS);
        mp += 1;
    }

    crate::println!("[mmap-v61] user mmap area mapped {:#x}..{:#x}", USER_MMAP_START, USER_MMAP_END);
    
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
        crate::println!("[external-init-v62] unexpected trap");
        loop { unsafe { asm!("wfi"); } }
    }
}

fn handle_syscall(cx: &mut TrapContext) {
    let id = cx.regs[17];
    let a0 = cx.regs[10];
    let a1 = cx.regs[11];
    let a2 = cx.regs[12];
    let a3 = cx.regs[13];
    let a4 = cx.regs[14];
    let a5 = cx.regs[15];

    crate::println!("[external-init-v50b] syscall id = {}", id);
    crate::println!("[syscall-dispatch-v54] central dispatch id = {}", id);

    let args = RuntimeSyscallArgs::new(id, a0, a1, a2, a3, a4, a5);

    match crate::syscall::dispatch_runtime_syscall(args) {
        RuntimeSyscallAction::Return(value) => {
            cx.regs[10] = value as usize;
        }
        RuntimeSyscallAction::Write { fd, user_ptr, len, target } => {
            let written = sys_write_user(fd, user_ptr, len, target);
            cx.regs[10] = written as usize;
        }
        RuntimeSyscallAction::Read { fd, user_ptr, len, target } => {
            let read = sys_read_user(fd, user_ptr, len, target);
            cx.regs[10] = read as usize;
        }
        RuntimeSyscallAction::OpenAt { dirfd, user_path, flags, mode } => {
            let fd = sys_openat_user(dirfd, user_path, flags, mode);
            cx.regs[10] = fd as usize;
        }
        RuntimeSyscallAction::Close { fd } => {
            let ret = sys_close_fd(fd);
            cx.regs[10] = ret as usize;
        }
        RuntimeSyscallAction::FStat { fd, user_stat } => {
            let ret = sys_fstat_user(fd, user_stat);
            cx.regs[10] = ret as usize;
        }
        RuntimeSyscallAction::LSeek { fd, offset, whence } => {
            let ret = sys_lseek(fd, offset, whence);
            cx.regs[10] = ret as usize;
        }
        RuntimeSyscallAction::GetDents64 { fd, user_dirent, len } => {
            let ret = sys_getdents64_user(fd, user_dirent, len);
            cx.regs[10] = ret as usize;
        }
        RuntimeSyscallAction::Brk { addr } => {
            let ret = sys_brk(addr);
            cx.regs[10] = ret as usize;
        }
        RuntimeSyscallAction::Mmap { addr, len, prot, flags, fd, offset } => {
            let ret = sys_mmap(addr, len, prot, flags, fd, offset);
            cx.regs[10] = ret as usize;
        }
        RuntimeSyscallAction::Munmap { addr, len } => {
            let ret = sys_munmap(addr, len);
            cx.regs[10] = ret as usize;
        }
        RuntimeSyscallAction::Mprotect { addr, len, prot } => {
            let ret = sys_mprotect(addr, len, prot);
            cx.regs[10] = ret as usize;
        }
        RuntimeSyscallAction::Madvise { addr, len, advice } => {
            let ret = sys_madvise(addr, len, advice);
            cx.regs[10] = ret as usize;
        }
        RuntimeSyscallAction::Exit { code } => {
            crate::println!("[external-init-v50b] exit code = {}", code);
            EXIT_SEEN.store(true, Ordering::SeqCst);
            crate::println!("[external-init-v50b] smoke passed");
            crate::println!("[external-init-v50b] kernel idle after external init ELF smoke");
            loop { unsafe { asm!("wfi"); } }
        }
    }
}




fn sys_mprotect(addr: usize, len: usize, prot: usize) -> isize {
    unsafe {
        crate::println!("[mprotect-v62] addr = {:#x}", addr);
        crate::println!("[mprotect-v62] len = {}", len);
        crate::println!("[mprotect-v62] prot = {:#x}", prot);

        if addr == USER_MMAP_START && len <= USER_MMAP_SIZE && USER_MMAP_ACTIVE {
            crate::println!("[mprotect-v62] ret = 0");
            0
        } else {
            crate::println!("[mprotect-v62] ret = -22");
            crate::syscall::EINVAL
        }
    }
}

fn sys_madvise(addr: usize, len: usize, advice: usize) -> isize {
    unsafe {
        crate::println!("[madvise-v62] addr = {:#x}", addr);
        crate::println!("[madvise-v62] len = {}", len);
        crate::println!("[madvise-v62] advice = {}", advice);

        if addr == USER_MMAP_START && len <= USER_MMAP_SIZE && USER_MMAP_ACTIVE {
            crate::println!("[madvise-v62] ret = 0");
            0
        } else {
            crate::println!("[madvise-v62] ret = -22");
            crate::syscall::EINVAL
        }
    }
}

fn sys_mmap(addr: usize, len: usize, prot: usize, flags: usize, fd: isize, offset: usize) -> isize {
    unsafe {
        crate::println!("[mmap-v61] request addr = {:#x}", addr);
        crate::println!("[mmap-v61] len = {}", len);
        crate::println!("[mmap-v61] prot = {:#x}", prot);
        crate::println!("[mmap-v61] flags = {:#x}", flags);
        crate::println!("[mmap-v61] fd = {}", fd);
        crate::println!("[mmap-v61] offset = {:#x}", offset);

        if len == 0 || len > USER_MMAP_SIZE {
            crate::println!("[mmap-v61] rejected invalid len");
            return crate::syscall::EINVAL;
        }

        USER_MMAP_ACTIVE = true;
        crate::println!("[mmap-v61] ret = {:#x}", USER_MMAP_START);
        USER_MMAP_START as isize
    }
}

fn sys_munmap(addr: usize, len: usize) -> isize {
    unsafe {
        crate::println!("[munmap-v61] addr = {:#x}", addr);
        crate::println!("[munmap-v61] len = {}", len);

        if addr == USER_MMAP_START && len <= USER_MMAP_SIZE && USER_MMAP_ACTIVE {
            USER_MMAP_ACTIVE = false;
            crate::println!("[munmap-v61] ret = 0");
            0
        } else {
            crate::println!("[munmap-v61] ret = -22");
            crate::syscall::EINVAL
        }
    }
}

fn sys_brk(addr: usize) -> isize {
    unsafe {
        crate::println!("[brk-v60] request = {:#x}", addr);
        crate::println!("[brk-v60] current = {:#x}", USER_BRK);

        if addr == 0 {
            crate::println!("[brk-v60] query returned {:#x}", USER_BRK);
            return USER_BRK as isize;
        }

        if addr >= USER_HEAP_START && addr <= USER_HEAP_END {
            USER_BRK = addr;
            crate::println!("[brk-v60] updated = {:#x}", USER_BRK);
            USER_BRK as isize
        } else {
            crate::println!("[brk-v60] rejected, keeping {:#x}", USER_BRK);
            USER_BRK as isize
        }
    }
}

fn sys_openat_user(dirfd: isize, user_path: usize, flags: usize, mode: usize) -> isize {
    crate::println!("[openat-v56] dirfd = {}", dirfd);
    crate::println!("[openat-v56] flags = {:#x}", flags);
    crate::println!("[openat-v56] mode = {:#x}", mode);

    let mut buf = [0u8; 64];
    let mut len = 0usize;

    with_sum_enabled(|| {
        while len + 1 < buf.len() {
            let ch = unsafe { core::ptr::read_volatile((user_path + len) as *const u8) };
            buf[len] = ch;
            if ch == 0 { break; }
            len += 1;
        }
    });

    if len == 9 && &buf[..9] == b"/dev/null" {
        let fd = crate::fd::runtime_open_devnull();
        crate::println!("[openat-v56] opened /dev/null fd = {}", fd);
        fd
    } else if len == 9 && &buf[..9] == b"/dev/zero" {
        let fd = crate::fd::runtime_open_devzero();
        crate::println!("[openat-v57] opened /dev/zero fd = {}", fd);
        fd
    } else if len == 4 && &buf[..4] == b"/dev" {
        let fd = crate::fd::runtime_open_devdir();
        crate::println!("[openat-v59] opened /dev fd = {}", fd);
        fd
    } else {
        crate::println!("[openat-v56] unsupported path");
        crate::fd::ENOENT
    }
}

fn sys_close_fd(fd: usize) -> isize {
    let ret = crate::fd::runtime_close_fd(fd);
    crate::println!("[close-v56] fd = {}", fd);
    crate::println!("[close-v56] ret = {}", ret);
    ret
}

fn sys_getdents64_user(fd: usize, user_dirent: usize, len: usize) -> isize {
    crate::println!("[getdents64-v59] fd = {}", fd);
    crate::println!("[getdents64-v59] buf = {:#x}", user_dirent);
    crate::println!("[getdents64-v59] len = {}", len);

    match crate::fd::runtime_getdents_kind(fd) {
        Ok(RuntimeFdKind::DevDir) => {}
        Ok(_) => return crate::fd::ENOTDIR,
        Err(err) => return err,
    }

    if len < 160 {
        return crate::fd::EINVAL;
    }

    let mut off = 0usize;

    with_sum_enabled(|| {
        off = write_dirent64(user_dirent, off, 1, 1, 4, b".\0");
        off = write_dirent64(user_dirent, off, 2, 2, 4, b"..\0");
        off = write_dirent64(user_dirent, off, 3, 3, 2, b"null\0");
        off = write_dirent64(user_dirent, off, 4, 4, 2, b"zero\0");
    });

    crate::println!("[getdents64-v59] wrote /dev entries bytes = {}", off);
    off as isize
}

fn write_dirent64(base: usize, off: usize, ino: u64, next_off: i64, dtype: u8, name: &[u8]) -> usize {
    let header = 19usize;
    let raw_len = header + name.len();
    let reclen = (raw_len + 7) & !7usize;
    let ptr = base + off;

    unsafe {
        core::ptr::write_volatile((ptr + 0) as *mut u64, ino);
        core::ptr::write_volatile((ptr + 8) as *mut i64, next_off);
        core::ptr::write_volatile((ptr + 16) as *mut u16, reclen as u16);
        core::ptr::write_volatile((ptr + 18) as *mut u8, dtype);

        let mut i = 0;
        while i < name.len() {
            core::ptr::write_volatile((ptr + 19 + i) as *mut u8, name[i]);
            i += 1;
        }

        while 19 + i < reclen {
            core::ptr::write_volatile((ptr + 19 + i) as *mut u8, 0);
            i += 1;
        }
    }

    off + reclen
}

fn sys_fstat_user(fd: usize, user_stat: usize) -> isize {
    crate::println!("[fstat-v58] fd = {}", fd);
    crate::println!("[fstat-v58] user stat = {:#x}", user_stat);

    let kind = match crate::fd::runtime_fstat_result(fd) {
        Ok(kind) => kind,
        Err(err) => return err,
    };

    with_sum_enabled(|| {
        for i in 0..128usize {
            unsafe { core::ptr::write_volatile((user_stat + i) as *mut u8, 0); }
        }

        let mode: u32 = match kind {
            RuntimeFdKind::DevDir => 0o040000 | 0o755,
            RuntimeFdKind::Stdin | RuntimeFdKind::Stdout | RuntimeFdKind::Stderr | RuntimeFdKind::DevNull | RuntimeFdKind::DevZero => 0o020000 | 0o666,
        };

        unsafe {
            core::ptr::write_volatile((user_stat + 0) as *mut u64, fd as u64);
            core::ptr::write_volatile((user_stat + 16) as *mut u32, mode);
            core::ptr::write_volatile((user_stat + 48) as *mut u64, 0);
        }
    });

    crate::println!("[fstat-v58] wrote minimal stat");
    0
}

fn sys_lseek(fd: usize, offset: isize, whence: usize) -> isize {
    crate::println!("[lseek-v58] fd = {}", fd);
    crate::println!("[lseek-v58] offset = {}", offset);
    crate::println!("[lseek-v58] whence = {}", whence);

    let ret = crate::fd::runtime_lseek_result(fd);
    crate::println!("[lseek-v58] ret = {}", ret);
    ret
}

fn sys_read_user(fd: usize, user_ptr: usize, len: usize, target: RuntimeReadTarget) -> isize {
    crate::println!("[read-v57] fd = {}", fd);
    crate::println!("[read-v57] len = {}", len);

    if len == 0 { return 0; }

    match target {
        RuntimeReadTarget::Stdin => {
            crate::println!("[read-v57] stdin returns EOF");
            0
        }
        RuntimeReadTarget::DevZero => {
            with_sum_enabled(|| {
                for i in 0..len {
                    unsafe { core::ptr::write_volatile((user_ptr + i) as *mut u8, 0); }
                }
            });
            crate::println!("[read-v57] /dev/zero filled buffer");
            len as isize
        }
    }
}

fn sys_write_user(fd: usize, user_ptr: usize, len: usize, target: RuntimeWriteTarget) -> isize {
    crate::println!("[fd-v55] write fd = {}", fd);
    crate::println!("[fd-v55] write len = {}", len);

    if len == 0 { return 0; }

    match target {
        RuntimeWriteTarget::Console => {
            with_sum_enabled(|| {
                for i in 0..len {
                    let ch = unsafe { core::ptr::read_volatile((user_ptr + i) as *const u8) };
                    crate::sbi::console_putchar(ch as usize);
                }
            });
            len as isize
        }
        RuntimeWriteTarget::DevNull => {
            crate::println!("[fd-v56] /dev/null swallowed write");
            len as isize
        }
    }
}

fn with_sum_enabled<F: FnOnce()>(f: F) {
    let old = read_sstatus();
    unsafe { asm!("csrs sstatus, {}", in(reg) SSTATUS_SUM); }
    f();
    unsafe { asm!("csrw sstatus, {}", in(reg) old); }
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

fn root_pa() -> usize { core::ptr::addr_of!(ROOT_TABLE) as usize }
const fn table_pte(pa: usize) -> usize { ((pa >> 12) << 10) | PTE_V }
const fn leaf_1g_pte(pa: usize, flags: usize) -> usize { ((pa >> 12) << 10) | flags }
const fn leaf_4k_pte(pa: usize, flags: usize) -> usize { ((pa >> 12) << 10) | flags }
const fn vpn0(va: usize) -> usize { (va >> 12) & 0x1ff }
const fn vpn1(va: usize) -> usize { (va >> 21) & 0x1ff }
const fn vpn2(va: usize) -> usize { (va >> 30) & 0x1ff }
