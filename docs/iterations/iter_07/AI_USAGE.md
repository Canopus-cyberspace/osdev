# Iteration 07 AI Usage

AI assistance was used to inspect the existing LoongArch basic runner, ELF loader, user-copy helpers, syscall dispatch, and BusyBox blocker; implement the multi-region user memory upgrade; add the focused BusyBox loader probe; run local validation; and summarize the evidence.

Accepted AI changes:

- Multi-region LoongArch user memory tracking in `real_elf.rs`.
- Larger real-ELF loading capacity for `/musl/busybox`.
- Region-translating user-copy helpers.
- Multi-slot mmap tracking.
- Non-scoring BusyBox loader/entry probe in `busybox_runner.rs`.
- Minimal syscall compatibility hooks for early BusyBox probing.
- User fault badv reporting.
- Iteration documentation and concise validation excerpts.

Rejected or deferred suggestions:

- BusyBox official group output was not emitted.
- BusyBox command success text was not hard-coded.
- A broad `runtime_dispatch.rs` rewrite was not made.
- A larger user-memory module split was deferred until real virtual-address mapping is implemented.

Human/environment verification is still required for an updated official score because Docker was unavailable locally.
