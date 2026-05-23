# Iteration 02 Test Report

## Commands

```bash
git status --short
git diff --stat
git diff > /tmp/uestc_kernel_before_change.diff
bash tools/prune_repair_logs.sh /home/lenovo/projects/uestc-kernel
cargo build --target riscv64gc-unknown-none-elf
make all
file kernel-rv kernel-la
readelf -h kernel-la | sed -n '1,80p'
timeout 120s qemu-system-loongarch64 -kernel kernel-la -m 1G -nographic -smp 1 \
  -drive file=/tmp/sdcard-la-s06j.img,if=none,format=raw,id=x0 \
  -device virtio-blk-pci,drive=x0 -no-reboot \
  -device virtio-net-pci,netdev=net0 -netdev user,id=net0 -rtc base=utc
timeout 30m bash /home/lenovo/oscomp-official-env/run_official_autotest.sh \
  /home/lenovo/oscomp-official-env \
  /home/lenovo/projects/uestc-kernel
```

## Results

```text
cargo build --target riscv64gc-unknown-none-elf: passed
make all: passed
kernel-rv: ELF 64-bit LSB executable, UCB RISC-V
kernel-la: ELF 64-bit LSB executable, LoongArch
kernel-la entry: 0x90000000
```

Local LoongArch smoke:

```text
qemu_status=0
[loongarch64-basic] attempted=24 completed=24 failed=none
```

The local LoongArch smoke log contained no `ENOSYS`, `panic`, `Failed to load ELF`, `unsupported`, or `blocker` marker.

## Official Validation

Official validation was attempted, but the runner failed before kernel evaluation because Docker was unavailable:

```text
failed to connect to the docker API at npipe:////./pipe/dockerDesktopLinuxEngine
```

The most recent prior successful official result remains:

```text
Verdict: Accpted
Score: 217
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 64.0
busybox-musl-la: 0.0
```

## Known Risk

The new trap stack is global. It is sufficient for the current single active LoongArch PLV3 execution path, but a future fork/clone scheduler should assign each runnable task its own kernel stack or otherwise serialize trap use explicitly.
