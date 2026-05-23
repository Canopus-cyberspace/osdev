# Iteration 06 Test Report

## Commands

```bash
git status --short
git diff --stat
git diff > /tmp/uestc_kernel_before_change.diff
git diff > /tmp/uestc_kernel_before_clone_mmap_busybox.diff
bash tools/prune_repair_logs.sh /home/lenovo/projects/uestc-kernel
cargo build --target riscv64gc-unknown-none-elf
make all
file kernel-rv kernel-la
readelf -h kernel-la | sed -n '1,80p'
timeout 180s qemu-system-loongarch64 -kernel kernel-la -m 1G -nographic -smp 1 \
  -drive file=/tmp/sdcard-la-clone-mmap.img,if=none,format=raw,id=x0 \
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

## Local LoongArch Smoke

```text
========== START test_clone ==========
  Child says successfully!
clone process successfully.
pid:2
========== END test_clone ==========
========== START test_mmap ==========
========== END test_mmap ==========
========== START test_munmap ==========
========== END test_munmap ==========
[loongarch64-basic] attempted=32 completed=32 failed=none
```

The local LoongArch smoke log contained no `ENOSYS`, `panic`, `Failed to load ELF`, `unsupported`, or `blocker` marker.

## BusyBox Inspection

```text
/musl/busybox: ELF 64-bit LSB executable, LoongArch, statically linked, stripped
size: 2065912 bytes
entry: 0x1201b640c
LOAD 0: vaddr 0x120000000, filesz 0x1f7888
LOAD 1: vaddr 0x1201fbe48, filesz 0x4e2, memsz 0x1d50
```

BusyBox was not enabled. Current blocker: the LoongArch loader is a 128 KiB direct-memory loader for basic-musl PIEs, while BusyBox is a fixed-address ET_EXEC requiring larger image storage and virtual-address mapping support.

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

- Official score needs a Docker-backed rerun.
- BusyBox needs a larger LoongArch user-image loader and virtual-address mapping support for the fixed `0x120000000` ET_EXEC image.
- After BusyBox can load, the runner should execute real commands and emit success lines only after real command exit status is known.
