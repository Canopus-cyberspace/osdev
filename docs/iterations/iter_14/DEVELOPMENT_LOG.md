# Iteration 14 Development Log

## Initial State Preservation

Before source changes, the current work state was captured:

```text
git status --short
git diff --stat
git diff > /tmp/uestc_kernel_before_change.diff
```

No reset, stash, or discard operation was used on user work.

## Feature Ownership Decision

Feature being attempted:

```text
BusyBox-only Virtual Scratch FS and gated promotion for real LoongArch BusyBox file-operation commands.
```

Owning subsystems:

```text
busybox_runner.rs: command classes, official markers, gated promotion, budget, reporting
user.rs: BusyBox active state, command budget, timeout state, log suppression
syscall.rs: syscall dispatch, errno behavior, narrow compatibility wrappers
fd_table.rs: fd behavior, pipe behavior, open flags, lseek, scratch files and directories
sdcard_ext4.rs: read-only ext4 backend
real_elf.rs and user_mmu.rs: ELF loading, fixed ET_EXEC VA mapping, user stack, mmap/brk, TLB sync
process.rs: fork, clone, execve, wait, exit
```

Existing code searched and reused:

```text
src/arch/loongarch64/busybox_runner.rs
src/arch/loongarch64/fd_table.rs
src/arch/loongarch64/syscall.rs
src/arch/loongarch64/user.rs
src/arch/loongarch64/process.rs
src/arch/loongarch64/real_elf.rs
src/arch/loongarch64/user_mmu.rs
src/arch/loongarch64/trap.rs
/home/lenovo/oscomp-official-env/autotest-for-oskernel/kernel/judge/judge_busybox-musl.py
```

Existing helpers reused during the attempted patches:

```text
fd_table::normalize_user_path
fd_table virtual file helpers
fd_table open/read/write/stat/getdents paths
user::start_syscall_budget
real_elf::load_user_elf_with_args
user_mem copy and cstring helpers
process exec/wait/exit/fork paths
```

Future search terms:

```text
BusyboxCommandClass
RUN_COMMANDS
DISABLED_COMMANDS
run_loongarch_busybox_loader_probe
normalize_user_path
is_virtual_file_path
write_virtual_file
read_virtual_file
virtual_file_size
syscall_openat
syscall_getdents64
SYS_LSEEK
SYS_RENAMEAT
SYS_TRUNCATE
start_syscall_budget
sync_user_entry
busybox_cmd.txt
Virtual Scratch FS
```

## Attempted Implementation

The main attempted patch added a BusyBox scratch overlay in `fd_table.rs`, plus syscall glue in `syscall.rs` for lseek, rename, truncate, ftruncate, and utimensat-style compatibility. The runner then tried to probe and promote exact BusyBox judge-style commands only after real PLV3 execution.

Observed blockers:

- `touch test.txt` stalled during real BusyBox execution.
- Adding the directory trio caused known-good `false` to stall before the new directory commands ran.
- The scratch overlay briefly regressed the basic `openat` path before a narrow local fix, which made the patch unsafe to ship.

The full risky patch was preserved outside the repository:

```text
/tmp/iter14_risky_busybox_scratch.diff
```

## Reduced Probe

A smaller `printf "abc\n"`-only runner patch was also tested. It passed once in the larger experiment, but a fresh reduced run stalled, so it was reverted as well.

The reduced patch was preserved outside the repository:

```text
/tmp/iter14_printf_only.diff
```

## Fallback

All source changes from the attempted expansion were reverted. The final tree intentionally keeps only the existing stable BusyBox scoring commands.

No new BusyBox command was promoted. `basename /aaa/bbb`, `uname`, and `ash -c exit` remain disabled. `echo hello` and `cat /musl/busybox_cmd.txt` remain smoke-only because their command lines do not expand the official score in the current runner.

## Decisions

The Virtual Scratch FS direction is still the right scoring path, but it needs a stronger no-hang mechanism before promotion. In particular, commands that stall in user mode before the next syscall need an outer execution guard, not just syscall-budget accounting.

The next safe path is:

- Keep scratch overlay work isolated from basic-musl path lookup.
- Add deterministic command abort for pre-syscall user-mode stalls.
- Re-test exact public judge commands one at a time on fresh sdcard images.
- Promote only after repeated fresh-image runs preserve the five known-good BusyBox scoring commands.

