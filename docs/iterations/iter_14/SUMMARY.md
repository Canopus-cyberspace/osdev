# Iteration 14 Summary

## Goal

Attempt an aggressive but gated LoongArch BusyBox file-operation scoring expansion while preserving the official score-260 baseline.

## Outcome

No source changes were shipped in this iteration.

Two BusyBox expansion patches were implemented and tested locally, then reverted under the fallback policy after they showed command-stability risk:

```text
/tmp/iter14_risky_busybox_scratch.diff
/tmp/iter14_printf_only.diff
```

The final repository preserves the known-good BusyBox classes:

- Scoring: `true`, `false`, `pwd`, `sh -c exit`, `ls`
- Smoke only: `echo hello`, `cat /musl/busybox_cmd.txt`
- Disabled: `basename /aaa/bbb`, `uname`, `ash -c exit`

No new BusyBox command was promoted to official scoring.

## Module Placement

The attempted implementation followed the established LoongArch module ownership:

- `busybox_runner.rs` for command classes, official markers, gated promotion, per-command budget, and result reporting.
- `fd_table.rs` for the attempted BusyBox-only Virtual Scratch FS, fd offsets, open flags, lseek, rename, directory state, and scratch file state.
- `syscall.rs` for attempted syscall wrappers and errno-compatible behavior.
- `user.rs` for BusyBox active state, command budget, timeout state, and command reset hooks.
- `real_elf.rs` and `user_mmu.rs` for fixed-address ELF loading, user VA mapping, user stack setup, mmap/brk, and TLB synchronization.
- `process.rs` for fork, clone, execve, wait, and exit behavior.

A new `scratch_fs.rs` file was considered for future work, but the attempted patch stayed in `fd_table.rs` because the responsibility was still tightly coupled to fd allocation, open flags, path resolution, offsets, and directory entries. No new source file was kept.

## Validation

The restored baseline was validated locally and officially.

```text
CPU_COUNT: 16
cargo build -j 16 --target riscv64gc-unknown-none-elf: passed
make -j 16 all: passed with the known local jobserver warning
make all: passed
kernel-rv: RISC-V ELF
kernel-la: LoongArch ELF
LoongArch local basic: attempted=32 completed=32 failed=none
LoongArch BusyBox smoke: completed=7 attempted=7 matched=7 failed=0 disabled=3
```

Official validation completed:

```text
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260521_233219/docker_evaluate.log
docker_evaluate.log size: 798848 bytes
Verdict: Accpted
Score: 260
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 102.0
busybox-musl-la: 5.0
```

