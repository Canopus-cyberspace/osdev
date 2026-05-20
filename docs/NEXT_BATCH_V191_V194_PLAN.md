# v191-v194 Plan: Real User-mode Execution

## Baseline

v190.

## Purpose

v190 validates canonical runtime readiness. v191-v194 should prove that rootfs ELF programs can actually enter U-mode, run syscalls, exit, and be accounted for.

## v191: real rootfs ELF execution bridge

Required:

- executable is stored in canonical rootfs;
- execve resolves it through VFS;
- ELF PT_LOAD bytes are mapped into user memory;
- sepc/sp are set;
- program runs in U-mode and performs write/getpid/exit.

Marker:

```text
[ucompat-v191] real rootfs elf execution bridge PASS
```

## v192: real multi-program U-mode matrix

Suggested programs:

```text
/bin/hello
/bin/exit7
/bin/fsprobe
/bin/procprobe
/bin/memprobe
```

Output should include per-program evidence:

```text
[userland-v192] /bin/hello PASS exit=0
[userland-v192] /bin/exit7 PASS exit=7
```

Marker:

```text
[ucompat-v192] real multi program umode matrix PASS
```

## v193: fork/exec/wait real path

Required:

- parent creates child;
- child execs rootfs ELF;
- child exits;
- parent wait4/waitid collects status.

Marker:

```text
[ucompat-v193] fork exec wait real path PASS
```

## v194: userland ABI hardening

Required:

- argc/argv visible to program;
- envp visible or safely empty;
- auxv basics;
- CLOEXEC verified across exec;
- invalid exec path and invalid ELF errors verified.

Marker:

```text
[ucompat-v194] userland abi hardening PASS
```

## Out of scope

No full dynamic linker, full libc, full scheduler, Ext4, virtio-blk, LoongArch64, GUI, or dynamic kernel modules.

## Final output

```text
[PASS] real usermode execution batch v191-v194 passed
[PASS] apply_fix.sh completed
[PASS] apply_fix.bat completed
```
