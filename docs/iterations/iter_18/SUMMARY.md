# Iteration 18 Summary

## Goal

Add minimal read-only `/proc` and kernel-info compatibility for direct LoongArch BusyBox commands without touching scratch files, redirection, pipelines, or file-write paths.

The verified official baseline entering the iteration was:

```text
Official score: 270
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 102.0
busybox-musl-la: 15.0
LoongArch basic local smoke: attempted=32 completed=32 failed=none
LoongArch BusyBox local smoke: completed=17 attempted=17 matched=17 failed=0 disabled=8
```

## Outcome

Added read-only virtual `/proc` entries in the LoongArch fd path and narrow syscall compatibility for direct/read-only BusyBox applets.

New locally passing BusyBox scoring commands:

- `dmesg`
- `df`
- `ps`
- `free`

These commands run the real `/musl/busybox` ELF in PLV3 and emit official testcase lines only after exiting with code 0.

Still disabled:

- `ash -c exit`: known user fault from previous diagnostics.
- `which ls`: prior probe returned exit code 1.
- `hwclock`: `/dev/misc/rtc` is still unavailable.
- `sleep 1`: previous user fault.

## Module Placement

`fd_table.rs` owns the virtual read-only `/proc` exposure because open/read/stat/getdents already route virtual file and directory paths through this module.

`syscall.rs` owns narrow syscall compatibility needed by these read-only commands:

- `lseek`
- `statfs`
- `fstatfs`
- `sysinfo`
- `syslog`

`busybox_runner.rs` owns only command classification and promotion.

`linker.ld` now pins the LoongArch `.user` section at `0x90010000` before `.rodata`. Local probing showed that adding proc strings could otherwise move the user-return stub and destabilize existing BusyBox commands. This is LoongArch-local layout stabilization and does not affect RISC-V.

A new source file was considered, but the final implementation stayed in the existing fd/syscall owners because the feature is small and directly extends their existing virtual path hooks.

Existing helpers reused:

- `fd_table::is_virtual_file_path`
- `fd_table::read_virtual_file`
- `fd_table::virtual_file_size`
- `fd_table::is_virtual_dir_path`
- `syscall_getdents64`
- `write_stat`
- `write_statx`
- `write_dirent64`
- `user_mem::copy_to_user`
- `emit_official_result`

New helpers added:

- `fd_table::is_proc_dir_path`
- `fd_table::proc_file_content`
- `syscall_lseek`
- `syscall_statfs`
- `syscall_fstatfs`
- `syscall_sysinfo`
- `syscall_syslog`
- `write_statfs`

Future search terms:

```text
PROC_MEMINFO
PROC_MOUNTS
PROC_PID_STAT
syscall_statfs
syscall_syslog
testcase busybox free success
testcase busybox df success
testcase busybox ps success
testcase busybox dmesg success
```

## Validation

Local validation:

```text
CPU_COUNT: 16
cargo build -j 16 --target riscv64gc-unknown-none-elf: passed
make -j 16 all: passed with the known local jobserver warning
make all: passed
kernel-rv: RISC-V ELF
kernel-la: LoongArch ELF
LoongArch local basic: attempted=32 completed=32 failed=none
LoongArch local BusyBox: completed=21 attempted=21 matched=21 failed=0 disabled=4
```

Official validation was attempted but did not complete:

```text
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260522_180412/docker_evaluate.log
docker_evaluate.log size: 0 bytes
result: outer timeout after 30 minutes
```

The latest completed official validation remains:

```text
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260522_144845/docker_evaluate.log
Verdict: Accpted
Score: 270
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 102.0
busybox-musl-la: 15.0
```

