# Iteration 10 Development Log

## Feature Discovery

Feature: convert the currently passing non-scoring LoongArch BusyBox smoke into a bounded scoring-capable official `busybox-musl` runner for only the safe real commands.

Subsystem ownership:

- `busybox_runner.rs` owns BusyBox command sequencing, argv construction, real PLV3 entry, exit-code capture, official group markers, and official testcase result emission.
- `user.rs` owns per-run and group-active state.
- `syscall.rs` owns LoongArch syscall dispatch and syscall diagnostic suppression.
- `trap.rs` owns trap diagnostics.
- `kernel.rs` owns top-level phase progress prints only.

Existing code searched and reused:

```bash
rg "run_loongarch_busybox_loader_probe|BusyboxCommand|COMMANDS" src/arch/loongarch64
rg "OS COMP TEST GROUP START|testcase busybox|busybox-musl" src/arch/loongarch64 /home/lenovo/oscomp-official-env/autotest-for-oskernel/kernel
rg "consume_syscall_budget|timeout_last_syscall_id|group_active" src/arch/loongarch64
rg "SYS_WRITEV|SYS_READV|SYS_FCNTL|SYS_SENDFILE|getdents64" src/arch/loongarch64
```

Search terms for future agents:

```text
BusyboxCommand
official_name
emit_official_result
set_busybox_group_active
is_any_group_active
scoring-capable group
testcase busybox
command-start
```

## Decisions

A new BusyBox scoring module was considered but not created. `busybox_runner.rs` already owns the BusyBox command list, command execution, and summary, so adding marker and official-result logic there kept the feature discoverable.

A new group-state module was considered but not created. The existing `user.rs` module already owns per-run status and the basic group-active flag, so the BusyBox group-active flag and combined `is_any_group_active()` helper belong there.

Official marker format was checked against the local official judge source. The runner now emits `#### OS COMP TEST GROUP START busybox-musl ####` and `#### OS COMP TEST GROUP END busybox-musl ####`, and only emits `testcase busybox ... success|fail` for real commands with official judge names.

`busybox echo hello` and `busybox cat /musl/busybox_cmd.txt` are still executed as smoke coverage, but they intentionally do not emit official testcase lines because those exact commands are not judge entries.

## Bugs Found And Fixed

The previous official wrapper timeout had no progress context. `kernel.rs` and `busybox_runner.rs` now print concise phase and command progress so a future timeout can be localized to basic phase, BusyBox phase, or a specific command.

The first official rerun failed before evaluation because generated project-root `sdcard-rv.img` and `sdcard-la.img` files from an earlier run blocked Docker's gzip extraction. Those generated raw images were removed from the project root, while the official `.img.gz` files in `/home/lenovo/oscomp-official-env/testdata` were left intact.

Local `make -j16 all` exposed a Rust jobserver warning in the local environment. Final local validation used normal `make all`, and the official wrapper was still run with the requested `CARGO_BUILD_JOBS=16` and `MAKEFLAGS=-j16` environment.

## Guardrails

- No official evaluation scripts were modified.
- No fake BusyBox output was added.
- No hard-coded command success was added.
- `busybox uname` and `busybox ash -c exit` remain disabled.
- `runtime_dispatch.rs` was not changed.
- Existing 32 LoongArch basic-musl cases remained passing locally.
- Generated raw disk images were removed from the project root after official validation.
