use core::arch::global_asm;

global_asm!(include_str!("../arch/riscv64/trap.S"));

#[repr(C)]
pub struct TrapContext {
    pub regs: [usize; 32],
    pub sstatus: usize,
    pub sepc: usize,
}

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

        const SSTATUS_SUM: usize = 1 << 18;
        core::arch::asm!("csrs sstatus, {}", in(reg) SSTATUS_SUM);
    }

    crate::println!("[trap] stvec initialized");
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
    let scause = read_scause();
    let stval = read_stval();

    crate::println!("[trap] scause = {:#x}", scause);
    crate::println!("[trap] sepc   = {:#x}", cx.sepc);
    crate::println!("[trap] stval  = {:#x}", stval);

    match scause {
        8 => {
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

fn user_sstatus() -> usize {
    let mut sstatus = read_sstatus();

    const SSTATUS_SPP: usize = 1 << 8;
    const SSTATUS_SPIE: usize = 1 << 5;

    sstatus &= !SSTATUS_SPP;
    sstatus |= SSTATUS_SPIE;

    sstatus
}

#[inline]
fn read_sstatus() -> usize {
    let value: usize;
    unsafe {
        core::arch::asm!("csrr {}, sstatus", out(reg) value);
    }
    value
}

#[inline]
fn read_scause() -> usize {
    let value: usize;
    unsafe {
        core::arch::asm!("csrr {}, scause", out(reg) value);
    }
    value
}

#[inline]
fn read_stval() -> usize {
    let value: usize;
    unsafe {
        core::arch::asm!("csrr {}, stval", out(reg) value);
    }
    value
}
