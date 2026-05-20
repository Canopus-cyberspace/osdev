# v165-v168 Process Lifecycle Report

## Files changed
- src/fs/runtime.rs
- src/mm/sv39_init_exec.rs
- src/syscall/mod.rs
- apply_fix.sh
- apply_fix.bat
- docs/v165_v168_process_lifecycle_report.md

## Canonical subsystem choices
- Task/process state: src/fs/runtime.rs KernelCore task table.
- Procfs process snapshots: src/fs/runtime.rs ProcSnapshot and RuntimeTaskSnapshot.
- Live syscall route: rust_main -> mm::sv39_init_exec::run_external_init_elf_smoke -> dispatch_runtime_syscall -> RuntimeSyscallAction.
- Existing VFS/fd state remains the canonical fd/OFD source, with per-task fd counts snapshotted for clone inheritance.

## Historical modules integrated
- v69 clone/wait4/exit scaffolds now route through canonical task-table helpers instead of fixed fake pids.
- v74 setpgid/getpgid/getsid/setsid now read and update the canonical task fields.
- v81 waitid now observes and reaps canonical zombie children.
- v66 rt_sigprocmask now stores a per-task signal-mask value in canonical runtime state.
- v156-style procfs process observability now reads selected task snapshots rather than only a singleton pid.

## Syscall paths now routed to canonical state
- getpid
- getppid
- gettid
- set_tid_address
- clone
- wait4
- waitid
- exit
- exit_group
- setpgid
- getpgid
- getsid
- setsid
- rt_sigprocmask

## v165 task table/process lifecycle
- Runtime tasks carry pid, ppid, tgid, pgid, sid, state, exit code, wait key, yield count, fork return value, fd count, cwd/root nodes, and signal mask.
- The current task starts as pid 1/tgid 1/pgid 1/sid 1 with three inherited stdio descriptors.
- proc_snapshot_for_pid reads the same task record used by lifecycle syscalls.

## v166 fork/clone child task
- clone_task allocates a real child task slot and deterministic child pid.
- The parent receives the child pid; the child snapshot stores fork_return = 0.
- The child inherits ppid, group/session fields, fd count, cwd/root, and signal mask from the parent snapshot.

## v167 exit/zombie/wait lifecycle
- exit_task_pid marks child tasks zombie when a live parent exists.
- wait4 observes a zombie child, writes the shifted wait status, and reaps the task slot.
- waitid observes a zombie child, returns its pid and exit code, and reaps the task slot.
- No-child waits return ECHILD.

## v168 per-task runtime snapshot
- RuntimeTaskSnapshot exposes fd_count, cwd_len, root_len, and signal_mask per selected pid.
- proc_snapshot_for_pid reads the selected task snapshot.
- Validation proves a cloned child keeps its inherited fd/cwd/root/signal snapshot while the parent closes an fd afterward.

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

## New runtime markers
- [ucompat-v165] task table process lifecycle PASS
- [ucompat-v166] fork clone child task PASS
- [ucompat-v167] exit zombie wait lifecycle PASS
- [ucompat-v168] per task runtime snapshot PASS

## Remaining incomplete Linux semantics
- clone does not duplicate address spaces or perform real child execution.
- clone flag handling is intentionally simplified and does not yet model all Linux sharing modes.
- wait without a zombie child does not block on the scheduler yet.
- per-task fdtable state is currently represented as inherited snapshots over the canonical fd/OFD table, not a full independent fdtable copy.
- signal delivery, execve replacement, mm/page-fault work, mount/security, and network-driver work remain out of scope for v165-v168.

## Build log path
.repair_logs/v165_v168_process_lifecycle_20260508_111103/cargo_build.log

## QEMU serial log path
.repair_logs/v165_v168_process_lifecycle_20260508_111103/qemu.serial.log

## QEMU wrapper stdout path
.repair_logs/v165_v168_process_lifecycle_20260508_111103/run-qemu.stdout.log

## Forbidden warning gate result
- PASS: build output did not contain "matches any value", "unreachable pattern", or "warning: unused variable:".
