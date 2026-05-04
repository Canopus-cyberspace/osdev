# LAST_APPLIED_FIX

## v45j - Formalize Sv39 + U-mode ecall smoke success

Status: PASS

This fix package does not rewrite the working v45 implementation. It formalizes the pass condition using the actual runtime markers currently emitted by the kernel:

- `[stage] Sv39 + U-mode ecall smoke v45g`
- `[sv39-umode-v45d] begin`
- `[sv39-umode-v45d] after satp`
- `hello from sv39 umode v45 syscall write`
- `umode getpid returned 1`
- `umode getppid returned 0`
- `unsupported syscall returned -38`
- `[sv39-umode-v45d] exit code = 0`
- `[sv39-umode-v45d] smoke passed`

Notes:

- Sv39 is actually enabled via `satp`.
- User text is mapped at a virtual address away from UART MMIO.
- U-mode ecall path reaches kernel trap handling.
- `sys_write`, `getpid`, `getppid`, unsupported syscall -> `-38`, and `exit` are verified under Sv39.
