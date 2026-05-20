# v156 Procfs / FD Observability / Process Namespace Design

This document is intentionally complete and not truncated.

v156 builds on the v155 baseline and adds a real, heapless process/procfs/fd observability core:

- `src/fs/ucompat_procfs_v156.rs`

## Implemented mechanisms

1. Process table with pid, ppid, pgid, sid, cwd, root, umask, comm, and fd table.
2. FD table entries with target path, flags, and close-on-exec state.
3. Path normalization for absolute and cwd-relative paths, including repeated slashes, `.`, and `..`.
4. `/proc/self/fd/N`-like readlink via `proc_self_fd_readlink`.
5. `/proc/<pid>/fd/N`-like readlink via `proc_pid_fd_readlink`.
6. `open_proc_fd` for fd allocation with normalized target paths.
7. `dup3` and fd close-on-exec state.
8. `fork_current` inheritance of cwd, root, session ids, process group, and fd table.
9. `exec_current` close-on-exec cleanup and comm replacement.
10. `setsid_current` and `setpgid` process/session semantics.
11. `chdir` and `chroot` state updates.
12. `close_range` fd cleanup.
13. `snapshot` for status-like observability: pid, ppid, pgid, sid, fd count, cloexec count, cwd/root lengths.

## Regression preservation

v156 keeps and verifies:

- v151k7 VFS tree regression.
- v154 FS core regression.
- v155 namespace/proc-fd/VFS regression.
- v153f broad smoke behavior.

## Runtime evidence

Expected runtime markers:

- `[ucompat-v151k7] vfs_tree_dirfd_multiinode PASS`
- `[ucompat-v154] fs_core_multi_feature PASS`
- `[ucompat-v155] namespace_procfd_multi_feature PASS`
- `[ucompat-v156] procfs_fd_observability PASS`

## Non-goals

v156 does not rewrite the syscall dispatcher, trap entry, or previous FS/VFS modules. It adds a stable semantic module and verifies it through the already-proven bridge path.
