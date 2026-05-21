# Iteration 15 Summary

## Goal

Add narrow non-scoring LoongArch BusyBox diagnostics for commands that have shown user-mode stall or fault risk, while preserving the official score-260 baseline and the existing five BusyBox scoring commands.

Diagnostic target commands:

- `basename /aaa/bbb`
- `printf "abc\n"`
- `uname`
- `ash -c exit`

No new BusyBox command was promoted to official scoring in this iteration.

## Outcome

Added a build-time diagnostic mode guarded by `--cfg loongarch64_busybox_diag`. Normal `make all` and official scoring builds do not enable this cfg.

The normal BusyBox scoring and smoke path remains unchanged:

- Scoring: `true`, `false`, `pwd`, `sh -c exit`, `ls`
- Smoke only: `echo hello`, `cat /musl/busybox_cmd.txt`
- Disabled in official mode: `basename /aaa/bbb`, `uname`, `ash -c exit`

The diagnostic build runs only the LoongArch basic phase and then one diagnostic BusyBox command at a time. It prints concise summary lines with entry PC, last observed ERA/BADV/ECODE, syscall count, trap count, same-ERA count, timer ticks, and outcome. It does not emit official BusyBox testcase success or fail lines.

## Diagnostic Results

Local diagnostic QEMU completed and preserved LoongArch basic 32/32:

```text
[loongarch64-basic] attempted=32 completed=32 failed=none
```

Diagnostic probe summary:

```text
diag-basename: exited, exit_code=0, entry=0x1201b640c, last_era=0x1201aa0bc, last_badv=0x1201aa0bc, last_ecode=11, syscalls=4, traps=4, stopped_before_syscall=no
diag-printf: exited, exit_code=0, entry=0x1201b640c, last_era=0x1201aa0bc, last_badv=0x1201aa0bc, last_ecode=11, syscalls=6, traps=6, stopped_before_syscall=no
diag-uname: exited, exit_code=0, entry=0x1201b640c, last_era=0x1201aa0bc, last_badv=0x1201aa0bc, last_ecode=11, syscalls=6, traps=6, stopped_before_syscall=no
diag-ash-exit: fault, entry=0x1201b640c, last_era=0x1201b64b8, last_badv=0x1201b64b8, last_ecode=15, syscalls=7, traps=8, stopped_before_syscall=no
```

The diagnostic timer path was added but did not fire for these probes because each command exited or faulted before the tick budget expired.

## Module Placement

- `busybox_runner.rs` owns the diagnostic command table, non-scoring diagnostic runner, per-command setup, and concise summary output. This matches its existing ownership of BusyBox command policy and result reporting.
- `user.rs` owns diagnostic active state, trap/syscall counters, last observed user PC/fault state, and diagnostic snapshots. This matches its existing ownership of per-command run state.
- `trap.rs` owns timer interrupt setup/teardown and trap observation hooks because it already owns EENTRY setup, trap classification, and PLV3 return mechanics.
- `kernel.rs` owns only top-level runner selection under the diagnostic cfg, preserving its boot-orchestration role.
- `Makefile` owns the `LOONGARCH_RUSTFLAGS` hook needed to build a separate diagnostic kernel without changing normal builds.

A new `busybox_diag.rs` file was considered, but the code stayed in `busybox_runner.rs` because the new responsibility is a guarded submode of the existing BusyBox runner rather than a reusable subsystem. A new timer module was also considered, but the diagnostic timer CSR access is tightly coupled to the trap path and is cfg-only.

## Validation

Normal local validation passed:

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

Official validation completed on the final run:

```text
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260521_235948/docker_evaluate.log
docker_evaluate.log size: 798847 bytes
Verdict: Accpted
Score: 260
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 102.0
busybox-musl-la: 5.0
```

The first official attempt in this iteration completed with a transient LoongArch virtio timeout/status failure after official image unpacking. It was not accepted as validation evidence. After removing generated root sdcard artifacts and rerunning with the normal kernel, official validation returned to the score-260 baseline.

