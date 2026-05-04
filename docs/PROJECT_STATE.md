# PROJECT_STATE

## Current Branch

feature/restore-umode

## Completed Milestones

- v35: kernel address-space dry-run passed
- v24: stable full mechanism skeleton passed
- v25: warning cleanup and QEMU serial-file runner passed
- v28: minimal U-mode syscall smoke test passed
- v29: U-mode trap path refactor passed
- v30c: U-mode syscall matrix passed
- v31: user-copy abstraction added
- v32e: user-copy bounds path verified with U-mode syscall matrix

## Verified Capabilities

- QEMU boots through OpenSBI
- boot.S enters rust_main
- serial-file logging works
- frame allocator test passes
- page table map/translate test passes
- direct user-copy self-test passes
- U-mode ecall works
- syscall write uses copy_from_user
- syscall getpid works
- syscall getppid works
- unsupported syscall returns -38 ENOSYS
- syscall exit path works

## Current Constraints

- Sv39 is not enabled
- ELF loader is not implemented
- VFS is stub
- process/thread/scheduler are still minimal/stub
- rootfs is not implemented
- signal/futex/timer are stub

## Current QEMU Method

- Use tools/run-qemu.sh
- Use serial file logging
- Logs are stored in .repair_logs/

## Next Planned Step

v33:
- Add brk syscall skeleton and process memory layout scaffolding
- Keep Sv39 disabled
- Keep U-mode syscall matrix passing

## v33 Update

- Added user address-space metadata scaffold.
- Added `UserAddressSpace`, `UserRegion`, `MapPermission`.
- Sv39 is still disabled.
- U-mode syscall matrix should remain passing.


## v34 - Sv39 Page Table Dry Run

Status: PASS expected after repair package smoke test.

Completed:
- Added `src/mm/sv39_preflight.rs`.
- Added dry-run user code and user stack page-table mappings.
- Verified user code is R/X/U and not W.
- Verified user stack is R/W/U and not X.
- Kept `satp` activation disabled.
- Kept U-mode syscall matrix path enabled.

Next:
- v35 kernel address-space identity-map preflight without enabling Sv39.

## v34b

- Sv39 dry-run page table preflight passed.
- User code page mapped as R/X/U and not W.
- User stack page mapped as R/W/U and not X.
- Sv39 remains disabled; no `satp` write yet.

## v34c Update

- v34c: Sv39 dry-run page table preflight passed.
- Sv39 remains disabled; this step does not write `satp`.
- U-mode syscall matrix remains the active smoke test path.

## v34d Status

- Added Sv39 page-table dry-run preflight.
- Fixed `sv39_preflight::test()` / `test_page_table_dry_run()` naming mismatch.
- Verified user code page R/X/U and user stack page R/W/U permissions without enabling satp.
- U-mode syscall matrix remains expected to pass.

## v34e - Sv39 Dry-run Non-destructive Fix

- Sv39 remains disabled.
- Dry-run uses dummy physical pages instead of allocating/zeroing user data frames.
- Page-table map/translate/permission check remains covered.
- U-mode syscall matrix remains the smoke-test target.

- v34f: Sv39 pure dry-run preflight passes without disturbing U-mode syscall matrix

## v35d

- Added kernel address-space dry-run.
- Sv39 still disabled.
- U-mode syscall matrix remains passing.

## v35e Kernel Space Dry Run

- Added kernel text/rodata/data-bss permission dry-run.
- Sv39 remains disabled.
- U-mode syscall matrix remains the main runtime smoke test.
