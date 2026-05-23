# Iteration 15 Test Report

## Build Environment

```text
CPU_COUNT: 16
```

Parallel build preparation was used for local builds. The official wrapper was run conservatively after unsetting `MAKEFLAGS`, `MFLAGS`, and `CARGO_BUILD_JOBS`.

## Commands Run

```text
cargo build -j 16 --target riscv64gc-unknown-none-elf
make -j 16 all
make all
file kernel-rv kernel-la
```

Results:

```text
cargo build -j 16 --target riscv64gc-unknown-none-elf: passed
make -j 16 all: passed with the known local jobserver warning
make all: passed
kernel-rv: ELF 64-bit LSB executable, UCB RISC-V
kernel-la: ELF 64-bit LSB executable, LoongArch
```

The final normal `make all` source logs did not contain Rust warnings.

## Diagnostic Build

Command:

```text
make KERNEL_LA=kernel-la-diag LOONGARCH_RUSTFLAGS='--cfg loongarch64_busybox_diag' loongarch-kernel
```

Result:

```text
diagnostic kernel build: passed
```

The diagnostic kernel was removed after testing and is not stored in the repository.

## Local Normal Smoke

A fresh LoongArch sdcard image was generated from official testdata into `/tmp` for local smoke testing and was not stored in the repository.

Result:

```text
qemu_status=0
[loongarch64-basic] attempted=32 completed=32 failed=none
[loongarch64-busybox] smoke completed=7 attempted=7 matched=7 failed=0 disabled=3
```

Known-good BusyBox scoring lines remained present:

```text
testcase busybox true success
testcase busybox false success
testcase busybox pwd success
testcase busybox sh -c exit success
testcase busybox ls success
```

The normal local smoke contained no `Failed to load ELF`, `ENOSYS`, panic marker, user fault, or timeout marker.

## Local Diagnostic Smoke

Result:

```text
qemu_status=0
[loongarch64-basic] attempted=32 completed=32 failed=none
```

Diagnostic command summaries:

```text
[loongarch64-busybox-diag] result command=diag-basename status=exited exit_code=0 entry=0x1201b640c last_era=0x1201aa0bc last_badv=0x1201aa0bc last_ecode=11 syscalls=4 traps=4 same_era=0 timer_ticks=0 stopped_before_syscall=no last_syscall=94
[loongarch64-busybox-diag] result command=diag-printf status=exited exit_code=0 entry=0x1201b640c last_era=0x1201aa0bc last_badv=0x1201aa0bc last_ecode=11 syscalls=6 traps=6 same_era=0 timer_ticks=0 stopped_before_syscall=no last_syscall=94
[loongarch64-busybox-diag] result command=diag-uname status=exited exit_code=0 entry=0x1201b640c last_era=0x1201aa0bc last_badv=0x1201aa0bc last_ecode=11 syscalls=6 traps=6 same_era=0 timer_ticks=0 stopped_before_syscall=no last_syscall=94
[loongarch64-busybox-diag] result command=diag-ash-exit status=fault entry=0x1201b640c last_era=0x1201b64b8 last_badv=0x1201b64b8 last_ecode=15 syscalls=7 traps=8 same_era=0 timer_ticks=0 stopped_before_syscall=no last_syscall=222
```

The diagnostic build did not emit official BusyBox testcase success or fail lines.

## Container Cleanup

Before official validation, stale official containers were inspected and removed if present:

```text
docker ps -a --filter ancestor=zhouzhouyi/os-contest:20260510 --format '{{.ID}} {{.Status}} {{.Names}}'
docker ps --filter ancestor=zhouzhouyi/os-contest:20260510 -q | xargs -r docker stop
docker ps -a --filter ancestor=zhouzhouyi/os-contest:20260510 -q | xargs -r docker rm
docker ps
ps aux | grep -E 'qemu|run_official|cargo|make' | grep -v grep || true
```

No stale official containers needed removal.

## Official Validation

Command:

```text
unset MAKEFLAGS
unset MFLAGS
unset CARGO_BUILD_JOBS
timeout 30m bash /home/lenovo/oscomp-official-env/run_official_autotest.sh \
  /home/lenovo/oscomp-official-env \
  /home/lenovo/projects/uestc-kernel
```

The first official attempt completed with a transient LoongArch virtio failure:

```text
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260521_235738/docker_evaluate.log
docker_evaluate.log size: 380083 bytes
[loongarch64-basic] blocker: failed to load /musl/basic/write: virtio_pci_timeout
[loongarch64-busybox] blocker: failed to load /musl/busybox: virtio_pci_status
```

Generated root sdcard artifacts were removed and the normal official validation was rerun.

Final official result:

```text
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260521_235948/docker_evaluate.log
docker_evaluate.log size: 798847 bytes
Verdict: Accpted
Score: 260
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 102.0
busybox-musl-la: 5.0
```

Final official LoongArch evidence:

```text
[loongarch64-basic] attempted=32 completed=32 failed=none
[loongarch64-busybox] smoke completed=7 attempted=7 matched=7 failed=0 disabled=3
testcase busybox true success
testcase busybox false success
testcase busybox pwd success
testcase busybox sh -c exit success
testcase busybox ls success
```

No diagnostic lines appeared in the official log. The final official log did not show `Failed to load ELF`, `ENOSYS`, a panic marker, a user fault in enabled commands, or a timeout.

