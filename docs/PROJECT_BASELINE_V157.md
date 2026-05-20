# Project Baseline: v157

## Current stable baseline

The current verified stable baseline is **v157**.

v157 completed a canonical integration pass in the live `RuntimeSyscallAction` path.

## Verified runtime path

```text
rust_main
-> mm::sv39_init_exec::run_external_init_elf_smoke
-> dispatch_runtime_syscall
-> RuntimeSyscallAction
```

## Key artifacts from v157

```text
docs/v157_unified_historical_kernel_integration_report.md
.repair_logs/v157_unified_historical_kernel_integration_20260507_211949/cargo_build.log
.repair_logs/v157_unified_historical_kernel_integration_20260507_211949/qemu.serial.log
apply_fix.sh
apply_fix.bat
```

## Verified fresh QEMU runtime markers

```text
[ucompat-v151k7] vfs_tree_dirfd_multiinode PASS
[ucompat-v154] fs_core_multi_feature PASS
[ucompat-v155] namespace_procfd_multi_feature PASS
[ucompat-v156] procfs_fd_observability PASS
[ucompat-v157] unified historical kernel integration PASS
```

## v157 canonical subsystem summary

- VFS / FD / OFD / inode / dentry / procfd / event / socket / IPC canonical state lives primarily in `src/fs/runtime.rs`.
- Usercopy and iovec helper logic lives in `src/mm/user_buffer.rs`.
- New subsystem work should route through the live `RuntimeSyscallAction` path.
- Historical `ucompat_*` modules may remain as regression evidence, but new real behavior should not be isolated in marker-only modules.

## What v157 does not mean

v157 means historical implementations were connected to a canonical runtime path.

It does not mean the kernel has complete Linux semantics.

The remaining work is to deepen real behavior in these areas:

- event/pipe/socket readiness
- process/fork/wait/exit
- execve from VFS
- VMA/page fault/lazy allocation
- signal delivery
- scheduler blocking and wakeup
- mount/rootfs/devfs/procfs
- IPC blocking/lifecycle
- credentials/permissions/capability
- realistic userland compatibility matrix
