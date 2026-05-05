# LAST_APPLIED_FIX

Version: v60

Goal:
- Add `brk` scaffold and map a small fixed user heap.

Verified:
- brk(0)
- brk(0x40031000)
- fstat/lseek
- getdents64 `/dev`
- open/read/write/close device scaffolds
- getpid/getppid
- unsupported syscall
- exit
