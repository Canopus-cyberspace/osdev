# Iteration 13 Summary

## Goal

Attempt a bounded LoongArch BusyBox scoring expansion for file-operation commands while preserving the official score-260 baseline.

## Outcome

No source changes were shipped in this iteration.

An aggressive Virtual Scratch FS patch was implemented and tested locally, but it was reverted under the fallback policy because it destabilized the known-good BusyBox command set. The risky patch was preserved outside the repository for analysis:

```text
/tmp/iter13_risky_busybox_scratch.diff
```

The repository was restored to the stable Iteration 12 BusyBox allowlist:

- Scoring: `true`, `false`, `pwd`, `sh -c exit`, `ls`
- Smoke only: `echo hello`, `cat /musl/busybox_cmd.txt`
- Disabled: `basename /aaa/bbb`, `uname`, `ash -c exit`

No new BusyBox command was promoted to scoring.

## Module Placement

The attempted patch used the existing ownership boundaries:

- `busybox_runner.rs` for command classes, official marker policy, per-command execution, and result reporting.
- `fd_table.rs` for the attempted persistent Virtual Scratch FS, fd offsets, open flags, lseek, and virtual scratch file and directory state.
- `syscall.rs` for attempted syscall wrappers around lseek, rename, truncate, and related compatibility calls.
- `user.rs` for attempted per-BusyBox-command state reset hooks.

Those module locations were correct because the responsibilities matched the existing LoongArch architecture split. No new source file was needed for the attempted work, and no attempted source helper remains in the final tree.

## Validation

Local and official validation were rerun after the fallback cleanup.

```text
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
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260521_225549/docker_evaluate.log
Verdict: Accpted
Score: 260
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 102.0
busybox-musl-la: 5.0
```

