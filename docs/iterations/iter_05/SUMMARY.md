# Iteration 05 Summary

## Focus

This iteration implemented LoongArch-local pipe ownership semantics and enabled the real `/musl/basic/pipe` case.

## Source Changes

- Added a focused `PipeState` object in `src/arch/loongarch64/fd_table.rs`.
- Replaced the old single global pipe positions with a small `PIPES` table.
- Added read/write endpoint reference counts.
- Made `close`, `dup`, `dup2`/`dup3`, and fork snapshot/restore update pipe endpoint ownership.
- Kept pipe syscall marshaling in `src/arch/loongarch64/syscall.rs` unchanged except for reusing the improved fd table behavior.
- Enabled `/musl/basic/pipe` in `src/arch/loongarch64/basic_runner.rs`.

## Non-Goals

- `clone` and BusyBox were not enabled.
- `runtime_dispatch.rs` was not changed.
- No output was faked or hard-coded.

## Result

Local LoongArch smoke completed all 31 enabled real PLV3 ELF cases:

```text
========== START test_pipe ==========
cpid: 0
cpid: 2
  Write to pipe successfully.
========== END test_pipe ==========
[loongarch64-basic] attempted=31 completed=31 failed=none
```

Official validation could not be refreshed because Docker was unavailable before kernel evaluation.
