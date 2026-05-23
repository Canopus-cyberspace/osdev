# Iteration 03 Summary

## Focus

This iteration safely enabled the first LoongArch fork-dependent basic-musl cases after the trap path moved onto a kernel-owned stack.

## Source Changes

- Changed the LoongArch trap stack from a single fixed top into a small nested stack allocator in `src/arch/loongarch64/trap.rs`.
- Preserved each active trap frame's stack slice top in `LoongArchTrapFrame` so nested child syscalls restore the trap stack cursor correctly.
- Enabled real PLV3 `/musl/basic/*` cases in `src/arch/loongarch64/basic_runner.rs`:
  - `exit`
  - `wait`
  - `waitpid`
  - `yield`
  - `fork`

## Reused Ownership

- `process.rs` already owned clone/fork-style child execution, child exit recording, and `wait4` status writeback.
- `real_elf.rs` already owned user memory snapshots.
- `fd_table.rs` already owned fd/cwd snapshots.
- `user_mem.rs` continued to own safe user status writes through `copy_to_user`.
- `basic_runner.rs` continued to own the basic-musl case list and official group boundaries.

## Result

Local LoongArch smoke completed all 29 enabled real PLV3 ELF cases:

```text
[loongarch64-basic] attempted=29 completed=29 failed=none
```

Official validation could not be refreshed because Docker was unavailable before kernel evaluation.
