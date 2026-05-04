# LAST_APPLIED_FIX

Version: v36e
Status: PASS

Verified:
- `cargo +nightly build`
- QEMU serial-file smoke test with 45s timeout
- Sv39 activation scaffold prints runtime markers
- Sv39 activation remains disabled
- U-mode syscall matrix still passes

Expected runtime markers:
- `[mm::sv39] activation scaffold init v36e`
- `[mm::sv39] activation scaffold test passed v36e`
- `hello from umode`
- `umode getpid returned 1`
- `unsupported syscall returned -38`
