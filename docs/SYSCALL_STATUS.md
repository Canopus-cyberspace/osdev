# SYSCALL_STATUS

## v59 - getdents64 scaffold

Runtime external init ELF now exercises:
- write(stdout)
- fstat(stdout)
- lseek(stdout)
- openat("/dev/null")
- write(devnull_fd)
- close(devnull_fd)
- openat("/dev/zero")
- read(devzero_fd, stack_buffer, 16)
- close(devzero_fd)
- openat("/dev")
- getdents64(dev_fd, stack_buffer, 256)
- close(dev_fd)
- getpid
- getppid
- unsupported -> -38
- exit

Implemented central dispatcher actions:
- GetDents64
- FStat
- LSeek
- Read
- OpenAt
- Close
- Write
- Return
- Exit
