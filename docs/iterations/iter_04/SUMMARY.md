# Iteration 04 Summary

## Focus

This iteration implemented and enabled the real LoongArch basic-musl `execve` case.

## Source Changes

- Enabled `/musl/basic/execve` in `src/arch/loongarch64/basic_runner.rs`.
- Implemented safe LoongArch `execve(path, argv, envp)` decoding in `src/arch/loongarch64/syscall.rs`.
- Added `read_user_usize` to `src/arch/loongarch64/user_mem.rs` for safe argv/envp pointer-array reads.
- Extended `src/arch/loongarch64/real_elf.rs` so the ELF stack can be rebuilt with copied argv/envp strings.
- Added an execve-specific user image snapshot so failed execve restores the old image without clobbering the fork parent snapshot.
- Updated `src/arch/loongarch64/process.rs` to replace the current image on successful execve while preserving pid, ppid, fd table, and cwd.

## Non-Goals

- `pipe`, `clone`, and further BusyBox work were not enabled.
- `runtime_dispatch.rs` was not changed.
- No case output was faked or hard-coded.

## Result

Local LoongArch smoke completed all 30 enabled real PLV3 ELF cases:

```text
========== START test_execve ==========
  I am test_echo.
execve success.
========== END main ==========
[loongarch64-basic] attempted=30 completed=30 failed=none
```

Official validation could not be refreshed because Docker was unavailable before kernel evaluation.
