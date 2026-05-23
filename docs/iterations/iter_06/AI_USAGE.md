# Iteration 06 AI Usage

AI assistance was used to inspect existing LoongArch clone, mmap, munmap, ext4, and BusyBox loader paths; identify the sparse-file blocker for `/musl/basic/clone`; implement the focused ext4 regular-file hole handling; enable the real clone case; and summarize validation evidence.

Accepted AI changes:

- `/musl/basic/clone` enablement in `basic_runner.rs`.
- Regular-file sparse extent zero-fill in `sdcard_ext4.rs`.
- Iteration documentation and concise validation excerpts.

Rejected or deferred suggestions:

- BusyBox group output was not emitted.
- BusyBox command success text was not hard-coded.
- A BusyBox runner was deferred until the LoongArch loader has safe support for the 2.0 MiB fixed-address ET_EXEC payload.
- No `runtime_dispatch.rs` change was made.

Human/environment verification is still required for an updated official score because Docker was unavailable locally.
