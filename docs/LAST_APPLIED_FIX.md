# LAST_APPLIED_FIX

Version: v58

Goal:
- Add `fstat` and `lseek` scaffolds.

Verified:
- fstat stdout copies a minimal stat structure to user stack
- lseek stdout returns -ESPIPE
- openat `/dev/null`
- write `/dev/null`
- openat `/dev/zero`
- read fd 4 into user stack buffer
- getpid/getppid
- unsupported syscall
- exit
