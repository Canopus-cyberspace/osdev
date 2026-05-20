# Phase 3: Filesystem, Devices, Security, Network, v175-v184

## Purpose

Phase 3 completes the runtime environment expected by real user programs.

## v175: rootfs/tmpfs backend

Goal:

- Add a stable in-memory filesystem backend.

Implementation:

- directories and files persist in RAM during boot.
- create/write/read/truncate/link/unlink/rename work on the same inode tree.
- rootfs can store multiple ELF test programs.

Marker:

```text
[ucompat-v175] rootfs tmpfs backend PASS
```

## v176: devfs core devices

Goal:

- Add usable device files.

Implementation:

- `/dev/null`
- `/dev/zero`
- `/dev/console`
- `/dev/tty`
- `/dev/random` or `/dev/urandom` simplified
- major/minor metadata
- read/write behavior per device

Marker:

```text
[ucompat-v176] devfs core devices PASS
```

## v177: procfs process/status/maps

Goal:

- Expand procfs for test program introspection.

Implementation:

- `/proc/self`
- `/proc/self/fd`
- `/proc/self/status`
- `/proc/self/stat`
- `/proc/self/maps`
- `/proc/meminfo` simplified
- `/proc/cpuinfo` simplified

Marker:

```text
[ucompat-v177] procfs process status maps PASS
```

## v178: mount tree and statfs/fstatfs

Goal:

- Add minimal mount model.

Implementation:

- mount rootfs/devfs/procfs.
- statfs/fstatfs identify fs type.
- path resolver respects mount points.
- chroot/pivot_root can remain simplified but coherent.

Marker:

```text
[ucompat-v178] mount tree statfs PASS
```

## v179: permissions and credentials

Goal:

- Enforce basic permission checks.

Implementation:

- per-task uid/gid/euid/egid.
- file mode bits.
- directory execute permission.
- open read/write permission.
- chmod/chown simplified behavior.
- access/faccessat uses credentials.

Marker:

```text
[ucompat-v179] permissions credentials PASS
```

## v180: capability and identity model

Goal:

- Add simplified capability state.

Implementation:

- capget/capset store capability bits.
- setuid/setgid/setresuid/setresgid update credentials under simplified privilege rules.
- privileged operations check capability or root euid where needed.
- personality preserved.

Marker:

```text
[ucompat-v180] capability identity model PASS
```

## v181: AF_UNIX socket loopback

Goal:

- Deepen socket behavior for local IPC.

Implementation:

- AF_UNIX stream sockets.
- bind/listen/accept/connect for local paths.
- socketpair remains supported.
- poll readiness works with sockets.
- sendmsg/recvmsg use iovec path.

Marker:

```text
[ucompat-v181] unix socket loopback PASS
```

## v182: local datagram socket

Goal:

- Add UDP-like local datagram model.

Implementation:

- datagram socket object.
- bind address.
- sendto queues datagram to bound receiver.
- recvfrom consumes datagram.
- poll readiness reflects queue.

Marker:

```text
[ucompat-v182] local datagram socket PASS
```

## v183: IPC blocking and scheduler integration

Goal:

- Connect IPC to wait queues.

Implementation:

- empty mq receive can block or return EAGAIN if nonblocking.
- full mq send can block or return EAGAIN.
- semop waits if value unavailable.
- wake waiters on send/receive/sem update.
- simplified scheduler queue is used.

Marker:

```text
[ucompat-v183] ipc blocking scheduler integration PASS
```

## v184: namespace basics

Goal:

- Add simplified namespace objects.

Implementation:

- mount namespace object.
- pid namespace object if feasible.
- IPC namespace object.
- setns/unshare update task namespace refs for supported flags.
- unsupported flags return stable errors.

Marker:

```text
[ucompat-v184] namespace basics PASS
```
