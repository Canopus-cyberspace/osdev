# Phase 2: Process, Exec, Memory, Signal, v165-v174

## Purpose

Phase 2 moves the kernel from one external init smoke path toward real multi-process execution.

## v165: task table and process lifecycle

Goal:

- Add canonical task table.
- Represent task id, parent id, thread group id, process group id, session id.
- Store task state and exit code.
- procfs reads this table.

Marker:

```text
[ucompat-v165] task table process lifecycle PASS
```

## v166: fork/clone child creation

Goal:

- Implement simplified fork/clone task creation.

Implementation:

- allocate child task id.
- copy or share fdtable according to simplified clone flags.
- copy cwd/root.
- child gets return value 0.
- parent gets child pid.
- clone flags unsupported by this kernel return `-EINVAL` or `-ENOSYS` consistently.

Marker:

```text
[ucompat-v166] fork clone child task PASS
```

## v167: exit, zombie, wait4, waitid

Goal:

- Parent can wait for child exit.

Implementation:

- exit marks task as zombie.
- exit_group marks thread group exited.
- wait4 finds child zombie.
- wait4 writes status.
- waitid writes siginfo-like status if already supported.
- reaping removes zombie from task table.

Marker:

```text
[ucompat-v167] exit zombie wait lifecycle PASS
```

## v168: per-task runtime snapshot

Goal:

- Move fdtable/cwd/root/signal mask into per-task state.

Implementation:

- current task owns or references fdtable.
- cwd/root are per-task.
- signal mask is per-task.
- procfs fd/status reads selected task, not global singleton only.

Marker:

```text
[ucompat-v168] per task runtime snapshot PASS
```

## v169: execve from canonical VFS

Goal:

- Load ELF from VFS path.

Implementation:

- `execve(path, argv, envp)` resolves path through canonical VFS.
- reads file bytes from inode data.
- validates ELF header.
- parses PT_LOAD.
- creates new user address space or replaces image in simplified model.
- returns Linux-like errors for ENOENT/EACCES/ENOEXEC/EFAULT.

Marker:

```text
[ucompat-v169] execve from canonical vfs PASS
```

## v170: argv/envp/auxv and CLOEXEC

Goal:

- Complete user stack and fd cleanup for execve.

Implementation:

- copy argv strings.
- copy envp strings.
- construct argc/argv/envp/auxv layout.
- enforce alignment.
- close fds marked FD_CLOEXEC.
- preserve non-CLOEXEC fds.

Marker:

```text
[ucompat-v170] execve user stack cloexec PASS
```

## v171: VMA and page fault foundation

Goal:

- Add real VMA tracking.

Implementation:

- VMA list/tree per address space.
- track start/end/prot/flags/backing.
- page fault validates address against VMA.
- allocate page on demand for anonymous writable mapping.
- invalid fault kills task or returns error in smoke path.

Marker:

```text
[ucompat-v171] vma page fault foundation PASS
```

## v172: lazy mmap/brk/munmap/mprotect

Goal:

- Deepen memory syscall behavior.

Implementation:

- brk expands heap VMA.
- mmap creates VMA.
- munmap removes or splits VMA.
- mprotect changes VMA permissions.
- lazy allocation occurs on first access.
- usercopy checks mapping/protection.

Marker:

```text
[ucompat-v172] lazy mmap brk munmap mprotect PASS
```

## v173: signal frame and rt_sigreturn

Goal:

- Deliver signal to user mode.

Implementation:

- store sigaction.
- store blocked mask.
- pending signal queue.
- build signal frame on user stack.
- redirect sepc to handler.
- rt_sigreturn restores saved trap context and mask.

Marker:

```text
[ucompat-v173] signal frame rt_sigreturn PASS
```

## v174: SIGCHLD and process-group signal basics

Goal:

- Connect signals to process lifecycle.

Implementation:

- child exit queues SIGCHLD to parent.
- kill sends to process.
- tgkill sends to task.
- process group signals are simplified but coherent.
- wait consumes child status.

Marker:

```text
[ucompat-v174] sigchld process group signal PASS
```
