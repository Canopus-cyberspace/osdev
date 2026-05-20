# v173-v174 Signal Delivery Report

## Files changed
- src/fs/runtime.rs
- src/syscall/mod.rs
- src/mm/sv39_init_exec.rs
- apply_fix.sh
- docs/v173_v174_signal_delivery_report.md

## Canonical subsystem choices
- Signal state lives in src/fs/runtime.rs beside the canonical task, fd, VFS, exec, and VMA state.
- The live syscall route remains rust_main -> mm::sv39_init_exec::run_external_init_elf_smoke -> dispatch_runtime_syscall -> RuntimeSyscallAction.
- kill, tkill, tgkill, rt_sigaction, rt_sigprocmask, and rt_sigreturn now route through RuntimeSyscallAction into shared runtime state.

## v173 signal frame and rt_sigreturn
- Added per-task signal action tables with handler, flags, restorer, and mask metadata.
- Added pending signal slots, blocked mask integration, and signal-frame metadata per task.
- Added deliverable-signal validation against the current blocked mask and installed action.
- Added a post-syscall delivery hook that writes a compact user signal frame and redirects the trap context to the installed handler.
- Added rt_sigreturn restore plumbing to restore sepc, user sp, and the saved blocked mask.

## v174 SIGCHLD and process-group basics
- Child exit now queues SIGCHLD to a live parent in canonical task state.
- kill, tkill, and tgkill perform pid/tgid validation and pending-signal bookkeeping.
- kill(0, sig), kill(-1, sig), and kill(-pgid, sig) use simplified process-group/all-task delivery behavior.
- Delivery counters record direct, tkill, tgkill, group, queued, delivered, returned, and SIGCHLD signal evidence.

## Out of scope
- No mount, security, network-driver, rootfs-devfs, or full scheduler work was added.
- Signal delivery is a compact frame/restore foundation; full POSIX signal semantics, siginfo/ucontext ABI fidelity, alternate stacks, and scheduler preemption are left for later batches.

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

## New runtime markers
- [ucompat-v173] signal frame rt_sigreturn PASS
- [ucompat-v174] sigchld process group signal PASS

## Build log path
.repair_logs/v173_v174_signal_delivery_20260508_213330/cargo_build.log

## QEMU serial log path
.repair_logs/v173_v174_signal_delivery_20260508_213330/qemu.serial.log

## QEMU wrapper stdout path
.repair_logs/v173_v174_signal_delivery_20260508_213330/run-qemu.stdout.log

## Forbidden warning gate result
- PASS: build output did not contain "matches any value", "unreachable pattern", or "warning: unused variable:".
