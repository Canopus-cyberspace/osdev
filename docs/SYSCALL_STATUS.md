# SYSCALL_STATUS

## v58 - fstat/lseek scaffold

Runtime external init ELF now exercises:
- write(stdout)
- fstat(stdout, user_stack_buffer)
- lseek(stdout, 0, SEEK_SET) -> -ESPIPE
- openat("/dev/null")
- write(devnull_fd)
- close(devnull_fd)
- openat("/dev/zero")
- read(devzero_fd, stack_buffer, 16)
- close(devzero_fd)
- getpid
- getppid
- unsupported -> -38
- exit

Implemented central dispatcher actions:
- FStat
- LSeek
- Read
- OpenAt
- Close
- Write
- Return
- Exit
