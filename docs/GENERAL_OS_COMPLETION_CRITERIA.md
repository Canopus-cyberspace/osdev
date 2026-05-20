# General OS / Linux-Compatible Subset Completion Criteria

A stronger general OS target should satisfy the following criteria.

## 1. Boot and architecture

- automatic boot under QEMU;
- stable trap/syscall path;
- user-mode execution;
- clean logs and reproducible build.

## 2. Process and execution

- init process;
- fork/clone;
- execve from filesystem;
- wait/waitid;
- exit/exit_group;
- signals;
- per-task fd/cwd/root/credentials/mm.

## 3. Memory

- physical page allocator;
- page table mapping;
- real page fault handler;
- lazy mmap/brk;
- munmap/mprotect;
- fork memory copy or COW;
- usercopy validation.

## 4. Filesystems

- rootfs/tmpfs/devfs/procfs;
- mount tree;
- block device;
- filesystem image support;
- stat/getdents/open/read/write/link/rename/unlink;
- statfs/fstatfs.

## 5. Scheduler and blocking

- run queue;
- context switch;
- timer tick;
- sleep queue;
- futex;
- pipe/poll/epoll blocking;
- wait/nanosleep/timerfd blocking.

## 6. Devices

- console read/write;
- tty basics;
- devfs nodes;
- random/urandom;
- clocksource/RTC;
- block device;
- optional network and framebuffer.

## 7. Networking

- AF_UNIX;
- loopback;
- UDP local;
- TCP subset if required;
- socket readiness and poll integration.

## 8. Security and namespaces

- uid/gid;
- capabilities;
- file permissions;
- chroot/pivot_root;
- mount/pid/ipc namespaces;
- seccomp subset optional.

## 9. Userland

- libc ABI or syscall wrapper layer;
- crt0;
- init;
- shell/BusyBox-like commands;
- rootfs layout;
- multi-program test matrix.

## 10. Submission and reproducibility

- one-command build;
- one-command QEMU;
- syscall support matrix;
- known limitations;
- performance report;
- final technical report;
- clean package.
