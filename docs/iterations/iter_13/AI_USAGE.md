# Iteration 13 AI Usage

AI was used to inspect the LoongArch BusyBox runner, fd table, syscall compatibility paths, user/process state, and the official BusyBox judge format; implement an aggressive scratch-file attempt; detect local instability; apply the fallback policy; rerun validation; and document the result.

Accepted suggestions:

- Preserve the official score-260 baseline when the scratch-FS attempt destabilized known-good commands.
- Keep only the existing five BusyBox scoring commands active.
- Preserve `echo hello` and `cat /musl/busybox_cmd.txt` as real non-scoring smoke commands.
- Keep `basename /aaa/bbb`, `uname`, and `ash -c exit` disabled.
- Record the reverted patch outside the repository for future analysis.
- Clean stale official Docker containers before official validation.

Rejected or avoided suggestions:

- No fake BusyBox output.
- No hard-coded command success.
- No promotion of unproven file-operation commands.
- No `runtime_dispatch.rs` change.
- No official script change.
- No broad risky source patch shipped after regression signals appeared.

Human verification was represented by local build checks, ELF checks, local LoongArch QEMU smoke, official Docker validation, and manual review of the score and BusyBox output evidence.

