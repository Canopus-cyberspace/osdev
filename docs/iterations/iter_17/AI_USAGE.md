# Iteration 17 AI Usage

AI was used to inspect the LoongArch BusyBox command table, public command candidates, local QEMU probe output, and official Docker score output.

Accepted AI-assisted changes:

- Promoted `du` after real local execution.
- Kept failed direct/read-only candidates in the disabled command table.
- Preserved the existing command execution path and official result emission semantics.
- Documented local and official validation.

Rejected or avoided changes:

- No `dmesg` syscall shim was shipped after a brief probe showed stability risk.
- No scratch-FS, redirection, pipeline, grep, or shell-complex support was touched.
- No syscall compatibility changes were added.
- No fake BusyBox output or hard-coded command success was added.
- `runtime_dispatch.rs` was not modified.

Human verification should focus on the official log at `/home/lenovo/oscomp-official-env/logs/evaluate_20260522_144845/docker_evaluate.log`, which records `Score: 270` and `busybox-musl-la: 15.0`.

