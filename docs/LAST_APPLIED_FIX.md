# LAST_APPLIED_FIX

Version: v42

Status: applied by repair package

Summary:
- Enabled isolated kernel-only Sv39 activation smoke path.
- Mapped UART MMIO and RAM identity before writing satp.
- Verified println and kernel data read/write after satp.
- U-mode is intentionally disabled in this isolated test.
