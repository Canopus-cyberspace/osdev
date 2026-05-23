# Iteration 13 Development Log

## Initial State Preservation

Before changing source, the current work state was captured:

```text
git status --short
git diff --stat
git diff > /tmp/uestc_kernel_before_change.diff
```

No reset, stash, or discard operation was used on user work.

## Feature Ownership Decision

Feature being attempted:

```text
Persistent LoongArch BusyBox Virtual Scratch FS for file-operation command scoring.
```

Owning subsystems:

```text
busybox_runner.rs: command classes, official marker policy, per-command budget, result reporting
fd_table.rs: fd behavior, pipe behavior, open flags, lseek, and scratch files
syscall.rs: syscall dispatch, errno behavior, compatibility wrappers
sdcard_ext4.rs: read-only ext4 backend
real_elf.rs and user_mmu.rs: ELF loading, fixed-address VA mapping, argv/envp stack, mmap/brk, TLB sync
process.rs: fork, clone, execve, wait, exit
```

Existing code searched and reused:

```text
src/arch/loongarch64/busybox_runner.rs
src/arch/loongarch64/fd_table.rs
src/arch/loongarch64/syscall.rs
src/arch/loongarch64/user.rs
src/arch/loongarch64/process.rs
/home/lenovo/oscomp-official-env/autotest-for-oskernel/kernel/judge/judge_busybox-musl.py
```

Existing helpers reused during the attempted patch:

```text
fd_table::normalize_user_path
fd_table virtual path helpers
fd_table open/read/write/stat/getdents paths
user::start_syscall_budget
real_elf::load_user_elf_with_args
user_mem copy and cstring helpers
process exec/wait/exit paths
```

Future search terms:

```text
BusyboxCommandClass
RUN_COMMANDS
DISABLED_COMMANDS
fd_table::normalize_user_path
is_virtual_file_path
syscall_openat
syscall_getdents64
process::sys_clone
user::start_syscall_budget
load_user_elf_with_args
busybox_cmd.txt
Virtual Scratch FS
```

## Attempted Implementation

The attempted patch added an in-memory scratch file and directory layer in `fd_table.rs`, plus syscall glue for additional file operations. It also tried to promote low-risk BusyBox file commands after executing real `/musl/busybox` commands in PLV3.

The patch exposed two important blockers:

- Shell redirection, specifically `echo "hello world" > test.txt`, stalled during real BusyBox execution.
- After trimming shell-redirection commands, `grep hello busybox_cmd.txt` also stalled.

During the same aggressive attempt, the basic `openat` case briefly regressed because the scratch path logic interfered with `/musl/basic/mnt/test_openat.txt`. That confirmed the scratch layer was not safe enough to ship.

## Fallback

The risky source patch was saved for evidence and reverted:

```text
/tmp/iter13_risky_busybox_scratch.diff
```

After the revert, the tree returned to the known-good BusyBox allowlist. No source helpers from the scratch attempt remain in the repository.

## Decisions

No new BusyBox command was promoted. `basename /aaa/bbb`, `uname`, and `ash -c exit` remain disabled. `echo hello` and `cat /musl/busybox_cmd.txt` remain smoke-only real commands.

The next safe path is to make the scratch layer fully isolated from basic-musl paths, add deterministic command abort for commands that stall before normal syscall progress, and only then re-test exact public judge file commands one at a time.

