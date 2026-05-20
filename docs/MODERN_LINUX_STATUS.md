# MODERN_LINUX_STATUS

## v77 - modern Linux syscall scaffold

Implemented as compatibility scaffolds:
- pidfd_open / pidfd_send_signal / pidfd_getfd
- clone3
- close_range
- openat2 / faccessat2
- epoll_pwait2
- io_uring_setup / io_uring_enter / io_uring_register
- open_tree / move_mount / fsopen / fsconfig / fsmount / fspick / mount_setattr
- quotactl_fd
- process_madvise / process_mrelease
- landlock_create_ruleset / landlock_add_rule / landlock_restrict_self
- memfd_secret
- futex_waitv
- set_mempolicy_home_node

Still TODO:
- real pidfd object table
- real clone3 argument parsing and task creation
- real openat2 path resolution flags
- real io_uring rings
- real new mount API state machine
- real Landlock policy enforcement
- real memfd_secret memory semantics
