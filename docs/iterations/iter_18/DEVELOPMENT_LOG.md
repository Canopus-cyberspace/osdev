# Iteration 18 Development Log

## Starting Point

The iteration began from the score-270 BusyBox baseline. The required before-change state preservation was run:

```text
git status --short
git diff --stat
git diff > /tmp/uestc_kernel_before_change.diff
```

The repository was clean at the start of the iteration.

## Feature Ownership

Feature added: minimal read-only `/proc` and kernel-info compatibility for direct BusyBox commands.

Subsystem owners:

- `fd_table.rs`: virtual path exposure for `/proc` files/directories.
- `syscall.rs`: narrow compatibility syscalls.
- `busybox_runner.rs`: command promotion only after real execution.
- `linker.ld`: LoongArch-local section stability for the `.user` stub.

No `runtime_dispatch.rs` change was made.

## Implementation

`fd_table.rs` now recognizes:

- `/proc`
- `/proc/meminfo`
- `/proc/mounts`
- `/proc/stat`
- `/proc/uptime`
- `/proc/filesystems`
- `/proc/cpuinfo`
- `/proc/1`
- `/proc/1/stat`
- `/proc/1/status`
- `/proc/1/cmdline`
- `/proc/self`
- `/proc/self/stat`
- `/proc/self/status`
- `/proc/self/cmdline`

These entries are read-only and use the existing virtual-file `open`, `read`, `fstat`, `statx`, and `getdents64` paths.

`syscall.rs` adds:

- `lseek` for regular files and virtual read-only files.
- `statfs` and `fstatfs` with a minimal static filesystem shape.
- `sysinfo` with plausible memory totals.
- `syslog` with empty log semantics for BusyBox `dmesg`.

`busybox_runner.rs` promotes:

- `dmesg`
- `df`
- `ps`
- `free`

## Layout Finding

The first proc implementation moved `.user` from `0x90010000` to `0x90011000` because `.rodata` grew past the previous alignment boundary. Local BusyBox runs then stalled before reaching the new proc probes. `linker.ld` now fixes `.user` at `0x90010000` and places `.rodata` after `.user`, preserving the existing user-return layout while allowing small read-only kernel metadata.

## Validation Notes

After removing stale temporary 4 GiB sdcard images from `/tmp`, local QEMU completed:

```text
[loongarch64-basic] attempted=32 completed=32 failed=none
[loongarch64-busybox] smoke completed=21 attempted=21 matched=21 failed=0 disabled=4
testcase busybox dmesg success
testcase busybox df success
testcase busybox ps success
testcase busybox free success
```

An official validation attempt timed out at the outer 30-minute wrapper and produced a 0-byte `docker_evaluate.log`. The stale Docker container and generated root sdcard images were cleaned up afterward.

