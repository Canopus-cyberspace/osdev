# SYSCALL_STATUS

## v63 - uname/time scaffold

Runtime external init ELF now exercises:
- fstat/lseek
- `/dev/null` write
- `/dev/zero` read
- `/dev` getdents64
- brk
- mmap/munmap
- mprotect/madvise
- uname
- clock_gettime
- gettimeofday
- getpid/getppid/unsupported/exit

Implemented central dispatcher actions:
- Uname
- ClockGettime
- Gettimeofday
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
