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

## v53 - Larger batch: process/fd/syscall scaffold

- Added process metadata scaffold
- Added Process / ProcessState
- Added PID allocator scaffold
- Added fd table scaffold
- Added syscall dispatch scaffold
- Added execve scaffold that validates the embedded `/init` program metadata
- External init ELF Sv39 U-mode smoke remains the runtime regression path

## v53 - Larger batch: process/fd/syscall scaffold

- Added process metadata scaffold
- Added Process / ProcessState
- Added PID allocator scaffold
- Added fd table scaffold
- Added syscall dispatch scaffold
- Added execve scaffold that validates the embedded `/init` program metadata
- External init ELF Sv39 U-mode smoke remains the runtime regression path

## v53b - Syscall module conflict fix

- Fixed E0761 by deleting `src/syscall.rs`
- Kept `src/syscall/mod.rs` as the canonical syscall module
- Preserved process/fd/syscall scaffold self-tests
- External init ELF Sv39 U-mode smoke remains the runtime regression path

## v53c - Safe process/fd/syscall scaffold regression fix

- Fixed syscall module conflict by keeping `src/syscall/mod.rs`
- Converted process/fd/syscall scaffold tests to non-panicking runtime checks
- Kept external init ELF Sv39 U-mode smoke as the primary regression path

## v53d - Isolate scaffold from runtime smoke

- Fixed syscall module conflict by keeping only `src/syscall/mod.rs`
- Kept process/fd/syscall scaffold compiled
- Isolated v53 scaffold self-tests from runtime
- Restored external init ELF Sv39 U-mode smoke as the only QEMU regression path

## v53f - Trap entry alignment fix

- Diagnosed v53d regression: external init reached `enter user`, then rebooted before first user ecall handler output.
- Root cause: `stvec` trap entry could become 2-byte aligned after code layout changes.
- RISC-V `stvec` direct mode needs low two bits clear, so the trap entry is now explicitly `.balign 4`.
- The external init ELF Sv39 U-mode smoke remains the primary regression path.

## v54 - Central syscall dispatcher

- Added central runtime syscall dispatcher in `src/syscall/mod.rs`
- Moved runtime syscall decision out of `sv39_init_exec.rs`
- External init ELF trap handler now delegates to `syscall::dispatch_runtime_syscall`
- Preserved v53f trap entry alignment fix
- External init ELF Sv39 U-mode smoke remains passing

## v54 - Central syscall dispatcher

- Added central runtime syscall dispatcher in `src/syscall/mod.rs`
- Moved runtime syscall decision out of `sv39_init_exec.rs`
- External init ELF trap handler now delegates to `syscall::dispatch_runtime_syscall`
- Preserved v53f trap entry alignment fix
- External init ELF Sv39 U-mode smoke remains passing

## v55 - fd-backed write dispatcher

- Added fd-backed write routing
- stdout/stderr route to console
- `/dev/null` scaffold target reserved at fd 3
- bad fd returns -EBADF
- External init ELF write/getpid/getppid/ENOSYS/exit remains passing

## v56 - openat/close `/dev/null` scaffold

- Extended external init ELF to call openat and close
- Added `/dev/null` open/close runtime scaffold
- Added central syscall actions for openat and close
- Kept external init ELF Sv39 U-mode smoke passing

## v57 - read `/dev/zero` scaffold

- Extended external init ELF to call read
- Added `/dev/zero` open/read/close runtime scaffold
- Added central syscall action for read
- Kept external init ELF Sv39 U-mode smoke passing

## v58 - fstat/lseek scaffold

- Extended external init ELF to call fstat and lseek
- Added minimal stat copy-out to user buffer
- Added lseek scaffold returning -ESPIPE for character devices
- Kept external init ELF Sv39 U-mode smoke passing

## v59 - getdents64 `/dev` scaffold

- Extended external init ELF to call getdents64
- Added `/dev` directory scaffold at fd 5
- Wrote minimal linux_dirent64 entries into user buffer
- Kept external init ELF Sv39 U-mode smoke passing
