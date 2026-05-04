# PROJECT_STATE

## Current verified milestone

- v45: Sv39 + U-mode ecall smoke passed.
- v46f: static ELF loader parser scaffold added while preserving Sv39 + U-mode smoke.

## Verified capabilities

- OpenSBI enters kernel.
- QEMU serial-file logging works.
- Kernel Sv39 activation works.
- Kernel Sv39 trap smoke works.
- Sv39 + U-mode ecall works.
- sys_write / getpid / getppid / ENOSYS / exit work in the Sv39 U-mode smoke path.
- Static ELF64 header and PT_LOAD parser scaffold exists.

## Current constraints

- ELF loader does not yet load segments into real user address space.
- execve is not implemented.
- VFS/rootfs are still scaffold/stub level.

## v46f

Static ELF loader parser scaffold passed; Sv39 + U-mode ecall smoke remains passing.

## v47 - ELF-linked user image loader scaffold

- Added static ELF parser scaffold.
- Added PT_LOAD parser scaffold.
- Added linked `.user` image metadata path.
- Kept existing Sv39 + U-mode syscall smoke as regression gate.

## v48

- v48: external user/init ELF scaffold.
- Synthetic user/init.elf is embedded via include_bytes!.
- Loader checks ELF64/RISC-V header and PT_LOAD metadata.
- Sv39 + U-mode ecall smoke remains the runtime regression path.
