# AI Usage

AI was used for code inspection, implementation planning, LoongArch-local syscall changes, and validation summarization. All enabled user-visible basic-musl cases remain real PLV3 ELF executions loaded from the official sdcard image.

## Iteration 02

AI was used to inspect the LoongArch trap entry, implement a dedicated trap stack switch, preserve user register state across the new entry sequence, and document validation results.

No AI-suggested fake output, parser-shaped success text, or broad runtime dispatcher rewrite was accepted. Official score refresh still requires human or environment validation because Docker was unavailable.

## Iteration 03

AI was used to inspect the LoongArch fork/wait path, identify nested trap-stack corruption, implement the trap stack cursor, enable the five real fork-dependent basic-musl cases, and document validation results.

No fake case output, hard-coded success text, or `runtime_dispatch.rs` rewrite was added.

## Iteration 04

AI was used to inspect the LoongArch `execve` path, implement safe argv/envp copying, extend ELF stack construction, enable the real `/musl/basic/execve` case, and document validation results.

No fake `execve` output, hard-coded success text, or `runtime_dispatch.rs` rewrite was added.

## Iteration 05

AI was used to inspect LoongArch pipe/fd ownership, implement pipe endpoint refcounts, enable the real `/musl/basic/pipe` case, and document validation results.

No fake pipe output, hard-coded success text, or `runtime_dispatch.rs` rewrite was added.

## Iteration 06

AI was used to inspect the LoongArch clone, mmap, munmap, ext4, and BusyBox loader paths; identify the sparse ext4 file blocker for `/musl/basic/clone`; enable the real clone case; and document validation results.

No BusyBox output, hard-coded command success text, fake basic-musl output, or `runtime_dispatch.rs` rewrite was added.

## Iteration 07

AI was used to inspect and extend the LoongArch ELF loader, user-region tracking, user-copy helpers, BusyBox probe path, and early BusyBox syscall compatibility hooks.

Accepted changes loaded the real `/musl/busybox` payload and entered PLV3 for a non-scoring `busybox true` probe. No BusyBox output, official BusyBox group marker, hard-coded command success text, fake basic-musl output, or `runtime_dispatch.rs` rewrite was added.
