# v158 Plan: Event / Pipe / Socket / Poll / Epoll Readiness

## Baseline

Current stable baseline: **v157**.

## Goal

Implement real readiness semantics for event-like fd objects in canonical runtime state.

## Why v158 should focus here

v157 already connected pipe/eventfd/timerfd/socket/socketpair/epoll objects to the canonical fdtable. v158 should deepen these objects so `poll` and `epoll` observe real readiness rather than unrelated fixed return values.

## Files to inspect

```text
src/fs/runtime.rs
src/fs/mod.rs
src/syscall/mod.rs
src/mm/sv39_init_exec.rs
src/mm/user_buffer.rs
user/build_init_elf.py
tools/run-qemu.sh
```

Also inspect any existing historical modules related to:

```text
pipe
eventfd
timerfd
socketpair
poll
epoll
ucompat
```

## Likely files to modify

```text
src/fs/runtime.rs
src/syscall/mod.rs
src/mm/sv39_init_exec.rs
user/build_init_elf.py
apply_fix.sh
apply_fix.bat
docs/v158_event_pipe_socket_readiness_report.md
```

## Implementation scope

### Pipe

- Add pipe buffer state.
- `pipe2` allocates two fds linked to the same pipe object.
- Write endpoint appends data.
- Read endpoint consumes data.
- Empty pipe read with O_NONBLOCK returns `-EAGAIN`.
- Non-empty pipe is read-ready.
- Pipe with capacity available is write-ready.

### Eventfd

- Add eventfd counter.
- `eventfd2(initval, flags)` creates canonical fd object.
- write increments counter.
- read returns counter and clears or decrements according to simplified semantics.
- counter > 0 means read-ready.
- counter capacity means write-ready.

### Socketpair

- Keep loopback pair objects in canonical fdtable.
- Sending on one endpoint appends to peer buffer.
- Receiving consumes peer buffer.
- peer buffer non-empty means read-ready.

### Epoll

- `epoll_create1` allocates canonical epoll object.
- `epoll_ctl` stores interest records.
- `epoll_pwait` scans watched fds and reports ready events.

### Poll / ppoll

- `ppoll` inspects real fd object readiness.
- It should not always return 0 when a watched fd is actually ready.

## Out of scope

- Full TCP/IP networking.
- Hardware network driver.
- Full scheduler blocking.
- Real timer expiration.
- Complete Linux epoll edge-triggered semantics.

## Preserved markers

```text
[ucompat-v151k7] vfs_tree_dirfd_multiinode PASS
[ucompat-v154] fs_core_multi_feature PASS
[ucompat-v155] namespace_procfd_multi_feature PASS
[ucompat-v156] procfs_fd_observability PASS
[ucompat-v157] unified historical kernel integration PASS
```

## New marker

Only print this after real readiness validation passes:

```text
[ucompat-v158] event pipe socket readiness PASS
```

## Validation scenario

The v158 external init/runtime validation should prove:

1. `pipe2` creates two fds.
2. Empty pipe is not read-ready.
3. After writing to pipe, `ppoll` reports read-ready.
4. Reading consumes the pipe buffer.
5. `eventfd2` creates an event fd.
6. Writing increments eventfd counter.
7. `ppoll` reports eventfd read-ready.
8. Reading eventfd consumes the counter.
9. `socketpair` creates linked fds.
10. Sending on fd A makes fd B read-ready.
11. Receiving from fd B consumes the buffer.
12. `epoll_create1` creates an epoll fd.
13. `epoll_ctl` registers pipe/eventfd/socketpair fd.
14. `epoll_pwait` reports readiness based on object state.
15. close releases these canonical fd objects.

## Build and test commands

```bash
source /home/lenovo/miniconda3/etc/profile.d/conda.sh
conda activate osdev
source ~/.cargo/env || true
export PATH="$HOME/.cargo/bin:$PATH"
rustup target add riscv64gc-unknown-none-elf || true

cargo build --target riscv64gc-unknown-none-elf
bash ./tools/run-qemu.sh
```

## Forbidden warning gate

Fail if build output contains:

```text
matches any value
unreachable pattern
warning: unused variable:
```

## Expected final output

```text
[PASS] event pipe socket readiness v158 passed
[PASS] apply_fix.sh completed
[PASS] apply_fix.bat completed
```
