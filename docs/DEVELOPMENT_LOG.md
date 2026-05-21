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

## Iteration 07

Upgraded the LoongArch user ELF loader and user memory model for BusyBox groundwork. `real_elf.rs` now tracks multiple user regions for image, stack, heap, and mmap backing, and the user-copy helpers translate through those regions instead of assuming one small contiguous basic-musl image.

Added `busybox_runner.rs` as a focused non-scoring probe. It loads the real `/musl/busybox` ET_EXEC from `sdcard-la.img` and attempts a PLV3 entry for `busybox true` without printing official BusyBox markers or success output.

Local smoke preserved all 32 enabled LoongArch basic-musl cases. BusyBox now loads and enters PLV3, then returns with a controlled fixed-address execution fault. The remaining blocker is real LoongArch user virtual-address mapping for ET_EXEC ranges such as `0x120000000`.

## Iteration 08

Added `src/arch/loongarch64/user_mmu.rs` as the focused LoongArch CSR/DMW/TLB owner and wired it into the existing `real_elf.rs` user image metadata. Static ET_EXEC payloads now keep their real user virtual entry and segment addresses, and the non-scoring BusyBox probe activates fixed-address mappings for image, stack, heap, and mmap before entering PLV3.

The real `/musl/busybox true` command now loads from `sdcard-la.img`, enters at `0x1201b640c`, reaches syscall traps in the BusyBox fixed virtual range, and exits with code 0. Official BusyBox scoring remains disabled because only the bounded non-scoring smoke is stable.

Local smoke preserved all 32 enabled LoongArch basic-musl cases. Official validation could not be refreshed because Docker was unavailable before kernel evaluation.

## Iteration 09

Expanded the non-scoring LoongArch BusyBox smoke to seven real commands: `true`, `false`, `echo hello`, `pwd`, `sh -c exit`, `ls`, and `cat /musl/busybox_cmd.txt`. All are loaded from the real `/musl/busybox` ET_EXEC on `sdcard-la.img` and run through PLV3 without official BusyBox group markers.

Added a BusyBox-only syscall budget in `user.rs`, narrow compatibility in `syscall.rs` for `writev`, `readv`, `fcntl`, `sendfile`, and signal setup calls, and fixed `getdents64` EOF/path behavior for BusyBox `ls`.

Local smoke preserved all 32 enabled LoongArch basic-musl cases and completed the BusyBox smoke with `completed=7 attempted=7 matched=7 failed=0`. Official validation was attempted but timed out before producing Docker evaluation output.

## Iteration 10

Converted the LoongArch BusyBox smoke into a bounded scoring-capable official `busybox-musl` runner for the safe command subset. The runner still executes real `/musl/busybox` PLV3 commands from `sdcard-la.img`, but now wraps them in official BusyBox group markers and emits judge-visible testcase lines only for real commands that map to official entries: `true`, `false`, `pwd`, `sh -c exit`, and `ls`.

Added BusyBox group-active state in `user.rs`, reused `busybox_runner.rs` for official-name mapping and result emission, kept syscall/trap diagnostics quiet inside active official groups, and added concise phase/command progress prints in `kernel.rs` and the BusyBox runner.

Local smoke preserved LoongArch basic `attempted=32 completed=32 failed=none` and BusyBox `completed=7 attempted=7 matched=7 failed=0`. Official validation completed with `Accpted`, score `256`, `basic-musl-la=98.0`, and `busybox-musl-la=5.0`.

## Iteration 11

Closed the LoongArch basic-musl `waitpid` scoring gap. Official score-256 evidence showed only `test_waitpid` missing on LoongArch, with serial output `waitpid error.` instead of `waitpid successfully.` and `wstatus: 3`.

Fixed `src/arch/loongarch64/process.rs` so child exit recording masks to the low 8 bits and `wait4` writes Linux-compatible normal-exit status as `(exit_code & 0xff) << 8` through `user_mem::copy_to_user`.

The corrected waitpid path exposed a quiet BusyBox timing sensitivity, so `src/arch/loongarch64/user_mmu.rs` now provides a small silent `ibar`/`dbar` user-entry settle loop. `real_elf.rs` calls it after fixed-address mapping activation and `trap.rs` calls it before PLV3 return.

Official validation completed with `Accpted`, score `260`, `basic-musl-la=102.0`, and `busybox-musl-la=5.0`.

## Iteration 12

Stabilized the LoongArch BusyBox runner after the aggressive expansion attempt. `src/arch/loongarch64/busybox_runner.rs` now explicitly separates commands into scoring, smoke, and disabled classes.

Only the proven real PLV3 commands `true`, `false`, `pwd`, `sh -c exit`, and `ls` emit official BusyBox testcase lines. `echo hello` and `cat /musl/busybox_cmd.txt` remain real non-scoring smoke commands. `basename /aaa/bbb`, `uname`, and `ash -c exit` are documented as disabled and are not executed by the official LoongArch runner.

Local validation preserved LoongArch basic 32/32 and the five BusyBox scoring commands. The latest completed official baseline remains `Accpted`, score `260`, `basic-musl-la=102.0`, and `busybox-musl-la=5.0`; the final official refresh attempt timed out in the wrapper with a 0-byte `docker_evaluate.log`.
