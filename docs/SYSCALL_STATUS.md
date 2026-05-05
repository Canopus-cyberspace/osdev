# SYSCALL_STATUS

## v55 - fd-backed write dispatcher

Runtime external init ELF smoke verifies through central dispatcher and fd-backed write:
- write(fd=1)
- getpid
- getppid
- unsupported -> -38
- exit

Implemented:
- `RuntimeSyscallAction::Write { fd, user_ptr, len, target }`
- central dispatcher performs fd write target validation
- runtime layer outputs console writes and supports `/dev/null` scaffold target

Still TODO:
- central read/openat/close/fstat/lseek/getdents64
- brk/mmap/munmap
- real execve syscall
- wait4/fork/clone
