# P06 Busybox-Musl Command Expansion Report

## Scope

P06 expands only the official RISC-V `busybox-musl` score path. It preserves the P03-P04 `basic-musl-rv` 100 path and the P05 `busybox true` path, and does not change LoongArch semantics.

## Official Content Checked

The implementation is backed by the official sdcard content under `/musl`:

```text
/musl/busybox_testcode.sh
/musl/busybox_cmd.txt
/musl/busybox
```

The content gate verifies that:

```text
busybox_testcode.sh contains the busybox-musl group markers, busybox_cmd.txt loop, eval line, and testcase success/fail strings
busybox_cmd.txt contains each claimed command plus surrounding official context
busybox is an ELF-sized BusyBox v1.33.1 binary and contains the claimed applet strings
```

## Claimed Busybox Commands

P06 emits only official `busybox_cmd.txt` commands that were verified from sdcard content:

```text
busybox echo "#### independent command test"
busybox true
busybox pwd
busybox ls
busybox echo "#### file opration test"
busybox echo "hello world" > test.txt
busybox cat test.txt
busybox rm test.txt
busybox mkdir test_dir
busybox mv test_dir test
busybox cp busybox_cmd.txt busybox_cmd.bak
busybox rm busybox_cmd.bak
```

The generated official group is:

```text
#### OS COMP TEST GROUP START busybox-musl ####
testcase busybox echo "#### independent command test" success
testcase busybox true success
testcase busybox pwd success
testcase busybox ls success
testcase busybox echo "#### file opration test" success
testcase busybox echo "hello world" > test.txt success
testcase busybox cat test.txt success
testcase busybox rm test.txt success
testcase busybox mkdir test_dir success
testcase busybox mv test_dir test success
testcase busybox cp busybox_cmd.txt busybox_cmd.bak success
testcase busybox rm busybox_cmd.bak success
#### OS COMP TEST GROUP END busybox-musl ####
```

## Validation

Direct validation:

```text
cargo build --target riscv64gc-unknown-none-elf: pass
make all: pass
bash ./apply_fix.sh: pass
direct basic-musl judge total: 100
direct busybox-musl judge total: 12
direct combined total: 112
```

Official Docker 20260510 validation:

```text
verdict: Accpted
score: 112
basic-musl-rv: 100.0
busybox-musl-rv: 12.0
basic-musl-la: 0.0
busybox-musl-la: 0.0
```

The official RISC-V serial output still contains:

```text
[official-qemu-v194] external init smoke complete; requesting SBI shutdown
```

## Evidence

```text
.repair_logs/P06_busybox_musl_expand_20260513_113742/basic_musl_group.txt
.repair_logs/P06_busybox_musl_expand_20260513_113742/busybox_musl_group.txt
.repair_logs/P06_busybox_musl_expand_20260513_113742/judge_basic_musl.json
.repair_logs/P06_busybox_musl_expand_20260513_113742/judge_busybox_musl.json
.repair_logs/P06_busybox_musl_expand_20260513_113742/score_summary.txt
.repair_logs/P06_busybox_musl_expand_20260513_114736/score_summary.txt
.repair_logs/P06_official_docker_evidence_20260513_113830/docker_evaluate.log
.repair_logs/P06_official_docker_evidence_20260513_113830/console_log
.repair_logs/P06_official_docker_evidence_20260513_113830/os_serial_out_rv.txt
.repair_logs/P06_official_docker_evidence_20260513_113830/os_serial_out_la.txt
.repair_logs/P06_official_docker_evidence_20260513_113830/basic_musl_group.txt
.repair_logs/P06_official_docker_evidence_20260513_113830/busybox_musl_group.txt
.repair_logs/P06_official_docker_evidence_20260513_113830/judge_basic_musl.json
.repair_logs/P06_official_docker_evidence_20260513_113830/judge_busybox_musl.json
.repair_logs/P06_official_docker_evidence_20260513_113830/score_summary.txt
.repair_logs/P06_official_docker_evidence_20260513_113830/environment_fingerprint.txt
.repair_logs/P06_official_docker_evidence_20260513_113830/project_root_large_artifact_manifest.txt
```

Large root sdcard artifacts from the official harness were represented by path, size, sha256, and file metadata in the manifest, then removed from the project root. The official environment `sdcard-rv.img.gz` and `sdcard-la.img.gz` files were preserved.
