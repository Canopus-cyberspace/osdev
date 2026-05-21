# Iteration 12 Test Report

## Build Environment

```text
CPU_COUNT: 16
```

Parallel build preparation was used where safe.

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

## Local LoongArch Smoke

Command:

```text
qemu-system-loongarch64 -kernel kernel-la -m 1G -nographic -smp 1 \
  -drive file=/tmp/sdcard-la-iter08.img,if=none,format=raw,id=x0 \
  -device virtio-blk-pci,drive=x0 -no-reboot \
  -device virtio-net-pci,netdev=net0 -netdev user,id=net0 -rtc base=utc
```

Result:

```text
qemu_status=0
[loongarch64-basic] attempted=32 completed=32 failed=none
[loongarch64-busybox] smoke completed=7 attempted=7 matched=7 failed=0 disabled=3
```

LoongArch BusyBox scoring lines remained limited to:

```text
testcase busybox true success
testcase busybox false success
testcase busybox pwd success
testcase busybox sh -c exit success
testcase busybox ls success
```

Smoke-only real commands also completed:

```text
busybox echo hello
busybox cat /musl/busybox_cmd.txt
```

Disabled commands were not executed in LoongArch official mode:

```text
basename /aaa/bbb
uname
ash -c exit
```

The local LoongArch smoke contained no `ENOSYS`, `panic`, `user fault`, `Failed to load ELF`, missing-syscall, or command-timeout marker.

## Official Validation

Command:

```text
export CARGO_BUILD_JOBS=16
export MAKEFLAGS=-j16
timeout 30m bash /home/lenovo/oscomp-official-env/run_official_autotest.sh \
  /home/lenovo/oscomp-official-env \
  /home/lenovo/projects/uestc-kernel
```

Final refresh attempt:

```text
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260521_214042/docker_evaluate.log
result: timeout guard exited 124 after 30 minutes
docker_evaluate.log size: 0 bytes
```

Because the final official wrapper run produced no Docker evaluation output, no new official score was claimed for the tightened seven-command smoke code.

Latest completed official validation before the final timeout:

```text
/home/lenovo/oscomp-official-env/logs/evaluate_20260521_213325/docker_evaluate.log
```

Score summary:

```text
Verdict: Accpted
Score: 260
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 102.0
busybox-musl-la: 5.0
```

The final local LoongArch runner evidence for the tightened code remains:

```text
[loongarch64-basic] attempted=32 completed=32 failed=none
[loongarch64-busybox] smoke completed=7 attempted=7 matched=7 failed=0 disabled=3
```

Generated project-root sdcard artifacts were removed after validation attempts; official testdata images under `/home/lenovo/oscomp-official-env/testdata/` were not touched.
