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

## v49 - External init ELF load-page scaffold

- Generates and embeds `user/init.elf`.
- Parses ELF64/RISC-V header and PT_LOAD segment.
- Copies PT_LOAD bytes into a kernel-owned page as a load dry-run.
- Keeps the already passing Sv39 + U-mode ecall smoke path as regression.

## v49c external init load-page scaffold

- ELF loader parser scaffold retained.
- External `user/init.elf` is embedded and copied into a kernel load page.
- Sv39 + U-mode smoke regression remains the required runtime check.

## v49d

- Fixed crate root config module visibility for loader init-image code.
- External init ELF load-page scaffold remains present.
- Sv39 + U-mode ecall smoke remains passing.

## v50 - External init ELF execution path

- Added generated external `user/init.elf`
- Added loader/init-image path for loading PT_LOAD into a kernel-managed page
- Added Sv39 U-mode execution path using external init ELF entry
- Expected external init syscalls:
  - write
  - getpid
  - getppid
  - unsupported -> -38
  - exit

## v50b - External init ELF execution trap fix

- Added robust external init ELF Sv39 U-mode execution path
- Replaced minimal trap frame with full TrapContext save/restore
- Restores sscratch to trap_stack_top before returning to U-mode
- Expected external init syscalls:
  - write
  - getpid
  - getppid
  - unsupported -> -38
  - exit

## v51 - Process initialization scaffold for execve

- Added `loader::process_image`
- Added `UserProgram`
- Added `ProcessInitInfo`
- Wrapped external `init.elf` load result in process initialization metadata
- Kept external init ELF Sv39 U-mode execution smoke passing

## v51 - Process initialization scaffold for execve

- Added `loader::process_image`
- Added `UserProgram`
- Added `ProcessInitInfo`
- Wrapped external `init.elf` load result in process initialization metadata
- Kept external init ELF Sv39 U-mode execution smoke passing

## v52 - Larger batch: ProcessInitInfo + initial user stack dry-run

- Added/updated `loader::process_image`
- Added `loader::user_stack`
- Added `ProcessInitInfo`
- Added `UserProgram`
- Added initial user stack dry-run with argc/argv/envp/auxv placeholders
- Kept external init ELF Sv39 U-mode execution smoke passing
