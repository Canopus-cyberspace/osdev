use crate::syscall::abi::RuntimeSyscallArgs;

#[inline(always)]
pub fn run_once_before_syscall() {
    super::legacy_markers::run_once_before_syscall();
}

#[inline(always)]
pub fn maybe_intercept_syscall(_args: RuntimeSyscallArgs) -> Option<isize> {
    None
}

#[inline(always)]
pub(crate) fn run_history_bus() -> Option<&'static str> {
    super::legacy_regression::run_history_bus()
}
