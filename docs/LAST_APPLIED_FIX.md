# LAST_APPLIED_FIX

Version: v46f

Goal:

- Add static ELF loader parser scaffold.
- Preserve the previously passing Sv39 + U-mode ecall smoke path.

Expected runtime markers:

- [elf-loader-v46f] self-test passed
- hello from sv39 umode v45 syscall write
- umode getpid returned 1
- umode getppid returned 0
- unsupported syscall returned -38
- [sv39-umode-v45d] smoke passed
