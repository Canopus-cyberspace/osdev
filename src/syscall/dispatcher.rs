use super::abi::RuntimeSyscallArgs;
use super::RuntimeSyscallAction;

pub fn dispatch_runtime_syscall(args: RuntimeSyscallArgs) -> RuntimeSyscallAction {
    super::dispatch_runtime_syscall_impl(args)
}
