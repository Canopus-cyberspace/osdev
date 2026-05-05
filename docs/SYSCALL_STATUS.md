# SYSCALL_STATUS

## v62 - mprotect/madvise scaffold

Runtime external init ELF now exercises:
- fstat/lseek
- `/dev/null` write
- `/dev/zero` read
- `/dev` getdents64
- brk
- mmap
- munmap
- mmap
- mprotect
- madvise
- munmap
- getpid/getppid/unsupported/exit

Implemented central dispatcher actions:
- Mprotect
- Madvise
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
