# Development Log

## Iteration 01

Expanded the LoongArch basic-musl runner with stable `mount` and `umount` execution. Added process syscall scaffolding but deferred fork-dependent cases after local testing showed the current LoongArch trap handler stack model is not safe for full fork continuation.

## Iteration 02

Added LoongArch trap-stack groundwork. `__loongarch64_trap_entry` now switches from the incoming PLV3 user stack to a dedicated kernel-owned trap stack before building the trap frame and entering Rust trap handling. The original user stack pointer is still saved in the trap frame and restored on PLV3 return.

This keeps trap/vector mechanics in `src/arch/loongarch64/trap.rs`. The only memory-side companion change is in `src/arch/loongarch64/real_elf.rs`, where full user stack snapshot restoration is safe again because trap handling no longer uses the user stack.

## Iteration 03

Enabled the LoongArch `exit`, `wait`, `waitpid`, `yield`, and `fork` basic-musl cases as real PLV3 ELFs. Initial testing exposed a nested trap-stack overwrite when a child trapped while the parent was still inside `sys_clone`.

Fixed that in `src/arch/loongarch64/trap.rs` by adding a `loongarch64_trap_stack_cursor` and reserving a separate 16 KiB stack slice per active trap. The existing process lifecycle code in `src/arch/loongarch64/process.rs` was reused for child execution, child exit recording, and `wait4`.
