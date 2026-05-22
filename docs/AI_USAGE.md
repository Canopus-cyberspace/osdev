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

## Iteration 08

AI was used to inspect the LoongArch fixed-address BusyBox blocker, implement the focused `user_mmu.rs` CSR/TLB helper module, wire fixed ET_EXEC mappings through `real_elf.rs`, validate the non-scoring BusyBox `true` probe, and document the evidence.

Accepted changes let the real `/musl/busybox true` enter at `0x1201b640c` and exit with code 0 through the LoongArch trap path. No official BusyBox group marker, fake BusyBox output, hard-coded success text, or `runtime_dispatch.rs` rewrite was added.

## Iteration 09

AI was used to inspect real BusyBox command traces, add a bounded BusyBox smoke runner, implement narrow syscall compatibility, preserve the 32-case basic-musl regression line, attempt official validation, and document the iteration.

Accepted changes expanded real non-scoring BusyBox coverage to seven commands. No official BusyBox group marker, fake BusyBox output, hard-coded success text, or `runtime_dispatch.rs` rewrite was added.

## Iteration 10

AI was used to inspect the existing BusyBox runner, official BusyBox judge format, group marker parsing, group-active state, diagnostic suppression paths, local QEMU output, official Docker output, and iteration documentation.

Accepted changes enabled official BusyBox group markers only around the safe real BusyBox command set and emitted official testcase lines only for real command outcomes that match official judge entries. No fake BusyBox output, hard-coded command success, unsafe `uname`/`ash` enablement, or `runtime_dispatch.rs` rewrite was added.

## Iteration 11

AI was used to inspect the official basic-musl score table, identify `test_waitpid` as the only LoongArch scoring miss, compare RISC-V and LoongArch serial output, fix LoongArch wait status encoding, diagnose quiet BusyBox timing sensitivity, validate locally and officially, and document the iteration.

Accepted changes were limited to real wait status semantics and LoongArch user-entry synchronization. No fake `waitpid` output, hard-coded testcase success, marker manipulation, unsafe BusyBox command enablement, or `runtime_dispatch.rs` rewrite was added.

## Iteration 12

AI was used to inspect the interrupted BusyBox expansion patch, reduce it to a focused allowlist stabilization in `busybox_runner.rs`, run local and official validation, and document the checkpoint.

Accepted changes only classify real BusyBox commands as scoring, smoke, or disabled. No fake BusyBox output, hard-coded command success, broad syscall rewrite, official script change, or `runtime_dispatch.rs` change was added.

## Iteration 13

AI was used to inspect the LoongArch BusyBox runner, fd table, syscall compatibility paths, user/process state, and official BusyBox judge format; attempt a scratch-file expansion; detect local instability; apply the fallback policy; validate the restored baseline; and document the iteration.

No scratch-FS source change was accepted because the attempt destabilized known-good command execution. No fake BusyBox output, hard-coded command success, official script change, or `runtime_dispatch.rs` change was added.

## Iteration 14

AI was used to inspect the LoongArch BusyBox runner, fd table, syscall compatibility paths, user/process state, MMU synchronization path, and official BusyBox judge format; attempt a gated Virtual Scratch FS expansion; test a reduced `printf` probe; detect local stalls; apply the fallback policy; validate the restored baseline; and document the iteration.

No source change was accepted because the attempts still risked known-good BusyBox stability. No fake BusyBox output, hard-coded command success, official script change, or `runtime_dispatch.rs` change was added.

## Iteration 15

AI was used to inspect the LoongArch BusyBox runner, user run-state ownership, trap observation path, Makefile LoongArch build flow, local QEMU diagnostic logs, and official Docker logs.

Accepted changes add only cfg-gated non-scoring diagnostics for `basename /aaa/bbb`, `printf "abc\n"`, `uname`, and `ash -c exit`. The normal BusyBox scoring path remains limited to the five known-good commands, and the diagnostic mode is disabled unless `--cfg loongarch64_busybox_diag` is explicitly passed.

No fake BusyBox output, hard-coded command success, scratch-FS reintroduction, official script change, or `runtime_dispatch.rs` change was added.

## Iteration 16

AI was used to inspect the LoongArch BusyBox command table, public `judge_busybox-musl.py` command keys, local direct-applet probe output, and official Docker score output.

Accepted changes only update direct BusyBox command descriptors in `busybox_runner.rs`. No syscall compatibility, fd-table, scratch-FS, redirection, pipeline, official script, or `runtime_dispatch.rs` change was added.

No fake BusyBox output or hard-coded command success was accepted. Commands are scored only through the existing real-execution result path after expected exit-status matching.
