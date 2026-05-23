# Iteration 13 Test Report

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

A fresh LoongArch sdcard image was generated from official testdata into `/tmp` for local smoke testing. It was removed after validation.

Command shape:

```text
timeout 120s qemu-system-loongarch64 -kernel kernel-la -m 1G -nographic -smp 1 \
  -drive file=/tmp/sdcard-la-iter13-fresh.img,if=none,format=raw,id=x0 \
  -device virtio-blk-pci,drive=x0 -no-reboot \
  -device virtio-net-pci,netdev=net0 -netdev user,id=net0 -rtc base=utc
```

Result:

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

## Container Cleanup

Before official validation, stale containers from the official image were inspected, stopped, and removed:

```text
docker ps -a --filter ancestor=zhouzhouyi/os-contest:20260510 --format '{{.ID}} {{.Status}} {{.Names}}'
docker ps --filter ancestor=zhouzhouyi/os-contest:20260510 -q | xargs -r docker stop
docker ps -a --filter ancestor=zhouzhouyi/os-contest:20260510 -q | xargs -r docker rm
docker ps
ps aux | grep -E 'qemu|run_official|cargo|make' | grep -v grep || true
```

Stale running containers found:

```text
5712a8abb84b
174c3ab496e8
8ea6d6359418
b32969e2cf1c
```

After cleanup, no official Docker containers or stale QEMU/build processes remained.

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
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260521_225549/docker_evaluate.log
docker_evaluate.log size: 798847 bytes
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

## Commands Attempted But Not Shipped

The reverted patch attempted BusyBox file-operation groundwork and partial command promotion. The notable observed blockers were:

```text
echo "hello world" > test.txt: stalled through real BusyBox shell redirection
grep hello busybox_cmd.txt: stalled after direct file-op trimming
```

No newly attempted command was promoted because the known-good five-command scoring set must remain stable.

