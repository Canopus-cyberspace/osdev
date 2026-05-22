# Iteration 16 Test Report

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

The final `make all` logs did not contain Rust source warnings.

## Local LoongArch Smoke

A fresh LoongArch sdcard image was decompressed into `/tmp` and removed after use. No sdcard image was stored in the repository.

Final local result:

```text
qemu_status=0
[loongarch64-basic] attempted=32 completed=32 failed=none
[loongarch64-busybox] smoke completed=16 attempted=16 matched=16 failed=0 disabled=4
```

Promoted BusyBox commands and exit status:

```text
true: exit_code=0
false: exit_code=1
pwd: exit_code=0
sh -c exit: exit_code=0
basename /aaa/bbb: exit_code=0
printf "abc\n": exit_code=0
uname: exit_code=0
dirname /aaa/bbb: exit_code=0
expr 1 + 1: exit_code=0
date: exit_code=0
uptime: exit_code=0
clear: exit_code=0
cal: exit_code=0
ls: exit_code=0
```

Smoke-only commands:

```text
echo hello: exit_code=0
cat /musl/busybox_cmd.txt: exit_code=0
```

Commands probed but not promoted:

```text
which ls: exit_code=1
free: exit_code=1, /proc/meminfo unavailable
sleep 1: user fault at era=0x12016f814
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
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260522_123151/docker_evaluate.log
docker_evaluate.log size: 802896 bytes
Verdict: Accpted
Score: 269
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 102.0
busybox-musl-la: 14.0
```

Official LoongArch evidence:

```text
[loongarch64-basic] attempted=32 completed=32 failed=none
[loongarch64-busybox] smoke completed=16 attempted=16 matched=16 failed=0 disabled=4
testcase busybox true success
testcase busybox false success
testcase busybox pwd success
testcase busybox sh -c exit success
testcase busybox basename /aaa/bbb success
testcase busybox printf "abc\n" success
testcase busybox uname success
testcase busybox dirname /aaa/bbb success
testcase busybox expr 1 + 1 success
testcase busybox date success
testcase busybox uptime success
testcase busybox clear success
testcase busybox cal success
testcase busybox ls success
```

The official log did not contain `Failed to load ELF`, `ENOSYS`, a panic marker, a timeout, a user fault, or a blocker marker.

Generated project-root `sdcard-rv.img` and `sdcard-la.img` artifacts from the official wrapper were removed after validation. Official testdata images were not touched.

