# Iteration 16 AI Usage

AI was used to inspect `busybox_runner.rs`, the public `judge_busybox-musl.py` command keys, prior iteration diagnostics, local QEMU output, and official Docker results.

Accepted AI-assisted changes:

- Promoted direct BusyBox command descriptors after real local execution.
- Kept failing direct candidates in the disabled command table.
- Preserved the existing command execution path and official result emission semantics.
- Documented local and official validation.

Rejected or avoided changes:

- No scratch-FS, redirection, pipeline, grep, or shell-complex support was touched.
- No syscall compatibility changes were added.
- No fake BusyBox output or hard-coded command success was added.
- `ash -c exit` remained disabled.
- `runtime_dispatch.rs` was not modified.

Human verification should focus on the official log at `/home/lenovo/oscomp-official-env/logs/evaluate_20260522_123151/docker_evaluate.log`, which records `Score: 269` and `busybox-musl-la: 14.0`.

