# Iteration 16 Development Log

## Starting Point

The iteration began from the score-260 BusyBox baseline. Iteration 15 diagnostics showed that `basename /aaa/bbb`, `printf "abc\n"`, and `uname` could exit cleanly in an isolated non-scoring diagnostic mode, while `ash -c exit` faulted.

The required before-change state preservation was run:

```text
git status --short
git diff --stat
git diff > /tmp/uestc_kernel_before_change.diff
```

The repository was clean at the start of the iteration.

## Feature Ownership

Feature added: direct BusyBox applet promotion.

Subsystem owner: `src/arch/loongarch64/busybox_runner.rs`.

Reason: this file already owns the BusyBox command descriptors, scoring/smoke/disabled classes, real `/musl/busybox` execution loop, expected exit matching, and official testcase line emission.

No new focused source file was needed because no new subsystem behavior was introduced. This iteration did not touch syscall compatibility, fd behavior, scratch files, redirection, pipelines, shell-complex command support, or `runtime_dispatch.rs`.

## Implementation

The primary commands were added to `RUN_COMMANDS` as scoring entries:

- `basename /aaa/bbb`
- `printf "abc\n"`
- `uname`

After local bounded execution passed for the primary set, a secondary direct-only batch was tested. The safe commands kept as scoring entries were:

- `dirname /aaa/bbb`
- `expr 1 + 1`
- `date`
- `uptime`
- `clear`
- `cal`

The trial batch found three unsafe or non-matching commands:

- `which ls`: command returned exit code 1, so it was not promoted.
- `free`: command returned exit code 1 because `/proc/meminfo` is unavailable, so it was not promoted.
- `sleep 1`: command faulted at user PC `0x12016f814`, so it was not promoted.

These three commands were moved into `DISABLED_COMMANDS` with `ash-exit` so the runner remains explicit about known disabled direct applets.

## Safety Notes

The promoted commands all run the real `/musl/busybox` ELF in PLV3, return through the normal trap/process path, and emit official testcase success lines only after matching the expected exit code.

No fake output or hard-coded success path was added. The only parser-shaped lines added are emitted by the existing `emit_official_result` path after real command completion.

`ash -c exit` remains disabled.

## Validation Decisions

The first secondary probe intentionally included more candidates than would be shipped. It timed out after `sleep 1` faulted and destabilized the later `ls` continuation. The final shipped set removes the failing `which`, `free`, and `sleep` candidates and was rerun from a fresh image.

The final local smoke completed:

```text
[loongarch64-basic] attempted=32 completed=32 failed=none
[loongarch64-busybox] smoke completed=16 attempted=16 matched=16 failed=0 disabled=4
```

Official validation completed with score 269 and `busybox-musl-la=14.0`.

