# v157 Unified Historical Kernel Integration Report

## Files changed
- src/fs/runtime.rs
- src/fs/mod.rs
- src/fs/ucompat_history_v158.rs
- src/mm/mod.rs
- src/mm/sv39_init_exec.rs
- src/mm/user_buffer.rs
- src/syscall/mod.rs
- user/build_init_elf.py
- user/init.elf
- apply_fix.sh
- apply_fix.bat

## Canonical subsystem choices
- VFS/FD/OFD/inode/dentry/procfd/event/socket/IPC: src/fs/runtime.rs.
- Runtime syscall path: rust_main -> mm::sv39_init_exec::run_external_init_elf_smoke -> dispatch_runtime_syscall -> RuntimeSyscallAction.
- Usercopy/iovec helpers: src/mm/user_buffer.rs.
- Historical active evidence bus: src/fs/ucompat_history_v158.rs for v151k7/v154/v155/v156 stable markers, with the old process-signal smoke deferred to the canonical runtime snapshot validation.

## Historical modules integrated
- v151k7 VFS tree behavior is now exercised by the external init program through canonical openat/read/write/lseek/link/rename/symlink/getdents/statx/readlink/unlink/close.
- v154/v155/v156 evidence modules remain connected through the history bus.
- v157 canonical validation connects VFS, fdtable/OFD, procfs fd views, eventfd/timerfd/pipe/socketpair/epoll, iovec-facing I/O, and POSIX/SysV IPC registries.

## Syscall paths routed to canonical state
- openat, read, write, lseek, close, dup, dup3, fcntl, fstat, newfstatat, statx, getdents64.
- mkdirat, unlinkat, renameat/renameat2, linkat, symlinkat, readlinkat, chdir, getcwd, faccessat/faccessat2, truncate/ftruncate, statfs/fstatfs.
- pipe2, eventfd2, timerfd_create/settime/gettime, socket, socketpair, sendto/recvfrom, sendmsg/recvmsg, ppoll, epoll_create1/ctl/pwait.
- mq_open/unlink/timedsend/timedreceive/getsetattr, msgget/msgsnd/msgrcv/msgctl, semget/semop/semtimedop/semctl, shmget/shmat/shmdt/shmctl.

## Pre-v42 boot/kernel skeleton inventory
- no_std/no_main entry: src/main.rs.
- RISC-V boot entry and stack setup: arch/riscv64/boot.S.
- Linker script: linker/riscv64.ld.
- BSS clearing: src/main.rs clear_bss.
- SBI/UART console: src/sbi.rs and src/console.rs.
- panic/lang items: src/lang_items.rs.
- QEMU/OpenSBI path: tools/run-qemu.sh.
- Early Sv39/trap runtime: src/mm/sv39_init_exec.rs.

## VFS/FD integration summary
- One canonical fdtable and OFD model allocate regular files, directories, procfs views, pipes, eventfd, timerfd, sockets, epoll, and mq descriptors.
- dup/dup3 share OFD offset and status flags.
- dirfd-relative path resolution uses the same resolver as cwd/root and metadata syscalls.
- getdents64/readlinkat/statx/fstat read canonical inode metadata.

## Process/procfs integration summary
- proc_snapshot and proc_fd_readlink read canonical task/process and fdtable state.
- pid/ppid/tgid/pgid/sid/cwd/root/fd_count are exposed from the canonical runtime snapshot.

## MM/usercopy/iovec integration summary
- copy_cstr_from_user, copy_from_user, copy_to_user, and read_iovec_array are common helpers in src/mm/user_buffer.rs.
- readv/writev/preadv/pwritev and sendmsg/recvmsg share the common iovec helper path for canonical fds.

## Event/socket/IPC integration summary
- pipe2/eventfd/timerfd/socket/socketpair/epoll allocate descriptors through the canonical fdtable.
- close and close_range release canonical fds.
- ppoll/epoll inspect object readiness.
- socketpair loopback data is held in canonical socket objects.
- POSIX mq and SysV msg/sem/shm use simplified canonical registries with IDs, lookup, send/receive, and remove operations.

## Remaining incomplete Linux semantics
- Permission, mount, namespace, scheduler, signal delivery, timer expiration, networking protocol, VMA, and persistent filesystem behavior remain simplified.
- Some historical smoke-only modules remain present for evidence/inventory but are no longer the live syscall state owner.

## Build log path
.repair_logs/v157_unified_historical_kernel_integration_20260507_211949/cargo_build.log

## QEMU serial log path
.repair_logs/v157_unified_historical_kernel_integration_20260507_211949/qemu.serial.log

## Observed historical regression markers
- [ucompat-v151k7] vfs_tree_dirfd_multiinode PASS
- [ucompat-v154] fs_core_multi_feature PASS
- [ucompat-v155] namespace_procfd_multi_feature PASS
- [ucompat-v156] procfs_fd_observability PASS

## Observed v157 integration marker
- [ucompat-v157] unified historical kernel integration PASS

## Forbidden warning gate result
- PASS: build output did not contain "matches any value", "unreachable pattern", or "warning: unused variable:".
