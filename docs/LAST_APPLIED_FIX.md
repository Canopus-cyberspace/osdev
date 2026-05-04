# LAST_APPLIED_FIX

Version: v41d

## Summary

- Fixed missing Sv39 smoke function names.
- Fixed `task::run_first_user_task() -> !` by ensuring all branches diverge.
- Kept real Sv39 activation disabled.
- Preserved U-mode syscall matrix regression.

## Expected Runtime Markers

- `[sv39-v41d] scaffold test passed`
- `[sv39-smoke-v41d] scaffold passed`
- `hello from umode v41d syscall write`
- `umode getpid returned 1`
- `umode getppid returned 0`
- `unsupported syscall returned -38`
