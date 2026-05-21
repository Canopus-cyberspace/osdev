# Iteration 09 Development Log

## Feature Discovery

Feature: broaden the non-scoring LoongArch BusyBox smoke command set without enabling official BusyBox scoring.

Subsystem ownership:

- `busybox_runner.rs` owns the BusyBox smoke command list, argv construction, per-command entry, exit-code capture, and summary.
- `user.rs` owns per-run status, including the BusyBox syscall-budget timeout state.
- `syscall.rs` owns LoongArch syscall compatibility for real BusyBox execution.
- `fd_table.rs` continues to own fd table state; no fd-table split was needed.
- `kernel.rs` and `trap.rs` were not changed.

Existing code searched and reused:

```bash
rg "run_loongarch_busybox_loader_probe|BusyboxCommand|COMMANDS" src/arch/loongarch64
rg "copy_from_user|copy_to_user|read_user_cstr|read_user_usize" src/arch/loongarch64
rg "handle_loongarch_syscall|SYS_|ENOSYS|ioctl|getuid|getgid|prlimit|access|readlink|getrandom|sysinfo|lseek" src/arch/loongarch64
rg "fd_table|dup|close|open|read|pipe" src/arch/loongarch64
debugfs -R "ls -l /musl" /tmp/sdcard-la-iter08.img
```

Search terms for future agents:

```text
BusyboxCommand
start_syscall_budget
consume_syscall_budget
timeout_last_syscall_id
syscall_writev
syscall_readv
syscall_sendfile
syscall_getdents64
busybox_cmd.txt
sh-exit
```

## Decisions

A new BusyBox smoke module was considered but not created. `busybox_runner.rs` already owns this responsibility, and expanding it kept the command sequencing searchable without spreading probe logic across the architecture.

A new timeout module was not created. The bounded-run state is part of the same per-user-run status already kept in `user.rs`, so the syscall budget was added there and consumed by `syscall.rs`.

Official `busybox-musl` markers remain disabled. Seven real commands now pass locally, but `uname` and `ash` still have blockers, so this is still groundwork rather than official scoring enablement.

## Bugs Found And Fixed

`busybox pwd` used `writev`, which previously fell through to the unsupported syscall path. A minimal `writev` implementation now forwards each iovec to the existing `write` behavior.

`busybox sh -c exit` used signal setup calls. Minimal `rt_sigaction` and `rt_sigprocmask` no-ops were added because no actual signal delivery is available yet and the command only needs compatibility.

`busybox ls` used `fcntl` and then repeatedly called `getdents64`. `fcntl` now returns safe compatibility success, and `getdents64` updates directory offsets so a second call returns EOF instead of replaying the same entries.

`busybox ls` also saw a synthetic `text.txt` entry under `/musl`, then failed `statx`. Directory records are now path-aware for `/`, `/musl`, and other basic directories.

`busybox cat /musl/busybox_cmd.txt` first tried `sendfile`. A minimal regular-file-to-stdout `sendfile` path now copies real sdcard file bytes to the console.

## Blockers

`busybox uname` is disabled because it entered PLV3 and then made no further syscalls before the local QEMU wrapper had to be stopped. The current syscall-budget guard cannot preempt a userspace path that stops before trapping.

`busybox ash -c exit` is disabled because it faulted with:

```text
ecode=15 era=0x1201b64b8 badv=0x1201b64b8
```

Future work should inspect whether that is an instruction permission, TLB, or unsupported instruction issue before enabling `ash`.

## Guardrails

- No official evaluation scripts were modified.
- No fake BusyBox output was added.
- No hard-coded command success text was added.
- No official BusyBox group markers were emitted.
- `runtime_dispatch.rs` was not changed.
- Existing 32 LoongArch basic cases were preserved locally.
