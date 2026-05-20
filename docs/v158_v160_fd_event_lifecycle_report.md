# v158-v160 FD/Event Lifecycle Report

## Files changed
- src/fs/runtime.rs
- src/mm/sv39_init_exec.rs
- apply_fix.sh
- apply_fix.bat
- docs/v158_v160_fd_event_lifecycle_report.md

## Canonical subsystem choices
- FD table, OFD status flags, object lifecycle, procfs fd views: src/fs/runtime.rs.
- Pipe, eventfd, timerfd, socketpair, and epoll readiness: src/fs/runtime.rs.
- Runtime syscall entry: rust_main -> mm::sv39_init_exec::run_external_init_elf_smoke -> dispatch_runtime_syscall -> RuntimeSyscallAction.
- User memory transfer for ppoll/epoll syscall results: src/mm/sv39_init_exec.rs using the existing SUM-enabled usercopy pattern.

## Historical modules integrated
- v67 pipe/event/poll/epoll scaffolds now route readiness through canonical runtime objects.
- v72 socket/socketpair loopback now uses canonical socket fd objects for readiness and data transfer.
- v75 timerfd scaffolds now allocate and report deterministic readiness through canonical timerfd objects.
- v77 close_range now routes to the canonical fdtable and supports CLOSE_RANGE_CLOEXEC.
- v151k7, v154, v155, v156, and v157 regression evidence remains preserved in fresh runtime output.

## Syscall paths deepened
- pipe2
- eventfd2
- socketpair
- socket
- read
- write
- sendto
- recvfrom
- ppoll
- epoll_create1
- epoll_ctl
- epoll_pwait
- epoll_pwait2
- timerfd_create
- timerfd_settime
- timerfd_gettime validation remains simplified
- close
- close_range
- dup3
- fcntl

## v158 event/pipe/socket/poll/epoll summary
- Pipe objects now track read/write endpoint lifecycle and report POLLIN/POLLOUT/POLLHUP/POLLERR from shared buffer state.
- Eventfd reports POLLIN only when counter > 0 and POLLOUT while counter can accept writes.
- Socketpair endpoints write into the peer receive buffer; poll readiness reflects peer buffer capacity and local receive data.
- ppoll reads pollfd.events, writes masked pollfd.revents, reports POLLNVAL for invalid fds, and uses canonical fd readiness.
- Epoll stores watched fd, interest mask, and data; ADD/MOD/DEL operate on the canonical epoll object; epoll_pwait returns real ready events.

## v159 timerfd summary
- timerfd_create allocates a canonical timerfd object.
- timerfd_settime deterministically creates one pending expiration.
- read consumes the expiration count.
- timerfd readiness clears after the read, and nonblocking empty reads return EAGAIN.
- Epoll sees timerfd readiness only while an expiration is pending.

## v160 fd lifecycle summary
- O_CLOEXEC is fd state, not an OFD status flag.
- F_GETFD/F_SETFD expose FD_CLOEXEC.
- F_GETFL/F_SETFL preserve O_NONBLOCK/O_APPEND OFD status flags.
- pipe/eventfd/timerfd/socket/epoll close release object lifecycle state when the last OFD reference closes.
- close_range supports normal close and CLOSE_RANGE_CLOEXEC.
- procfs fd readlink observes real fdtable state and returns EBADF after close.

## Preserved runtime markers
- [ucompat-v151k7] vfs_tree_dirfd_multiinode PASS
- [ucompat-v154] fs_core_multi_feature PASS
- [ucompat-v155] namespace_procfd_multi_feature PASS
- [ucompat-v156] procfs_fd_observability PASS
- [ucompat-v157] unified historical kernel integration PASS

## New runtime markers
- [ucompat-v158] event pipe socket readiness PASS
- [ucompat-v159] timerfd deterministic readiness PASS
- [ucompat-v160] fd lifecycle cloexec close_range PASS

## Remaining incomplete Linux semantics
- Blocking waits still do not sleep on scheduler queues.
- Timerfd expiration is deterministic one-shot behavior, not clock-driven.
- Network protocol stacks, process fork/exec, signal delivery, mount namespaces, and security policy remain out of v158-v160 scope.
- Epoll is bounded by the fixed canonical runtime object capacity.

## Build log path
.repair_logs/v158_v160_fd_event_lifecycle_20260507_220558/cargo_build.log

## QEMU serial log path
.repair_logs/v158_v160_fd_event_lifecycle_20260507_220558/qemu.serial.log

## QEMU wrapper stdout path
.repair_logs/v158_v160_fd_event_lifecycle_20260507_220558/run-qemu.stdout.log

## Forbidden warning gate result
- PASS: build output did not contain "matches any value", "unreachable pattern", or "warning: unused variable:".
