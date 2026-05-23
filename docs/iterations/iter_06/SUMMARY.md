# Iteration 06 Summary

## Focus

This iteration finished the remaining currently reachable LoongArch basic-musl cases and inspected the real BusyBox payload for the next expansion step.

## Source Changes

- Enabled the real `/musl/basic/clone` case in `src/arch/loongarch64/basic_runner.rs`.
- Reused the existing LoongArch `process::sys_clone` full-copy child path.
- Kept `mmap` and `munmap` enabled through the existing `real_elf.rs` mapping helpers.
- Added sparse regular-file block handling to `src/arch/loongarch64/sdcard_ext4.rs`.

The clone ELF on `sdcard-la.img` has a sparse hole at logical block 3:

```text
EXTENTS:
(0-2):630752-630754, (4-16):630756-630768
```

The ext4 reader now zero-fills missing extents only for regular file payload reads. Directory lookup still uses the stricter inode block reader.

## BusyBox Groundwork

The real LoongArch `/musl/busybox` payload was inspected but not enabled.

```text
/musl/busybox size: 2065912 bytes
ELF type: EXEC
entry: 0x1201b640c
first LOAD vaddr: 0x120000000
```

The current LoongArch real-ELF loader is sized and shaped for the basic-musl PIE payloads:

```text
OFFICIAL_ELF_CAP = 128 KiB
USER_IMAGE_SIZE = 128 KiB
```

BusyBox requires a larger loader and a real virtual-address mapping strategy for the fixed `0x120000000` ET_EXEC image before command execution can be enabled safely.

## Non-Goals

- No BusyBox output was emitted.
- No BusyBox command success was hard-coded.
- No new parser-shaped output was added.
- `runtime_dispatch.rs` was not changed.

## Result

Local LoongArch smoke completed all enabled real PLV3 ELF basic-musl cases:

```text
========== START test_clone ==========
  Child says successfully!
clone process successfully.
pid:2
========== END test_clone ==========
========== START test_mmap ==========
========== END test_mmap ==========
========== START test_munmap ==========
========== END test_munmap ==========
[loongarch64-basic] attempted=32 completed=32 failed=none
```

Official validation could not be refreshed because Docker was unavailable before kernel evaluation.
