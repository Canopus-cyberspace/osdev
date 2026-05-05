# SYSCALL_STATUS

## v56 - openat/close scaffold

Runtime external init ELF now exercises:
- write(stdout)
- openat(AT_FDCWD, "/dev/null", O_WRONLY, 0)
- write(devnull_fd)
- close(devnull_fd)
- write(stdout)
- getpid
- getppid
- unsupported -> -38
- exit

Implemented central dispatcher actions:
- OpenAt
- Close
- Write
- Return
- Exit
