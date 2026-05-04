# LAST_APPLIED_FIX

Version: v47

Goal:

- Add ELF-like linked user image loader metadata path.
- Keep Sv39 + U-mode ecall smoke passing.

Expected runtime markers:

- `[elf-loader-v47] synthetic ELF self-test passed`
- `[elf-loader-v47] linked user image metadata passed`
- `hello from sv39 umode v45 syscall write`
- `[sv39-umode-v45d] smoke passed`
