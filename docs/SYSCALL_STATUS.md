# SYSCALL_STATUS

## v64 - process/resource/random scaffold

Runtime external init ELF now exercises:
- prior fd/mm/time syscalls
- set_tid_address
- set_robust_list
- getuid/geteuid/getgid/getegid
- gettid
- sysinfo
- prlimit64
- getrandom
- getpid/getppid/unsupported/exit

Implemented central dispatcher actions:
- SetTidAddress
- SetRobustList
- Sysinfo
- Prlimit64
- Getrandom
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
