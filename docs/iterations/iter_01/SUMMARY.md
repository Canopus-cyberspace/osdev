# Iteration 01 Summary

## Goal

Aggressively expand LoongArch `basic-musl-la` while preserving the existing RISC-V path and avoiding fake official output.

## Result

- Added real LoongArch `mount` and `umount` basic-musl case execution.
- Local LoongArch smoke now completes 24 real PLV3 `/musl/basic/*` ELFs from `sdcard-la.img`.
- Preserved the existing stable mmap/munmap work.
- Added LoongArch-local process syscall scaffolding for clone/wait/execve, but did not enable fork-dependent ELFs because the current LoongArch trap path runs Rust on the user stack.

## Enabled Cases

- Newly enabled: `mount`, `umount`.
- Preserved: `write`, `getpid`, `getppid`, `uname`, `gettimeofday`, `times`, `sleep`, `getcwd`, `brk`, `mmap`, `munmap`, `close`, `dup`, `dup2`, `open`, `read`, `openat`, `fstat`, `getdents`, `chdir`, `mkdir`, `unlink`.

## Still Blocked

`exit`, `fork`, `clone`, `execve`, `wait`, `waitpid`, `yield`, and `pipe` need a LoongArch trap-stack/scheduler-safe continuation model before their real ELFs can be enabled without corrupting the active trap handler stack.

