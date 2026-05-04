# ELF_STATUS

## v50b

Status: implemented

Implemented:
- external `user/init.elf` generation without requiring a host RISC-V assembler
- ELF64 RISC-V header parser
- PT_LOAD parser
- external init ELF load-page path
- external init ELF execution path under Sv39 U-mode
- robust trap save/restore path that resets `sscratch` before returning to U-mode

Expected runtime markers:
- `[stage] external init.elf Sv39 U-mode smoke v50b`
- `[elf-loader-v50b] parser self-test passed`
- `[init-image-v50b] external init image self-test passed`
- `hello from external init.elf v50 syscall write`
- `external init getpid returned 1`
- `external init getppid returned 0`
- `external init unsupported returned -38`
- `[external-init-v50b] smoke passed`
