# Iteration 10 AI Usage

AI assistance was used to inspect the existing BusyBox runner, official BusyBox judge format, group-marker parsing, LoongArch group-active state, syscall diagnostic paths, local and official validation output, and iteration documentation.

Accepted AI changes:

- Added official-name mapping to `BusyboxCommand`.
- Added official `busybox-musl` markers around the safe real BusyBox command set.
- Added `emit_official_result` so only real command outcomes produce official testcase lines.
- Added BusyBox group-active state and `is_any_group_active()` to suppress noisy diagnostics during official groups.
- Added concise basic and BusyBox phase progress prints.
- Documented validation, score change, and remaining BusyBox blockers.

Rejected or deferred suggestions:

- No fake BusyBox output or hard-coded command success was added.
- `busybox uname` was not enabled.
- `busybox ash -c exit` was not enabled.
- Non-official smoke commands did not emit official testcase lines.
- `runtime_dispatch.rs` was not changed.

Human/environment verification was performed through local QEMU smoke, local official judge parsing, and a completed official Docker evaluation.
