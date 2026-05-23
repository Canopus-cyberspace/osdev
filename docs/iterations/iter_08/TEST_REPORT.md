# Iteration 08 Test Report

## Commands

```bash
git status --short
git diff --stat
git diff > /tmp/uestc_kernel_before_change.diff
git diff > /tmp/uestc_kernel_before_iter08_loongarch_va_mapping.diff
bash tools/prune_repair_logs.sh /home/lenovo/projects/uestc-kernel
cargo build --target riscv64gc-unknown-none-elf
make all
file kernel-rv kernel-la
readelf -h kernel-la | sed -n '1,80p'
timeout 180s qemu-system-loongarch64 -kernel kernel-la -m 1G -nographic -smp 1 \
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
========== START test_clone ==========
========== END test_clone ==========
========== START test_mmap ==========
========== END test_mmap ==========
========== START test_munmap ==========
========== END test_munmap ==========
[loongarch64-basic] attempted=32 completed=32 failed=none
```

The local LoongArch smoke log contained no `ENOSYS`, `panic`, `Failed to load ELF`, `user fault`, or `missing syscall` marker.

## BusyBox Smoke

The real `/musl/busybox` ELF loaded from the LoongArch sdcard image and entered PLV3 at the fixed ET_EXEC entry:

```text
[loongarch64-busybox] loaded /musl/busybox file_size=2065912 entry=0x1201b640c first_load=0x120000000 load_size=2087832 segments=2 command=true
[loongarch64-busybox] entering command=true
[loongarch64-busybox] mapped entry=0x1201b640c
[loongarch64-busybox] command=true exit_code=0
[loongarch64-busybox] smoke completed=1 attempted=1
```

No official `busybox-musl` group markers were emitted, and no BusyBox output was faked.

## Official Validation

Official validation was attempted but failed before kernel evaluation:

```text
failed to connect to the docker API at npipe:////./pipe/dockerDesktopLinuxEngine
```

The official score was not refreshed in this iteration.
