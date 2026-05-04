#[repr(C)]
pub struct TrapContext {
    pub regs: [usize; 32],
    pub sstatus: usize,
    pub sepc: usize,
}

impl TrapContext {
    pub const fn zero() -> Self {
        Self {
            regs: [0; 32],
            sstatus: 0,
            sepc: 0,
        }
    }

    pub fn init_user_context(entry: usize, user_sp: usize) -> Self {
        let mut cx = Self::zero();
        cx.regs[2] = user_sp;
        cx.sepc = entry;
        cx.sstatus = user_sstatus();
        cx
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

fn read_sstatus() -> usize {
    let value: usize;
    unsafe {
        core::arch::asm!("csrr {}, sstatus", out(reg) value);
    }
    value
}
