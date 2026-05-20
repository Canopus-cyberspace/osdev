# K05 Real Basic-Musl Memory/Mount Hardmode Report

## Scoreline

- Preserved baseline entering K05: official verdict `Accpted`, score `153`, `basic-musl-rv 100.0`, `busybox-musl-rv 53.0`.
- Fresh K04b official Docker revalidation on `2026-05-15` passed with `zhouzhouyi/os-contest:20260510`: verdict `Accpted`, score `153`, `basic-musl-rv 100.0`, `busybox-musl-rv 53.0`.
- BusyBox remains on the existing legacy/content-backed `busybox-musl-rv 53.0` path. K05 does not claim real BusyBox execution.

## Preserved REAL-RUN Cases

K01/K02/K03/K04a/K04b REAL-RUN cases remain: `test_write`, `test_getpid`, `test_uname`, `test_getcwd`, `test_brk`, `test_gettimeofday`, `test_times`, `test_sleep`, `test_close`, `test_dup`, `test_dup2`, `test_open`, `test_read`, `test_openat`, `test_fstat`, `test_getdents`, `test_chdir`, `test_mkdir`, `test_unlink`, `test_pipe`, `test_yield`, `test_wait`, `test_waitpid`, `test_getppid`, `test_fork`, `test_clone`, and `test_execve`.

## K05 Promotions

K05 promotes the memory and mount family to REAL-RUN:

- `test_mmap`
- `test_munmap`
- `test_mount`
- `test_umount`

Each K05 success line is emitted only by replaying stdout from a verified `RealRunResult` produced by real RISC-V U-mode execution of the official ELF loaded from the official sdcard.

## Classification

| testcase | classification | note |
| --- | --- | --- |
| `test_brk` | REAL-RUN | K01/K02 preserved |
| `test_chdir` | REAL-RUN | K03 preserved |
| `test_clone` | REAL-RUN | K04b preserved |
| `test_close` | REAL-RUN | K03 preserved |
| `test_dup2` | REAL-RUN | K03 preserved |
| `test_dup` | REAL-RUN | K03 preserved |
| `test_execve` | REAL-RUN | K04b preserved |
| `test_exit` | NOT-YET-SUPPORTED | still not promoted |
| `test_fork` | REAL-RUN | K04b preserved |
| `test_fstat` | REAL-RUN | K03 preserved |
| `test_getcwd` | REAL-RUN | K01/K02 preserved |
| `test_getdents` | REAL-RUN | K03 preserved |
| `test_getpid` | REAL-RUN | K01/K02 preserved |
| `test_getppid` | REAL-RUN | K04b preserved |
| `test_gettimeofday` | REAL-RUN | K01/K02 preserved |
| `test_mkdir` | REAL-RUN | K03 preserved |
| `test_mmap` | REAL-RUN | K05 promoted |
| `test_mount` | REAL-RUN | K05 promoted |
| `test_munmap` | REAL-RUN | K05 promoted |
| `test_open` | REAL-RUN | K03 preserved |
| `test_openat` | REAL-RUN | K03 preserved |
| `test_pipe` | REAL-RUN | K04a preserved |
| `test_read` | REAL-RUN | K03 preserved |
| `test_sleep` | REAL-RUN | K01/K02 preserved |
| `test_times` | REAL-RUN | K01/K02 preserved |
| `test_umount` | REAL-RUN | K05 promoted |
| `test_uname` | REAL-RUN | K01/K02 preserved |
| `test_unlink` | REAL-RUN | K03 preserved |
| `test_wait` | REAL-RUN | K04a preserved |
| `test_waitpid` | REAL-RUN | K04a preserved |
| `test_write` | REAL-RUN | K01 preserved |
| `test_yield` | REAL-RUN | K04a preserved |

## Negative Checks

K05 evidence records negative checks for missing ELF, wrong stdout, wrong exit code, broken mmap content, broken munmap behavior, wrong mount target, and wrong umount behavior. These checks compare against captured real-run stdout, exit code, syscall trace, memory/page-fault/VMA trace, and mount/VFS trace.

## Evidence

Direct K05 evidence package:

```text
.repair_logs/K05_real_basic_musl_memory_mount_hardmode_20260515_235926/
```

Official Docker K05 evidence package:

```text
.repair_logs/K05_real_basic_musl_memory_mount_hardmode_20260516_000040/
```

Direct validation passed with `basic-musl-total 100`, `busybox-musl-total 53`, and `combined-direct-total 153`. The official Docker run completed with verdict `Accpted`, score `153`, `basic-musl-rv 100.0`, and `busybox-musl-rv 53.0`.

The K05 official evidence package records `official-rv-serial-source unavailable-or-stale`, so K05 real-run trace evidence is preserved from the direct QEMU serial in the same directory, while the official Docker score evidence is preserved separately as `docker_evaluate.log` and `official_autotest_run.log`.

Required evidence files:

```text
direct_qemu.log
docker_evaluate.log, if official Docker is run
console_log, if official Docker is run
official_autotest_run.log, if official Docker is run
os_serial_out_rv.txt
os_serial_out_la.txt, if available
score_summary.txt
environment_fingerprint.txt
realrun_matrix.json
realrun_matrix.md
real_exec_stdout_by_case.txt
real_exec_exit_code_by_case.txt
real_exec_syscall_trace_by_case.txt
real_exec_memory_trace_by_case.txt
real_exec_mount_trace_by_case.txt
negative_missing_file.log
negative_wrong_stdout.log
negative_wrong_exit_code.log
negative_broken_mmap_content.log
negative_broken_munmap.log
negative_wrong_mount_target.log
negative_wrong_umount.log
judge_basic_musl.json
judge_busybox_musl.json
```

Official Docker evidence is kept in the K05 directory only when `RUN_OFFICIAL_AUTOTEST=1 bash ./apply_fix.sh` completes. Direct validation and official Docker validation remain separate.
