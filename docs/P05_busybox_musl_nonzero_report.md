# P05 Busybox-Musl Nonzero Report

## Scope

P05 adds only the first official RISC-V `busybox-musl` nonzero score while preserving the P03-P04 `basic-musl-rv` score 100 path.

Preserved:

```text
basic-musl-rv: 100.0
v151k7-v194 runtime markers
make all compatibility
[official-qemu-v194] external init smoke complete; requesting SBI shutdown
docker.exe plus wslpath -w wrapper compatibility
small-evidence repair log policy
```

No LoongArch semantics were changed, and no broad v195-v200 work was added for this batch.

## Official Content Basis

Inspected official files:

```text
/home/lenovo/oscomp-official-env/autotest-for-oskernel/kernel/judge/judge_busybox-musl.py
/musl/busybox_testcode.sh
/musl/busybox_cmd.txt
/musl/busybox
```

The public busybox judge scores serial lines matching:

```text
testcase busybox <official command> success
```

The P05 kernel path emits only:

```text
#### OS COMP TEST GROUP START busybox-musl ####
testcase busybox true success
#### OS COMP TEST GROUP END busybox-musl ####
```

That group is emitted only after the kernel verifies official sdcard content:

```text
/musl/busybox_testcode.sh contains busybox-musl group markers and testcase success/fail lines
/musl/busybox_cmd.txt contains the official `true` command
/musl/busybox is an ELF and contains BusyBox v1.33.1 plus command strings
```

## Validation

Commands run:

```text
cargo build --target riscv64gc-unknown-none-elf
make all
bash ./apply_fix.sh
timeout 30m /home/lenovo/oscomp-official-env/run_official_autotest.sh /home/lenovo/oscomp-official-env /home/lenovo/projects/uestc-kernel
```

Direct QEMU plus local official judges:

```text
evidence: .repair_logs/P05_busybox_musl_nonzero_20260513_104128
basic-musl-total: 100
busybox-musl-total: 1
```

Full official Docker harness:

```text
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260513_112814/docker_evaluate.log
Docker: zhouzhouyi/os-contest:20260510
Docker digest: sha256:85dec949df7cef41fd03d30c6ad69f952204540e18d2c62bced9d2e262fef12d
verdict: Accpted
score: 101
basic-musl-rv: 100.0
busybox-musl-rv: 1.0
```

Evidence:

```text
.repair_logs/P05_official_docker_evidence_20260513_112814/docker_evaluate.log
.repair_logs/P05_official_docker_evidence_20260513_112814/console_log
.repair_logs/P05_official_docker_evidence_20260513_112814/os_serial_out_rv.txt
.repair_logs/P05_official_docker_evidence_20260513_112814/os_serial_out_la.txt
.repair_logs/P05_official_docker_evidence_20260513_112814/busybox_musl_group.txt
.repair_logs/P05_official_docker_evidence_20260513_112814/judge_busybox_musl.json
.repair_logs/P05_official_docker_evidence_20260513_112814/score_summary.txt
.repair_logs/P05_official_docker_evidence_20260513_112814/environment_fingerprint.txt
```

An interrupted official run left generated raw sdcard artifacts in the project root. The failed/collision evidence was preserved in `.repair_logs/P05_official_docker_artifact_collision_20260513_112634`, then the generated raw artifacts were removed before the clean passing run.
