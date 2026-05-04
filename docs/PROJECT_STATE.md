# PROJECT_STATE

## Current Stage

Kernel-only Sv39 trap smoke v43.

## Verified Before v43

- Stable full mechanism skeleton.
- QEMU serial-file logging.
- U-mode syscall matrix without Sv39.
- User-copy abstraction.
- User/kernel address-space dry-runs.
- Real page table build dry-run.
- Kernel-only Sv39 activation.

## v43 Goal

Enable Sv39 in kernel-only mode, trigger a supervisor ecall, handle the trap, return, and continue printing.

## Important Constraint

v43 does not enter U-mode. U-mode under Sv39 will be a later step.
