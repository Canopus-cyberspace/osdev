# MM_STATUS

## Completed

- frame allocator smoke test
- page table map / translate smoke test
- user-copy abstraction
- user-copy bounds test
- user address-space metadata scaffold
- Sv39 pure dry-run preflight
- kernel address-space dry-run
- Sv39 activation scaffold v36d

## Current Sv39 Status

- make_satp / satp_mode / satp_ppn helpers exist
- real satp activation is disabled by default
- no Sv39 address-space switch is performed in the normal smoke path

## Next

- build a real kernel address space using existing dry-run metadata
- activate Sv39 only after a dedicated kernel-only smoke test

## v36e

- Added safe `mm::sv39` activation scaffold.
- `make_satp`, `satp_mode`, `satp_ppn`, `read_satp`, `sfence_vma`, and `activate_satp_unchecked` exist.
- `ENABLE_SV39_ACTIVATION_TEST = false`; v36e never writes `satp` during smoke tests.
