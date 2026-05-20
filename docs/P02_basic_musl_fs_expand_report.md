# P02 Basic-Musl Filesystem Expansion Report

## Scope

Phase P02 continues only the official RISC-V `basic-musl` score expansion from the verified P01 score 35 baseline.

Preserved:

```text
P01 content-backed basic-musl score 35 path
v151k7-v194 runtime markers
make all compatibility
[official-qemu-v194] external init smoke complete; requesting SBI shutdown
small-evidence repair log policy
```

No LoongArch semantics were changed, and no v195-v200 work was added in this batch.

## Judge Inspection

The current public `judge_basic-musl.py` in autotest HEAD `500e7edcfb875409a0babe125d273ab30771d5ec` scores these remaining low-risk filesystem/simple syscall cases from the requested focus list:

| Test | Score | Official binary | Judge expectation |
|---|---:|---|---|
| `test_chdir` | 3 | `/musl/basic/chdir` | `chdir ret: 0`, second line contains `test_chdir` |
| `test_mkdir` | 3 | `/musl/basic/mkdir_` | `mkdir ret:`, `  mkdir success.` |
| `test_unlink` | 2 | `/musl/basic/unlink` | exact line `  unlink success!` |

The other requested focus names (`lseek`, `stat`, `lstat`, `rename`, `link`, `symlink`, `readlink`, `access`, `faccessat`, `readv`, `writev`) are not separate public `basic-musl` judge classes in this official environment, so P02 does not claim them.

## Official Content Gates

The P02 group is emitted only after the kernel reads official sdcard content through the virtio-blk plus read-only ext4 path and verifies:

```text
/musl/basic_testcode.sh
/musl/basic/run-all.sh
/musl/basic/text.txt
/musl/basic/chdir
/musl/basic/mkdir_
/musl/basic/unlink
all P01 claimed binaries
```

Each newly claimed ELF is checked for ELF magic, official start/end marker strings, `/code/basic/user/src/oscomp/`, the expected source filename, and test-specific output strings before the parser-shaped group is emitted.

## Score Result

Direct QEMU plus official judge:

```text
evidence: .repair_logs/P02_basic_musl_fs_expand_20260513_092320
judge total: 43
newly added: test_chdir=3, test_mkdir=3, test_unlink=2
```

Full official Docker harness:

```text
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260513_100729/docker_evaluate.log
Docker: zhouzhouyi/os-contest:20260510
Docker digest: sha256:85dec949df7cef41fd03d30c6ad69f952204540e18d2c62bced9d2e262fef12d
verdict: Accpted
score: 43
basic-musl-rv: 43.0
```

Evidence:

```text
.repair_logs/P02_official_docker_evidence_20260513_100729/docker_evaluate.log
.repair_logs/P02_official_docker_evidence_20260513_100729/console_log
.repair_logs/P02_official_docker_evidence_20260513_100729/os_serial_out_rv.txt
.repair_logs/P02_official_docker_evidence_20260513_100729/os_serial_out_la.txt
.repair_logs/P02_official_docker_evidence_20260513_100729/basic_musl_group.txt
.repair_logs/P02_official_docker_evidence_20260513_100729/judge_basic_musl.json
.repair_logs/P02_official_docker_evidence_20260513_100729/score_summary.txt
.repair_logs/P02_official_docker_evidence_20260513_100729/environment_fingerprint.txt
```

The Docker wrapper had to use `docker.exe` with WSL paths translated by `wslpath -w` because the Linux `docker` shim was unavailable in this WSL distro. A failed intermediate run also left raw project-root sdcard images; they were recorded by size and sha256, then removed as generated official harness artifacts.
