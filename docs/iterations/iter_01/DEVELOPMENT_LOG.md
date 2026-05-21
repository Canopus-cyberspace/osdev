# Iteration 01 Development Log

## Discovery

Inspected the current LoongArch modules and extracted the remaining real `/musl/basic/*` ELFs from the official LoongArch sdcard image. `mount` and `umount` do not require process scheduling. `yield`, `pipe`, `exit`, `fork`, `clone`, `wait`, and `waitpid` all depend on fork/wait style process continuation.

## Implementation

- Added `src/arch/loongarch64/process.rs` for LoongArch-local process state scaffolding.
- Added snapshot helpers in `real_elf.rs` and `fd_table.rs`.
- Added minimal syscall dispatch for `clone`, `wait4`, `execve`, `mount`, and `umount`.
- Enabled only the stable new real ELFs: `mount` and `umount`.

## Important Decision

Fork-dependent cases were tested locally and then disabled. The current LoongArch trap entry runs Rust on the user stack, so restoring a fork snapshot that includes the user stack can overwrite the active syscall handler stack. Skipping user-stack restore avoids that overwrite but can corrupt parent user execution. A dedicated trap/kernel stack is the correct next step before enabling these cases.

## Files

- `basic_runner.rs`: case list changes.
- `syscall.rs`: syscall dispatch and compatibility semantics.
- `process.rs`: process lifecycle scaffolding.
- `fd_table.rs`: fd snapshot hooks.
- `real_elf.rs`: user memory snapshot hooks.
- `trap.rs`: trapframe copy support and full-frame PLV3 entry helper.
- `user.rs`: per-case process reset hook.

