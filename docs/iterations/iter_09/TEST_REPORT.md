# Iteration 09 Test Report

## Commands

```bash
git status --short
git diff --stat
git diff > /tmp/uestc_kernel_before_change.diff
git diff > /tmp/uestc_kernel_before_iter09_busybox_smoke.diff
cargo build --target riscv64gc-unknown-none-elf
bash tools/prune_repair_logs.sh /home/lenovo/projects/uestc-kernel
make all
file kernel-rv kernel-la
readelf -h kernel-la | sed -n '1,80p'
timeout 120s qemu-system-loongarch64 -kernel kernel-la -m 1G -nographic -smp 1 \
  -drive file=/tmp/sdcard-la-iter08.img,if=none,format=raw,id=x0 \
  -device virtio-blk-pci,drive=x0 -no-reboot \
  -device virtio-net-pci,netdev=net0 -netdev user,id=net0 -rtc base=utc
timeout 30m bash /home/lenovo/oscomp-official-env/run_official_autotest.sh \
  /home/lenovo/oscomp-official-env \
  /home/lenovo/projects/uestc-kernel
```

## Build Results

```text
cargo build --target riscv64gc-unknown-none-elf: passed
make all: passed
kernel-rv: ELF 64-bit LSB executable, UCB RISC-V
kernel-la: ELF 64-bit LSB executable, LoongArch
kernel-la entry: 0x90000000
```

No compiler warnings were observed in final local build output.

## Local LoongArch Basic Smoke

```text
[loongarch64-basic] attempted=32 completed=32 failed=none
```

## Local BusyBox Smoke

Enabled commands:

```text
busybox true
busybox false
busybox echo hello
busybox pwd
busybox sh -c exit
busybox ls
busybox cat /musl/busybox_cmd.txt
```

Final result:

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

Real output evidence:

```text
hello
/musl
basic
busybox
busybox_cmd.txt
grep hello busybox_cmd.txt
```

The final local LoongArch smoke log contained no `unsupported`, `missing syscall`, `timeout`, `user fault`, `panic`, `Failed to load ELF`, or `ENOSYS` marker.

## Development Blockers Observed

`busybox uname` was attempted and hard-stopped after entering PLV3 without another syscall. It is not enabled in the final smoke list.

`busybox ash -c exit` was attempted and returned a controlled user fault:

```text
ecode=15 era=0x1201b64b8 badv=0x1201b64b8
```

It is not enabled in the final smoke list.

## Official Validation

Official validation was attempted. The wrapper timed out after 30 minutes before producing Docker log output:

```text
/home/lenovo/oscomp-official-env/logs/evaluate_20260521_163721/docker_evaluate.log
size: 0 bytes
```

The official score was not refreshed in this iteration.
