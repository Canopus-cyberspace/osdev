# Iteration 14 Test Report

## Build Environment

```text
CPU_COUNT: 16
```

Parallel build preparation was used for local builds. The official wrapper was run conservatively with `MAKEFLAGS`, `MFLAGS`, and `CARGO_BUILD_JOBS` unset.

## Commands Run

```text
cargo build -j 16 --target riscv64gc-unknown-none-elf
make -j 16 all
make all
file kernel-rv kernel-la
```

Results:

```text
cargo build -j 16 --target riscv64gc-unknown-none-elf: passed
make -j 16 all: passed with the known local jobserver warning
make all: passed
kernel-rv: ELF 64-bit LSB executable, UCB RISC-V
kernel-la: ELF 64-bit LSB executable, LoongArch
```

## Local LoongArch Smoke

A fresh LoongArch sdcard image was generated from official testdata into `/tmp` for local smoke testing and was not stored in the repository.

Result after reverting the risky patches:

```text
qemu_status=0
[loongarch64-basic] attempted=32 completed=32 failed=none
[loongarch64-busybox] smoke completed=7 attempted=7 matched=7 failed=0 disabled=3
```

BusyBox scoring lines remained limited to the five known-good official commands:

```text
testcase busybox true success
testcase busybox false success
testcase busybox pwd success
testcase busybox sh -c exit success
testcase busybox ls success
```

Smoke-only real commands still completed without scoring promotion:

```text
busybox echo hello
busybox cat /musl/busybox_cmd.txt
```

Disabled commands were not enabled:

```text
basename /aaa/bbb
uname
ash -c exit
```

The final local smoke contained no `ENOSYS`, panic marker, user fault, command timeout, or `Failed to load ELF` marker.

## Commands Attempted But Not Shipped

The reverted patches attempted BusyBox file-operation groundwork and partial command probing:

```text
printf "abc\n": passed once in the larger patch, then stalled in the reduced fresh run
touch test.txt: stalled
cp busybox_cmd.txt busybox_cmd.bak: passed once inside the risky patch
rm busybox_cmd.bak: passed once inside the risky patch
mkdir test_dir / mv test_dir test / rmdir test: destabilized known-good false before the directory commands ran
```

No attempted command was promoted because the known-good five-command scoring set must remain stable.

## Container Cleanup

Before official validation, stale containers from the official image were inspected, stopped, and removed if present:

```text
docker ps -a --filter ancestor=zhouzhouyi/os-contest:20260510 --format '{{.ID}} {{.Status}} {{.Names}}'
docker ps --filter ancestor=zhouzhouyi/os-contest:20260510 -q | xargs -r docker stop
docker ps -a --filter ancestor=zhouzhouyi/os-contest:20260510 -q | xargs -r docker rm
docker ps
ps aux | grep -E 'qemu|run_official|cargo|make' | grep -v grep || true
```

No stale official containers needed removal in this run.

## Official Validation

Command:

```text
unset MAKEFLAGS
unset MFLAGS
unset CARGO_BUILD_JOBS
timeout 30m bash /home/lenovo/oscomp-official-env/run_official_autotest.sh \
  /home/lenovo/oscomp-official-env \
  /home/lenovo/projects/uestc-kernel
```

Result:

```text
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260521_233219/docker_evaluate.log
docker_evaluate.log size: 798848 bytes
Verdict: Accpted
Score: 260
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 102.0
busybox-musl-la: 5.0
```

Official LoongArch evidence:

```text
[loongarch64-basic] attempted=32 completed=32 failed=none
[loongarch64-busybox] smoke completed=7 attempted=7 matched=7 failed=0 disabled=3
testcase busybox true success
testcase busybox false success
testcase busybox pwd success
testcase busybox sh -c exit success
testcase busybox ls success
```

The official log did not report `Failed to load ELF`, `ENOSYS`, a panic marker, a user fault in enabled commands, or a timeout.

