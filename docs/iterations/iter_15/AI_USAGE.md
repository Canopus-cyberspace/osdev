# Iteration 15 AI Usage

AI was used to inspect the LoongArch BusyBox runner, user run-state ownership, trap observation path, Makefile LoongArch build flow, local QEMU logs, and official Docker logs.

Accepted AI-assisted changes:

- Added a cfg-gated, non-scoring BusyBox diagnostic runner.
- Added diagnostic state and concise snapshots for last user PC, fault address, exception code, syscall count, trap count, same-ERA count, and timer ticks.
- Added a local-only `LOONGARCH_RUSTFLAGS` Makefile hook for building a separate diagnostic kernel.
- Added a cfg-gated diagnostic timer hook in the LoongArch trap path.
- Documented local and official validation evidence.

Rejected or avoided changes:

- No new BusyBox scoring command was promoted.
- No fake BusyBox output or hard-coded success line was added.
- No scratch-FS or redirection change was reintroduced.
- No official evaluation script was modified.
- `runtime_dispatch.rs` was not modified.

Human verification should focus on the final official score-260 log and the fact that diagnostic mode is disabled unless `--cfg loongarch64_busybox_diag` is explicitly passed to the LoongArch rustc invocation.

