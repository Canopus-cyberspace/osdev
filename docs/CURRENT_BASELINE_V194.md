# Current Baseline: v194

## Stable baseline

Current stable baseline: **v194**.

## What is confirmed

v191-v194 achieved:

- rootfs writes small RISC-V ELF files;
- execve_from_vfs parses ELF/PT_LOAD;
- ELF bytes are copied into user pages;
- user stack is constructed;
- trap context is updated;
- `sret` enters U-mode;
- user ELF executes `write`, `getpid`, and `exit`;
- multiple independent user ELFs run and return expected exit codes;
- fork/exec/wait real path works;
- argc/argv/envp/auxv, AT_PAGESZ, CLOEXEC, ENOENT, and ENOEXEC are validated.

## Meaning

The kernel is beyond marker-only runtime validation.

It now has real rootfs ELF U-mode execution.

## Remaining major gaps toward general OS / Linux-compatible subset

- real physical page allocator and page-fault mapping;
- scheduler context switch and blocking;
- block devices and filesystem images;
- libc / BusyBox / shell compatibility;
- full POSIX signal ABI;
- stronger file permissions / namespaces / credentials;
- TCP/IP / virtio-net if needed;
- TTY/input stack;
- performance and submission hardening.
