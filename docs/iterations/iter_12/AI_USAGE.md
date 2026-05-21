# Iteration 12 AI Usage

AI was used to inspect the interrupted BusyBox expansion state, narrow the patch to the LoongArch BusyBox runner, preserve the known-good official BusyBox command set, run local and official validation, and document the iteration.

Accepted suggestions:

- Keep scoring, smoke, and disabled BusyBox commands explicit in `busybox_runner.rs`.
- Emit official BusyBox testcase lines only for the scoring class.
- Preserve non-scoring smoke commands for real PLV3 coverage without claiming additional official score.
- Keep `basename`, `uname`, and `ash -c exit` disabled.

Rejected or avoided suggestions:

- No fake BusyBox output.
- No hard-coded command success.
- No broad syscall/fd/process changes.
- No `runtime_dispatch.rs` changes.
- No official script changes.

Human verification was represented by local QEMU smoke, build checks, ELF checks, and the official Docker validation log.

