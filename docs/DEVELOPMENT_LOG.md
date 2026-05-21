# Development Log

## Iteration 01

Expanded the LoongArch basic-musl runner with stable `mount` and `umount` execution. Added process syscall scaffolding but deferred fork-dependent cases after local testing showed the current LoongArch trap handler stack model is not safe for full fork continuation.

## Iteration 02

Added LoongArch trap-stack groundwork. `__loongarch64_trap_entry` now switches from the incoming PLV3 user stack to a dedicated kernel-owned trap stack before building the trap frame and entering Rust trap handling. The original user stack pointer is still saved in the trap frame and restored on PLV3 return.

This keeps trap/vector mechanics in `src/arch/loongarch64/trap.rs`. The only memory-side companion change is in `src/arch/loongarch64/real_elf.rs`, where full user stack snapshot restoration is safe again because trap handling no longer uses the user stack.
