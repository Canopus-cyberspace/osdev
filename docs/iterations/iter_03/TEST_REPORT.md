# Iteration 03 Test Report

## Commands

```bash
git status --short
git diff --stat
git diff > /tmp/uestc_kernel_before_change.diff
bash tools/prune_repair_logs.sh /home/lenovo/projects/uestc-kernel
cargo build --target riscv64gc-unknown-none-elf
make all
file kernel-rv kernel-la
readelf -h kernel-la | sed -n '1,40p'
timeout 180s qemu-system-loongarch64 -kernel kernel-la -m 1G -nographic -smp 1 \
  -drive file=/tmp/sdcard-la-s06j.img,if=none,format=raw,id=x0 \
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

No compiler warnings were observed in the local build output.

## Local LoongArch Smoke

```text
qemu_status=0
========== START test_exit ==========
========== END test_exit ==========
========== START test_wait ==========
========== END test_wait ==========
========== START test_waitpid ==========
========== END test_waitpid ==========
========== START test_yield ==========
========== END test_yield ==========
========== START test_fork ==========
========== END test_fork ==========
[loongarch64-basic] attempted=29 completed=29 failed=none
```

The local smoke log contained no `ENOSYS`, `panic`, `Failed to load ELF`, `unsupported`, or `blocker` marker.

## Official Validation

Official validation was attempted but failed before kernel evaluation:

```text
failed to connect to the docker API at npipe:////./pipe/dockerDesktopLinuxEngine
```

Latest prior successful official score remains:

```text
Verdict: Accpted
Score: 217
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 64.0
busybox-musl-la: 0.0
```

## Known Remaining Work

- `clone` remains disabled.
- `execve` remains disabled.
- `pipe` remains disabled.
- Official score needs a Docker-backed rerun.
