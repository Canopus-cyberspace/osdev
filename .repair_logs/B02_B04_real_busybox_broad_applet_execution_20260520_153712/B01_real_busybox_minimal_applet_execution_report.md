# B01 Real BusyBox Minimal Applet Execution

## Preserved Scoreline

- Official baseline preserved before B01: verdict `Accpted`, score `153`, `basic-musl-rv 100.0`, `busybox-musl-rv 53.0`.
- B01 keeps `basic-musl-rv 100.0` on the existing REAL-RUN path.
- B01 keeps `busybox-musl-rv 53.0`; only the applets listed below are promoted to REAL-RUN. The remaining BusyBox score path is still legacy/content-backed.

## Basic-Musl Classification

REAL-RUN basic-musl scored cases:

`test_write`, `test_getpid`, `test_uname`, `test_getcwd`, `test_brk`, `test_gettimeofday`, `test_times`, `test_sleep`, `test_close`, `test_dup`, `test_dup2`, `test_open`, `test_read`, `test_openat`, `test_fstat`, `test_getdents`, `test_chdir`, `test_mkdir`, `test_unlink`, `test_pipe`, `test_yield`, `test_wait`, `test_waitpid`, `test_getppid`, `test_fork`, `test_clone`, `test_execve`, `test_mmap`, `test_munmap`, `test_mount`, `test_umount`.

NOT-YET-SUPPORTED basic-musl case:

`test_exit`.

## B01 BusyBox REAL-RUN Applets

The following commands are executed by loading the real official `/musl/busybox` ELF from the official RISC-V sdcard, entering RISC-V U-mode, passing applet argv, capturing stdout/stderr and exit code, and emitting success only after a verified `B01-realrun-busybox-result`:

- `/musl/busybox true`
- `/musl/busybox echo "#### independent command test"`
- `/musl/busybox pwd`
- `/musl/busybox ls`
- `/musl/busybox cat test.txt`

All other busybox-musl cases remain `LEGACY-CONTENT-BACKED` in B01. No real BusyBox execution is claimed for those remaining applets.

## Evidence

B01 evidence is written under:

`.repair_logs/B01_real_busybox_minimal_applet_execution_<timestamp>/`

Required files include `direct_qemu.log`, `score_summary.txt`, `environment_fingerprint.txt`, `realrun_busybox_matrix.json`, `realrun_busybox_matrix.md`, `real_busybox_stdout_by_case.txt`, `real_busybox_exit_code_by_case.txt`, `real_busybox_syscall_trace_by_case.txt`, `real_busybox_vfs_trace_by_case.txt`, `negative_missing_busybox.log`, `negative_wrong_argv.log`, `negative_wrong_stdout.log`, `negative_wrong_exit_code.log`, `judge_basic_musl.json`, and `judge_busybox_musl.json`.

Official Docker validation is recorded only when `RUN_OFFICIAL_AUTOTEST=1 bash ./apply_fix.sh` can run successfully. If Docker Desktop is unavailable, B01 reports direct QEMU and local judge validation only.
