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
