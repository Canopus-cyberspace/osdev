# v157 process/thread/signal/proc-task semantics

This change builds on the v156 stable baseline and adds a no_std-friendly fixed-table semantic model in `src/fs/ucompat_proc_task_v157.rs`.

## New real semantic coverage

- process table with pid, ppid, tgid, pgid, sid
- task/thread table with tid and tgid relation
- cwd/root normalization and chroot/chdir-style snapshots
- fd table targets, cloexec flag, dup3, close_range
- procfs-like `/proc/<pid>/fd/N` readlink behavior
- fork inheritance of cwd/root/fd table
- clone-thread task visibility under `/proc/<pid>/task`
- setsid/setpgid process/session semantics
- signal mask, pending, delivered state
- kill, tgkill, SIGCHLD-on-exit, waitpid exit-code collection
- execve close-on-exec cleanup

## Regression policy

The active smoke matrix keeps v151k7, v154, v155, v156 and v157 checks in the same active_once path. The script also performs build, static feature inventory, stale module exposure checks, and binary-safe fresh QEMU runtime log scanning.

## Non-goals

This patch does not rewrite the central syscall dispatcher or trap path. The active bridge is only a one-shot evidence bridge used for broad smoke validation and does not alter syscall return values.
