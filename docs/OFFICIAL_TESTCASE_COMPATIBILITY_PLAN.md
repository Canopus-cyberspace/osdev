# Official Testcase Compatibility Plan

## Goal

Move from a clean RISC-V official QEMU boot/shutdown to a nonzero official RISC-V score without faking PASS markers, editing judge outputs, or pretending LoongArch support exists.

The next changes should make `kernel-rv` execute real official test content from `sdcard-rv.img` and emit official serial output that the existing judge scripts can parse.

## Highest Priority Target

The first scoring target should be `basic-musl-rv`.

Reasons:

- It is syscall-focused and closest to the existing v194 userland work.
- It has simple per-test scoring, so partial progress can produce nonzero score.
- Its expected output is deterministic and easy to validate offline with the official judge.
- Musl is a better first target than glibc because glibc dynamic loader compatibility adds another layer.

The first nonzero target can be one or more real programs from:

```text
/musl/basic/write
/musl/basic/getpid
/musl/basic/uname
/musl/basic/open
/musl/basic/read
```

These should be run through the real user execution path wherever possible.

## Minimal Next Kernel Capabilities

To score official RISC-V tests, the kernel needs these pieces in order:

1. Official-mode boot path
   - Preserve all current v151k7 through v194 runtime markers.
   - Preserve the current clean official QEMU shutdown behavior for failure paths.
   - Add clear diagnostics when official image discovery or program execution fails.

2. Virtio-mmio block discovery and reads
   - Under official RISC-V QEMU, the sdcard is attached as:

```text
-drive file=sdcard-rv.img,if=none,format=raw,id=x0
-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0
```

   - The kernel should either scan the device tree or probe the known virtio-mmio device layout for `virt`.
   - Initial implementation can be read-only.

3. Raw ext4 rootfs reader
   - The sdcard image is a raw ext4 filesystem, not a partitioned disk.
   - The kernel should mount/read the root and find:

```text
/musl
/glibc
```

   - Initial implementation only needs read-only lookup and file reads for the first basic binaries and support files.

4. Official test script or suite launcher
   - Preferred path: run `/musl/basic_testcode.sh` through shell support once BusyBox/shell is available.
   - Earlier path: a kernel-side official launcher may directly execute real `/musl/basic/<test>` binaries and frame their output with the same group boundaries that the official script would print.
   - The launcher must not fabricate test success. It should only frame and report output from genuine program execution or genuine syscall behavior.

5. ELF loading from sdcard files
   - Load real RISC-V ELF binaries from `/musl/basic`.
   - Build a Linux-compatible initial user stack with `argc`, `argv`, `envp`, and auxv enough for the target binary.
   - Support file-backed program reads from ext4 rather than embedded rootfs only.

6. Basic process and fd environment
   - Provide cwd support, stdin/stdout/stderr, path lookup, and enough fd semantics for the first basic tests.
   - Preserve existing fork/exec/wait and runtime syscall paths from v193-v194.

## Recommended Implementation Order

1. Add official image diagnostics only
   - Boot under the official QEMU command.
   - Print whether the virtio block device is visible.
   - If unavailable, print a clear serial error and shut down.

2. Add read-only virtio block and ext4 root listing
   - Read the ext4 superblock from `sdcard-rv.img`.
   - List `/`, `/musl`, and `/musl/basic`.
   - Serial diagnostics should identify missing files explicitly.

3. Run one real basic-musl binary
   - Start with `/musl/basic/write`.
   - Emit:

```text
#### OS COMP TEST GROUP START basic-musl ####
```

   - Execute the real binary and relay its serial output.
   - Emit:

```text
#### OS COMP TEST GROUP END basic-musl ####
```

   - Validate offline by feeding the captured group into `judge_basic-musl.py`.

4. Expand basic-musl short syscall set
   - `write`
   - `getpid`
   - `getppid`
   - `uname`
   - `getcwd`
   - `close`
   - `dup`
   - `dup2`
   - `open`
   - `read`
   - `openat`
   - `fstat`
   - `getdents`

5. Expand process and memory tests
   - `brk`
   - `mmap`
   - `munmap`
   - `fork`
   - `clone`
   - `execve`
   - `wait`
   - `waitpid`
   - `pipe`
   - `yield`
   - `sleep`
   - `times`

6. Add mount-related compatibility
   - `mkdir_`
   - `unlink`
   - `mount`
   - `umount`
   - Keep this grounded in real VFS/mount behavior.

7. Move to BusyBox musl
   - Run `/musl/busybox_testcode.sh` or equivalent real command execution.
   - This needs shell-like argv handling, cwd, redirects, pipes, file utilities, and process lifecycle robustness.

8. Add Lua musl and libctest static coverage
   - Lua provides another deterministic functional suite.
   - Libctest static cases are a good ABI hardening target after basic syscall behavior stabilizes.

9. Add glibc support
   - Load `/glibc/lib/ld-linux-riscv64-lp64d.so.1`.
   - Support dynamic linking expectations, auxv entries, TLS, and glibc startup assumptions.
   - Then repeat basic, BusyBox, Lua, and libctest glibc.

10. Add LTP and performance suites
   - LTP requires broad Linux ABI behavior and accurate error semantics.
   - iozone, lmbench, libcbench, cyclictest, iperf, and netperf require performance, timing, networking, and scheduler stability.

11. Add real LoongArch support if required
   - Current `kernel-la` is not a valid LoongArch kernel.
   - Do not claim LoongArch compatibility until a real LoongArch boot path, linker, entry, traps, console, and drivers exist.

## Suite Priority

Recommended scoring priority:

1. `basic-musl-rv`
2. `busybox-musl-rv`
3. `lua-musl-rv`
4. `libctest-musl-rv`
5. `basic-glibc-rv`
6. `busybox-glibc-rv`
7. `lua-glibc-rv`
8. `libctest-glibc-rv`
9. `ltp-musl-rv`
10. `ltp-glibc-rv`
11. `libcbench-*`
12. `iozone-*`
13. `lmbench-*`
14. `cyclictest-*`
15. `iperf-*`
16. `netperf-*`
17. `*-la` once real LoongArch support exists

## Validation Strategy

Each implementation step should be validated in three layers:

1. Build validation

```bash
cargo build --target riscv64gc-unknown-none-elf
make all
```

2. Local official QEMU validation

```bash
timeout 20m qemu-system-riscv64 -machine virt -kernel kernel-rv -m 1G -nographic -smp 1 -bios default -drive file=sdcard-rv.img,if=none,format=raw,id=x0 -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 -no-reboot -device virtio-net-device,netdev=net -netdev user,id=net -rtc base=utc
```

3. Official wrapper validation

```bash
cd /mnt/c/Users/lenovo/Downloads/oscomp_env_setup/v214_official_oscomp_env_setup_with_bat
timeout 20m ./scripts/run_official_autotest.sh /home/lenovo/oscomp-official-env /home/lenovo/projects/uestc-kernel
```

After each official run, inspect:

```text
/home/lenovo/oscomp-official-env/testdata/console_log
/home/lenovo/oscomp-official-env/logs/evaluate_*/docker_evaluate.log
os_serial_out_rv.txt
os_serial_out_la.txt
```

The important acceptance signal for the next batch is not the wrapper verdict string. It is a nonzero `*-rv` rank entry caused by genuine official test output.

## Offline Judge Checks

Before running the full official wrapper, captured RISC-V group output can be tested directly with the relevant judge:

```bash
python3 /home/lenovo/oscomp-official-env/autotest-for-oskernel/kernel/judge/judge_basic-musl.py < captured_basic_musl_group.txt
```

The captured file should contain only the lines between the official group start and end markers, matching what `run.py` passes to the judge.

## Guardrails

Do not print official success text unless it is produced by real official program execution or by a real compatibility path exercising the same behavior.

Do not edit official judge scripts, logs, or score output to obtain a score.

Do not remove or alter the existing v151k7 through v194 runtime markers.

Do not turn `kernel-la` into another RISC-V artifact and describe it as LoongArch support.

Do not introduce build warnings matching:

```text
matches any value
unreachable pattern
warning: unused variable:
```

