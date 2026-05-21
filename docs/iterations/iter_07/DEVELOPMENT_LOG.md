# Iteration 07 Development Log

## Feature Discovery

Feature: make the LoongArch user memory and ELF loading path scale beyond small basic-musl PIEs so the real static `/musl/busybox` ET_EXEC can be loaded and probed.

Subsystem ownership:

- `busybox_runner.rs` owns the non-scoring BusyBox loader/entry probe.
- `real_elf.rs` owns LoongArch ELF loading, user image storage, stack/argv/envp construction, brk, mmap, region metadata, and low-level real-user copy/range internals.
- `syscall.rs` owns LoongArch-local syscall ABI dispatch and compatibility return values.
- `user.rs` owns per-user-run exit, fault, and missing-syscall reporting state.
- `trap.rs` owns trap classification and passes badv into user fault reporting.
- `kernel.rs` owns only top-level runner sequencing.

Existing code searched and reused:

```bash
rg "copy_from_user|copy_to_user|validate_user|read_user_cstr|write_user_usize" src/arch/loongarch64 src/
rg "load_basic_case|load_basic_case_with_args|PT_LOAD|program header|auxv|EXEC_ARG_MAX" src/arch/loongarch64 src/
rg "handle_loongarch_syscall|SYS_NEWFSTATAT|SYS_FACCESSAT|SYS_READLINKAT|SYS_PRLIMIT64" src/arch/loongarch64 src/syscall/numbers.rs
rg "run_loongarch_basic_musl_group|basic-musl|busybox" src/arch/loongarch64 src/
rg "save_user_snapshot|restore_user_snapshot|save_exec_snapshot|restore_exec_snapshot" src/arch/loongarch64
```

Search terms for future agents:

```text
run_loongarch_busybox_loader_probe
load_user_elf_with_args
MAX_USER_REGIONS
UserRegion
translate_user_chunk
USER_MMAP_SLOTS
syscall_newfstatat
syscall_faccessat
syscall_readlinkat
UserRunSnapshot
fixed-address ET_EXEC
0x120000000
```

## Decisions

`busybox_runner.rs` was created because BusyBox probing is separate from official basic-musl case sequencing. Keeping it out of `kernel.rs` preserves `kernel.rs` as top-level orchestration only.

A separate user-memory file was considered, but not created in this iteration. The current LoongArch module boundary already has `real_elf.rs` owning user image, stack, brk, mmap, snapshot, and low-level real-user copy/range internals. Moving those internals during this loader upgrade would have made the change larger and risked regressing the 32 stable basic cases. Future work can split a focused `user_space.rs` or expand `user_mem.rs` after real page-table mapping lands.

`real_elf.rs` now tracks user regions instead of a fixed four-range model. Each region has a user virtual start, host backing start, length, and kind. This lets kernel copy helpers validate fixed-address ET_EXEC pointers while preserving the direct-host addresses used by the current PLV3 execution path.

`load_basic_case_with_args` was kept as the stable basic-musl entry and now forwards to `load_user_elf_with_args`. That avoids duplicating ELF parsing for BusyBox while preserving existing basic call sites.

The BusyBox runner intentionally does not print official `busybox-musl` markers. It only runs a real load and PLV3 entry probe, then reports the actual exit/fault/missing-syscall result outside any official group.

## Bugs Found And Fixed

The old user-copy helpers directly dereferenced pointers after checking one small contiguous range. They now translate each copy chunk through the region table and return a clean error on invalid user addresses.

The old mmap state allowed only one active mmap region. It now tracks multiple slots in the existing mmap backing area so basic `mmap`/`munmap` behavior survives the multi-region user model.

The BusyBox probe initially needed clearer fault evidence. `user.rs` and `trap.rs` now preserve and report `badv` for user-mode faults.

## Current Blocker

BusyBox loads and enters PLV3, but faults at relocated execution:

```text
[loongarch64-busybox] blocker: user fault ecode=8 era=0x9019d85c badv=0x90016138
```

The loader records the original ET_EXEC virtual base `0x120000000`, but execution still uses relocated direct backing addresses. Static BusyBox needs real LoongArch user virtual-address mappings for its fixed code/data references.

## Guardrails

- No official evaluation scripts were modified.
- No fake BusyBox output was added.
- No hard-coded command success was added.
- No `busybox-musl` group markers were emitted.
- `runtime_dispatch.rs` was not changed.
- RISC-V build output remained valid.
