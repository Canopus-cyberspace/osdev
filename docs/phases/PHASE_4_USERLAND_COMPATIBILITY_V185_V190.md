# Phase 4: Userland Compatibility and Final Hardening, v185-v190

## Purpose

Phase 4 moves the kernel from internal smoke scenarios to real multi-program compatibility evidence.

## v185: multi-ELF rootfs runner

Goal:

- Load and run multiple user ELF programs from rootfs.

Implementation:

- include or build several user programs.
- store them in rootfs/tmpfs image.
- run each by execve.
- collect exit code.
- report pass/fail per program.

Marker:

```text
[ucompat-v185] multi elf rootfs runner PASS
```

## v186: libc syscall matrix

Goal:

- Add libc-style syscall coverage.

Implementation:

- tests for open/read/write/stat/getcwd/chdir.
- tests for mmap/brk.
- tests for fork/wait/exec.
- tests for signal.
- tests for pipe/poll.
- tests for clock/time/random.
- produce matrix report.

Marker:

```text
[ucompat-v186] libc syscall matrix PASS
```

## v187: filesystem/process/memory suite

Goal:

- Stress major core kernel subsystems.

Implementation:

- create many files/directories.
- rename/link/unlink cycles.
- fork/wait tree.
- exec multiple programs.
- mmap/munmap/mprotect checks.
- procfs verification.

Marker:

```text
[ucompat-v187] fs process memory suite PASS
```

## v188: signal/pipe/poll/ipc suite

Goal:

- Validate asynchronous behavior.

Implementation:

- signal handler delivery.
- rt_sigreturn.
- SIGCHLD on child exit.
- pipe poll readiness.
- epoll multi-fd readiness.
- mq/msg/sem/shm basic behavior.
- futex wake behavior.

Marker:

```text
[ucompat-v188] signal pipe poll ipc suite PASS
```

## v189: stress, error-path, and regression hardening

Goal:

- Harden invalid inputs and resource exhaustion.

Implementation:

- invalid user pointers.
- bad fd.
- ENOENT/EEXIST/ENOTDIR/EISDIR/EINVAL/EBADF/EFAULT/EAGAIN.
- fd exhaustion.
- inode/object exhaustion.
- repeated open/close.
- repeated fork/wait if supported.
- no forbidden warnings.
- all old markers preserved.

Marker:

```text
[ucompat-v189] stress error path hardening PASS
```

## v190: final competition readiness report

Goal:

- Freeze the implementation and produce final evidence.

Implementation:

- run all preserved markers.
- run all userland compatibility tests.
- generate full non-truncated report.
- list implemented syscalls.
- list simplified semantics.
- list known unsupported features.
- package scripts and logs.

Marker:

```text
[ucompat-v190] final competition kernel readiness PASS
```
