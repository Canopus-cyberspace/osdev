# Test Report

## Iteration 01

Local build and LoongArch smoke validation passed.

```text
cargo build --target riscv64gc-unknown-none-elf: passed
make all: passed
kernel-rv: RISC-V ELF
kernel-la: LoongArch ELF
LoongArch local smoke: attempted=24 completed=24 failed=none
```

Official validation was blocked by Docker connectivity:

```text
failed to connect to the docker API at npipe:////./pipe/dockerDesktopLinuxEngine
```

## Iteration 02

Local build, image generation, ELF checks, and LoongArch smoke validation passed.

```text
cargo build --target riscv64gc-unknown-none-elf: passed
make all: passed
kernel-rv: RISC-V ELF
kernel-la: LoongArch ELF
kernel-la entry: 0x90000000
LoongArch local smoke: attempted=24 completed=24 failed=none
```

The local LoongArch smoke log contained no `ENOSYS`, `panic`, `Failed to load ELF`, `unsupported`, or `blocker` marker.

Official validation was attempted but did not reach kernel evaluation because Docker was unavailable:

```text
failed to connect to the docker API at npipe:////./pipe/dockerDesktopLinuxEngine
```

The latest prior successful official score remains:

```text
Verdict: Accpted
Score: 217
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 64.0
busybox-musl-la: 0.0
```
