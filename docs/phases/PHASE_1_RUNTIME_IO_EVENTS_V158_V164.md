# Phase 1: Runtime I/O and Event Readiness, v158-v164

## Purpose

Phase 1 turns v157's canonical fd object integration into usable event-driven behavior.

The key idea:

```text
fd objects must have state, readiness, lifecycle, and shared usercopy/iovec behavior.
```

## v158: event/pipe/socket/poll/epoll readiness

### Goal

Implement real readiness semantics for canonical fd objects.

### Implementation details

Pipe:

- `pipe2` creates two fds backed by one pipe object.
- pipe object has buffer, reader count, writer count, and flags.
- write appends to buffer.
- read consumes buffer.
- empty pipe read with O_NONBLOCK returns `-EAGAIN`.
- non-empty pipe is `POLLIN`.
- pipe with capacity is `POLLOUT`.

Eventfd:

- `eventfd2` creates counter object.
- write adds to counter.
- read returns counter and consumes it.
- counter > 0 is `POLLIN`.

Socketpair:

- `socketpair` creates linked endpoint objects.
- send/write on endpoint A appends to endpoint B receive buffer.
- recv/read on endpoint B consumes buffer.
- non-empty receive buffer is `POLLIN`.

Poll:

- `ppoll` reads user pollfd array.
- each fd is resolved through canonical fdtable.
- returned events come from fd object readiness.

Epoll:

- `epoll_create1` creates epoll object.
- `epoll_ctl` stores fd/event interest.
- `epoll_pwait` scans interest list and returns ready events.

### Marker

```text
[ucompat-v158] event pipe socket readiness PASS
```

## v159: timerfd deterministic readiness

### Goal

Add deterministic timer readiness suitable for QEMU smoke and future scheduler integration.

### Implementation details

- Create a simple monotonic tick counter or deterministic event time.
- `timerfd_create` allocates timerfd object.
- `timerfd_settime` arms the object.
- `timerfd_gettime` reports simplified remaining/interval state.
- runtime validation can trigger deterministic expiration without real hardware timer dependency.
- expired timerfd becomes `POLLIN`.
- reading timerfd returns expiration count.

### Marker

```text
[ucompat-v159] timerfd deterministic readiness PASS
```

## v160: fd lifecycle, close_range, CLOEXEC

### Goal

Harden fd lifecycle before real process/exec work.

### Implementation details

- `close` releases one fd.
- `close_range` releases a range.
- `fcntl(F_SETFD)` stores `FD_CLOEXEC`.
- `fcntl(F_GETFD)` reads it.
- `fcntl(F_SETFL)` stores status flags such as `O_NONBLOCK`.
- procfs fd view reflects live fdtable after close/close_range.
- descriptor reuse is deterministic and safe.

### Marker

```text
[ucompat-v160] fd lifecycle cloexec close_range PASS
```

## v161: unified iovec IO path

### Goal

Make iovec behavior shared by multiple fd object types.

### Implementation details

- `read_iovec_array` remains the common helper.
- `readv/writev` work on regular files, pipe, and socketpair where applicable.
- `preadv/pwritev` preserve file offset on regular files.
- `sendmsg/recvmsg` use the same iovec path for socketpair.
- invalid iovec pointers return error consistently.

### Marker

```text
[ucompat-v161] unified iovec io path PASS
```

## v162: IPC registry lifecycle

### Goal

Deepen POSIX mq and SysV IPC from ID registry into object lifecycle.

### Implementation details

POSIX mq:

- create/open by name.
- unlink by name.
- send appends message.
- receive consumes message.
- getsetattr exposes capacity/message count.

SysV msg:

- `msgget` creates or finds queue.
- `msgsnd` appends typed message.
- `msgrcv` receives by simplified type rule.
- `msgctl IPC_RMID` removes queue.

SysV sem:

- `semget` creates set.
- `semop` changes values under simplified rules.
- `semctl` gets/sets/removes.

SysV shm:

- `shmget` creates segment object.
- `shmat` returns deterministic user mapping placeholder or real mapping if VMA exists.
- `shmdt` detaches.
- `shmctl IPC_RMID` removes.

### Marker

```text
[ucompat-v162] ipc registry lifecycle PASS
```

## v163: futex wait/wake object model

### Goal

Prepare futex for real scheduler blocking.

### Implementation details

- futex key is based on user address.
- wait records waiter object if value matches expected.
- wake removes up to N waiters.
- timeout can remain simplified.
- validation proves wait/wake count behavior.

### Marker

```text
[ucompat-v163] futex wait wake object model PASS
```

## v164: scheduler wait queue foundation

### Goal

Introduce scheduler state needed by blocking syscalls.

### Implementation details

- task state enum includes running, ready, sleeping, zombie.
- wait queues can hold task identifiers.
- wake operation moves sleeping task to ready.
- blocking may remain cooperative until timer interrupts are deepened.
- pipe/eventfd/futex/poll can share wait queue structures.

### Marker

```text
[ucompat-v164] scheduler wait queue foundation PASS
```
