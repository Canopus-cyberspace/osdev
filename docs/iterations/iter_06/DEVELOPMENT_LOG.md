# Iteration 06 Development Log

## Feature Discovery

Feature: finish LoongArch `clone`, preserve `mmap` and `munmap`, and inspect BusyBox launch readiness.

Subsystem ownership:

- `basic_runner.rs` owns basic-musl case sequencing and enablement.
- `process.rs` owns clone/fork process lifecycle semantics.
- `real_elf.rs` owns LoongArch user ELF image, stack, brk, mmap, and munmap state.
- `sdcard_ext4.rs` owns ext4 path lookup and file payload reads.
- BusyBox runner work was not added because the loader cannot yet safely run the fixed-address BusyBox ET_EXEC.

Existing code searched and reused:

```bash
rg "clone|mmap|munmap|busybox|LaBasicCase|run_loongarch|basic-musl|busybox-musl" src/arch/loongarch64 src
rg "SYS_CLONE|SYS_MMAP|SYS_MUNMAP|sys_clone|sys_mmap|sys_munmap" src/arch/loongarch64 src/syscall/numbers.rs
rg "read_user_cstr|copy_to_user|copy_from_user|load_basic_case_with_args|save_user_snapshot|restore_user_snapshot|fd_snapshot|Pipe" src/arch/loongarch64
```

Search terms for future agents:

```text
LaBasicCase clone
sys_clone
read_file_block
ext4_extent_missing
sys_mmap
sys_munmap
OFFICIAL_ELF_CAP
USER_IMAGE_SIZE
/musl/busybox
```

## Decisions

`clone` was enabled only after confirming the existing syscall implementation was already present. The first local smoke failed before `START test_clone`, which showed the blocker was ELF loading rather than clone semantics.

`debugfs` confirmed `/musl/basic/clone` exists and is a sparse regular file. The existing ext4 file reader treated a missing extent as a hard error. The fix adds `Ext4::read_file_block`, which converts `ext4_extent_missing` to a zero-filled file block only for regular file reads.

No new module was created. The sparse-file behavior belongs beside `read_file_into` in `sdcard_ext4.rs`; adding a new file would have made the ext4 responsibility less searchable.

`mmap` and `munmap` did not need code changes. They were already enabled in `basic_runner.rs` and handled by `real_elf::sys_mmap` and `real_elf::sys_munmap`.

BusyBox was inspected but left disabled. The real payload is a 2.0 MiB static ET_EXEC with a fixed load address above the current direct-memory basic loader's model. Enabling BusyBox now would require a larger image path plus virtual address mapping work, not just argv/envp wiring.

## Bugs Found And Fixed

Bug: `/musl/basic/clone` failed to load because the regular file has an unallocated sparse extent.

Fix: `sdcard_ext4.rs` now zero-fills missing regular-file extents during file payload reads.

## Guardrails

- No official evaluation scripts were modified.
- No fake test output was added.
- No hard-coded case success was added.
- `runtime_dispatch.rs` was not changed.
- RISC-V build output remained valid.
