# Iteration 11 AI Usage

AI assistance was used to inspect the official score-256 log, parse the basic-musl per-case table, compare RISC-V and LoongArch `waitpid` serial output, inspect the LoongArch process/wait implementation, implement the wait status semantic fix, diagnose BusyBox timing sensitivity after the fix, run validation, and document the iteration.

Accepted AI changes:

- Mask child exit code to the low 8 bits before recording waitable child state.
- Encode normal wait status as `(exit_code & 0xff) << 8`.
- Reuse `user_mem::copy_to_user` for wait status writes.
- Add a focused LoongArch `sync_user_entry` barrier loop in `user_mmu.rs`.
- Call the user-entry synchronization after fixed-address user mappings are installed and before PLV3 trap return.

Rejected or deferred suggestions:

- No fake `waitpid` output was added.
- No official marker changes were made.
- No hard-coded testcase success was added.
- No BusyBox unsafe command was enabled.
- `runtime_dispatch.rs` was not changed.

Human/environment verification was performed through local QEMU smoke and a completed official Docker evaluation.
