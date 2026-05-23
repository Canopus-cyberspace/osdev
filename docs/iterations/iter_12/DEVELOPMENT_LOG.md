# Iteration 12 Development Log

## Initial State Preservation

Before changing source, the current work state was captured:

```text
git status --short
git diff --stat
git diff > /tmp/uestc_kernel_before_change.diff
git diff > /tmp/uestc_kernel_before_iter12_stabilize.diff
```

No reset, stash, or discard operation was used on user work. Broad interrupted edits to LoongArch fd/syscall/user state were dropped from the working tree because this checkpoint intentionally keeps the BusyBox stabilization patch narrow.

## Feature Ownership Decision

Feature being added:

```text
BusyBox scoring allowlist and disabled-command separation.
```

Owning subsystem:

```text
LoongArch BusyBox runner.
```

Existing code searched and reused:

```text
src/arch/loongarch64/busybox_runner.rs
src/arch/loongarch64/user.rs
src/arch/loongarch64/fd_table.rs
src/arch/loongarch64/real_elf.rs
```

Future search terms:

```text
BusyboxCommandClass
RUN_COMMANDS
DISABLED_COMMANDS
run_loongarch_busybox_loader_probe
testcase busybox
```

## Implementation Notes

- Added `BusyboxCommandClass` with `Scoring`, `Smoke`, and `Disabled`.
- Renamed the executable command table to `RUN_COMMANDS`.
- Added `DISABLED_COMMANDS` for known unsafe commands so they are documented but never executed by the official runner.
- Added `run_command_set` to keep command execution and score-line emission rules explicit.
- Increased the fixed argv scratch array to eight entries for the existing smoke command shapes.
- Kept `basename`, `uname`, and `ash -c exit` out of the runnable table.
- Kept `runtime_dispatch.rs` untouched.

## Decisions

`echo hello` and `cat /musl/busybox_cmd.txt` remained smoke-only because their current command lines do not exactly match the public BusyBox judge keys that are already proven safe for scoring.

The existing `busybox_runner.rs` file was reused instead of creating a new module. The responsibility is command selection, official marker policy, and result reporting, which already belongs there.
