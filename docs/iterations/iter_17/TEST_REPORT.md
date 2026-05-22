# Iteration 17 Test Report

## Build Environment

```text
CPU_COUNT: 16
```

## Commands Run

```text
bash tools/prune_repair_logs.sh /home/lenovo/projects/uestc-kernel
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

`runtime_dispatch.rs` had no diff.

## Local LoongArch Smoke

An official-style local LoongArch QEMU run was executed with a temporary sdcard image in `/tmp`.

Final local result:

```text
qemu_status=0
[loongarch64-basic] attempted=32 completed=32 failed=none
[loongarch64-busybox] smoke completed=17 attempted=17 matched=17 failed=0 disabled=8
testcase busybox du success
```

Promoted BusyBox command:

```text
du: exit_code=0
```

Candidates probed but not promoted:

```text
dmesg: exit_code=1, klogctl not implemented
df: exit_code=1, /proc/mounts unavailable
ps: exit_code=1, /proc unavailable
hwclock: exit_code=1, /dev/misc/rtc unavailable
```

The final local run showed no `Failed to load ELF`, `ENOSYS`, panic marker, timeout, user fault, or blocker in enabled commands.

## Container Cleanup

Before official validation:

```text
docker ps -a --filter ancestor=zhouzhouyi/os-contest:20260510 --format '{{.ID}} {{.Status}} {{.Names}}'
docker ps --filter ancestor=zhouzhouyi/os-contest:20260510 -q | xargs -r docker stop
docker ps -a --filter ancestor=zhouzhouyi/os-contest:20260510 -q | xargs -r docker rm
docker ps
ps aux | grep -E 'qemu|run_official|cargo|make' | grep -v grep || true
```

No stale official containers or QEMU processes were present.

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
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260522_144845/docker_evaluate.log
docker_evaluate.log size: 803263 bytes
Verdict: Accpted
Score: 270
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 102.0
busybox-musl-la: 15.0
```

Official LoongArch evidence:

```text
[loongarch64-basic] attempted=32 completed=32 failed=none
[loongarch64-busybox] smoke completed=17 attempted=17 matched=17 failed=0 disabled=8
testcase busybox du success
```

The official log did not contain `Failed to load ELF`, `ENOSYS`, a panic marker, a timeout, a user fault, or a blocker marker.

Generated project-root `sdcard-rv.img` and `sdcard-la.img` artifacts from the official wrapper were removed after validation. Official testdata images were not touched.

