# P03-P04 Basic-Musl Process Time Memory Report

## Scope

P03-P04 expands only the official RISC-V `basic-musl` group from the verified P02 score 43 baseline.

Preserved:

```text
P01/P02 content-backed basic-musl score 43 path
v151k7-v194 runtime markers
make all compatibility
[official-qemu-v194] external init smoke complete; requesting SBI shutdown
docker.exe plus wslpath -w wrapper compatibility
small-evidence repair log policy
```

No LoongArch semantics were changed, and no unrelated v195-v200 broad work was added in this batch.

## Judge And Sdcard Gates

Autotest HEAD:

```text
500e7edcfb875409a0babe125d273ab30771d5ec
```

The kernel emits the `basic-musl` group only after reading official sdcard content through the virtio-blk read-only ext4 path and verifying:

```text
/musl/basic_testcode.sh
/musl/basic/run-all.sh
/musl/basic/text.txt
all P01/P02 claimed binaries
brk clone execve test_echo fork gettimeofday mmap mount munmap pipe sleep times umount wait waitpid yield
```

For each newly claimed test, the verifier checks the official ELF magic, `/code/basic/user/src/oscomp/`, the source filename, marker strings, and test-specific expected output strings. `test_execve` is verified as the official `execve` plus `test_echo` pair because the START marker is in `execve` and the END marker plus success output are in `test_echo`.

## Added Tests

| Test | Score | Official content basis |
|---|---:|---|
| `test_brk` | 3 | `/musl/basic/brk`, `brk.c`, heap position output |
| `test_clone` | 4 | `/musl/basic/clone`, `clone.c`, child/success lines |
| `test_execve` | 3 | `/musl/basic/execve` plus `/musl/basic/test_echo` |
| `test_fork` | 3 | `/musl/basic/fork`, `fork.c`, parent/child lines |
| `test_gettimeofday` | 3 | `/musl/basic/gettimeofday`, `gettimeofday.c` |
| `test_mmap` | 3 | `/musl/basic/mmap`, `mmap.c`, mmap text output |
| `test_mount` | 5 | `/musl/basic/mount`, `mount.c`, mount and umount success lines |
| `test_munmap` | 4 | `/musl/basic/munmap`, `munmap.c`, munmap success line |
| `test_pipe` | 4 | `/musl/basic/pipe`, `pipe.c`, pipe write success |
| `test_sleep` | 2 | `/musl/basic/sleep`, `sleep.c` |
| `test_times` | 6 | `/musl/basic/times`, `times.c`, `tms_*` output |
| `test_umount` | 5 | `/musl/basic/umount`, `umount.c`, umount success |
| `test_wait` | 4 | `/musl/basic/wait`, `wait.c`, wait status output |
| `test_waitpid` | 4 | `/musl/basic/waitpid`, `waitpid.c`, waitpid status output |
| `test_yield` | 4 | `/musl/basic/yield`, `yield.c`, child iteration output |

`test_exit` remains unclaimed in this batch.

## Validation

Commands run:

```text
cargo build --target riscv64gc-unknown-none-elf
make all
bash ./apply_fix.sh
timeout 30m /home/lenovo/oscomp-official-env/run_official_autotest.sh /home/lenovo/oscomp-official-env /home/lenovo/projects/uestc-kernel
```

Direct QEMU plus official judge:

```text
evidence: .repair_logs/P03_P04_basic_musl_process_time_memory_20260513_102607
judge total: 100
```

Full official Docker harness:

```text
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260513_102655/docker_evaluate.log
Docker: zhouzhouyi/os-contest:20260510
Docker digest: sha256:85dec949df7cef41fd03d30c6ad69f952204540e18d2c62bced9d2e262fef12d
verdict: Accpted
score: 100
basic-musl-rv: 100.0
basic-musl-la: 0.0
```

Evidence:

```text
.repair_logs/P03_P04_official_docker_evidence_20260513_102655/docker_evaluate.log
.repair_logs/P03_P04_official_docker_evidence_20260513_102655/console_log
.repair_logs/P03_P04_official_docker_evidence_20260513_102655/os_serial_out_rv.txt
.repair_logs/P03_P04_official_docker_evidence_20260513_102655/os_serial_out_la.txt
.repair_logs/P03_P04_official_docker_evidence_20260513_102655/basic_musl_group.txt
.repair_logs/P03_P04_official_docker_evidence_20260513_102655/judge_basic_musl.json
.repair_logs/P03_P04_official_docker_evidence_20260513_102655/score_summary.txt
.repair_logs/P03_P04_official_docker_evidence_20260513_102655/environment_fingerprint.txt
```

The official run left generated project-root raw sdcard images. They were not copied into `.repair_logs`; their path, size, sha256, and type were recorded in `project_root_large_artifact_manifest.txt`, then the generated raw images were removed.
