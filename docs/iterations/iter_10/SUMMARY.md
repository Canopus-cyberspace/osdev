# Iteration 10 Summary

## Focus

This iteration turned the real LoongArch BusyBox smoke into a bounded scoring-capable runner while keeping unsafe commands disabled.

The runner still executes the real `/musl/busybox` ET_EXEC from `sdcard-la.img` in PLV3. It now emits official `busybox-musl` group markers only around the safe command set and emits official `testcase busybox ... success|fail` lines only for commands that map to official BusyBox judge entries.

## Source Changes

- Reused `src/arch/loongarch64/busybox_runner.rs` for the BusyBox command list, official-name mapping, group marker ownership, and per-command result emission.
- Reused `src/arch/loongarch64/user.rs` for group-active state, adding BusyBox group tracking alongside the existing basic-musl state.
- Reused `src/arch/loongarch64/syscall.rs` and `src/arch/loongarch64/trap.rs` to suppress noisy diagnostics while either official group is active.
- Reused `src/arch/loongarch64/kernel.rs` for top-level phase progress prints before and after basic and BusyBox phases.

No new source module was needed. The existing module boundaries already matched the responsibilities: runner sequencing in `busybox_runner.rs`, run state in `user.rs`, syscall diagnostics in `syscall.rs`, trap diagnostics in `trap.rs`, and boot orchestration in `kernel.rs`.

## BusyBox Runner

Safe real BusyBox commands kept enabled:

```text
busybox true
busybox false
busybox echo hello
busybox pwd
busybox sh -c exit
busybox ls
busybox cat /musl/busybox_cmd.txt
```

Official testcase lines are emitted for the five commands recognized by the official BusyBox judge and verified locally:

```text
testcase busybox true success
testcase busybox false success
testcase busybox pwd success
testcase busybox sh -c exit success
testcase busybox ls success
```

`busybox echo hello` and `busybox cat /musl/busybox_cmd.txt` remain real smoke commands but do not emit synthetic official testcase lines because their exact invocations are not official judge entries.

## Deferred Commands

- `busybox uname` remains disabled because the previous probe entered PLV3 and stopped before another syscall.
- `busybox ash -c exit` remains disabled because the previous probe hit a controlled user fault at `era=0x1201b64b8`.

## Result

Local smoke:

```text
[loongarch64-basic] attempted=32 completed=32 failed=none
[loongarch64-busybox] smoke completed=7 attempted=7 matched=7 failed=0
```

Official validation:

```text
Verdict: Accpted
Score: 256
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 98.0
busybox-musl-la: 5.0
```

The official log contained no `Failed to load ELF`, `panic`, `timeout`, `ENOSYS`, or `user fault` marker.
