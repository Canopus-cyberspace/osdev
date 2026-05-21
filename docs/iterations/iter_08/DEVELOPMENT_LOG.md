# Iteration 08 Development Log

## Feature Discovery

Feature: make LoongArch PLV3 instruction fetch and user data access work for fixed ET_EXEC virtual ranges, starting with `/musl/busybox` at `0x120000000`.

Subsystem ownership:

- `user_mmu.rs` owns LoongArch CSR, DMW, TLB flush, and TLB entry install details.
- `real_elf.rs` owns ELF segment layout, user image/stack/heap/mmap backing, user-region metadata, and activation of the current user mappings.
- `busybox_runner.rs` owns the non-scoring BusyBox command probe.
- `trap.rs` stays limited to trap vector setup, trapframe handling, trap dispatch, and PLV3 return.
- `kernel.rs` stays limited to top-level boot and runner orchestration.

Existing code searched and reused:

```bash
rg "copy_from_user|copy_to_user|validate_user" src/
rg "load_user_elf|PT_LOAD|auxv|program header" src/
rg "run_loongarch_busybox_loader_probe|busybox" src/arch/loongarch64
rg "mmap|munmap|brk|UserRegion|USER_MMAP" src/arch/loongarch64
rg "csrwr|csrrd|tlbfill|invtlb|dmw" src/arch/loongarch64 src/
```

Search terms for future agents:

```text
user_mmu
activate_current_user_mmu
map_huge_range
loongarch64_mmu_tlbfill
FIXED_STACK_VA
FIXED_HEAP_VA
FIXED_MMAP_VA
current_entry
dump_user_regions
run_loongarch_busybox_loader_probe
0x1201b640c
0x120000000
```

## Decisions

`user_mmu.rs` was created because CSR/TLB/DMW operations are a new architecture responsibility. Keeping them out of `trap.rs` preserves the existing trap boundary, and keeping them out of `kernel.rs` avoids turning top-level boot orchestration into a feature module.

The user memory region model stayed in `real_elf.rs` for this iteration. A new user-space mapping file was considered, but `real_elf.rs` already owns the image, stack, heap, mmap, snapshot, and low-level real-user copy metadata. Splitting that while adding the first TLB mapping path would have made the change riskier for the 32 stable basic cases.

The first mapping implementation uses 2 MiB LoongArch TLB pages. The backing arrays for image, stack, heap, and mmap were aligned to 2 MiB so the ET_EXEC image, fixed stack, brk heap, and mmap area can be installed with a small number of TLB entries.

The BusyBox probe was kept non-scoring and narrowed to `busybox true`. Additional commands were investigated during development, but wider command probing is deferred until command execution can be bounded safely without risking official-run hangs.

## Bugs Found And Fixed

The first fixed virtual stack and heap placement used adjacent 2 MiB ranges that collided inside one huge-page TLB pair. BusyBox then faulted when heap access resolved to the wrong mapping. The fixed stack, heap, and mmap virtual ranges were moved onto separate 4 MiB-spaced regions.

The previous BusyBox fault showed execution from relocated backing addresses. After the mapping change, the probe reports the actual ET_EXEC entry `0x1201b640c`, and syscall ERAs appear in the BusyBox fixed virtual range before `busybox true` exits.

## Guardrails

- No official evaluation scripts were modified.
- No fake BusyBox output was added.
- No hard-coded command success text was added.
- No official BusyBox group markers were emitted.
- `runtime_dispatch.rs` was not changed.
- Existing 32 LoongArch basic cases were preserved locally.
