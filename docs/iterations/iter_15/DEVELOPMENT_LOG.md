# Iteration 15 Development Log

## Starting Point

The repository entered this iteration from the stable score-260 BusyBox checkpoint:

```text
LoongArch basic local smoke: attempted=32 completed=32 failed=none
LoongArch BusyBox smoke: completed=7 attempted=7 matched=7 failed=0 disabled=3
Official score: 260
basic-musl-la: 102.0
busybox-musl-la: 5.0
```

The previous scratch-FS and reduced BusyBox probing attempts were not present in the source tree.

## First Actions

The required state-preservation commands were run before source edits:

```text
git status --short
git diff --stat
git diff > /tmp/uestc_kernel_before_change.diff
```

The tree was clean at that point and the saved diff was empty.

## Ownership Decision

Feature being added: non-scoring diagnostics for BusyBox commands that can stall or fault in user mode before clean exit.

Subsystem owners:

- `busybox_runner.rs`: BusyBox diagnostic command mode and reporting.
- `user.rs`: diagnostic active state and observed run counters.
- `trap.rs`: timer interrupt and trap observation.
- `kernel.rs`: cfg-gated top-level diagnostic phase selection.
- `Makefile`: local diagnostic rustc cfg hook.

Existing code searched and reused first:

- BusyBox execution path in `busybox_runner.rs`.
- `real_elf::load_user_elf_with_args` for real `/musl/busybox` loading.
- `trap::enter_user_entry` for PLV3 entry.
- `user::run_snapshot`, `user::start_syscall_budget`, and run-state snapshots.
- `fd_table::set_cwd` and current cwd handling.
- `real_elf::dump_user_regions` for fault diagnostics.

Search terms future agents should use:

```text
loongarch64_busybox_diag
run_loongarch_busybox_diagnostics
run_diagnostic_probe
DIAGNOSTIC_ACTIVE
record_trap_observation
start_diagnostic_timer
diag-ash-exit
```

## Implementation Notes

The `LOONGARCH_RUSTFLAGS` Makefile hook lets local validation build a separate diagnostic kernel:

```text
make KERNEL_LA=kernel-la-diag LOONGARCH_RUSTFLAGS='--cfg loongarch64_busybox_diag' loongarch-kernel
```

Normal builds leave `LOONGARCH_RUSTFLAGS` empty, so the official runner does not execute the diagnostic path.

The first diagnostic build still ran the normal BusyBox smoke before the diagnostic phase. That made diagnosis noisy and reproduced a smoke `cat` stall, so the cfg-gated top-level runner was tightened: diagnostic builds now run the normal LoongArch basic phase, then diagnostic probes only.

Dead-code warnings appeared in the first cfg implementation because normal builds did not use diagnostic-only fields and helpers. The fields and helpers were moved behind `#[cfg(loongarch64_busybox_diag)]` or guarded with `cfg_attr(..., allow(dead_code))` where appropriate. The final normal and diagnostic builds completed without Rust source warnings.

## Diagnostic Findings

In the isolated diagnostic build:

- `basename /aaa/bbb` exited through the normal syscall path with exit code 0.
- `printf "abc\n"` exited through the normal syscall path with exit code 0.
- `uname` exited through the normal syscall path with exit code 0.
- `ash -c exit` faulted at `ERA=0x1201b64b8`, `BADV=0x1201b64b8`, `ECODE=15` after 7 syscalls and 8 traps.

The `ash` fault location sits inside the BusyBox mapped entry range and is now reproducible without enabling it for official scoring.

## Safety Decisions

No diagnostic command was promoted to official BusyBox scoring.

The normal runner continues to expose only the five known-good scoring commands:

```text
true
false
pwd
sh -c exit
ls
```

The disabled official-mode blockers remain disabled:

```text
basename /aaa/bbb
uname
ash -c exit
```

`runtime_dispatch.rs` was not modified.

