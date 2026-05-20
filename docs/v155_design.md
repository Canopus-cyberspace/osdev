# v155 Namespace / Proc-FD / VFS Core Design

This document is intentionally complete and not truncated.

v155 builds on v154. It adds a new implementation module:

- `src/fs/ucompat_vfs_core_v155.rs`

The module implements multiple kernel-like mechanisms in one stable, heapless core:

1. path normalization with `/`, `.`, and `..`
2. cwd and root tracking
3. dirfd-relative `openat`, `mkdirat`, `linkat`, `renameat`, `unlinkat`, `readlinkat`
4. directory file descriptors and `fchdir`
5. create/open file semantics with `O_CREAT`, `O_TRUNC`, `O_APPEND`, `O_DIRECTORY`, `O_CLOEXEC`
6. read/write/lseek with file offsets
7. hardlink nlink accounting through owner inode aliases
8. symlink and readlink behavior
9. rmdir empty vs non-empty behavior
10. chmod/access permission checks
11. close-on-exec cleanup for proc fd tables
12. stat and statfs-like accounting

Regression preservation:

- v151k7 directory tree smoke remains required.
- v154 FS core smoke remains required.
- v155 namespace/proc-fd smoke is added.
- The package keeps the existing minimal active bridge and does not rewrite the syscall dispatcher.
