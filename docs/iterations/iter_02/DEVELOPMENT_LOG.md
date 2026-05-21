# Iteration 02 Development Log

## Initial State

The LoongArch path could boot, load real `/musl/basic/*` ELFs from `sdcard-la.img`, enter PLV3, and complete 24 local basic-musl cases. Inspection showed the trap entry allocated the trap frame directly on the current stack, which is the user stack when a syscall or exception arrives from PLV3.

## Feature Ownership Decision

- Feature: LoongArch trap stack switching.
- Owning subsystem: LoongArch trap entry and PLV3 return mechanics.
- Existing code searched first: `src/arch/loongarch64/trap.rs`, `src/arch/loongarch64/real_elf.rs`, and RISC-V trap stack assembly for shape.
- File chosen: `src/arch/loongarch64/trap.rs`, because stack switching, trap frame save/restore, and return mechanics are explicitly trap responsibilities.
- New source file considered: no. A separate file would split a single assembly entry path away from the trap mechanics it protects.
- Future search terms: `loongarch64_trap_stack_top`, `__loongarch64_trap_entry`, `KernelReturnState`, `loongarch64_enter_user_frame`, `save_user_snapshot`, `restore_user_snapshot`.

## Implementation Notes

The trap entry now saves the incoming PLV3 user stack pointer in a scratch CSR, saves the temporary register used for stack switching in another scratch CSR, switches to `loongarch64_trap_stack_top`, and only then allocates the trap frame. The saved user stack pointer is written into `LoongArchTrapFrame.regs[3]`, matching the existing restore path.

The trap stack is a kernel-owned 32 KiB `.trap_stack` allocation. Existing register save and restore coverage was preserved, including callee-saved registers, syscall id and arguments, return value register, ERA, ESTAT, BADV, and PRMD.

`real_elf.rs` restored full per-case user stack snapshot copying. This is now safe because kernel trap work no longer scribbles over the PLV3 user stack.

## Decisions

- Kept the implementation global rather than per-task for this iteration. The current LoongArch runner is single-core and executes one active PLV3 context at a time; a per-task stack can be layered in once fork/clone scheduling is introduced.
- Kept non-trap logic out of `kernel.rs`.
- Kept feature logic out of `runtime_dispatch.rs`.
- Did not enable fork-dependent real ELFs in this iteration.

## Blockers

Official validation could not run because the local Docker API endpoint was unavailable.
