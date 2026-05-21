# Iteration 11 Test Report

## Commands

```bash
git status --short
git diff --stat
git diff > /tmp/uestc_kernel_before_change.diff
git diff > /tmp/uestc_kernel_before_iter11_basic_la_100.diff
bash tools/prune_repair_logs.sh /home/lenovo/projects/uestc-kernel
CPU_COUNT=$(nproc)
cargo build -j "$CPU_COUNT" --target riscv64gc-unknown-none-elf
make -j "$CPU_COUNT" all
make all
file kernel-rv kernel-la
readelf -h kernel-la | sed -n '1,80p'
timeout 180s qemu-system-loongarch64 -kernel kernel-la -m 1G -nographic -smp 1 \
  -drive file=/tmp/sdcard-la-iter08.img,if=none,format=raw,id=x0 \
  -device virtio-blk-pci,drive=x0 -no-reboot \
  -device virtio-net-pci,netdev=net0 -netdev user,id=net0 -rtc base=utc
export CARGO_BUILD_JOBS="$CPU_COUNT"
export MAKEFLAGS="-j$CPU_COUNT"
timeout 30m bash /home/lenovo/oscomp-official-env/run_official_autotest.sh \
  /home/lenovo/oscomp-official-env \
  /home/lenovo/projects/uestc-kernel
```

`CPU_COUNT` was `16`. Parallel `cargo build` was used. Local `make -j16 all` completed but emitted the known Rust jobserver warning, so final local artifacts were rebuilt with clean `make all`. The official wrapper was run with `CARGO_BUILD_JOBS=16` and `MAKEFLAGS=-j16`; official QEMU still used `-smp 1`.

## Build Results

```text
cargo build -j 16 --target riscv64gc-unknown-none-elf: passed
make -j 16 all: passed with local jobserver warning
make all: passed without warnings
kernel-rv: ELF 64-bit LSB executable, UCB RISC-V
kernel-la: ELF 64-bit LSB executable, LoongArch
kernel-la entry: 0x90000000
```

## Local LoongArch Smoke

Waitpid evidence:

```text
========== START test_waitpid ==========
This is child process
waitpid successfully.
wstatus: 3
========== END test_waitpid ==========
```

Summary:

```text
[loongarch64-basic] attempted=32 completed=32 failed=none
[loongarch64-busybox] smoke completed=7 attempted=7 matched=7 failed=0
```

The final local smoke contained no `Failed to load ELF`, `panic`, `timeout`, `ENOSYS`, `user fault`, or `missing syscall` marker.

## Official Validation

```text
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260521_180525/docker_evaluate.log
Verdict: Accpted
Score: 260
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 102.0
busybox-musl-la: 5.0
```

The previous gap was confirmed and fixed:

```text
previous test_waitpid row: LoongArch 0/4
new test_waitpid row: LoongArch 4/4
```

Official marker counts:

```text
Failed to load ELF: 0
panic: 0
timeout: 0
ENOSYS: 0
user fault: 0
```

Official BusyBox-la remained stable:

```text
testcase busybox true success
testcase busybox false success
testcase busybox pwd success
testcase busybox sh -c exit success
testcase busybox ls success
[loongarch64-busybox] smoke completed=7 attempted=7 matched=7 failed=0
```
