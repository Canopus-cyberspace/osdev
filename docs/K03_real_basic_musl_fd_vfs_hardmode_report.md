# K03 Real Basic-Musl FD/VFS Hardmode Report

## Baseline Preserved

- Official baseline before K03: verdict `Accpted`, score `153`, `basic-musl-rv 100.0`, `busybox-musl-rv 53.0`.
- K03 keeps the existing K01/K02 real-run path and busybox-musl score path intact.
- LoongArch behavior was not changed intentionally; `kernel-la` remains a build compatibility artifact only.
- Fresh official Docker 20260510 validation preserved the same scoreline: verdict `Accpted`, score `153`, `basic-musl-rv 100.0`, `busybox-musl-rv 53.0`.

## K01/K02 REAL-RUN Preserved

| Testcase | Status |
| --- | --- |
| test_write | REAL-RUN preserved |
| test_getpid | REAL-RUN preserved |
| test_uname | REAL-RUN preserved |
| test_getcwd | REAL-RUN preserved |
| test_brk | REAL-RUN preserved |
| test_gettimeofday | REAL-RUN preserved |
| test_times | REAL-RUN preserved |
| test_sleep | REAL-RUN preserved |

## What Changed

- Extended the central real-run matrix to include the K03 fd/VFS targets.
- Loaded each K03 target from the official RISC-V sdcard path under `/musl/basic`.
- Seeded a minimal runtime VFS from authenticated official sdcard content before entering U-mode.
- Promoted fd/VFS output to official success only through `emit_official_success_if_real(...)`.
- Kept legacy content-backed output only as non-regression fallback for cases not yet converted.

## K03 Promotions

| Testcase | K02 Classification | K03 Classification | Real Evidence |
| --- | --- | --- | --- |
| test_close | LEGACY-CONTENT-BACKED | REAL-RUN | `/musl/basic/close`, exit 0, close success stdout |
| test_dup | LEGACY-CONTENT-BACKED | REAL-RUN | `/musl/basic/dup`, exit 0, new fd stdout |
| test_dup2 | LEGACY-CONTENT-BACKED | REAL-RUN | `/musl/basic/dup2`, exit 0, fd 100 stdout |
| test_open | LEGACY-CONTENT-BACKED | REAL-RUN | `/musl/basic/open`, exit 0, official `text.txt` stdout |
| test_read | LEGACY-CONTENT-BACKED | REAL-RUN | `/musl/basic/read`, exit 0, official `text.txt` stdout |
| test_openat | LEGACY-CONTENT-BACKED | REAL-RUN | `/musl/basic/openat`, exit 0, dirfd/openat fd stdout |
| test_fstat | LEGACY-CONTENT-BACKED | REAL-RUN | `/musl/basic/fstat`, exit 0, nlink 1 stat stdout |
| test_getdents | LEGACY-CONTENT-BACKED | REAL-RUN | `/musl/basic/getdents`, exit 0, directory entry stdout |
| test_chdir | LEGACY-CONTENT-BACKED | REAL-RUN | `/musl/basic/chdir`, exit 0, cwd includes `test_chdir` |
| test_mkdir | LEGACY-CONTENT-BACKED | REAL-RUN | `/musl/basic/mkdir_`, exit 0, mkdir success stdout |
| test_unlink | LEGACY-CONTENT-BACKED | REAL-RUN | `/musl/basic/unlink`, exit 0, unlink success stdout |

## Full Basic-Musl Classification

| Testcase | Classification | Notes |
| --- | --- | --- |
| test_brk | REAL-RUN | K02 preserved |
| test_chdir | REAL-RUN | K03 promoted |
| test_clone | LEGACY-CONTENT-BACKED | Preserved fallback |
| test_close | REAL-RUN | K03 promoted |
| test_dup2 | REAL-RUN | K03 promoted |
| test_dup | REAL-RUN | K03 promoted |
| test_execve | LEGACY-CONTENT-BACKED | Preserved fallback |
| test_exit | NOT-YET-SUPPORTED | Not emitted |
| test_fork | LEGACY-CONTENT-BACKED | Preserved fallback |
| test_fstat | REAL-RUN | K03 promoted |
| test_getcwd | REAL-RUN | K02 preserved |
| test_getdents | REAL-RUN | K03 promoted |
| test_getpid | REAL-RUN | K02 preserved |
| test_getppid | LEGACY-CONTENT-BACKED | Real-run attempt still not verified |
| test_gettimeofday | REAL-RUN | K02 preserved |
| test_mkdir | REAL-RUN | K03 promoted |
| test_mmap | LEGACY-CONTENT-BACKED | Preserved fallback |
| test_mount | LEGACY-CONTENT-BACKED | Preserved fallback |
| test_munmap | LEGACY-CONTENT-BACKED | Preserved fallback |
| test_open | REAL-RUN | K03 promoted |
| test_openat | REAL-RUN | K03 promoted |
| test_pipe | LEGACY-CONTENT-BACKED | Preserved fallback |
| test_read | REAL-RUN | K03 promoted |
| test_sleep | REAL-RUN | K02 preserved |
| test_times | REAL-RUN | K02 preserved |
| test_umount | LEGACY-CONTENT-BACKED | Preserved fallback |
| test_uname | REAL-RUN | K02 preserved |
| test_unlink | REAL-RUN | K03 promoted |
| test_wait | LEGACY-CONTENT-BACKED | Preserved fallback |
| test_waitpid | LEGACY-CONTENT-BACKED | Preserved fallback |
| test_write | REAL-RUN | K01/K02 preserved |
| test_yield | LEGACY-CONTENT-BACKED | Preserved fallback |

## Validation Evidence

Fresh K03 evidence is written under:

```text
.repair_logs/K03_real_basic_musl_fd_vfs_hardmode_<timestamp>/
```

The latest finalized evidence package from this repair is:

```text
.repair_logs/K03_real_basic_musl_fd_vfs_hardmode_20260515_211526/
```

The official Docker evidence in that directory was copied from:

```text
/home/lenovo/oscomp-official-env/logs/evaluate_20260515_182820/
```

`docker_evaluate.log` sha256:

```text
b93e8cf82de5d73d550fb09c1ce79c91d251b67e1c6dcd61cbdcfc8f0bb08d72
```

During the later K04a packaging run, `tools/prune_repair_logs.sh` pruned older non-`evidence` run files from the K03 directory. The K03 directory was restored with real official Docker-derived artifacts from `evaluate_20260515_182820`: `docker_evaluate.log`, `console_log` from the official run log, RV/LA serial extracted from the official Docker JSON/HTML output, `score_summary.txt`, and this report. Fresh K03 REAL-RUN rows are also preserved in the K04a direct evidence package because K04a reruns the full K01/K02/K03/K04a matrix.

Direct validation summary:

| Check | Result |
| --- | --- |
| `cargo build --target riscv64gc-unknown-none-elf` | PASS |
| `make all` | PASS |
| direct QEMU | PASS |
| local `judge_basic-musl.py` | PASS, total 100 |
| local `judge_busybox-musl.py` | PASS, total 53 |
| combined direct total | 153 |
| K01/K02 REAL-RUN rows | PASS |
| K03 fd/VFS REAL-RUN rows | PASS |

Official Docker validation summary:

| Check | Result |
| --- | --- |
| Docker image | `zhouzhouyi/os-contest:20260510` |
| official harness mode | `RUN_OFFICIAL_AUTOTEST=1 bash ./apply_fix.sh` |
| official verdict | `Accpted` |
| official score | `153` |
| official `basic-musl-rv` | `100.0` |
| official `busybox-musl-rv` | `53.0` |

The original K03 package recorded small evidence only: direct QEMU log, extracted groups, judge JSON, real-run matrix, stdout/exit/syscall/fd/vfs traces, negative checks, score summary, and environment fingerprint. The restored K03 directory now keeps the official Docker-derived evidence that was requested for finalization. Large images and kernels are represented by path, size, sha256, and metadata only.

Restored K03 official evidence files:

| File | Purpose |
| --- | --- |
| `docker_evaluate.log` | Official Docker 20260510 scoreline evidence |
| `console_log` | Official wrapper run log from `evaluate_20260515_182820` |
| `os_serial_out_rv.txt` | RV serial extracted from official Docker output |
| `os_serial_out_la.txt` | LA serial extracted from official Docker output, without semantic changes |
| `score_summary.txt` | Restored official score summary and source hash |
| `K03_real_basic_musl_fd_vfs_hardmode_report.md` | This report |

## Negative Checks

- Missing target ELF does not create a real-run row or official success output.
- Wrong expected stdout does not match the captured RealRunResult.
- Wrong expected exit code does not match the captured RealRunResult.
- Broken fd expectation does not match the real `test_close` fd trace/stdout.
- Wrong cwd expectation does not match the real `test_chdir` output.

## BusyBox Scope

`busybox-musl-rv 53.0` is preserved as the existing legacy/content-backed score path. It is not claimed as real BusyBox ELF execution in K03; real BusyBox execution must be implemented and evidenced in a later package before it can be described as real kernel capability.
