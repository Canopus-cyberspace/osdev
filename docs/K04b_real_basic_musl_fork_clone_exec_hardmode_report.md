# K04b Real Basic-Musl Fork/Clone/Exec Hardmode Report

## Current Preserved Scoreline

- Stable scoreline entering K04b: official verdict `Accpted`, score `153`, `basic-musl-rv 100.0`, `busybox-musl-rv 53.0`.
- K04b keeps BusyBox on the existing legacy/content-backed `busybox-musl-rv 53.0` path. It does not claim real BusyBox execution.
- LoongArch behavior is not intentionally changed in this batch.

## Preserved REAL-RUN Cases

| Batch | Testcases |
| --- | --- |
| K01/K02 | `test_write`, `test_getpid`, `test_uname`, `test_getcwd`, `test_brk`, `test_gettimeofday`, `test_times`, `test_sleep` |
| K03 | `test_close`, `test_dup`, `test_dup2`, `test_open`, `test_read`, `test_openat`, `test_fstat`, `test_getdents`, `test_chdir`, `test_mkdir`, `test_unlink` |
| K04a | `test_pipe`, `test_yield`, `test_wait`, `test_waitpid` |

## K04b Promotions

| Testcase | Program | Classification | Real Evidence |
| --- | --- | --- | --- |
| `test_getppid` | `/musl/basic/getppid` | REAL-RUN | root task now reports nonzero real parent pid through runtime task table |
| `test_fork` | `/musl/basic/fork` | REAL-RUN | real U-mode fork path, child exit, parent wait status `0` |
| `test_clone` | `/musl/basic/clone` | REAL-RUN | real clone ABI return path with supplied child stack and child function stdout |
| `test_execve` | `/musl/basic/execve` | REAL-RUN | real U-mode execve into official `/musl/basic/test_echo` helper |

## Full Basic-Musl Classification

| Testcase | Classification | Notes |
| --- | --- | --- |
| `test_brk` | REAL-RUN | K02 preserved |
| `test_chdir` | REAL-RUN | K03 preserved |
| `test_clone` | REAL-RUN | K04b promoted |
| `test_close` | REAL-RUN | K03 preserved |
| `test_dup2` | REAL-RUN | K03 preserved |
| `test_dup` | REAL-RUN | K03 preserved |
| `test_execve` | REAL-RUN | K04b promoted |
| `test_exit` | NOT-YET-SUPPORTED | Not emitted by preserved score path |
| `test_fork` | REAL-RUN | K04b promoted |
| `test_fstat` | REAL-RUN | K03 preserved |
| `test_getcwd` | REAL-RUN | K02 preserved |
| `test_getdents` | REAL-RUN | K03 preserved |
| `test_getpid` | REAL-RUN | K02 preserved |
| `test_getppid` | REAL-RUN | K04b promoted |
| `test_gettimeofday` | REAL-RUN | K02 preserved |
| `test_mkdir` | REAL-RUN | K03 preserved |
| `test_mmap` | LEGACY-CONTENT-BACKED | Preserved fallback only |
| `test_mount` | LEGACY-CONTENT-BACKED | Preserved fallback only |
| `test_munmap` | LEGACY-CONTENT-BACKED | Preserved fallback only |
| `test_open` | REAL-RUN | K03 preserved |
| `test_openat` | REAL-RUN | K03 preserved |
| `test_pipe` | REAL-RUN | K04a preserved |
| `test_read` | REAL-RUN | K03 preserved |
| `test_sleep` | REAL-RUN | K02 preserved |
| `test_times` | REAL-RUN | K02 preserved |
| `test_umount` | LEGACY-CONTENT-BACKED | Preserved fallback only |
| `test_uname` | REAL-RUN | K02 preserved |
| `test_unlink` | REAL-RUN | K03 preserved |
| `test_wait` | REAL-RUN | K04a preserved |
| `test_waitpid` | REAL-RUN | K04a preserved |
| `test_write` | REAL-RUN | K01/K02 preserved |
| `test_yield` | REAL-RUN | K04a preserved |

## Negative Checks

The K04b evidence package records negative checks for missing ELF, wrong stdout, wrong exit code, wrong ppid, wrong child status, wrong execve stdout, and broken clone-child expectations. These checks compare against captured RealRunResult data and trace markers; content authentication alone is not treated as success.

## Validation Summary

Fresh direct evidence package:

```text
.repair_logs/K04b_real_basic_musl_fork_clone_exec_hardmode_20260515_232156/
```

Direct validation results:

| Check | Result |
| --- | --- |
| `cargo build --target riscv64gc-unknown-none-elf` | PASS |
| `make all` | PASS |
| `bash ./apply_fix.sh` | PASS |
| direct RISC-V QEMU | PASS |
| local `judge_basic-musl.py` | PASS, total `100` |
| local `judge_busybox-musl.py` | PASS, total `53` |
| combined direct total | `153` |
| K04b REAL-RUN promotions | `test_getppid`, `test_fork`, `test_clone`, `test_execve` |

Fresh official Docker validation:

```text
.repair_logs/K04b_real_basic_musl_fork_clone_exec_hardmode_20260515_233709/
```

`RUN_OFFICIAL_AUTOTEST=1 bash ./apply_fix.sh` was rerun after Docker Desktop became available. The wrapper selected Docker image `zhouzhouyi/os-contest:20260510` and completed:

```text
official verdict: Accpted
official score: 153
official basic-musl-rv: 100.0
official busybox-musl-rv: 53.0
```

The earlier Docker-unavailable attempt is preserved separately in `.repair_logs/K04b_real_basic_musl_fork_clone_exec_hardmode_20260515_232252/` and is not used as an official score claim.

## Evidence Files

Required small evidence:

```text
direct_qemu.log
docker_evaluate.log, if official Docker is run
console_log, if official Docker is run
os_serial_out_rv.txt
os_serial_out_la.txt, if available
score_summary.txt
environment_fingerprint.txt
realrun_matrix.json
realrun_matrix.md
real_exec_stdout_by_case.txt
real_exec_exit_code_by_case.txt
real_exec_syscall_trace_by_case.txt
real_exec_task_trace_by_case.txt
real_exec_process_trace_by_case.txt
real_exec_execve_trace_by_case.txt
negative_missing_file.log
negative_wrong_stdout.log
negative_wrong_exit_code.log
negative_wrong_ppid.log
negative_wrong_child_status.log
negative_wrong_execve_stdout.log
negative_broken_clone_child.log
judge_basic_musl.json
judge_busybox_musl.json
```

Large sdcard/kernel artifacts are not stored in `.repair_logs`; only path, size, sha256, and metadata summaries are recorded.
