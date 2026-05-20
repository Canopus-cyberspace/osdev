# v161-v164 Iovec/IPC/Futex/Scheduler Report

## Files changed
- src/fs/runtime.rs
- src/mm/sv39_init_exec.rs
- apply_fix.sh
- apply_fix.bat
- docs/v161_v164_iovec_ipc_futex_scheduler_report.md

## Canonical subsystem choices
- Unified fd, OFD, object, IPC, futex, task, and wait queue state: src/fs/runtime.rs.
- Unified iovec data movement owner after usercopy: src/fs/runtime.rs RuntimeIovec helpers.
- User iovec parsing and SUM-enabled user memory copy: src/mm/user_buffer.rs and src/mm/sv39_init_exec.rs.
- Live syscall entry: rust_main -> mm::sv39_init_exec::run_external_init_elf_smoke -> dispatch_runtime_syscall -> RuntimeSyscallAction.

## Historical modules integrated
- v73 vector I/O scaffolds now route canonical fds through RuntimeIovec read/write helpers.
- v82 sendmsg/recvmsg use the same RuntimeIovec helper path as readv/writev.
- v82 POSIX mq and SysV msg/sem/shm routes validate create, lookup, operation, and removal on canonical registries.
- v66/v77 futex scaffolds now use a canonical futex object table and scheduler wait queue keys.
- v66 scheduler-yield/nanosleep scaffolds now touch the canonical task/wait-queue foundation.

## Syscall paths deepened
- readv
- writev
- preadv
- pwritev
- sendmsg
- recvmsg
- mq_open
- mq_unlink
- mq_timedsend
- mq_timedreceive
- mq_getsetattr
- msgget
- msgsnd
- msgrcv
- msgctl
- semget
- semop
- semtimedop
- semctl
- shmget
- shmat
- shmdt
- shmctl
- futex
- futex_waitv
- sched_yield
- nanosleep

## v161 unified iovec summary
- RuntimeIovec is the shared kernel-side iovec representation.
- readv/writev/preadv/pwritev convert user iovecs once, then call canonical runtime read_iovec/write_iovec helpers.
- sendmsg/recvmsg reuse the same helpers with msg_io accounting.
- RuntimeIoStats records read/write, positioned, msg, and byte totals for validation.

## v162 IPC registry summary
- POSIX mq supports repeated lookup by key, fd-backed send/receive, unlink, close, and recreate.
- SysV msg supports key lookup, send/receive, IPC_RMID, and post-remove ENOENT behavior.
- SysV sem supports key lookup, simple semop value mutation, IPC_RMID, and post-remove ENOENT behavior.
- SysV shm supports key lookup, attach/detach, IPC_RMID, and post-remove ENOENT behavior.

## v163 futex summary
- Futex objects are keyed by user address.
- FUTEX_WAIT checks observed vs expected value and records a waiter on match.
- FUTEX_WAKE decrements waiter counts and wakes the scheduler wait queue by key.
- Timeout-bearing waits are simplified and safe: they enqueue and immediately resolve without blocking the smoke path.

## v164 scheduler wait queue summary
- Canonical runtime has a current task snapshot with Running, Ready, and Waiting states.
- Wait queues are keyed objects with waiter and wakeup counts.
- sched_yield records yield activity without leaving the task blocked.
- nanosleep uses the timeout wait path foundation without implementing real time or blocking.

## Preserved runtime markers
- [ucompat-v151k7] vfs_tree_dirfd_multiinode PASS
- [ucompat-v154] fs_core_multi_feature PASS
- [ucompat-v155] namespace_procfd_multi_feature PASS
- [ucompat-v156] procfs_fd_observability PASS
- [ucompat-v157] unified historical kernel integration PASS
- [ucompat-v158] event pipe socket readiness PASS
- [ucompat-v159] timerfd deterministic readiness PASS
- [ucompat-v160] fd lifecycle cloexec close_range PASS

## New runtime markers
- [ucompat-v161] unified iovec io path PASS
- [ucompat-v162] ipc registry lifecycle PASS
- [ucompat-v163] futex wait wake object model PASS
- [ucompat-v164] scheduler wait queue foundation PASS

## Remaining incomplete Linux semantics
- Futex waits do not perform real task descheduling or priority inheritance.
- Scheduler wait queues do not yet drive a real run queue or context switching.
- POSIX mq and SysV IPC remain fixed-capacity simplified registries.
- sendmsg/recvmsg do not implement ancillary data, address routing, or protocol networking.
- execve/mm/signal/mount/security/network-driver work remains intentionally out of scope for v161-v164.

## Build log path
.repair_logs/v161_v164_iovec_ipc_futex_scheduler_20260508_105416/cargo_build.log

## QEMU serial log path
.repair_logs/v161_v164_iovec_ipc_futex_scheduler_20260508_105416/qemu.serial.log

## QEMU wrapper stdout path
.repair_logs/v161_v164_iovec_ipc_futex_scheduler_20260508_105416/run-qemu.stdout.log

## Forbidden warning gate result
- PASS: build output did not contain "matches any value", "unreachable pattern", or "warning: unused variable:".
