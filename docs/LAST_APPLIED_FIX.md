# LAST_APPLIED_FIX

Version: v39
Title: Real page table build dry-run
Status: PASS expected after apply_fix

## Changes

- Added `src/mm/page_table_build.rs`.
- Builds real `AddressSpace` page-table objects for representative kernel identity mappings.
- Builds real `AddressSpace` page-table objects for representative user text and stack mappings.
- Checks translate results and permissions.
- Does not write `satp`.
- Does not enable Sv39.
- Keeps U-mode syscall matrix regression running.

## Expected runtime markers

- `[page-table-build-v39] real page table build passed`
- `[sv39-preflight-v34f] pure dry-run passed`
- `hello from umode`
- `umode getpid returned 1`
- `unsupported syscall returned -38`
