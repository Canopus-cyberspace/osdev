# K04a Real Basic-Musl Process/IPC Hardmode Report

## Baseline Preserved

- Stable scoreline entering K04a: official verdict `Accpted`, score `153`, `basic-musl-rv 100.0`, `busybox-musl-rv 53.0`.
- K01/K02/K03 real-run cases remain verified through the central `RealRunResult` path.
- K04a promotes process/IPC cases only after loading the official RISC-V ELF from the official sdcard, entering real U-mode, capturing stdout, exit code, syscall trace, and task/pipe traces.
- BusyBox remains the existing legacy/content-backed `busybox-musl-rv 53.0` score path; K04a does not claim real BusyBox ELF execution.
- LoongArch behavior was not intentionally changed.

## Preserved REAL-RUN Cases

| Batch | Testcases |
| --- | --- |
| K01/K02 | `test_write`, `test_getpid`, `test_uname`, `test_getcwd`, `test_brk`, `test_gettimeofday`, `test_times`, `test_sleep` |
| K03 | `test_close`, `test_dup`, `test_dup2`, `test_open`, `test_read`, `test_openat`, `test_fstat`, `test_getdents`, `test_chdir`, `test_mkdir`, `test_unlink` |

## K04a Promotions

| Testcase | Program | Classification | Real Evidence |
| --- | --- | --- | --- |
| `test_pipe` | `/musl/basic/pipe` | REAL-RUN | real fork/pipe path, pipe fd trace, child write, parent read, exit 0 |
| `test_yield` | `/musl/basic/yield` | REAL-RUN | real `sched_yield` path, task switch trace, 15 captured child lines, exit 0 |
| `test_wait` | `/musl/basic/wait` | REAL-RUN | real child exit collection, wait status `0`, exit 0 |
| `test_waitpid` | `/musl/basic/waitpid` | REAL-RUN | real child exit collection, waitpid status `3`, exit 0 |

## Full Basic-Musl Classification

| Testcase | Classification | Notes |
| --- | --- | --- |
| `test_brk` | REAL-RUN | K02 preserved |
| `test_chdir` | REAL-RUN | K03 preserved |
| `test_clone` | LEGACY-CONTENT-BACKED | Preserved fallback only |
| `test_close` | REAL-RUN | K03 preserved |
| `test_dup2` | REAL-RUN | K03 preserved |
| `test_dup` | REAL-RUN | K03 preserved |
| `test_execve` | LEGACY-CONTENT-BACKED | Preserved fallback only |
| `test_exit` | NOT-YET-SUPPORTED | Not emitted by preserved score path |
| `test_fork` | LEGACY-CONTENT-BACKED | Preserved fallback only |
| `test_fstat` | REAL-RUN | K03 preserved |
| `test_getcwd` | REAL-RUN | K02 preserved |
| `test_getdents` | REAL-RUN | K03 preserved |
| `test_getpid` | REAL-RUN | K02 preserved |
| `test_getppid` | LEGACY-CONTENT-BACKED | Real-run attempt remains unverified; score fallback is legacy only |
| `test_gettimeofday` | REAL-RUN | K02 preserved |
| `test_mkdir` | REAL-RUN | K03 preserved |
| `test_mmap` | LEGACY-CONTENT-BACKED | Preserved fallback only |
| `test_mount` | LEGACY-CONTENT-BACKED | Preserved fallback only |
| `test_munmap` | LEGACY-CONTENT-BACKED | Preserved fallback only |
| `test_open` | REAL-RUN | K03 preserved |
| `test_openat` | REAL-RUN | K03 preserved |
| `test_pipe` | REAL-RUN | K04a promoted |
| `test_read` | REAL-RUN | K03 preserved |
| `test_sleep` | REAL-RUN | K02 preserved |
| `test_times` | REAL-RUN | K02 preserved |
| `test_umount` | LEGACY-CONTENT-BACKED | Preserved fallback only |
| `test_uname` | REAL-RUN | K02 preserved |
| `test_unlink` | REAL-RUN | K03 preserved |
| `test_wait` | REAL-RUN | K04a promoted |
| `test_waitpid` | REAL-RUN | K04a promoted |
| `test_write` | REAL-RUN | K01/K02 preserved |
| `test_yield` | REAL-RUN | K04a promoted |

## Negative Checks

| Check | Required Result |
| --- | --- |
| missing target ELF | no REAL-RUN result and no official success emission |
| wrong expected stdout | does not match captured `test_write` RealRunResult |
| wrong expected exit code | does not match captured `test_write` exit code |
| intentionally broken pipe operation | does not match captured `test_pipe` stdout and pipe trace |
| wrong yield ordering expectation | does not match captured `test_yield` stdout line count/order |
| wrong wait status expectation | does not match captured `test_wait`/`test_waitpid` status stdout |

## Validation Summary

Latest evidence package:

```text
.repair_logs/K04a_real_basic_musl_process_ipc_hardmode_20260515_215615/
```

Official Docker source log:

```text
/home/lenovo/oscomp-official-env/logs/evaluate_20260515_215706/docker_evaluate.log
```

`docker_evaluate.log` sha256:

```text
4f3c0136ae11b1d0ec7925186d540b88e38917ab7c2a01a17425145e927836cb
```

Validation results from the package:

| Check | Result |
| --- | --- |
| `cargo build --target riscv64gc-unknown-none-elf` | PASS |
| `make all` | PASS |
| direct QEMU | PASS |
| local `judge_basic-musl.py` | PASS, total `100` |
| local `judge_busybox-musl.py` | PASS, total `53` |
| combined direct total | `153` |
| official Docker 20260510 harness | PASS with `RUN_OFFICIAL_AUTOTEST=1` |
| official scoreline | verdict `Accpted`, score `153`, `basic-musl-rv 100.0`, `busybox-musl-rv 53.0` |

The final package uses direct QEMU serial evidence for K04a real-run traces and Docker evidence for the official scoreline. The official wrapper did not provide a fresh separate RV serial file with K04a markers, so `score_summary.txt` records `official-rv-serial-source unavailable-or-stale` and preserves the direct-QEMU serial source for real-run trace evidence.

## Evidence Files

The repair package records small evidence only:

| File | Purpose |
| --- | --- |
| `docker_evaluate.log` | Official Docker 20260510 scoreline evidence |
| `console_log` | Official harness console output |
| `os_serial_out_rv.txt` | RV serial evidence containing K01/K02/K03/K04a real-run markers |
| `os_serial_out_la.txt` | LA serial evidence, without semantic changes |
| `score_summary.txt` | Direct and official score summary |
| `environment_fingerprint.txt` | Autotest, Docker, judge, and sdcard fingerprints |
| `realrun_matrix.json` | Structured RealRunResult matrix |
| `realrun_matrix.md` | Human-readable RealRunResult matrix |
| `real_exec_stdout_by_case.txt` | Captured stdout by testcase |
| `real_exec_exit_code_by_case.txt` | Captured exit code by testcase |
| `real_exec_syscall_trace_by_case.txt` | Captured syscall traces |
| `real_exec_task_trace_by_case.txt` | Captured task/process/scheduler traces |
| `real_exec_pipe_trace_by_case.txt` | Captured pipe traces |
| `negative_missing_file.log` | Negative missing-ELF result |
| `negative_wrong_stdout.log` | Negative stdout mismatch result |
| `negative_wrong_exit_code.log` | Negative exit-code mismatch result |
| `negative_broken_pipe_operation.log` | Negative pipe expectation result |
| `negative_wrong_yield_order.log` | Negative yield expectation result |
| `negative_wrong_wait_status.log` | Negative wait-status expectation result |
| `judge_basic_musl.json` | Local official basic-musl judge result |
| `judge_busybox_musl.json` | Local official busybox-musl judge result |
| `direct_qemu.log` | Fresh direct RISC-V QEMU serial log |

Large sdcard/kernel artifacts are not stored in `.repair_logs`; only path, size, sha256, and metadata summaries are recorded.
