# Iteration 08 Summary

## Focus

This iteration added the first real LoongArch virtual-address mapping path for fixed-address ET_EXEC user binaries such as `/musl/busybox`.

BusyBox scoring remains disabled. The only BusyBox path enabled is a non-scoring smoke probe for real `/musl/busybox true`; it loads the binary from `sdcard-la.img`, maps its fixed user virtual range, enters PLV3 at the actual BusyBox entry, and reports the real exit status.

## Source Changes

- Added `src/arch/loongarch64/user_mmu.rs` for LoongArch-local CSR, DMW, TLB flush, and huge-page TLB install operations.
- Reused `src/arch/loongarch64/real_elf.rs` for user image ownership and extended it to keep fixed ET_EXEC user virtual addresses for image, stack, heap, and mmap regions.
- Added `real_elf::activate_current_user_mmu()` and `real_elf::deactivate_current_user_mmu()` so the BusyBox probe can install mappings only around the non-scoring ET_EXEC run.
- Kept `src/arch/loongarch64/trap.rs` focused on trap/vector/PLV3 mechanics. No feature logic was moved into it.
- Kept `src/arch/loongarch64/kernel.rs` as top-level orchestration; it only exposes the new module.
- Reused `src/arch/loongarch64/busybox_runner.rs` as the non-scoring probe owner and kept it from emitting official BusyBox group markers.

## BusyBox Result

The real `/musl/busybox` payload now reaches PLV3 at its fixed ET_EXEC entry:

```text
[loongarch64-busybox] loaded /musl/busybox file_size=2065912 entry=0x1201b640c first_load=0x120000000 load_size=2087832 segments=2 command=true
[loongarch64-busybox] entering command=true
[loongarch64-busybox] mapped entry=0x1201b640c
[loongarch64-busybox] command=true exit_code=0
[loongarch64-busybox] smoke completed=1 attempted=1
```

This is progress from a fixed-address user fault to a real BusyBox command exit through the LoongArch trap path.

## Non-Goals

- No official `busybox-musl` group markers were emitted.
- No BusyBox output or command success was faked.
- No RISC-V syscall dispatch behavior was changed.
- `runtime_dispatch.rs` was not changed.
- Broader BusyBox commands remain disabled until they run without hang or fake output.

## Result

Existing LoongArch basic-musl coverage stayed stable in local QEMU:

```text
[loongarch64-basic] attempted=32 completed=32 failed=none
```

Official validation could not be refreshed because Docker was unavailable before kernel evaluation.
