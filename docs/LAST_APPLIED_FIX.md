# LAST_APPLIED_FIX

Version: v57

Goal:
- Add `read` scaffold and exercise `/dev/zero` from external init.elf.

Verified:
- stdout write
- openat `/dev/null`
- write `/dev/null`
- close fd 3
- openat `/dev/zero`
- read fd 4 into user stack buffer
- close fd 4
- getpid/getppid
- unsupported syscall
- exit
