# PROJECT_STATE

## Current milestone

v36c: Sv39 activation scaffold added.

## Verified capabilities

- QEMU OpenSBI boot works.
- serial-file logging works.
- U-mode ecall works.
- syscall write/getpid/getppid/unsupported/exit matrix works.
- user-copy abstraction works.
- user address-space metadata scaffold works.
- Sv39 pure dry-run works without touching real satp.
- kernel address-space dry-run works.
- Sv39 satp encode/decode scaffold works with real activation disabled.

## Current constraints

- Sv39 is not actually enabled yet.
- ELF loader is not implemented.
- VFS is still mostly stub.
- Process/thread/scheduler are still mostly stub.

## Next planned step

v37: kernel-only Sv39 activation experiment behind a feature flag, still keeping stable U-mode regression path safe.


## v36d - Sv39 Activation Scaffold

- Added src/mm/sv39.rs
- Added make_satp / satp_mode / satp_ppn helpers
- Real satp activation remains disabled by default
- U-mode syscall matrix must continue to pass

## v36e

- Added safe Sv39 activation scaffold.
- Sv39 remains disabled.
- U-mode syscall matrix remains the regression target.


## v37 Kernel Address-Space Builder

- Added `KernelAddressSpaceBuilder` dry-run scaffold.
- Describes kernel text / rodata / data+bss / stacks as mapping regions.
- Checks permissions without enabling Sv39.
- U-mode syscall matrix remains the regression test.

## v38 user mapping builder

- Added non-destructive user mapping builder dry-run.
- User text / guard / stack region metadata is validated.
- Sv39 remains disabled.
- U-mode syscall matrix remains the regression test.

## v39 Update

- v39: real AddressSpace page-table build dry-run passed
- Sv39 still disabled; no `satp` activation in normal path.
- U-mode syscall matrix should remain passing.
