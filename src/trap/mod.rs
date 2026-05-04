use core::arch::global_asm;

global_asm!(include_str!("../../arch/riscv64/trap.S"));

pub mod context;
pub mod handler;

pub use context::TrapContext;

static mut INIT_TRAP_CONTEXT: TrapContext = TrapContext::zero();

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
        INIT_TRAP_CONTEXT = TrapContext::init_user_context(entry, user_sp);
        let cx = core::ptr::addr_of_mut!(INIT_TRAP_CONTEXT);

        crate::println!("[trap] enter user mode via __restore");
        __restore(cx as usize);
    }
}

#[no_mangle]
pub extern "C" fn rust_trap_handler(cx: &mut TrapContext) {
    handler::handle(cx);
}
