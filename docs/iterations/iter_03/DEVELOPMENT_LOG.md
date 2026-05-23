# Iteration 03 Development Log

## Feature Discovery

- Feature: enable LoongArch fork-dependent basic-musl cases.
- Owning subsystem: process lifecycle in `process.rs`, case sequencing in `basic_runner.rs`, and trap stack mechanics in `trap.rs`.
- Existing code searched and reused first: `sys_clone`, `sys_wait4`, `exit_current_and_maybe_restore_parent`, user snapshots, fd snapshots, and user-copy helpers.
- New focused source file considered: no. The lifecycle code already had a focused `process.rs`; the bug found during enablement was in trap stack mechanics and belongs in `trap.rs`.
- Future search terms: `loongarch64_trap_stack_cursor`, `trap_stack_top`, `sys_clone`, `sys_wait4`, `exit_current_and_maybe_restore_parent`, `LaBasicCase`, `START test_fork`.

## Investigation

Simply enabling the target cases exposed a real nested trap bug. `test_exit` forks internally. The parent was still executing `sys_clone` on the global trap stack while the child entered PLV3 and trapped back for exit. Because the trap entry always reused the same stack top, the child's trap overwrote the parent's live syscall frame.

## Fix

`__loongarch64_trap_entry` now uses `loongarch64_trap_stack_cursor` to reserve a 16 KiB stack slice for each active trap. The trap frame records the previous slice top, and trap return restores the cursor. The stack reserve was increased to 64 KiB to support the current nested parent/child path with room for future shallow nesting.

This preserves the existing trap responsibility boundary:

- stack switching
- trapframe save/restore
- exception dispatch
- PLV3 return mechanics

No process logic was moved into `trap.rs`, and no trap logic was moved into `kernel.rs`.

## Case Enablement

The enabled cases are all real ELFs loaded from `sdcard-la.img` and entered in PLV3:

- `/musl/basic/exit`
- `/musl/basic/wait`
- `/musl/basic/waitpid`
- `/musl/basic/yield`
- `/musl/basic/fork`

`clone`, `execve`, and `pipe` remain disabled for later focused iterations.

## Official Validation

Official validation was attempted but could not start the Docker container because the Docker API endpoint was unavailable.
