# Full Kernel Completion Master Plan

## Answer to the key planning question

This plan is intended to cover the path from the current v157 baseline to a competition-grade complete kernel.

However, implementation must still be incremental.

The correct strategy is:

```text
Long-range roadmap: complete and explicit.
Codex execution: one version and one subsystem group per run.
```

Do not ask Codex to implement the whole kernel in one prompt.

## Current baseline

Stable baseline: v157.

Preserved runtime markers:

```text
[ucompat-v151k7] vfs_tree_dirfd_multiinode PASS
[ucompat-v154] fs_core_multi_feature PASS
[ucompat-v155] namespace_procfd_multi_feature PASS
[ucompat-v156] procfs_fd_observability PASS
[ucompat-v157] unified historical kernel integration PASS
```

## Roadmap structure

The remaining work is split into four phases.

### Phase 1: Runtime I/O and event readiness

Versions:

```text
v158-v164
```

Purpose:

- Deepen canonical fd objects.
- Make event-like syscalls observe real object state.
- Prepare blocking/wakeup foundations.

### Phase 2: Process, exec, memory, signal

Versions:

```text
v165-v174
```

Purpose:

- Move from one external init scenario to real task/process execution.
- Load ELF from VFS.
- Build VMA/page fault model.
- Deliver signals.

### Phase 3: Filesystem, devices, security, network

Versions:

```text
v175-v184
```

Purpose:

- Complete userland environment.
- Add devfs/procfs/rootfs/mount basics.
- Add permission/credentials/capability.
- Deepen network and IPC enough for compatibility tests.

### Phase 4: Userland compatibility and competition hardening

Versions:

```text
v185-v190
```

Purpose:

- Run multiple ELF programs.
- Add compatibility matrix.
- Harden error paths.
- Freeze kernel API behavior.
- Produce final evidence report.

## Phase 1 overview: v158-v164

### v158: event/pipe/socket/poll/epoll readiness

Main deliverable:

- Canonical fd readiness works for pipe, eventfd, socketpair, poll, epoll.

Marker:

```text
[ucompat-v158] event pipe socket readiness PASS
```

### v159: timerfd and deterministic timer readiness

Main deliverable:

- timerfd has expiration state and readiness.
- clock/nanosleep/timerfd share simplified time source.

Marker:

```text
[ucompat-v159] timerfd deterministic readiness PASS
```

### v160: close_range, CLOEXEC, fd lifecycle hardening

Main deliverable:

- close_range works on canonical fdtable.
- CLOEXEC is stored and observed.
- fd release updates procfs view.

Marker:

```text
[ucompat-v160] fd lifecycle cloexec close_range PASS
```

### v161: readv/writev/sendmsg/recvmsg unified iovec hardening

Main deliverable:

- common iovec helper path is used by regular files, pipe, socketpair, and message APIs.

Marker:

```text
[ucompat-v161] unified iovec io path PASS
```

### v162: POSIX mq and SysV IPC lifecycle deepening

Main deliverable:

- object create/lookup/send/receive/remove behavior is coherent.

Marker:

```text
[ucompat-v162] ipc registry lifecycle PASS
```

### v163: futex wait/wake object model

Main deliverable:

- futex keys, wait queues, and wake counts are modeled.
- Blocking can be simplified but state must be coherent.

Marker:

```text
[ucompat-v163] futex wait wake object model PASS
```

### v164: scheduler wait queue foundation

Main deliverable:

- introduce task states and wait queues enough for future blocking.

Marker:

```text
[ucompat-v164] scheduler wait queue foundation PASS
```

## Phase 2 overview: v165-v174

### v165: task table and process lifecycle

Marker:

```text
[ucompat-v165] task table process lifecycle PASS
```

### v166: fork/clone child creation

Marker:

```text
[ucompat-v166] fork clone child task PASS
```

### v167: exit/zombie/wait4/waitid

Marker:

```text
[ucompat-v167] exit zombie wait lifecycle PASS
```

### v168: per-task fdtable/cwd/root/signal snapshot

Marker:

```text
[ucompat-v168] per task runtime snapshot PASS
```

### v169: execve from canonical VFS

Marker:

```text
[ucompat-v169] execve from canonical vfs PASS
```

### v170: argv/envp/auxv user stack and CLOEXEC

Marker:

```text
[ucompat-v170] execve user stack cloexec PASS
```

### v171: VMA list and page fault foundation

Marker:

```text
[ucompat-v171] vma page fault foundation PASS
```

### v172: mmap/brk/munmap/mprotect lazy allocation

Marker:

```text
[ucompat-v172] lazy mmap brk munmap mprotect PASS
```

### v173: signal delivery frame and rt_sigreturn

Marker:

```text
[ucompat-v173] signal frame rt_sigreturn PASS
```

### v174: SIGCHLD and process-group signal basics

Marker:

```text
[ucompat-v174] sigchld process group signal PASS
```

## Phase 3 overview: v175-v184

### v175: rootfs/tmpfs persistent-in-memory backend

Marker:

```text
[ucompat-v175] rootfs tmpfs backend PASS
```

### v176: devfs null zero console tty random

Marker:

```text
[ucompat-v176] devfs core devices PASS
```

### v177: procfs process fd status stat maps

Marker:

```text
[ucompat-v177] procfs process status maps PASS
```

### v178: mount tree and statfs/fstatfs

Marker:

```text
[ucompat-v178] mount tree statfs PASS
```

### v179: path permission checks and credentials

Marker:

```text
[ucompat-v179] permissions credentials PASS
```

### v180: capability and setuid/setgid model

Marker:

```text
[ucompat-v180] capability identity model PASS
```

### v181: socket AF_UNIX and loopback deepening

Marker:

```text
[ucompat-v181] unix socket loopback PASS
```

### v182: UDP-like local datagram model

Marker:

```text
[ucompat-v182] local datagram socket PASS
```

### v183: IPC blocking and scheduler integration

Marker:

```text
[ucompat-v183] ipc blocking scheduler integration PASS
```

### v184: namespace basics

Marker:

```text
[ucompat-v184] namespace basics PASS
```

## Phase 4 overview: v185-v190

### v185: multi-ELF rootfs runner

Marker:

```text
[ucompat-v185] multi elf rootfs runner PASS
```

### v186: libc compatibility syscall matrix

Marker:

```text
[ucompat-v186] libc syscall matrix PASS
```

### v187: filesystem/process/memory test suite

Marker:

```text
[ucompat-v187] fs process memory suite PASS
```

### v188: signal/pipe/poll/ipc test suite

Marker:

```text
[ucompat-v188] signal pipe poll ipc suite PASS
```

### v189: stress, error-path, and regression hardening

Marker:

```text
[ucompat-v189] stress error path hardening PASS
```

### v190: final competition readiness report

Marker:

```text
[ucompat-v190] final competition kernel readiness PASS
```

## Why the plan still runs one part at a time

Every version must:

1. preserve all old markers;
2. deepen exactly one subsystem group;
3. add one new marker;
4. produce build/QEMU evidence;
5. generate a non-truncated report;
6. produce `apply_fix.sh` and `apply_fix.bat`.

This is the safest way to reach a complete kernel without losing the stable v157 base.
