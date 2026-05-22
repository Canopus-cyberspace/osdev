# Iteration 18 Test Report

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
kernel-la .user: 0x90010000
```

`runtime_dispatch.rs` had no diff.

## Local LoongArch Smoke

An official-style local LoongArch QEMU run was executed with a temporary sdcard image in `/tmp`.

Final local result:

```text
qemu_status=0
[loongarch64-basic] attempted=32 completed=32 failed=none
[loongarch64-busybox] smoke completed=21 attempted=21 matched=21 failed=0 disabled=4
```

Newly promoted BusyBox commands:

```text
dmesg: exit_code=0
df: exit_code=0
ps: exit_code=0
free: exit_code=0
```

Existing scoring commands continued to run, including:

```text
true
false
pwd
sh -c exit
ls
basename /aaa/bbb
printf "abc\n"
uname
dirname /aaa/bbb
expr 1 + 1
date
uptime
clear
cal
du
```

Disabled commands:

```text
ash -c exit: known previous user fault
which ls: previous exit_code=1
hwclock: /dev/misc/rtc unavailable
sleep 1: previous user fault
```

The final local run showed no `Failed to load ELF`, `ENOSYS`, panic marker, timeout, user fault, unsupported syscall, or blocker in enabled commands.

## Container Cleanup

Before official validation:

```text
docker ps -a --filter ancestor=zhouzhouyi/os-contest:20260510 --format '{{.ID}} {{.Status}} {{.Names}}'
docker ps --filter ancestor=zhouzhouyi/os-contest:20260510 -q | xargs -r docker stop
docker ps -a --filter ancestor=zhouzhouyi/os-contest:20260510 -q | xargs -r docker rm
docker ps
ps aux | grep -E 'qemu|run_official|cargo|make' | grep -v grep || true
```

No stale official containers were present before the attempt. After the timeout, one stale official container was stopped/removed.

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
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260522_180412/docker_evaluate.log
docker_evaluate.log size: 0 bytes
result: timed out at outer 30-minute wrapper
```

Because the official run did not complete, no refreshed official score is claimed for this iteration.

Latest completed official validation remains:

```text
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260522_144845/docker_evaluate.log
Verdict: Accpted
Score: 270
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 102.0
busybox-musl-la: 15.0
```

Generated project-root `sdcard-rv.img` and `sdcard-la.img` artifacts from the timed-out wrapper were removed after validation. Official testdata images were not touched.

