# MM_STATUS

## Current Status

- frame allocator: PASS
- page table map/translate: PASS
- user-copy: PASS
- user address-space metadata: PASS
- Sv39 pure dry-run: PASS
- kernel/user mapping builders: PASS
- real page table build dry-run: PASS
- Sv39 activation scaffold: PASS
- kernel-only Sv39 activation: PASS
- kernel Sv39 trap smoke v43: under test

## Notes

v43 enables Sv39 in kernel-only mode and tests a supervisor ecall trap round trip.
U-mode is intentionally skipped in this isolated test.

## v43b kernel Sv39 trap isolated smoke

- Static identity Sv39 page table for low MMIO and RAM.
- satp activation enabled for isolated kernel test.
- Temporary kernel trap vector handles S-mode ecall.
- U-mode is intentionally not entered in this test.

## v43e - Kernel Sv39 trap smoke

- PASS: kernel-only Sv39 activation
- PASS: UART output after satp
- PASS: kernel data access after satp
- PASS: S-mode ebreak trap through stvec under Sv39
- PASS: scause = 0x3
- U-mode intentionally disabled in this isolated test
