# LAST_APPLIED_FIX

Version: v59

Goal:
- Add `getdents64` scaffold and expose a minimal `/dev` directory.

Verified:
- openat `/dev`
- getdents64 fd 5 writes `.`, `..`, `null`, `zero`
- close fd 5
- fstat/lseek
- open/read/write/close device scaffolds
- getpid/getppid
- unsupported syscall
- exit
