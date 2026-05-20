# <VERSION> Completion Report

## Version

`<VERSION>`

## Baseline

`<BASELINE_VERSION>`

## Goal

`<GOAL>`

## Changed files

- `<file 1>`
- `<file 2>`

## Real implementation summary

Describe actual runtime/kernel state changes here.

Do not only list tests or marker edits.

## Canonical state touched

- VFS:
- FD table:
- OFD:
- inode/dentry:
- process/task:
- procfs:
- event/socket/IPC:
- MM/usercopy/iovec:
- scheduler/wait queue:
- signal:
- credentials/security:

## Syscalls changed or deepened

- `<syscall>`

## Preserved markers

Fresh QEMU runtime output preserved:

```text
[ucompat-v151k7] vfs_tree_dirfd_multiinode PASS
[ucompat-v154] fs_core_multi_feature PASS
[ucompat-v155] namespace_procfd_multi_feature PASS
[ucompat-v156] procfs_fd_observability PASS
[ucompat-v157] unified historical kernel integration PASS
```

Add newer preserved markers if applicable.

## New marker

```text
<new marker>
```

## Build log

```text
<path>
```

## QEMU serial log

```text
<path>
```

## Forbidden warning gate

PASS / FAIL

Forbidden strings checked:

```text
matches any value
unreachable pattern
warning: unused variable:
```

## Remaining incomplete semantics

- `<remaining TODO>`

## Final lines

```text
[PASS] <version-specific pass line>
[PASS] apply_fix.sh completed
[PASS] apply_fix.bat completed
```
