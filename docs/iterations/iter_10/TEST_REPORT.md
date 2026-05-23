# Iteration 10 Test Report

## Commands

```bash
git status --short
git diff --stat
git diff > /tmp/uestc_kernel_before_change.diff
git diff > /tmp/uestc_kernel_before_iter10_busybox_scoring_runner.diff
bash tools/prune_repair_logs.sh /home/lenovo/projects/uestc-kernel
CPU_COUNT=$(nproc)
cargo build -j "$CPU_COUNT" --target riscv64gc-unknown-none-elf
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

`CPU_COUNT` was `16`. Parallel `cargo build` was used. A local `make -j16 all` attempt exposed a local Rust jobserver warning, so final local validation used normal `make all`. The official wrapper was run with `CARGO_BUILD_JOBS=16` and `MAKEFLAGS=-j16` as requested. Official QEMU still used one guest CPU core:

```text
qemu-system-loongarch64 ... -smp 1 ...
```

## Build Results

```text
cargo build -j 16 --target riscv64gc-unknown-none-elf: passed
make all: passed
kernel-rv: ELF 64-bit LSB executable, UCB RISC-V
kernel-la: ELF 64-bit LSB executable, LoongArch
kernel-la entry: 0x90000000
```

No compiler warnings were observed in the final local build. The official Docker build log contained only the pre-existing `/root/.cargo/config` deprecation warning from the container environment.

## Local LoongArch Smoke

```text
[loongarch64-basic] attempted=32 completed=32 failed=none
```

BusyBox commands:

```text
[loongarch64-busybox] command=true exit_code=0
[loongarch64-busybox] command=false exit_code=1
[loongarch64-busybox] command=echo exit_code=0
[loongarch64-busybox] command=pwd exit_code=0
[loongarch64-busybox] command=sh-exit exit_code=0
[loongarch64-busybox] command=ls exit_code=0
[loongarch64-busybox] command=cat exit_code=0
[loongarch64-busybox] smoke completed=7 attempted=7 matched=7 failed=0
```

Official BusyBox judge check against the local BusyBox group recognized five successes:

```text
busybox true
busybox false
busybox pwd
busybox sh -c exit
busybox ls
```

No local `unsupported`, `missing syscall`, `timeout`, `user fault`, `panic`, `Failed to load ELF`, or `ENOSYS` marker was observed.

## Official Validation

The first official attempt produced a non-empty log but failed before evaluation because stale generated project-root raw images blocked gzip extraction:

```text
gzip: sdcard-rv.img already exists; not overwritten
```

Only generated project-root `sdcard-rv.img` and `sdcard-la.img` were removed; official compressed testdata images were not touched.

The second official run completed:

```text
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260521_173700/docker_evaluate.log
Verdict: Accpted
Score: 256
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 98.0
busybox-musl-la: 5.0
```

Official LoongArch evidence:

```text
[loongarch64-basic] attempted=32 completed=32 failed=none
[loongarch64-busybox] smoke completed=7 attempted=7 matched=7 failed=0
testcase busybox true success
testcase busybox false success
testcase busybox pwd success
testcase busybox sh -c exit success
testcase busybox ls success
```

Official marker counts:

```text
Failed to load ELF: 0
panic: 0
timeout: 0
ENOSYS: 0
user fault: 0
```
