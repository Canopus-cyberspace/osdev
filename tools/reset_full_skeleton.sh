#!/usr/bin/env bash
set -euo pipefail

rm -rf src arch linker user
mkdir -p .cargo
mkdir -p arch/riscv64
mkdir -p linker
mkdir -p src
mkdir -p src/trap
mkdir -p src/mm
mkdir -p src/task
mkdir -p src/syscall
mkdir -p src/fs
mkdir -p src/loader
mkdir -p src/sync
mkdir -p src/signal
mkdir -p src/futex
mkdir -p src/timer
mkdir -p src/drivers
mkdir -p src/net
mkdir -p tools
mkdir -p user

cat > rust-toolchain.toml <<'RS'
[toolchain]
channel = "nightly"
components = ["rust-src", "llvm-tools-preview"]
targets = ["riscv64gc-unknown-none-elf"]
RS

cat > Cargo.toml <<'RS'
[package]
name = "uestc-kernel"
version = "0.1.0"
edition = "2021"

[dependencies]

[profile.dev]
panic = "abort"
opt-level = 1
debug = false
lto = false

[profile.release]
panic = "abort"
opt-level = 3
debug = false
lto = true
RS

cat > .cargo/config.toml <<'RS'
[build]
target = "riscv64gc-unknown-none-elf"

[target.riscv64gc-unknown-none-elf]
rustflags = [
  "-C", "link-arg=-Tlinker/riscv64.ld",
  "-C", "force-frame-pointers=yes",
]

[unstable]
build-std = ["core", "compiler_builtins"]
RS

cat > Makefile <<'RS'
.RECIPEPREFIX := >

KERNEL_ELF := target/riscv64gc-unknown-none-elf/debug/uestc-kernel

.PHONY: build run clean objdump

build:
>cargo +nightly build

run: build
>bash tools/run-qemu.sh $(KERNEL_ELF)

objdump: build
>rust-objdump -d $(KERNEL_ELF) | less

clean:
>cargo clean
RS

cat > tools/run-qemu.sh <<'RS'
#!/usr/bin/env bash
set -e

KERNEL_ELF="$1"

qemu-system-riscv64 \
  -machine virt \
  -nographic \
  -bios default \
  -kernel "${KERNEL_ELF}"
RS

chmod +x tools/run-qemu.sh

cat > linker/riscv64.ld <<'RS'
OUTPUT_ARCH(riscv)
ENTRY(_start)

BASE_ADDRESS = 0x80200000;

SECTIONS
{
  . = BASE_ADDRESS;

  skernel = .;

  .text : {
    stext = .;
    *(.text.entry)
    *(.text .text.*)
    etext = .;
  }

  .rodata : {
    srodata = .;
    *(.rodata .rodata.*)
    erodata = .;
  }

  .data : {
    sdata = .;
    *(.data .data.*)
    edata = .;
  }

  .bss : {
    sbss = .;
    *(.bss .bss.*)
    ebss = .;
  }

  . = ALIGN(4096);
  .boot_stack (NOLOAD) : {
    *(.boot_stack)
  }

  ekernel = .;
}
RS

cat > arch/riscv64/boot.S <<'RS'
    .section .text.entry
    .globl _start

_start:
    la sp, boot_stack_top
    call rust_main

park:
    wfi
    j park

    .section .boot_stack, "aw", @nobits
    .align 12
boot_stack:
    .space 4096 * 16
boot_stack_top:
RS

cat > arch/riscv64/trap.S <<'RS'
    .section .text
    .globl __alltraps
    .globl __restore

__alltraps:
    addi sp, sp, -272

    sd x1,   8(sp)
    sd x3,  24(sp)
    sd x4,  32(sp)
    sd x5,  40(sp)

    addi t0, sp, 272
    sd t0, 16(sp)

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
    call rust_trap_handler

    mv a0, sp
    j __restore

__restore:
    mv sp, a0

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
RS

cat > src/main.rs <<'RS'
#![no_std]
#![no_main]

use core::arch::global_asm;

global_asm!(include_str!("../arch/riscv64/boot.S"));

mod config;
mod console;
mod drivers;
mod fs;
mod futex;
mod lang_items;
mod loader;
mod mm;
mod net;
mod sbi;
mod signal;
mod sync;
mod syscall;
mod task;
mod timer;
mod trap;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    clear_bss();

    crate::println!("UESTC-Kernel booting...");
    crate::println!("[arch] riscv64");
    crate::println!("[stage] full mechanism skeleton");

    mm::init();
    mm::test();

    fs::init();
    loader::init();
    sync::init();
    signal::init();
    futex::init();
    timer::init();
    drivers::init();
    net::init();
    syscall::init();
    task::init();

    trap::init();

    task::run_first_user_task();
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    unsafe {
        let start = sbss as *const () as usize;
        let end = ebss as *const () as usize;
        core::slice::from_raw_parts_mut(start as *mut u8, end - start).fill(0);
    }
}
RS

cat > src/config.rs <<'RS'
pub const PAGE_SIZE: usize = 4096;
pub const MEMORY_END: usize = 0x8800_0000;

pub const USER_STACK_SIZE: usize = PAGE_SIZE * 4;

pub const ENOSYS: isize = -38;
RS

cat > src/sbi.rs <<'RS'
#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;

    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("a0") arg0 => ret,
            in("a1") arg1,
            in("a2") arg2,
            in("a7") which,
        );
    }

    ret
}

pub fn console_putchar(ch: usize) {
    const SBI_CONSOLE_PUTCHAR: usize = 1;
    sbi_call(SBI_CONSOLE_PUTCHAR, ch, 0, 0);
}

#[allow(dead_code)]
pub fn shutdown() -> ! {
    const SBI_SHUTDOWN: usize = 8;
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);

    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}
RS

cat > src/console.rs <<'RS'
use core::fmt::{self, Write};

use crate::sbi::console_putchar;

pub struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for ch in s.bytes() {
            console_putchar(ch as usize);
        }

        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n");
    };
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
RS

cat > src/lang_items.rs <<'RS'
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crate::println!("[kernel panic] {}", info);

    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}
RS

cat > src/mm/mod.rs <<'RS'
pub mod address_space;
pub mod cow;
pub mod frame_allocator;
pub mod page_table;
pub mod vm_area;

pub use frame_allocator::{frame_alloc, frame_dealloc};

pub fn init() {
    frame_allocator::init();
    address_space::init();
    vm_area::init();
    cow::init();

    crate::println!("[mm] init");
}

pub fn test() {
    frame_allocator::test();
    page_table::test();
}
RS

cat > src/mm/frame_allocator.rs <<'RS'
use crate::config::{MEMORY_END, PAGE_SIZE};

static mut FRAME_ALLOCATOR: FrameAllocator = FrameAllocator {
    current: 0,
    end: 0,
    recycled: [0; 1024],
    recycled_len: 0,
};

pub fn init() {
    extern "C" {
        fn ekernel();
    }

    let start = align_up(ekernel as *const () as usize, PAGE_SIZE);
    let end = MEMORY_END;

    unsafe {
        FRAME_ALLOCATOR.init(start, end);
    }

    crate::println!("[mm::frame] start = {:#x}", start);
    crate::println!("[mm::frame] end   = {:#x}", end);
}

pub fn frame_alloc() -> Option<usize> {
    unsafe { FRAME_ALLOCATOR.alloc() }
}

pub fn frame_dealloc(ppn: usize) {
    unsafe {
        FRAME_ALLOCATOR.dealloc(ppn);
    }
}

pub fn test() {
    crate::println!("[mm::frame] test begin");

    let a = frame_alloc().unwrap();
    let b = frame_alloc().unwrap();
    let c = frame_alloc().unwrap();

    crate::println!("[mm::frame] alloc a = {:#x}", a);
    crate::println!("[mm::frame] alloc b = {:#x}", b);
    crate::println!("[mm::frame] alloc c = {:#x}", c);

    frame_dealloc(b);

    let d = frame_alloc().unwrap();

    crate::println!("[mm::frame] realloc d = {:#x}", d);
    assert_eq!(b, d);

    crate::println!("[mm::frame] test passed");
}

struct FrameAllocator {
    current: usize,
    end: usize,
    recycled: [usize; 1024],
    recycled_len: usize,
}

impl FrameAllocator {
    fn init(&mut self, start: usize, end: usize) {
        self.current = start / PAGE_SIZE;
        self.end = end / PAGE_SIZE;
        self.recycled_len = 0;
    }

    fn alloc(&mut self) -> Option<usize> {
        if self.recycled_len > 0 {
            self.recycled_len -= 1;
            return Some(self.recycled[self.recycled_len]);
        }

        if self.current < self.end {
            let ppn = self.current;
            self.current += 1;
            Some(ppn)
        } else {
            None
        }
    }

    fn dealloc(&mut self, ppn: usize) {
        if self.recycled_len >= self.recycled.len() {
            panic!("[mm::frame] recycled stack overflow");
        }

        self.recycled[self.recycled_len] = ppn;
        self.recycled_len += 1;
    }
}

const fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}
RS

cat > src/mm/page_table.rs <<'RS'
use crate::config::PAGE_SIZE;

const PTE_COUNT: usize = 512;

pub const PTE_V: usize = 1 << 0;
pub const PTE_R: usize = 1 << 1;
pub const PTE_W: usize = 1 << 2;
pub const PTE_X: usize = 1 << 3;
pub const PTE_U: usize = 1 << 4;
pub const PTE_A: usize = 1 << 6;
pub const PTE_D: usize = 1 << 7;

#[derive(Copy, Clone)]
#[repr(transparent)]
struct PageTableEntry {
    bits: usize,
}

impl PageTableEntry {
    fn new(ppn: usize, flags: usize) -> Self {
        let mut real_flags = flags;

        if flags & (PTE_R | PTE_W | PTE_X) != 0 {
            real_flags |= PTE_A | PTE_D;
        }

        Self {
            bits: (ppn << 10) | real_flags,
        }
    }

    fn is_valid(&self) -> bool {
        self.bits & PTE_V != 0
    }

    fn ppn(&self) -> usize {
        self.bits >> 10
    }

    fn flags(&self) -> usize {
        self.bits & 0x3ff
    }
}

#[repr(C, align(4096))]
struct PageTable {
    entries: [PageTableEntry; PTE_COUNT],
}

pub struct AddressSpace {
    root_ppn: usize,
}

impl AddressSpace {
    pub fn new() -> Self {
        let root_ppn = crate::mm::frame_alloc().expect("[page_table] alloc root failed");
        zero_frame(root_ppn);
        Self { root_ppn }
    }

    pub fn root_ppn(&self) -> usize {
        self.root_ppn
    }

    pub fn map(&mut self, va: usize, pa: usize, flags: usize) {
        assert_eq!(va % PAGE_SIZE, 0);
        assert_eq!(pa % PAGE_SIZE, 0);

        let vpn = va / PAGE_SIZE;
        let ppn = pa / PAGE_SIZE;

        let pte = self.find_pte_create(vpn);
        assert!(!pte.is_valid());

        *pte = PageTableEntry::new(ppn, flags | PTE_V);
    }

    pub fn translate(&self, va: usize) -> Option<(usize, usize)> {
        let vpn = va / PAGE_SIZE;
        let offset = va % PAGE_SIZE;
        let indexes = vpn_indexes(vpn);

        let mut ppn = self.root_ppn;

        for level in (0..3).rev() {
            let table = ppn_to_table(ppn);
            let pte = table.entries[indexes[level]];

            if !pte.is_valid() {
                return None;
            }

            let flags = pte.flags();
            let is_leaf = flags & (PTE_R | PTE_W | PTE_X) != 0;

            if is_leaf {
                return Some((pte.ppn() * PAGE_SIZE + offset, flags));
            }

            ppn = pte.ppn();
        }

        None
    }

    fn find_pte_create(&mut self, vpn: usize) -> &'static mut PageTableEntry {
        let indexes = vpn_indexes(vpn);
        let mut ppn = self.root_ppn;

        for level in (1..3).rev() {
            let table = ppn_to_table(ppn);
            let pte = &mut table.entries[indexes[level]];

            if !pte.is_valid() {
                let new_ppn = crate::mm::frame_alloc().expect("[page_table] alloc lower failed");
                zero_frame(new_ppn);
                *pte = PageTableEntry::new(new_ppn, PTE_V);
            }

            ppn = pte.ppn();
        }

        let table = ppn_to_table(ppn);
        &mut table.entries[indexes[0]]
    }
}

fn zero_frame(ppn: usize) {
    let start = ppn * PAGE_SIZE;
    unsafe {
        core::slice::from_raw_parts_mut(start as *mut u8, PAGE_SIZE).fill(0);
    }
}

fn ppn_to_table(ppn: usize) -> &'static mut PageTable {
    let pa = ppn * PAGE_SIZE;
    unsafe { &mut *(pa as *mut PageTable) }
}

fn vpn_indexes(vpn: usize) -> [usize; 3] {
    [
        vpn & 0x1ff,
        (vpn >> 9) & 0x1ff,
        (vpn >> 18) & 0x1ff,
    ]
}

pub fn test() {
    crate::println!("[mm::page_table] test begin");

    let mut space = AddressSpace::new();

    let va = 0x1000_0000usize;
    let ppn = crate::mm::frame_alloc().expect("[page_table] alloc data frame failed");
    let pa = ppn * PAGE_SIZE;

    space.map(va, pa, PTE_R | PTE_W | PTE_X | PTE_U);

    let result = space.translate(va).expect("[page_table] translate failed");

    crate::println!("[mm::page_table] root ppn = {:#x}", space.root_ppn());
    crate::println!("[mm::page_table] va       = {:#x}", va);
    crate::println!("[mm::page_table] pa       = {:#x}", result.0);
    crate::println!("[mm::page_table] flags    = {:#x}", result.1);

    assert_eq!(result.0, pa);
    assert!(result.1 & PTE_U != 0);

    crate::println!("[mm::page_table] test passed");
}
RS

cat > src/mm/address_space.rs <<'RS'
pub fn init() {
    crate::println!("[mm::address_space] stub init");
}

pub fn activate_kernel_space_stub() {
    crate::println!("[mm::address_space] Sv39 activate stub");
}
RS

cat > src/mm/vm_area.rs <<'RS'
pub fn init() {
    crate::println!("[mm::vm_area] stub init");
}
RS

cat > src/mm/cow.rs <<'RS'
pub fn init() {
    crate::println!("[mm::cow] stub init");
}
RS

cat > src/trap/mod.rs <<'RS'
use core::arch::global_asm;

global_asm!(include_str!("../../arch/riscv64/trap.S"));

pub mod context;
pub mod handler;

pub use context::TrapContext;

static mut INIT_TRAP_CONTEXT: TrapContext = TrapContext {
    regs: [0; 32],
    sstatus: 0,
    sepc: 0,
};

extern "C" {
    fn __alltraps();
    fn __restore(cx_addr: usize) -> !;
}

pub fn init() {
    unsafe {
        let addr = __alltraps as *const () as usize;
        core::arch::asm!("csrw stvec, {}", in(reg) addr);
    }

    crate::println!("[trap] init");
}

pub fn enter_user(entry: usize, user_sp: usize) -> ! {
    unsafe {
        let cx = core::ptr::addr_of_mut!(INIT_TRAP_CONTEXT);

        (*cx).regs = [0; 32];
        (*cx).regs[2] = user_sp;
        (*cx).sstatus = user_sstatus();
        (*cx).sepc = entry;

        crate::println!("[trap] enter user mode");
        __restore(cx as usize);
    }
}

#[no_mangle]
pub extern "C" fn rust_trap_handler(cx: &mut TrapContext) {
    handler::handle(cx);
}

fn user_sstatus() -> usize {
    let mut sstatus = read_sstatus();

    const SSTATUS_SPP: usize = 1 << 8;
    const SSTATUS_SPIE: usize = 1 << 5;

    sstatus &= !SSTATUS_SPP;
    sstatus |= SSTATUS_SPIE;

    sstatus
}

fn read_sstatus() -> usize {
    let value: usize;
    unsafe {
        core::arch::asm!("csrr {}, sstatus", out(reg) value);
    }
    value
}
RS

cat > src/trap/context.rs <<'RS'
#[repr(C)]
pub struct TrapContext {
    pub regs: [usize; 32],
    pub sstatus: usize,
    pub sepc: usize,
}
RS

cat > src/trap/handler.rs <<'RS'
use super::TrapContext;

pub fn handle(cx: &mut TrapContext) {
    let scause = read_scause();
    let stval = read_stval();

    crate::println!("[trap] scause = {:#x}", scause);
    crate::println!("[trap] sepc   = {:#x}", cx.sepc);
    crate::println!("[trap] stval  = {:#x}", stval);

    match scause {
        8 => handle_user_ecall(cx),
        _ => {
            crate::println!("[trap] unsupported trap");
            loop {
                unsafe {
                    core::arch::asm!("wfi");
                }
            }
        }
    }
}

fn handle_user_ecall(cx: &mut TrapContext) {
    let syscall_id = cx.regs[17];
    let args = [
        cx.regs[10],
        cx.regs[11],
        cx.regs[12],
        cx.regs[13],
        cx.regs[14],
        cx.regs[15],
    ];

    crate::println!("[trap] user syscall id = {}", syscall_id);

    let ret = crate::syscall::syscall(syscall_id, args);
    cx.regs[10] = ret as usize;
    cx.sepc += 4;
}

fn read_scause() -> usize {
    let value: usize;
    unsafe {
        core::arch::asm!("csrr {}, scause", out(reg) value);
    }
    value
}

fn read_stval() -> usize {
    let value: usize;
    unsafe {
        core::arch::asm!("csrr {}, stval", out(reg) value);
    }
    value
}
RS

cat > src/syscall/mod.rs <<'RS'
pub mod fs;
pub mod mm;
pub mod process;
pub mod time;

const SYS_WRITE: usize = 64;
const SYS_EXIT: usize = 93;
const SYS_GETPID: usize = 172;
const SYS_GETPPID: usize = 173;

pub fn init() {
    crate::println!("[syscall] init");
}

pub fn syscall(id: usize, args: [usize; 6]) -> isize {
    match id {
        SYS_WRITE => fs::sys_write(args[0], args[1], args[2]),
        SYS_EXIT => process::sys_exit(args[0] as i32),
        SYS_GETPID => process::sys_getpid(),
        SYS_GETPPID => process::sys_getppid(),
        _ => {
            crate::println!("[syscall] unsupported id = {}", id);
            crate::config::ENOSYS
        }
    }
}
RS

cat > src/syscall/fs.rs <<'RS'
pub fn sys_write(fd: usize, buf: usize, len: usize) -> isize {
    if fd != 1 && fd != 2 {
        return -1;
    }

    let bytes = unsafe { core::slice::from_raw_parts(buf as *const u8, len) };

    for &ch in bytes {
        crate::sbi::console_putchar(ch as usize);
    }

    len as isize
}

pub fn sys_openat() -> isize {
    crate::config::ENOSYS
}

pub fn sys_read() -> isize {
    crate::config::ENOSYS
}

pub fn sys_close() -> isize {
    crate::config::ENOSYS
}

pub fn sys_getdents64() -> isize {
    crate::config::ENOSYS
}
RS

cat > src/syscall/process.rs <<'RS'
pub fn sys_exit(code: i32) -> ! {
    crate::println!("[syscall] exit code = {}", code);

    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

pub fn sys_getpid() -> isize {
    1
}

pub fn sys_getppid() -> isize {
    0
}

pub fn sys_fork() -> isize {
    crate::config::ENOSYS
}

pub fn sys_execve() -> isize {
    crate::config::ENOSYS
}

pub fn sys_wait4() -> isize {
    crate::config::ENOSYS
}
RS

cat > src/syscall/mm.rs <<'RS'
pub fn sys_brk() -> isize {
    crate::config::ENOSYS
}

pub fn sys_mmap() -> isize {
    crate::config::ENOSYS
}

pub fn sys_munmap() -> isize {
    crate::config::ENOSYS
}
RS

cat > src/syscall/time.rs <<'RS'
pub fn sys_clock_gettime() -> isize {
    crate::config::ENOSYS
}

pub fn sys_nanosleep() -> isize {
    crate::config::ENOSYS
}
RS

cat > src/task/mod.rs <<'RS'
use core::arch::asm;

pub mod context;
pub mod process;
pub mod scheduler;
pub mod thread;

const USER_STACK_SIZE: usize = crate::config::USER_STACK_SIZE;

#[repr(align(16))]
struct UserStack([u8; USER_STACK_SIZE]);

static mut USER_STACK: UserStack = UserStack([0; USER_STACK_SIZE]);

pub fn init() {
    process::init();
    thread::init();
    scheduler::init();

    crate::println!("[task] init");
}

pub fn run_first_user_task() -> ! {
    let user_entry_addr = user_entry as *const () as usize;
    let user_stack_bottom = core::ptr::addr_of_mut!(USER_STACK) as usize;
    let user_stack_top = user_stack_bottom + USER_STACK_SIZE;

    crate::println!("[task] run first user task");
    crate::trap::enter_user(user_entry_addr, user_stack_top);
}

#[no_mangle]
extern "C" fn user_entry() -> ! {
    let msg = b"hello from user mode syscall write\n";
    user_syscall3(64, 1, msg.as_ptr() as usize, msg.len());

    let pid = user_syscall0(172);

    if pid == 1 {
        let ok = b"getpid returned 1\n";
        user_syscall3(64, 1, ok.as_ptr() as usize, ok.len());
    } else {
        let bad = b"getpid returned unexpected value\n";
        user_syscall3(64, 1, bad.as_ptr() as usize, bad.len());
    }

    user_syscall1(93, 0);

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

fn user_syscall0(id: usize) -> isize {
    let ret: isize;

    unsafe {
        asm!(
            "ecall",
            lateout("a0") ret,
            in("a7") id,
        );
    }

    ret
}

fn user_syscall1(id: usize, a0: usize) -> isize {
    let ret: isize;

    unsafe {
        asm!(
            "ecall",
            inlateout("a0") a0 => ret,
            in("a7") id,
        );
    }

    ret
}

fn user_syscall3(id: usize, a0: usize, a1: usize, a2: usize) -> isize {
    let ret: isize;

    unsafe {
        asm!(
            "ecall",
            inlateout("a0") a0 => ret,
            in("a1") a1,
            in("a2") a2,
            in("a7") id,
        );
    }

    ret
}
RS

cat > src/task/context.rs <<'RS'
pub fn switch_context_stub() {
    crate::println!("[task::context] switch stub");
}
RS

cat > src/task/process.rs <<'RS'
pub struct Process {
    pub pid: usize,
}

pub fn init() {
    crate::println!("[task::process] stub init");
}
RS

cat > src/task/thread.rs <<'RS'
pub struct Thread {
    pub tid: usize,
}

pub fn init() {
    crate::println!("[task::thread] stub init");
}
RS

cat > src/task/scheduler.rs <<'RS'
pub fn init() {
    crate::println!("[task::scheduler] stub init");
}

pub fn schedule_stub() {
    crate::println!("[task::scheduler] schedule stub");
}
RS

cat > src/fs/mod.rs <<'RS'
pub mod devfs;
pub mod ext4;
pub mod fat32;
pub mod fd_table;
pub mod file;
pub mod pipe;
pub mod procfs;
pub mod tmpfs;
pub mod vfs;

pub fn init() {
    vfs::init();
    fd_table::init();
    devfs::init();
    procfs::init();
    tmpfs::init();
    fat32::init();
    ext4::init();
    pipe::init();

    crate::println!("[fs] init");
}
RS

cat > src/fs/vfs.rs <<'RS'
pub fn init() {
    crate::println!("[fs::vfs] stub init");
}
RS

cat > src/fs/file.rs <<'RS'
pub struct File {
    pub readable: bool,
    pub writable: bool,
}
RS

cat > src/fs/fd_table.rs <<'RS'
pub fn init() {
    crate::println!("[fs::fd_table] stub init");
}
RS

cat > src/fs/devfs.rs <<'RS'
pub fn init() {
    crate::println!("[fs::devfs] stub init");
}
RS

cat > src/fs/procfs.rs <<'RS'
pub fn init() {
    crate::println!("[fs::procfs] stub init");
}
RS

cat > src/fs/tmpfs.rs <<'RS'
pub fn init() {
    crate::println!("[fs::tmpfs] stub init");
}
RS

cat > src/fs/fat32.rs <<'RS'
pub fn init() {
    crate::println!("[fs::fat32] stub init");
}
RS

cat > src/fs/ext4.rs <<'RS'
pub fn init() {
    crate::println!("[fs::ext4] stub init");
}
RS

cat > src/fs/pipe.rs <<'RS'
pub fn init() {
    crate::println!("[fs::pipe] stub init");
}
RS

cat > src/loader/mod.rs <<'RS'
pub mod elf;

pub fn init() {
    elf::init();
    crate::println!("[loader] init");
}
RS

cat > src/loader/elf.rs <<'RS'
pub fn init() {
    crate::println!("[loader::elf] stub init");
}

pub fn load_elf_stub() -> isize {
    crate::config::ENOSYS
}
RS

cat > src/sync/mod.rs <<'RS'
pub mod mutex;
pub mod wait_queue;

pub fn init() {
    mutex::init();
    wait_queue::init();

    crate::println!("[sync] init");
}
RS

cat > src/sync/mutex.rs <<'RS'
pub fn init() {
    crate::println!("[sync::mutex] stub init");
}
RS

cat > src/sync/wait_queue.rs <<'RS'
pub fn init() {
    crate::println!("[sync::wait_queue] stub init");
}
RS

cat > src/signal/mod.rs <<'RS'
pub fn init() {
    crate::println!("[signal] stub init");
}

pub fn deliver_signal_stub() -> isize {
    crate::config::ENOSYS
}
RS

cat > src/futex/mod.rs <<'RS'
pub fn init() {
    crate::println!("[futex] stub init");
}

pub fn futex_wait_stub() -> isize {
    crate::config::ENOSYS
}
RS

cat > src/timer/mod.rs <<'RS'
pub fn init() {
    crate::println!("[timer] stub init");
}

pub fn tick_stub() {}
RS

cat > src/drivers/mod.rs <<'RS'
pub mod uart;
pub mod virtio_blk;
pub mod virtio_net;

pub fn init() {
    uart::init();
    virtio_blk::init();
    virtio_net::init();

    crate::println!("[drivers] init");
}
RS

cat > src/drivers/uart.rs <<'RS'
pub fn init() {
    crate::println!("[drivers::uart] stub init");
}
RS

cat > src/drivers/virtio_blk.rs <<'RS'
pub fn init() {
    crate::println!("[drivers::virtio_blk] stub init");
}
RS

cat > src/drivers/virtio_net.rs <<'RS'
pub fn init() {
    crate::println!("[drivers::virtio_net] stub init");
}
RS

cat > src/net/mod.rs <<'RS'
pub fn init() {
    crate::println!("[net] stub init");
}
RS

cat > user/README.md <<'RS'
User programs will be added here later.

Current skeleton uses an embedded Rust function as the first U-mode task.
RS

cat > .gitignore <<'RS'
/target/

/user/*.elf
/user/*.bin
/user/*.o
/user/*.map

*.img
*.qcow2
*.raw
*.iso

*.log
*.out

.vscode/*
!.vscode/settings.json
!.vscode/extensions.json

.DS_Store
Thumbs.db

__pycache__/
*.pyc
.pytest_cache/
.venv/
.env
.conda/
RS

echo "full mechanism skeleton generated"
