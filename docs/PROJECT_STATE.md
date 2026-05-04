# PROJECT_STATE

## Current Milestone

v45j: Sv39 + U-mode ecall smoke verified.

## Verified Capabilities

- OpenSBI enters kernel.
- QEMU serial-file logging works.
- Kernel Sv39 activation works.
- Kernel trap under Sv39 works.
- U-mode under Sv39 works.
- U-mode syscall matrix under Sv39 works:
  - write
  - getpid
  - getppid
  - unsupported syscall returns -38
  - exit

## Current Branch Recommendation

Use a dedicated branch such as:

- `feature/kernel-sv39-activation`

## Next Recommended Step

v46: consolidate Sv39 U-mode smoke into a clean test-mode selector, then restore normal module initialization around the working Sv39 U-mode path.

## v45j Verification Log

- Repair log: /home/lenovo/projects/uestc-kernel/.repair_logs/fix_sv39_umode_v45j_20260505_004034.log
- Serial log: /home/lenovo/projects/uestc-kernel/.repair_logs/qemu_smoke_v45j_20260505_004034.serial.log
- Stderr log: /home/lenovo/projects/uestc-kernel/.repair_logs/qemu_smoke_v45j_20260505_004034.stderr.log
