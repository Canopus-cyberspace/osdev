# Iteration 17 Development Log

## Starting Point

The iteration began from the score-269 direct BusyBox applet baseline. The required before-change state preservation was run:

```text
git status --short
git diff --stat
git diff > /tmp/uestc_kernel_before_change.diff
```

The repository was clean at the start of the iteration.

## Feature Ownership

Feature added: direct/read-only BusyBox applet promotion.

Subsystem owner: `src/arch/loongarch64/busybox_runner.rs`.

Reason: this file already owns BusyBox command descriptors, scoring/smoke/disabled classes, real `/musl/busybox` execution, expected-exit matching, and official testcase result emission.

No new focused source file was needed because no new subsystem behavior was introduced. This iteration did not touch syscall compatibility, fd behavior, scratch files, redirection, pipelines, shell-complex command support, or `runtime_dispatch.rs`.

## Probing

The primary candidates were first probed as non-scoring smoke entries:

- `dmesg`
- `df`
- `du`
- `ps`
- `hwclock`

Local QEMU probe results:

```text
du: exit_code=0
dmesg: exit_code=1, klogctl not implemented
df: exit_code=1, /proc/mounts unavailable
ps: exit_code=1, /proc unavailable
hwclock: exit_code=1, /dev/misc/rtc unavailable
```

A narrow `syslog` compatibility shim was briefly tested for `dmesg`, but it was not shipped because the resulting command set showed stability risk. The final source tree has no `syscall.rs` change.

## Implementation

`du` was added to `RUN_COMMANDS` as a scoring entry:

```text
argv: busybox du
expected_exit: 0
official_name: du
class: Scoring
```

The failed primary candidates were added to `DISABLED_COMMANDS` so the BusyBox policy table documents the current blockers explicitly:

- `dmesg`
- `df`
- `ps`
- `hwclock`

## Safety Notes

`du` is a real BusyBox command loaded from `/musl/busybox`, run in LoongArch PLV3, and accepted only through the existing expected-exit result path.

No fake output or hard-coded success path was added. No parser-shaped line is printed unless the real command exits with its expected status.

## Validation Decisions

The final local LoongArch run completed:

```text
[loongarch64-basic] attempted=32 completed=32 failed=none
[loongarch64-busybox] smoke completed=17 attempted=17 matched=17 failed=0 disabled=8
testcase busybox du success
```

Official validation completed with score 270 and `busybox-musl-la=15.0`.

