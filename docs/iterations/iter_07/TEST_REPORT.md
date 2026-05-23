# Iteration 07 Test Report

## Commands

```bash
git status --short
git diff --stat
git diff > /tmp/uestc_kernel_before_change.diff
git diff > /tmp/uestc_kernel_before_iter07_busybox_loader.diff
bash tools/prune_repair_logs.sh /home/lenovo/projects/uestc-kernel
cargo build --target riscv64gc-unknown-none-elf
make all
file kernel-rv kernel-la
readelf -h kernel-la | sed -n '1,80p'
timeout 180s qemu-system-loongarch64 -kernel kernel-la -m 1G -nographic -smp 1 \
  -drive file=/tmp/sdcard-la-iter07.img,if=none,format=raw,id=x0 \
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

The local LoongArch smoke log contained no `ENOSYS`, `panic`, or `Failed to load ELF` marker.

## BusyBox Probe

BusyBox loaded from the real LoongArch sdcard image:

```text
[loongarch64-busybox] loaded /musl/busybox file_size=2065912 entry=0x901ca40c first_load=0x120000000 load_size=2087832 segments=2
[loongarch64-busybox] entering command=true
```

The probe did not hang. It returned through the LoongArch user fault path:

```text
[loongarch64-busybox] blocker: user fault ecode=8 era=0x9019d85c badv=0x90016138
```

No BusyBox command was claimed as passing. No `busybox-musl` group marker was printed.

## Official Validation

Official validation was attempted but failed before kernel evaluation:

```text
failed to connect to the docker API at npipe:////./pipe/dockerDesktopLinuxEngine
```

Latest prior successful official score remains unrefreshed:

```text
Verdict: Accpted
Score: 217
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 64.0
busybox-musl-la: 0.0
```

## Known Remaining Work

- Add real LoongArch user virtual-address mapping for fixed ET_EXEC ranges such as `0x120000000`.
- Re-enter BusyBox at its fixed entry after page-table or TLB support exists.
- Enable a `busybox-musl-la` runner only after real commands execute and exit through the trap path.
