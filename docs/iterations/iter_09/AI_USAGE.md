# Iteration 09 AI Usage

AI assistance was used to inspect BusyBox smoke ownership, identify missing compatibility syscalls from local QEMU output, implement narrow LoongArch-local syscall support, run local validation, attempt official validation, and document results.

Accepted AI changes:

- Expanded `BusyboxCommand` coverage in `busybox_runner.rs`.
- Added per-command syscall-budget state in `user.rs`.
- Added narrow `writev`, `readv`, `fcntl`, `sendfile`, and signal compatibility in `syscall.rs`.
- Made `getdents64` path-aware enough for `busybox ls` and added EOF behavior.
- Iteration documentation and concise validation excerpts.

Rejected or deferred suggestions:

- Official BusyBox group markers were not emitted.
- BusyBox command success text was not hard-coded.
- `busybox uname` was not kept enabled after a no-syscall hang.
- `busybox ash -c exit` was not kept enabled after a user fault.
- A broad `runtime_dispatch.rs` rewrite was not made.

Human/environment verification is still required for an updated official score because the official wrapper timed out before producing a Docker evaluation log.
