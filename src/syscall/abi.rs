#[derive(Copy, Clone, Debug)]
pub struct SyscallArgs {
    pub id: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
}

pub type RuntimeSyscallArgs = SyscallArgs;

impl SyscallArgs {
    pub const fn new(
        id: usize,
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
    ) -> Self {
        Self { id, a0, a1, a2, a3, a4, a5 }
    }

    #[inline(always)]
    pub fn from_regs(regs: &[usize; 32]) -> Self {
        Self::new(regs[17], regs[10], regs[11], regs[12], regs[13], regs[14], regs[15])
    }
}

#[inline(always)]
pub const fn return_value(value: isize) -> usize {
    value as usize
}
