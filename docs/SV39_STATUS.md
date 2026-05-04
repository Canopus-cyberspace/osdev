# SV39_STATUS

## Current Status

- v42: kernel-only Sv39 activation passed.
- v43e: kernel Sv39 trap smoke passed with `ebreak` and `scause = 0x3`.
- v44: Sv39 U-mode preparation scaffold passed.
- v45j: Sv39 + U-mode ecall smoke is formally verified using QEMU serial-file logs.

## Verified in v45j

- Kernel identity mapping remains usable after writing `satp`.
- UART output works after Sv39 activation.
- U-mode entry under Sv39 works.
- U-mode `ecall` traps into the kernel with `scause = 0x8`.
- `write`, `getpid`, `getppid`, unsupported syscall -> `-38`, and `exit` work under Sv39.

## Still TODO

- General process address spaces.
- ELF loader integration.
- Real process/thread model under Sv39.
- Page fault handling.
- copy_from_user/copy_to_user via page-table walking rather than direct access/SUM-only smoke path.

## v45j Verification Log

- Repair log: /home/lenovo/projects/uestc-kernel/.repair_logs/fix_sv39_umode_v45j_20260505_004034.log
- Serial log: /home/lenovo/projects/uestc-kernel/.repair_logs/qemu_smoke_v45j_20260505_004034.serial.log
- Stderr log: /home/lenovo/projects/uestc-kernel/.repair_logs/qemu_smoke_v45j_20260505_004034.stderr.log

## v53f

Trap entry alignment fix:
- `__sv39_init_v50b_alltraps` is explicitly 4-byte aligned.
- `stvec` is written with low two bits cleared for direct mode.
