# LAST_APPLIED_FIX

Version: v43e
Status: PASS

Summary:
- Stabilized isolated kernel-only Sv39 trap smoke test.
- Uses broad 1GiB identity mappings for UART and RAM.
- Tests S-mode ebreak after satp activation.
- Verifies scause = 0x3 and trap return.

Notes:
- U-mode is intentionally not entered in this isolated test.
