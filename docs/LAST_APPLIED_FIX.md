# LAST_APPLIED_FIX

## v37 - Kernel address-space builder dry-run

Status: APPLIED BY SCRIPT

Changes:
- Added `src/mm/kernel_builder.rs`.
- Added `KernelAddressSpaceBuilder` metadata plan for kernel text/rodata/data+bss/stacks.
- No `satp` write.
- No Sv39 activation.
- U-mode syscall matrix must remain passing.

Expected runtime markers:
- `[kernel-builder-v37] dry-run passed`
- `hello from umode`
- `umode getpid returned 1`
- `unsupported syscall returned -38`
