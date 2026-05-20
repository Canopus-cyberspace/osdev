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
- `hello from external init.elf v108 syscall write fd-vfs syscall write`
- `external init getpid returned 1`
- `external init getppid returned 0`
- `external init unsupported returned -38`
- `[external-init-v82] smoke passed`

## v51

Status: process initialization scaffold added

Implemented:
- `UserProgram` metadata wrapper around loaded external `init.elf`
- `ProcessInitInfo` placeholder for future `execve`
- Runtime self-test marker: `[process-init-v51] self-test passed`

Still TODO:
- copy argv/envp/auxv to the user stack
- expose this path as `execve`
- load binaries from VFS/rootfs instead of `include_bytes!`

## v51

Status: process initialization scaffold added

Implemented:
- `UserProgram` metadata wrapper around loaded external `init.elf`
- `ProcessInitInfo` placeholder for future `execve`
- Runtime self-test marker: `[process-init-v51] self-test passed`

Still TODO:
- copy argv/envp/auxv to the user stack
- expose this path as `execve`
- load binaries from VFS/rootfs instead of `include_bytes!`

## v52

Status: execve/user-stack scaffold added

Implemented:
- `ProcessInitInfo` metadata validation
- initial user stack dry-run
- argv/envp/auxv placeholder layout

Runtime self-test markers:
- `[process-init-v52] self-test passed`
- `[user-stack-v52] dry-run passed`
- `[loader-v52] self-test passed`
