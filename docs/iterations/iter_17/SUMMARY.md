# Iteration 17 Summary

## Goal

Continue the low-risk LoongArch BusyBox expansion path by promoting only direct or read-only applets that run as real `/musl/busybox` PLV3 commands.

The baseline entering the iteration was:

```text
Official score: 269
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 102.0
busybox-musl-la: 14.0
LoongArch basic local smoke: attempted=32 completed=32 failed=none
LoongArch BusyBox local smoke: completed=16 attempted=16 matched=16 failed=0 disabled=4
```

## Outcome

`src/arch/loongarch64/busybox_runner.rs` now promotes one additional real BusyBox direct applet:

- `du`

The command runs the real `/musl/busybox` ELF in PLV3 from `sdcard-la.img` and emits the official testcase line only after exiting with code 0.

The following candidates were probed but kept disabled:

- `dmesg`: exits 1 because `klogctl` is not implemented.
- `df`: exits 1 because `/proc/mounts` is unavailable.
- `ps`: exits 1 because `/proc` is unavailable.
- `hwclock`: exits 1 because `/dev/misc/rtc` is unavailable.

No syscall, fd-table, scratch-FS, redirection, pipeline, shell-complex, or `runtime_dispatch.rs` changes were shipped.

## Module Placement

The source change belongs in `busybox_runner.rs` because that module owns BusyBox command classes, official markers, per-command execution, expected-exit matching, and result reporting.

A new source file was considered unnecessary: this iteration only changes command descriptors and disabled-candidate documentation. No new helper was added.

Existing helpers reused:

- `real_elf::load_user_elf_with_args`
- `real_elf::activate_current_user_mmu`
- `trap::enter_user_entry`
- `user::reset_case_state`
- `user::start_syscall_budget`
- `user::run_snapshot`
- `fd_table::set_cwd`
- `emit_official_result`

Future search terms:

```text
BusyboxCommandClass::Scoring
BusyboxCommandClass::Disabled
testcase busybox du success
disabled=8
dmesg
df
ps
hwclock
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
LoongArch local BusyBox: completed=17 attempted=17 matched=17 failed=0 disabled=8
```

Official validation:

```text
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260522_144845/docker_evaluate.log
docker_evaluate.log size: 803263 bytes
Verdict: Accpted
Score: 270
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 102.0
busybox-musl-la: 15.0
```

