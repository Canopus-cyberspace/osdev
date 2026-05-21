# Development Log

## Iteration 01

Expanded the LoongArch basic-musl runner with stable `mount` and `umount` execution. Added process syscall scaffolding but deferred fork-dependent cases after local testing showed the current LoongArch trap handler stack model is not safe for full fork continuation.

## Iteration 02

Added LoongArch trap-stack groundwork. `__loongarch64_trap_entry` now switches from the incoming PLV3 user stack to a dedicated kernel-owned trap stack before building the trap frame and entering Rust trap handling. The original user stack pointer is still saved in the trap frame and restored on PLV3 return.

This keeps trap/vector mechanics in `src/arch/loongarch64/trap.rs`. The only memory-side companion change is in `src/arch/loongarch64/real_elf.rs`, where full user stack snapshot restoration is safe again because trap handling no longer uses the user stack.

## Iteration 03

Enabled the LoongArch `exit`, `wait`, `waitpid`, `yield`, and `fork` basic-musl cases as real PLV3 ELFs. Initial testing exposed a nested trap-stack overwrite when a child trapped while the parent was still inside `sys_clone`.

Fixed that in `src/arch/loongarch64/trap.rs` by adding a `loongarch64_trap_stack_cursor` and reserving a separate 16 KiB stack slice per active trap. The existing process lifecycle code in `src/arch/loongarch64/process.rs` was reused for child execution, child exit recording, and `wait4`.

## Iteration 04

Implemented LoongArch `execve` for the real `/musl/basic/execve` case. `syscall.rs` now safely reads path, argv, and envp from user memory; `real_elf.rs` rebuilds the replacement program stack with copied argv/envp strings; and `process.rs` switches the current trap frame to the new entry and user stack while preserving process identity, fd table, and cwd.

Added exec-specific image snapshot helpers so failed execve can restore the current image without disturbing the fork parent snapshot.

## Iteration 05

Implemented LoongArch pipe endpoint ownership in `src/arch/loongarch64/fd_table.rs`. The fd table now owns `PipeState` objects with shared buffers and read/write endpoint reference counts. `close`, `dup`, `dup3`, and fork fd snapshot/restore update those references so child writes remain visible to the parent without invalidating parent-owned endpoints.

Enabled `/musl/basic/pipe` as a real PLV3 ELF case.

## Iteration 06

Enabled the real LoongArch `/musl/basic/clone` case. The existing `process::sys_clone` full-copy child path was reused; no process semantics change was needed.

Local smoke initially failed before `START test_clone`. Inspection of `sdcard-la.img` showed that `/musl/basic/clone` is a sparse ext4 file with a missing regular-file extent. Added `Ext4::read_file_block` in `src/arch/loongarch64/sdcard_ext4.rs` so regular file payload reads zero-fill sparse holes while directory lookup remains strict.

`mmap` and `munmap` stayed on their existing `real_elf.rs` implementation and remained passing. BusyBox was inspected but left disabled because `/musl/busybox` is a 2.0 MiB fixed-address ET_EXEC at `0x120000000`, beyond the current 128 KiB direct-memory LoongArch basic loader.
