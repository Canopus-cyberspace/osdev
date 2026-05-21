# Iteration 01 Test Report

## Local Validation

```text
cargo build --target riscv64gc-unknown-none-elf
make all
file kernel-rv kernel-la
readelf -h kernel-la | sed -n '1,80p'
```

Result:

```text
kernel-rv: ELF 64-bit LSB executable, UCB RISC-V
kernel-la: ELF 64-bit LSB executable, LoongArch
kernel-la entry point: 0x90000000
No compiler warnings in the final local build.
```

## Local LoongArch QEMU Smoke

Command used the official-style LoongArch QEMU command with a local decompressed copy of `sdcard-la.img`.

Result excerpt:

```text
========== START test_mount ==========
mount return: 0
mount successfully
umount return: 0
========== END test_mount ==========
========== START test_umount ==========
mount return: 0
umount success.
return: 0
========== END test_umount ==========
[loongarch64-basic] attempted=24 completed=24 failed=none
```

No local `ENOSYS`, panic marker, timeout, or `Failed to load ELF` appeared in the safe smoke log.

## Official Validation

The official runner did not execute because Docker was unavailable:

```text
failed to connect to the docker API at npipe:////./pipe/dockerDesktopLinuxEngine
```

Latest prior successful official score available locally:

```text
Verdict: Accpted
Score: 217
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 64.0
busybox-musl-la: 0.0
```

This iteration needs an official rerun after Docker is available.

