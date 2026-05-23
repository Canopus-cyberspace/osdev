# Iteration 01 AI Usage

AI assistance was used to inspect the LoongArch module boundaries, reason about remaining basic-musl blockers, implement scoped LoongArch-local syscall changes, and summarize validation evidence.

Accepted suggestions:

- Keep new case sequencing in `basic_runner.rs`.
- Keep syscall dispatch in `syscall.rs`.
- Add process lifecycle scaffolding in a focused `process.rs`.
- Enable only stable real PLV3 cases after local QEMU validation.

Rejected or deferred suggestions:

- Enabling fork-dependent cases without a safe trap-stack model.
- Faking case output or printing parser-shaped case success from the kernel.
- Broad rewrites of the RISC-V syscall runtime dispatcher.

Human-verifiable checks:

- Local build and ELF headers.
- Local LoongArch QEMU output showing real user program START/END lines.
- Official runner failure captured as infrastructure, not a kernel result.

