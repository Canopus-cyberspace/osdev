# Definition of "Complete Kernel" for This Project

## Important distinction

A "complete kernel" here does not mean a full Linux replacement.

It means a competition-grade educational kernel that can run a meaningful multi-program userland test matrix with coherent semantics across:

- boot
- virtual memory
- trap/syscall
- process lifecycle
- execve
- VFS/FD
- scheduler
- signal
- IPC
- event/poll/epoll
- procfs/devfs
- credentials/permissions
- compatibility test runner

## Minimum complete kernel criteria

The kernel should be considered complete for competition purposes when all criteria below are satisfied.

### 1. Boot and architecture

Required:

- OpenSBI enters kernel.
- kernel stack and BSS initialization work.
- Sv39 is enabled.
- trap entry is stable.
- U-mode ecall traps to kernel.
- QEMU fresh runtime logs are stable.
- build/QEMU smoke can run from scripts.

Current status:

- Mostly complete since v42-v57 and preserved through v157.

### 2. Syscall path

Required:

- There is one live syscall dispatch path.
- Runtime syscall handlers route to canonical subsystem state.
- Dead or old trap paths are not used for new semantics.
- Unsupported syscalls return stable Linux-like errors.

Current status:

- v157 established live `RuntimeSyscallAction` integration.
- Future versions must deepen subsystem semantics behind that path.

### 3. VFS and FD

Required:

- One fdtable model.
- One OFD model.
- shared offset across dup/dup3.
- regular files, directories, symlinks, procfs, devfs, pipe, eventfd, timerfd, socket objects all use canonical fd lifecycle.
- openat/read/write/lseek/close/statx/fstat/getdents/readlink/link/rename/unlink behavior is coherent.
- close-on-exec support works.

Current status:

- v157 has a strong canonical VFS/FD base.
- Need deeper devfs/mount/rootfs/permission and CLOEXEC integration.

### 4. Process and task lifecycle

Required:

- task table.
- fork/clone creates child task.
- parent-child relationship.
- exit creates zombie.
- wait4/waitid reaps children.
- per-task fdtable/cwd/root/credentials/signal mask.
- process groups and sessions at least simplified.
- procfs sees real task state.

Current status:

- procfs observability exists.
- real fork/wait/exit lifecycle remains incomplete.

### 5. execve and user program loading

Required:

- execve loads ELF from canonical VFS/rootfs.
- PT_LOAD segments are mapped to user address space.
- argv/envp/auxv are copied to user stack.
- CLOEXEC fds are closed.
- old address space is replaced.
- errors are returned for invalid ELF/path/permissions.

Current status:

- external init ELF path works.
- real execve from VFS remains incomplete.

### 6. Memory management

Required:

- VMA list/tree.
- page fault handling.
- brk/mmap lazy allocation.
- munmap VMA split/removal.
- mprotect permission changes.
- usercopy validates user mappings.
- file-backed mapping can be simplified but must be consistent.
- fork COW is highly desirable.

Current status:

- Sv39 and simple mmap/brk semantics exist.
- VMA/page fault/lazy allocation remains a major gap.

### 7. Scheduler and blocking

Required:

- run queue.
- sleep/wait queues.
- timer interrupt or equivalent scheduling tick.
- blocking syscalls do not freeze the entire system.
- futex wait/wake integrates with task state.
- pipe/poll/epoll/eventfd/timerfd can wake waiters.

Current status:

- scheduler/futex smoke exists.
- real blocking scheduler semantics remain incomplete.

### 8. Signal

Required:

- sigaction.
- signal mask.
- pending signals.
- delivery to user mode.
- signal frame.
- rt_sigreturn restores trap context.
- kill/tkill/tgkill.
- SIGCHLD integration.

Current status:

- signal scaffolds and guards exist.
- real delivery/trampoline remains incomplete.

### 9. IPC and event objects

Required:

- pipe buffers.
- eventfd counter.
- timerfd expiration or deterministic test-time expiration.
- epoll interest list.
- poll readiness.
- POSIX mq object lifecycle.
- SysV msg/sem/shm object lifecycle.
- simplified blocking behavior.

Current status:

- v157 canonical registry exists.
- v158+ should deepen readiness and IPC semantics.

### 10. Filesystem environment

Required:

- rootfs or tmpfs.
- devfs with null/zero/console/tty/random where needed.
- procfs with process/fd/status information.
- mount tree basics.
- statfs/fstatfs.
- path normalization and cwd/root constraints.

Current status:

- VFS and procfd exist.
- full mount/devfs/procfs environment remains incomplete.

### 11. Credentials and permissions

Required:

- uid/gid state.
- effective/real/saved ids.
- chmod/chown behavior at least simplified.
- file permission checks.
- directory execute checks.
- capability bitset model at least simplified.
- setuid/setgid behavior if required by tests.

Current status:

- scaffolds exist.
- real enforcement remains incomplete.

### 12. Userland compatibility

Required:

- run multiple ELF user programs from a rootfs.
- collect each program exit status.
- report test results.
- keep external-init smoke but move beyond single marker.
- include filesystem, process, memory, signal, pipe, poll, IPC, and libc-style tests.

Current status:

- external init smoke and integrated markers exist.
- full multi-program test matrix remains incomplete.
