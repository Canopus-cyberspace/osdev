# LAST_APPLIED_FIX

Version: v61

Goal:
- Add mmap/munmap scaffold after brk.

Verified:
- mmap anonymous private page returns fixed user mapping
- munmap succeeds on that fixed mapping
- brk
- fstat/lseek
- getdents64 `/dev`
- open/read/write/close device scaffolds
- getpid/getppid
- unsupported syscall
- exit
