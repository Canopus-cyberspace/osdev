# v201-v206 Scheduler Blocking Report

## Scope

- Bounded batch: v201-v206 only.
- Stable baseline: v200.
- Preserved fresh QEMU runtime markers from v151k7 through v200.
- Intentionally not implemented in this batch: ext4, virtio-blk, LoongArch64, GUI, networking, and dynamic kernel modules.

## Implementation

### v201 run queue and task switch

- Added canonical scheduler fields to `KernelCore`: run queue slots, run queue length, tick count, switch count, block count, wake count, timer wake count, and last switch endpoints.
- `clone_task` now inserts Ready children into the run queue.
- Task switching now updates Running/Ready state, current task runtime snapshot fields, and deterministic switch accounting.

Fresh runtime evidence:

```text
[ucompat-v201] evidence runq=3 pid_a=2 pid_b=3 switches=2 PASS
[ucompat-v201] run queue task switch PASS
```

### v202 timer tick scheduling

- Added a scheduler tick path that rebuilds the runnable queue and round-robins to the next runnable task.
- The v202 runtime proof advances multiple ticks over three runnable tasks and checks tick and switch counters.

Fresh runtime evidence:

```text
[ucompat-v202] evidence ticks=5 switches=5 last=2->3 PASS
[ucompat-v202] timer tick scheduling PASS
```

### v203 blocking wait and wakeup

- `sched_wait_on` now records a canonical wait queue entry, marks the current task Waiting, removes it from the run queue, and increments block accounting.
- `sched_wake` now scans canonical task state for matching wait keys, marks matching waiters Ready, reinserts them into the run queue, and preserves queue wake accounting.

Fresh runtime evidence:

```text
[ucompat-v203] evidence wait_pid=1 run_pid=2 blocks=1 wakes=1 PASS
[ucompat-v203] blocking wait wakeup PASS
```

### v204 pipe, poll, and futex blocking

- Added fd wait-key mapping for pipe and timerfd-backed waits.
- Pipe writes now wake readers through the canonical wait queue key for the pipe object.
- The suite blocks a task on a pipe/poll wait, wakes it with a real pipe write, verifies epoll readiness, then blocks and wakes a child through the futex model.

Fresh runtime evidence:

```text
[ucompat-v204] evidence pipe_parent_state=ready epoll_ready=1 futex_wakes=2 blocks=2 PASS
[ucompat-v204] pipe poll futex blocking PASS
```

### v205 wait4, nanosleep, and timerfd blocking

- `wait4` now registers a parent wait queue when a matching child exists but has not exited yet.
- `exit_task_pid` wakes the parent child-wait key after SIGCHLD bookkeeping.
- `sched_timeout_wait` records timeout wake accounting.
- `timerfd_settime` wakes waiters on the timerfd wait key.

Fresh runtime evidence:

```text
[ucompat-v205] evidence wait_status=5888 sleep_wakes=2 timer_wakes=2 blocks=3 PASS
[ucompat-v205] wait sleep timer blocking PASS
```

### v206 scheduler regression suite

- The combined runtime suite covers task switch, tick scheduling, blocking, wakeup, wait4, timeout sleep, timerfd readiness, pipe readiness, epoll readiness, and futex wake.

Fresh runtime evidence:

```text
[ucompat-v206] evidence switches=5 blocks=6 wakes=6 timer_wakes=2 PASS
[ucompat-v206] scheduler regression suite PASS
```

## Preserved Markers

The fresh QEMU serial log `.repair_logs/qemu-run-20260509_195556.serial.log` was scanned for all required markers from v151k7 through v206. The scan passed, including the previously sensitive IPC and signal suite markers:

```text
[ucompat-v183] ipc blocking scheduler integration PASS
[ucompat-v188] signal pipe poll ipc suite PASS
[ucompat-v190] final competition kernel readiness PASS
```

## Verification

Commands run:

```bash
cargo build --target riscv64gc-unknown-none-elf
bash ./tools/run-qemu.sh
```

Build result:

- PASS: `cargo build --target riscv64gc-unknown-none-elf`
- PASS: no forbidden build output: `matches any value`, `unreachable pattern`, or `warning: unused variable:`

Fresh QEMU result:

- PASS: all v151k7-v206 markers present in `.repair_logs/qemu-run-20260509_195556.serial.log`
- PASS: v201-v206 evidence lines prove real task state transitions and wakeups
- PASS: no forbidden runtime output in the fresh serial/runtime logs

