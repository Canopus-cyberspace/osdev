# v175-v178 FS Environment Report

## Files changed
- src/fs/runtime.rs
- src/mm/sv39_init_exec.rs
- apply_fix.sh
- apply_fix.bat
- docs/v175_v178_fs_env_report.md

## Canonical subsystem choices
- The implementation lives in the shared canonical runtime state in src/fs/runtime.rs.
- The live syscall route remains rust_main -> mm::sv39_init_exec::run_external_init_elf_smoke -> dispatch_runtime_syscall -> RuntimeSyscallAction.
- Rootfs, tmpfs, devfs, procfs, mount metadata, statfs/fstatfs, and generated proc files all share the same runtime node, fd, task, and VMA tables.

## v175 rootfs/tmpfs backend
- Added per-node filesystem-kind metadata and a mount registry.
- Rootfs files and directories retain inode metadata, file contents, truncate state, link counts, unlink behavior, and rename behavior through the existing runtime VFS node table.
- Added tmpfs mount support that marks mounted subtrees as tmpfs while preserving canonical VFS lookup and fd behavior.
- Executable file storage remains in the same VFS file bytes used by execve, and the v175 check validates execve preparation from a file created in rootfs.

## v176 devfs core devices
- Added /dev/null, /dev/zero, /dev/console, /dev/tty, /dev/random, and /dev/urandom during runtime reset.
- /dev/null discards writes and returns EOF on reads.
- /dev/zero returns zero-filled reads and accepts writes.
- /dev/console and /dev/tty accept writes through the runtime console path.
- /dev/random and /dev/urandom provide deterministic simplified byte streams suitable for the current runtime smoke path.

## v177 procfs process status/maps
- Added dynamic procfs nodes for /proc/self/fd, /proc/self/status, /proc/self/stat, and /proc/self/maps.
- /proc/self/status is generated from canonical task/fd/signal state.
- /proc/self/stat is generated from canonical task identity, parent, process group, session, and fd state.
- /proc/self/maps is generated from canonical per-task VMA metadata, including exec, heap, stack, and mmap regions.
- /proc/self/fd getdents and proc fd readlink are backed by the live fd table.

## v178 mount tree and statfs
- Added rootfs, tmpfs, devfs, and procfs mount records.
- Added mount and umount2 runtime helpers for bounded mount-tree behavior.
- statfs and fstatfs now report filesystem kind, magic, block size, file counts, fd usage, and mount count from canonical runtime state.
- Live statfs/fstatfs syscall helpers route through the runtime VFS and fd tables.

## Out of scope
- No permissions/capabilities, network-driver, full namespace, or userland matrix work was added.
- This batch does not attempt a complete POSIX VFS, device, or procfs ABI; it adds the bounded canonical foundations required for v175-v178.

## Preserved runtime markers
- [ucompat-v151k7] vfs_tree_dirfd_multiinode PASS
- [ucompat-v154] fs_core_multi_feature PASS
- [ucompat-v155] namespace_procfd_multi_feature PASS
- [ucompat-v156] procfs_fd_observability PASS
- [ucompat-v157] unified historical kernel integration PASS
- [ucompat-v158] event pipe socket readiness PASS
- [ucompat-v159] timerfd deterministic readiness PASS
- [ucompat-v160] fd lifecycle cloexec close_range PASS
- [ucompat-v161] unified iovec io path PASS
- [ucompat-v162] ipc registry lifecycle PASS
- [ucompat-v163] futex wait wake object model PASS
- [ucompat-v164] scheduler wait queue foundation PASS
- [ucompat-v165] task table process lifecycle PASS
- [ucompat-v166] fork clone child task PASS
- [ucompat-v167] exit zombie wait lifecycle PASS
- [ucompat-v168] per task runtime snapshot PASS
- [ucompat-v169] execve from canonical vfs PASS
- [ucompat-v170] execve user stack cloexec PASS
- [ucompat-v171] vma page fault foundation PASS
- [ucompat-v172] lazy mmap brk munmap mprotect PASS
- [ucompat-v173] signal frame rt_sigreturn PASS
- [ucompat-v174] sigchld process group signal PASS

## New runtime markers
- [ucompat-v175] rootfs tmpfs backend PASS
- [ucompat-v176] devfs core devices PASS
- [ucompat-v177] procfs process status maps PASS
- [ucompat-v178] mount tree statfs PASS

## Build log path
.repair_logs/v175_v178_fs_env_20260508_220333/cargo_build.log

## QEMU serial log path
.repair_logs/v175_v178_fs_env_20260508_220333/qemu.serial.log

## QEMU wrapper stdout path
.repair_logs/v175_v178_fs_env_20260508_220333/run-qemu.stdout.log

## Forbidden warning gate result
- PASS: build output did not contain "matches any value", "unreachable pattern", or "warning: unused variable:".
