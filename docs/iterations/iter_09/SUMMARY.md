# Iteration 09 Summary

## Focus

This iteration broadened the non-scoring LoongArch BusyBox smoke after fixed-address ET_EXEC mapping started working.

BusyBox scoring remains disabled. The runner still does not emit official `busybox-musl` group markers, and every enabled command is the real `/musl/busybox` ELF loaded from `sdcard-la.img` and entered in PLV3.

## Source Changes

- Reused `src/arch/loongarch64/busybox_runner.rs` for the BusyBox smoke command list and per-command reporting.
- Reused `src/arch/loongarch64/user.rs` for per-run status and added a BusyBox-only syscall budget escape hatch.
- Reused `src/arch/loongarch64/syscall.rs` for narrow BusyBox compatibility syscalls:
  - `writev`
  - `readv`
  - `fcntl`
  - `sendfile`
  - `rt_sigaction`
  - `rt_sigprocmask`
- Reused existing `fd_table.rs` state while fixing `getdents64` EOF behavior in `syscall.rs`.

No new source module was needed. The new behavior belongs to the existing BusyBox runner, per-run user state, and LoongArch syscall compatibility owner.

## BusyBox Smoke Result

Final local smoke:

```text
[loongarch64-busybox] command=true exit_code=0
[loongarch64-busybox] command=false exit_code=1
[loongarch64-busybox] command=echo exit_code=0
[loongarch64-busybox] command=pwd exit_code=0
[loongarch64-busybox] command=sh-exit exit_code=0
[loongarch64-busybox] command=ls exit_code=0
[loongarch64-busybox] command=cat exit_code=0
[loongarch64-busybox] smoke completed=7 attempted=7 matched=7 failed=0
```

Observed real command output included:

```text
hello
/musl
basic
busybox
busybox_cmd.txt
```

## Deferred Commands

- `busybox uname` was attempted during development and entered PLV3, then stopped before the next syscall. It is disabled to preserve the no-hang rule.
- `busybox ash -c exit` was attempted and returned a controlled user fault at `era=0x1201b64b8`, `badv=0x1201b64b8`. It is disabled until the instruction/fault cause is understood.

## Result

Existing LoongArch basic-musl coverage stayed stable in local QEMU:

```text
[loongarch64-basic] attempted=32 completed=32 failed=none
```

The final local smoke log contained no `unsupported`, `missing syscall`, `timeout`, `user fault`, `panic`, `Failed to load ELF`, or `ENOSYS` marker.

Official validation was attempted but timed out before producing Docker log output, so the official score was not refreshed.
