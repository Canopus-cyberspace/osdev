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

## Iteration 13

Local build, image generation, ELF checks, local LoongArch smoke, and official Docker validation passed after reverting the risky scratch-FS attempt.

```text
CPU_COUNT: 16
cargo build -j 16 --target riscv64gc-unknown-none-elf: passed
make -j 16 all: passed with the known local jobserver warning
make all: passed
kernel-rv: RISC-V ELF
kernel-la: LoongArch ELF
LoongArch local basic smoke: attempted=32 completed=32 failed=none
LoongArch local BusyBox smoke: completed=7 attempted=7 matched=7 failed=0 disabled=3
```

Before official validation, four stale containers from `zhouzhouyi/os-contest:20260510` were stopped or removed:

```text
5712a8abb84b
174c3ab496e8
8ea6d6359418
b32969e2cf1c
```

Official validation:

```text
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260521_225549/docker_evaluate.log
docker_evaluate.log size: 798847 bytes
Verdict: Accpted
Score: 260
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 102.0
busybox-musl-la: 5.0
```

Official LoongArch evidence:

```text
[loongarch64-basic] attempted=32 completed=32 failed=none
[loongarch64-busybox] smoke completed=7 attempted=7 matched=7 failed=0 disabled=3
testcase busybox true success
testcase busybox false success
testcase busybox pwd success
testcase busybox sh -c exit success
testcase busybox ls success
```

The official log did not show `Failed to load ELF`, `ENOSYS`, a panic marker, a timeout, or a user fault in enabled commands.

## Iteration 14

Local build, image generation, ELF checks, local LoongArch smoke, and official Docker validation passed after reverting the risky BusyBox scratch-FS and reduced-probe attempts.

```text
CPU_COUNT: 16
cargo build -j 16 --target riscv64gc-unknown-none-elf: passed
make -j 16 all: passed with the known local jobserver warning
make all: passed
kernel-rv: RISC-V ELF
kernel-la: LoongArch ELF
LoongArch local basic smoke: attempted=32 completed=32 failed=none
LoongArch local BusyBox smoke: completed=7 attempted=7 matched=7 failed=0 disabled=3
```

Before official validation, stale containers from `zhouzhouyi/os-contest:20260510` were inspected. None needed removal in this run.

Official validation:

```text
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260521_233219/docker_evaluate.log
docker_evaluate.log size: 798848 bytes
Verdict: Accpted
Score: 260
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 102.0
busybox-musl-la: 5.0
```

Official LoongArch evidence:

```text
[loongarch64-basic] attempted=32 completed=32 failed=none
[loongarch64-busybox] smoke completed=7 attempted=7 matched=7 failed=0 disabled=3
testcase busybox true success
testcase busybox false success
testcase busybox pwd success
testcase busybox sh -c exit success
testcase busybox ls success
```

The official log did not show `Failed to load ELF`, `ENOSYS`, a panic marker, a timeout, or a user fault in enabled commands.

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

## Iteration 09

Local build, image generation, ELF checks, and LoongArch smoke validation passed.

```text
cargo build --target riscv64gc-unknown-none-elf: passed
make all: passed
kernel-rv: RISC-V ELF
kernel-la: LoongArch ELF
kernel-la entry: 0x90000000
LoongArch local smoke: attempted=32 completed=32 failed=none
BusyBox smoke: completed=7 attempted=7 matched=7 failed=0
```

Final BusyBox command evidence:

```text
[loongarch64-busybox] command=true exit_code=0
[loongarch64-busybox] command=false exit_code=1
[loongarch64-busybox] command=echo exit_code=0
[loongarch64-busybox] command=pwd exit_code=0
[loongarch64-busybox] command=sh-exit exit_code=0
[loongarch64-busybox] command=ls exit_code=0
[loongarch64-busybox] command=cat exit_code=0
```

The final local LoongArch smoke log contained no `unsupported`, `missing syscall`, `timeout`, `user fault`, `panic`, `Failed to load ELF`, or `ENOSYS` marker.

Official validation was attempted but timed out before producing Docker log output. The generated latest `docker_evaluate.log` was 0 bytes, so the official score was not refreshed.

## Iteration 10

Local build, image generation, ELF checks, local QEMU smoke, local BusyBox judge parsing, and official validation passed.

```text
CPU_COUNT: 16
cargo build -j 16 --target riscv64gc-unknown-none-elf: passed
make all: passed
kernel-rv: RISC-V ELF
kernel-la: LoongArch ELF
kernel-la entry: 0x90000000
LoongArch local smoke: attempted=32 completed=32 failed=none
BusyBox local smoke: completed=7 attempted=7 matched=7 failed=0
```

Local `make -j16 all` exposed a jobserver warning in the local Rust environment, so final local validation used normal `make all`. The official wrapper was run with `CARGO_BUILD_JOBS=16` and `MAKEFLAGS=-j16`; official QEMU still used `-smp 1`.

Official validation initially hit stale generated project-root raw images:

```text
gzip: sdcard-rv.img already exists; not overwritten
```

Only generated project-root `sdcard-rv.img` and `sdcard-la.img` were removed. The official `.img.gz` testdata was left intact. The rerun completed:

```text
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260521_173700/docker_evaluate.log
Verdict: Accpted
Score: 256
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 98.0
busybox-musl-la: 5.0
```

LoongArch BusyBox official testcase lines observed:

```text
testcase busybox true success
testcase busybox false success
testcase busybox pwd success
testcase busybox sh -c exit success
testcase busybox ls success
```

The official log contained no `Failed to load ELF`, `panic`, `timeout`, `ENOSYS`, or `user fault` marker.

## Iteration 11

Local build, image generation, ELF checks, local QEMU smoke, and official validation passed.

```text
CPU_COUNT: 16
cargo build -j 16 --target riscv64gc-unknown-none-elf: passed
make -j 16 all: passed with local jobserver warning
make all: passed without warnings
kernel-rv: RISC-V ELF
kernel-la: LoongArch ELF
kernel-la entry: 0x90000000
```

Local smoke evidence:

```text
waitpid successfully.
wstatus: 3
[loongarch64-basic] attempted=32 completed=32 failed=none
[loongarch64-busybox] smoke completed=7 attempted=7 matched=7 failed=0
```

Official validation:

```text
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260521_180525/docker_evaluate.log
Verdict: Accpted
Score: 260
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 102.0
busybox-musl-la: 5.0
```

The previous gap was `test_waitpid`, which changed from LoongArch `0/4` to `4/4`. The official log contained no `Failed to load ELF`, `panic`, `timeout`, `ENOSYS`, or `user fault` marker.

## Iteration 12

Local build, image generation, ELF checks, local QEMU smoke, and official validation passed.

```text
CPU_COUNT: 16
cargo build -j 16 --target riscv64gc-unknown-none-elf: passed
make -j 16 all: passed with local jobserver warning
make all: passed
kernel-rv: RISC-V ELF
kernel-la: LoongArch ELF
```

Local smoke evidence:

```text
[loongarch64-basic] attempted=32 completed=32 failed=none
[loongarch64-busybox] smoke completed=7 attempted=7 matched=7 failed=0 disabled=3
```

LoongArch BusyBox official testcase lines remained limited to:

```text
testcase busybox true success
testcase busybox false success
testcase busybox pwd success
testcase busybox sh -c exit success
testcase busybox ls success
```

Official validation:

```text
final refresh log: /home/lenovo/oscomp-official-env/logs/evaluate_20260521_214042/docker_evaluate.log
final refresh result: timeout guard exited 124 after 30 minutes
final refresh docker_evaluate.log size: 0 bytes
```

Latest completed official baseline:

```text
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260521_213325/docker_evaluate.log
Verdict: Accpted
Score: 260
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 102.0
busybox-musl-la: 5.0
```

Generated project-root sdcard artifacts were removed after validation. Official testdata images were not touched.

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

## Iteration 07

Local build, image generation, ELF checks, and LoongArch smoke validation passed.

```text
cargo build --target riscv64gc-unknown-none-elf: passed
make all: passed
kernel-rv: RISC-V ELF
kernel-la: LoongArch ELF
kernel-la entry: 0x90000000
LoongArch local smoke: attempted=32 completed=32 failed=none
```

BusyBox loader probe evidence:

```text
[loongarch64-busybox] loaded /musl/busybox file_size=2065912 entry=0x901ca40c first_load=0x120000000 load_size=2087832 segments=2
[loongarch64-busybox] entering command=true
[loongarch64-busybox] blocker: user fault ecode=8 era=0x9019d85c badv=0x90016138
```

No BusyBox command success or official `busybox-musl` group marker was emitted. The local LoongArch smoke log contained no `ENOSYS`, `panic`, or `Failed to load ELF` marker.

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

## Iteration 08

Local build, image generation, ELF checks, and LoongArch smoke validation passed.

```text
cargo build --target riscv64gc-unknown-none-elf: passed
make all: passed
kernel-rv: RISC-V ELF
kernel-la: LoongArch ELF
kernel-la entry: 0x90000000
LoongArch local smoke: attempted=32 completed=32 failed=none
```

The real BusyBox probe now exits successfully through PLV3:

```text
[loongarch64-busybox] loaded /musl/busybox file_size=2065912 entry=0x1201b640c first_load=0x120000000 load_size=2087832 segments=2 command=true
[loongarch64-busybox] mapped entry=0x1201b640c
[loongarch64-busybox] command=true exit_code=0
[loongarch64-busybox] smoke completed=1 attempted=1
```

The final local LoongArch smoke log contained no `ENOSYS`, `panic`, `Failed to load ELF`, `user fault`, or `missing syscall` marker.

Official validation was attempted but failed before kernel evaluation because Docker was unavailable:

```text
failed to connect to the docker API at npipe:////./pipe/dockerDesktopLinuxEngine
```
