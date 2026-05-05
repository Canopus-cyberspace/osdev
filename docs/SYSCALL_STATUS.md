# SYSCALL_STATUS

## v65 - path/tty/fcntl scaffold

Runtime external init ELF now exercises:
- prior fd/mm/time/proc syscalls
- getcwd
- fcntl
- ioctl TIOCGWINSZ
- readlinkat
- umask
- chdir
- getpid/getppid/unsupported/exit

Implemented central dispatcher actions:
- Getcwd
- Fcntl
- Ioctl
- Readlinkat
- Umask
- Chdir
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
