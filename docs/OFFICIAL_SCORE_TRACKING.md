# Official Score Tracking

## Current baseline

```text
verdict: Accpted
score: 153
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
```

## Score table

| Date | Batch | Total score | basic-musl-rv | busybox-musl-rv | libctest-musl-rv | ltp-musl-rv | basic-musl-la | Notes |
|---|---|---:|---:|---:|---:|---:|---:|---|
| 2026-05-12 | basic write nonzero | 2 | 2.0 | 0 | 0 | 0 | 0 | content-backed `/musl/basic/write` |
| 2026-05-13 | P01 basic-musl expansion batch 1 | 35 | 35.0 | 0 | 0 | 0 | 0 | content-backed `/musl/basic` subset: getpid, getppid, uname, getcwd, close, dup, dup2, open, read, openat, fstat, getdents, write |
| 2026-05-13 | P02 basic-musl filesystem expansion | 43 | 43.0 | 0 | 0 | 0 | 0 | added content-backed `chdir`, `mkdir_`/`test_mkdir`, and `unlink`; full Docker 20260510 harness passed |
| 2026-05-13 | P03-P04 process/time/pipe/memory expansion | 100 | 100.0 | 0 | 0 | 0 | 0 | added content-backed `brk`, `clone`, `execve`, `fork`, `gettimeofday`, `mmap`, `mount`, `munmap`, `pipe`, `sleep`, `times`, `umount`, `wait`, `waitpid`, and `yield`; full Docker 20260510 harness passed |
| 2026-05-13 | P05 busybox-musl nonzero | 101 | 100.0 | 1.0 | 0 | 0 | 0 | added content-backed `busybox true`; full Docker 20260510 harness passed |
| 2026-05-13 | P06 busybox-musl command expansion | 112 | 100.0 | 12.0 | 0 | 0 | 0 | added content-backed busybox `echo`, `true`, `pwd`, `ls`, `cat`, `rm`, `mkdir`, `mv`, and `cp` command cases; full Docker 20260510 harness passed |
| 2026-05-15 | P07 busybox-musl command expansion | 152 | 100.0 | 52.0 | 0 | 0 | 0 | expanded to 52 content-backed official busybox command cases; full Docker 20260510 harness passed |
| 2026-05-15 | P08 busybox-musl remaining command expansion | 153 | 100.0 | 53.0 | 0 | 0 | 0 | added the final content-backed public judge busybox key `busybox kill 10`, backed by official sdcard line `sh -c 'sleep 5' & ./busybox kill $!`; full Docker 20260510 harness passed |
| 2026-05-16 | R01-R03 architecture split | 153 | 100.0 | 53.0 | 0 | 0 | 0 | no score-claim change; full Docker 20260510 harness passed after official/compat/real-mm split |

## Evidence required per row

- `docker_evaluate.log`
- `console_log`
- `os_serial_out_rv.txt`
- `os_serial_out_la.txt` if relevant
- direct judge result if run separately
- `environment_fingerprint.txt` and `score_summary.txt` for P01 and later

Repair logs are small-evidence-only. Files larger than 50 MB must be represented by path, size, sha256, and short metadata instead of copied into `.repair_logs`.
