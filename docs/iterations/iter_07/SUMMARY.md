# Iteration 07 Summary

## Focus

This iteration upgraded the LoongArch user ELF loader and user memory model enough to load the real `/musl/busybox` payload from `sdcard-la.img` and enter it under PLV3 without hanging.

BusyBox scoring remains disabled. The probe emits only diagnostic blocker text after the real loader/user-entry attempt; it does not emit `busybox-musl` group markers or command success output.

## Source Changes

- Added `src/arch/loongarch64/busybox_runner.rs` as a focused non-scoring BusyBox loader probe.
- Reused `src/arch/loongarch64/real_elf.rs` for LoongArch user image ownership and expanded it from a small direct basic-loader into a multi-region model.
- Extended LoongArch user-copy translation so `copy_to_user`, `copy_from_user`, `read_user_cstr`, and usize write helpers walk registered user regions instead of directly trusting a single contiguous buffer.
- Taught the ELF loader to parse all `PT_LOAD` segments, support the larger BusyBox payload, track multiple image/stack/heap/mmap regions, and keep fork/exec snapshots consistent.
- Added small LoongArch syscall compatibility hooks for early BusyBox probing, including `newfstatat`, `faccessat`, `readlinkat`, ids, tid, robust-list, and rlimit-style queries.
- Added fault badv capture to LoongArch user-run reporting so fixed-address execution blockers are visible.

## BusyBox Loader Result

The real `/musl/busybox` ELF now loads from the official LoongArch sdcard image:

```text
[loongarch64-busybox] loaded /musl/busybox file_size=2065912 entry=0x901ca40c first_load=0x120000000 load_size=2087832 segments=2
```

The probe enters PLV3 for `busybox true`, then returns through the kernel trap path with a controlled fault:

```text
[loongarch64-busybox] blocker: user fault ecode=8 era=0x9019d85c badv=0x90016138
```

This is progress from "too large to load" to "loaded and entered safely", but it is not yet a runnable BusyBox command.

## Current Blocker

`/musl/busybox` is a static ET_EXEC image with fixed user virtual addresses beginning at `0x120000000`. This iteration can store and translate those regions for kernel user-copy operations, but actual PLV3 execution still runs from relocated direct backing addresses because the LoongArch path does not yet install a real user page table/TLB mapping for the fixed ET_EXEC virtual range.

Next BusyBox work should add real LoongArch user virtual-address mappings before enabling any `busybox-musl-la` group.

## Non-Goals

- No BusyBox command success was hard-coded.
- No BusyBox official group markers were emitted.
- No fake BusyBox output was printed.
- No RISC-V runtime dispatch behavior was changed.
- `runtime_dispatch.rs` was not changed.

## Result

Existing LoongArch basic-musl coverage stayed stable in local QEMU:

```text
[loongarch64-basic] attempted=32 completed=32 failed=none
```

Official validation could not be refreshed because Docker was unavailable before kernel evaluation.
