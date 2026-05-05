# SYSCALL_STATUS

## v57 - read scaffold

Runtime external init ELF now exercises:
- write(stdout)
- openat("/dev/null")
- write(devnull_fd)
- close(devnull_fd)
- openat("/dev/zero")
- read(devzero_fd, stack_buffer, 16)
- close(devzero_fd)
- write(stdout)
- getpid
- getppid
- unsupported -> -38
- exit

Implemented central dispatcher actions:
- Read
- OpenAt
- Close
- Write
- Return
- Exit
