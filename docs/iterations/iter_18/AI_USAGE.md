# Iteration 18 AI Usage

AI was used to inspect the LoongArch fd table, syscall compatibility path, BusyBox command runner, linker layout, local QEMU output, and official wrapper logs.

Accepted AI-assisted changes:

- Added minimal read-only virtual `/proc` entries in `fd_table.rs`.
- Added narrow syscall compatibility for `lseek`, `statfs`, `fstatfs`, `sysinfo`, and `syslog`.
- Promoted only real locally passing BusyBox commands: `dmesg`, `df`, `ps`, and `free`.
- Stabilized the LoongArch `.user` section address in `linker.ld`.
- Documented local validation and the official timeout honestly.

Rejected or avoided changes:

- No scratch-FS, redirection, pipeline, grep, or file-write path work was added.
- `sleep 1` and `ash -c exit` remain disabled.
- No fake BusyBox output or hard-coded command success was added.
- `runtime_dispatch.rs` was not modified.

Human verification should focus on the local log `/tmp/la_iter18_final_local2.log` and the official timeout log directory `/home/lenovo/oscomp-official-env/logs/evaluate_20260522_180412/`.

