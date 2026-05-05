# LAST_APPLIED_FIX

Version: v56

Goal:
- Add `openat`/`close` scaffold and exercise `/dev/null` from external init.elf.

Verified:
- stdout write
- openat `/dev/null`
- write to `/dev/null`
- close fd 3
- getpid/getppid
- unsupported syscall
- exit
