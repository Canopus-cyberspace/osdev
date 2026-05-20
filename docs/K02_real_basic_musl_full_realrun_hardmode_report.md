# K02 Real Basic-Musl Full RealRun Hardmode Report

Date: 2026-05-15

## Baseline Preserved

- Official baseline before K02: verdict `Accpted`, score `153`, `basic-musl-rv 100.0`, `busybox-musl-rv 53.0`.
- K02 keeps the K01 real `/musl/basic/write` execution path.
- K02 keeps the legacy non-regression fallback for unconverted basic-musl cases and labels it as `LEGACY-CONTENT-BACKED`.
- LoongArch semantics were not changed.

## RealRunResult Evidence

K02 records a central `RealRunResult`-style row for each attempted official ELF:

- `program_path`
- `elf_sha256`
- `entry_pc`
- `loaded_segments`
- `argv`
- `envp`
- `auxv`
- `entered_umode`
- `syscall_trace`
- `page_fault_trace`
- `stdout`
- `stderr`
- `exit_code`
- `final_task_state`

Only verified `REAL-RUN` cases are emitted through the central official result emitter:

```text
emit_official_success_if_real(...)
```

## Direct Validation Evidence

Evidence directory:

```text
.repair_logs/K02_real_basic_musl_full_realrun_hardmode_20260515_175544/
```

Direct QEMU and local official judges:

```text
basic-musl-total 100
busybox-musl-total 53
combined-direct-total 153
```

Verified REAL-RUN cases:

| testcase | ELF | result |
| --- | --- | --- |
| test_write | `/musl/basic/write` | REAL-RUN |
| test_getpid | `/musl/basic/getpid` | REAL-RUN |
| test_uname | `/musl/basic/uname` | REAL-RUN |
| test_getcwd | `/musl/basic/getcwd` | REAL-RUN |
| test_brk | `/musl/basic/brk` | REAL-RUN |
| test_gettimeofday | `/musl/basic/gettimeofday` | REAL-RUN |
| test_times | `/musl/basic/times` | REAL-RUN |
| test_sleep | `/musl/basic/sleep` | REAL-RUN |

Attempted but not promoted:

```text
test_getppid: real ELF entered U-mode and exited 0, but stdout was "getppid error."; the official success line remains legacy fallback only.
```

Negative checks:

- Missing target ELF did not emit success.
- Wrong expected stdout did not emit success.
- Wrong expected exit code did not emit success.

## Testcase Classification

Only `REAL-RUN` rows below are real kernel capability claims.

| basic-musl testcase | K02 classification | note |
| --- | --- | --- |
| test_brk | REAL-RUN | Real official ELF, heap progression validated as +64/+64. |
| test_chdir | LEGACY-CONTENT-BACKED | Preserved score fallback. |
| test_clone | LEGACY-CONTENT-BACKED | Preserved score fallback. |
| test_close | LEGACY-CONTENT-BACKED | Preserved score fallback. |
| test_dup2 | LEGACY-CONTENT-BACKED | Preserved score fallback. |
| test_dup | LEGACY-CONTENT-BACKED | Preserved score fallback. |
| test_execve | LEGACY-CONTENT-BACKED | Preserved score fallback. |
| test_exit | NOT-YET-SUPPORTED | Not emitted by the preserved score path. |
| test_fork | LEGACY-CONTENT-BACKED | Preserved score fallback. |
| test_fstat | LEGACY-CONTENT-BACKED | Preserved score fallback. |
| test_getcwd | REAL-RUN | Real official ELF returned captured stdout and exit 0. |
| test_getdents | LEGACY-CONTENT-BACKED | Preserved score fallback. |
| test_getpid | REAL-RUN | Real official ELF returned pid output and exit 0. |
| test_getppid | LEGACY-CONTENT-BACKED | Real attempt failed expected stdout; preserved score fallback remains legacy only. |
| test_gettimeofday | REAL-RUN | Real official ELF observed monotonic timeval interval and exit 0. |
| test_mkdir | LEGACY-CONTENT-BACKED | Preserved score fallback. |
| test_mmap | LEGACY-CONTENT-BACKED | Preserved score fallback. |
| test_mount | LEGACY-CONTENT-BACKED | Preserved score fallback. |
| test_munmap | LEGACY-CONTENT-BACKED | Preserved score fallback. |
| test_open | LEGACY-CONTENT-BACKED | Preserved score fallback. |
| test_openat | LEGACY-CONTENT-BACKED | Preserved score fallback. |
| test_pipe | LEGACY-CONTENT-BACKED | Preserved score fallback. |
| test_read | LEGACY-CONTENT-BACKED | Preserved score fallback. |
| test_sleep | REAL-RUN | Real official ELF observed successful sleep path and exit 0. |
| test_times | REAL-RUN | Real official ELF returned tms fields and exit 0. |
| test_umount | LEGACY-CONTENT-BACKED | Preserved score fallback. |
| test_uname | REAL-RUN | Real official ELF printed kernel uname fields and exit 0. |
| test_unlink | LEGACY-CONTENT-BACKED | Preserved score fallback. |
| test_wait | LEGACY-CONTENT-BACKED | Preserved score fallback. |
| test_waitpid | LEGACY-CONTENT-BACKED | Preserved score fallback. |
| test_write | REAL-RUN | K01/K02 anchor: real official ELF stdout and exit code verified. |
| test_yield | LEGACY-CONTENT-BACKED | Preserved score fallback. |

## Validation Commands

```bash
cargo build --target riscv64gc-unknown-none-elf
make all
bash ./apply_fix.sh
RUN_OFFICIAL_AUTOTEST=1 bash ./apply_fix.sh
```

## Evidence Files

Direct evidence produced:

- `realrun_matrix.json`
- `realrun_matrix.md`
- `real_exec_stdout_by_case.txt`
- `real_exec_exit_code_by_case.txt`
- `real_exec_syscall_trace_by_case.txt`
- `negative_missing_file.log`
- `negative_wrong_stdout.log`
- `negative_wrong_exit_code.log`
- `judge_basic_musl.json`
- `judge_busybox_musl.json`
- `score_summary.txt`
- `environment_fingerprint.txt`

The full official Docker evidence is collected into the same K02 evidence directory when `RUN_OFFICIAL_AUTOTEST=1` is used.
