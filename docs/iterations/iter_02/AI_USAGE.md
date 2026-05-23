# Iteration 02 AI Usage

AI assistance was used to inspect the LoongArch trap entry path, identify that Rust trap handling ran on the PLV3 user stack, implement the stack switch in `trap.rs`, and summarize validation results.

Accepted AI changes:

- Dedicated LoongArch trap stack in trap assembly.
- Preservation of the incoming user stack pointer and temporary register through scratch CSRs.
- Full user stack snapshot restore in `real_elf.rs` after the trap stack made it safe.
- Iteration documentation and concise validation excerpts.

Rejected or deferred suggestions:

- Per-task kernel stacks were deferred until fork/clone scheduling introduces multiple active LoongArch tasks.
- Fork/clone/execve/wait real ELF cases were not enabled in this iteration.
- No official output or test-case success output was synthesized.

Human verification remains required for official scoring because Docker was unavailable during this iteration.
