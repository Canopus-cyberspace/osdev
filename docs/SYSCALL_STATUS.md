# SYSCALL_STATUS

## v61 - mmap/munmap scaffold

Runtime external init ELF now exercises:
- write(stdout)
- fstat(stdout)
- lseek(stdout)
- openat/write/close `/dev/null`
- openat/read/close `/dev/zero`
- openat/getdents64/close `/dev`
- brk(0)
- brk(0x40031000)
- mmap(NULL, 4096, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0)
- munmap(mapped_addr, 4096)
- getpid
- getppid
- unsupported -> -38
- exit

Implemented central dispatcher actions:
- Mmap
- Munmap
- Brk
- GetDents64
- FStat
- LSeek
- Read
- OpenAt
- Close
- Write
- Return
- Exit
