# PROJECT_STATE

## Current Branch

feature/restore-umode

## Completed Milestones

- v24: stable full mechanism skeleton passed
- v25: warning cleanup and QEMU serial-file runner passed
- v28: minimal U-mode syscall smoke test passed
- v29: U-mode trap path refactor passed
- v30c: U-mode syscall matrix passed
- v31: user-copy abstraction passed
- v32e: user-copy bounds and getppid U-mode test passed
- v33: user address-space metadata scaffold passed
- v34f: Sv39 pure dry-run passed
- v35: kernel address-space dry-run passed
- v36e: safe Sv39 activation scaffold passed
- v37: kernel mapping builder dry-run passed
- v38: user mapping builder dry-run passed
- v39: real page-table build dry-run passed
- v40: kernel Sv39 activation scaffold added, disabled by default

## Verified Capabilities

- QEMU boots through OpenSBI
- boot.S enters rust_main
- serial-file logging works
- frame allocator test passes
- page-table map/translate test passes
- U-mode ecall works
- syscall write works through user-copy path
- syscall getpid works
- syscall getppid works
- unsupported syscall returns -38 ENOSYS
- syscall exit path works

## Current Constraints

- Sv39 real activation is not enabled yet
- ELF loader is not implemented
- VFS is still stub
- process/thread/scheduler are still scaffold/stub
- rootfs is not implemented
- signal/futex/timer are stub

## Current QEMU Method

- Use `tools/run-qemu.sh`
- Use QEMU `-serial file:<log>`
- Logs are stored in `.repair_logs/`

## Next Planned Step

v41:
- isolated kernel-only Sv39 activation test
- do not enter U-mode while first testing actual `satp` switch

## v40b

- Added safe Sv39 activation scaffold.
- `ENABLE_KERNEL_SV39_SMOKE = false`.
- `satp` is not written.
- U-mode syscall matrix remains the regression baseline.

## Latest
- v40c: Sv39 activation scaffold compile fix passed; Sv39 remains disabled.

## v40d

Sv39 smoke scaffold compile fix applied. Sv39 is still disabled; U-mode syscall matrix remains the regression test.
