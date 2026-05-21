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

## Iteration 03

Local build, image generation, ELF checks, and LoongArch smoke validation passed.

```text
cargo build --target riscv64gc-unknown-none-elf: passed
make all: passed
kernel-rv: RISC-V ELF
kernel-la: LoongArch ELF
kernel-la entry: 0x90000000
LoongArch local smoke: attempted=29 completed=29 failed=none
```

Newly enabled local case markers:

```text
START/END test_exit
START/END test_wait
START/END test_waitpid
START/END test_yield
START/END test_fork
```

The local LoongArch smoke log contained no `ENOSYS`, `panic`, `Failed to load ELF`, `unsupported`, or `blocker` marker.

Official validation was attempted but failed before kernel evaluation because Docker was unavailable:

```text
failed to connect to the docker API at npipe:////./pipe/dockerDesktopLinuxEngine
```

## Iteration 06

Local build, image generation, ELF checks, and LoongArch smoke validation passed.

```text
cargo build --target riscv64gc-unknown-none-elf: passed
make all: passed
kernel-rv: RISC-V ELF
kernel-la: LoongArch ELF
kernel-la entry: 0x90000000
LoongArch local smoke: attempted=32 completed=32 failed=none
```

Newly enabled local case evidence:

```text
START test_clone
  Child says successfully!
clone process successfully.
pid:2
END test_clone
```

The local LoongArch smoke also preserved `START/END test_mmap` and `START/END test_munmap`, and contained no `ENOSYS`, `panic`, `Failed to load ELF`, `unsupported`, or `blocker` marker.

BusyBox inspection found the next loader blocker:

```text
/musl/busybox size: 2065912 bytes
ELF type: EXEC
entry: 0x1201b640c
first LOAD vaddr: 0x120000000
```

Official validation was attempted but failed before kernel evaluation because Docker was unavailable:

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

## Iteration 04

Local build, image generation, ELF checks, and LoongArch smoke validation passed.

```text
cargo build --target riscv64gc-unknown-none-elf: passed
make all: passed
kernel-rv: RISC-V ELF
kernel-la: LoongArch ELF
kernel-la entry: 0x90000000
LoongArch local smoke: attempted=30 completed=30 failed=none
```

Execve evidence:

```text
START test_execve
  I am test_echo.
execve success.
END main
```

The local LoongArch smoke log contained no `ENOSYS`, `panic`, `Failed to load ELF`, `unsupported`, or `blocker` marker.

Official validation was attempted but failed before kernel evaluation because Docker was unavailable:

```text
failed to connect to the docker API at npipe:////./pipe/dockerDesktopLinuxEngine
```

## Iteration 05

Local build, image generation, ELF checks, and LoongArch smoke validation passed.

```text
cargo build --target riscv64gc-unknown-none-elf: passed
make all: passed
kernel-rv: RISC-V ELF
kernel-la: LoongArch ELF
kernel-la entry: 0x90000000
LoongArch local smoke: attempted=31 completed=31 failed=none
```

Pipe evidence:

```text
START test_pipe
cpid: 0
cpid: 2
  Write to pipe successfully.
END test_pipe
```

The local LoongArch smoke log contained no `ENOSYS`, `panic`, `Failed to load ELF`, `unsupported`, or `blocker` marker.

Official validation was attempted but failed before kernel evaluation because Docker was unavailable:

```text
failed to connect to the docker API at npipe:////./pipe/dockerDesktopLinuxEngine
```
