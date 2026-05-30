# Source Architecture Blueprint

## 1. Purpose

`source/` is the active official artifact implementation of the kernel.

`source.1/` is an archived/reference snapshot only. Do not edit or build it unless
explicitly requested.

`src/` is legacy/reference code only. It may be studied for verified mechanisms,
but it is not the active implementation target unless explicitly requested.

The root `make all` path is expected to build the official `kernel-rv` and
`kernel-la` artifacts from `source/`.

This document describes the intended architecture boundary and implementation
spine for a high-performance monolithic kernel. Detailed rules live in
`source/RULES.md`; agent operation rules live in root `AGENTS.md`.

---

## 2. Architecture Summary

The kernel is a high-performance monolithic kernel with explicit subsystem
boundaries:

- two BSPs:
  - RISC-V BSP
  - LoongArch BSP
- one shared core for operating-system semantics
- narrow architecture contracts for traps, timers, IRQ, MMU/TLB, user entry,
  context switch, console, and device binding
- isolated official output through `source/official/judge_output`
- quiet-by-default production behavior
- no fake success, no testcase-specific paths, no permanent gates

The kernel is monolithic for performance and implementation simplicity, but
ownership boundaries must remain strict.

Architecture-specific code owns hardware mechanisms. Shared core code owns
kernel semantics.

---

## 3. Layer Diagram

```text
+--------------------------------------------------+
| Runner / official adapter                         |
| chooses requests, never fakes subsystem success   |
+-------------------------+------------------------+
                          |
+-------------------------v------------------------+
| Kernel orchestration                              |
| boot, context, runtime config, trap coordination  |
+-------------------------+------------------------+
                          |
+-------------------------v------------------------+
| Shared core                                       |
| syscall, task, scheduler, mm, fs, loader, time,   |
| block/cache, sync, process lifecycle              |
+-------------------------+------------------------+
                          |
+-------------------------v------------------------+
| Neutral BSP contracts                             |
| trap, timer, IRQ, MMU/TLB, user return, switch,   |
| console byte, block provider, halt                |
+------------+---------------------------+---------+
             |                           |
+------------v------------+  +-----------v----------+
| RISC-V BSP              |  | LoongArch BSP        |
| CSR, satp, sret, MMIO   |  | CSR/TLB, ertn, PCI   |
+------------+------------+  +-----------+----------+
             |                           |
+------------v---------------------------v----------+
| Hardware / QEMU devices                            |
| CPU, timer/IRQ, UART, virtio block, future net     |
+--------------------------------------------------+
```

---

## 4. Boot Boundary

Boot owns one-time initialization and runtime-mode selection.

Boot may:

- establish early stack and kernel entry
- zero or initialize kernel runtime state
- initialize early allocator inputs
- install BSP capability providers
- select a default `PlanOnly` or explicitly requested controlled mode
- install trap vectors only under the configured explicit policy
- construct the initial `KernelContext`

Boot must not:

- choose official testcase paths
- hardcode `/musl`, `/busybox`, `/bin`, `/lib`, `basic`, or testcase names
- create fake `ProgramLaunchRequest`s
- print official markers directly
- fake rootfs, syscall, user-entry, process, or runner success
- enable hardware execution by default

Boot is setup path, not hot path.

---

## 5. BSP Boundary

BSP owns hardware and ISA mechanics only:

- boot entry and early machine state
- trap vector setup and trapframe save/restore boundary
- syscall trap handoff
- interrupt enable, disable, acknowledgement, and routing primitives
- timer source and timer programming
- MMU, page-table root activation, and TLB primitives
- final user entry / return instruction boundary
- context-switch assembly boundary
- low-level console byte primitive
- platform MMIO / PCI / virtio binding
- halt and reboot primitives

BSP must not own kernel semantics:

- syscall behavior
- VFS, fd, and open-file-description semantics
- scheduler policy
- task, process, wait, fork, exec, or exit lifecycle
- path resolution
- procfs, devfs, tmpfs, or pipe semantics
- official testcase or score-family behavior

BSP translates hardware events into shared kernel events such as syscall trap,
page fault, timer interrupt, external interrupt, illegal instruction, or fatal
trap. Shared core handles semantics.

---

## 6. Shared Core Boundary

Shared core owns reusable kernel behavior across both architectures:

- `core::syscall`: dispatcher, argument validation, errno conversion, blocking
  contract
- `core::task`: process, task, thread, pid, fork, exec, exit, wait, pending user
  entry ownership
- `core::scheduler`: run queue, wait queue, sleep, wakeup, timer wakeups,
  scheduling decisions
- `core::mm`: frame allocation, address spaces, page tables, VMAs, user copy,
  page faults, mapping policy
- `core::fs`: VFS, fd table, open file descriptions, path resolver, inode,
  dentry, pipes, procfs, devfs, tmpfs
- `core::loader`: ELF, load segments, user stack, argv/envp/auxv, entry context
- `core::time`: clock, sleep, timeout, timer API
- `core::drivers` or `core::block`: neutral block-device, request, completion,
  and cache contracts
- future `core::sync` and `core::net`: synchronization and network stack
  semantics

Shared core may call BSP only through narrow neutral contracts. It must not
import concrete `riscv64` or `loongarch64` modules.

---

## 7. Kernel Orchestration Boundary

`source/kernel` coordinates subsystems but must not become the owner of every
state.

Kernel orchestration may:

- hold runtime policy/configuration
- coordinate provider registry access
- coordinate program launch read-through
- dispatch traps to shared core
- connect scheduler decisions to architecture-neutral switch contracts
- connect pending user entry to activation and final-return preparation

Kernel orchestration must not own:

- process lifecycle
- `PendingUserEntry` as a mirrored second source of truth
- scheduler run queues
- VFS tree
- page tables and frame ownership
- block-cache contents
- architecture-specific hardware state

Ownership belongs to the subsystem that defines the state.

---

## 8. Runtime Ownership Map

```text
Boot                 -> one-time init and runtime mode selection
BSP                  -> hardware mechanisms and unsafe boundaries
KernelContext        -> orchestration and policy/config coordination
ProcessTable         -> process/task/thread lifecycle and PendingUserEntry
Scheduler            -> run queues, wait queues, blocking, wakeups, decisions
MemoryRuntime        -> frames, page tables, backing storage, hardware roots
VFS / FD             -> files, fd table, OFD, inode, dentry, path resolver
BlockCache           -> cached sectors and commit validation
PageCache            -> cached file pages and population state
Loader               -> ELF, user stack, auxv, exec memory plan
Syscall dispatcher   -> syscall routing and ABI result conversion
Runner               -> workload/request selection only
Official output      -> marker output through judge_output only
```

Every major runtime state must have one owner. Other layers may hold prepared
views, handles, or borrowed references, but not mirrored ownership.

---

## 9. Hardware-First Vertical Implementation Spine

The first implementation should establish a minimal real hardware/user path
before broad compatibility features.

Recommended spine:

1. Boot entry and early stack.
2. Fatal-only console byte primitive.
3. Physical memory discovery and frame allocator.
4. Kernel page table with stable kernel global mappings.
5. Trap vector installation under explicit boot policy.
6. User address space with inherited/preserved kernel global mappings.
7. ELF loader, user stack, argv/envp/auxv basics, and `PendingUserEntry`.
8. Controlled MMU activation.
9. Controlled final user return.
10. Minimal syscall loop: `write`, `exit`, `getpid`, and basic memory syscalls.
11. Block device and block cache.
12. Rootfs/VFS file read and program launch.
13. Scheduler, timer, wait/wakeup, and context switch.
14. Runner and official evaluation integration.
15. Performance convergence: cache fast paths, page cache, COW, fair scheduling,
    futex, signal, TLS, networking, and SMP.

This is real kernel behavior, not fake official success output.

---

## 10. Module Boundaries

### Boot

Interface examples:

```rust
boot_kernel(config) -> BootResult
```

Boot chooses runtime mode and initializes kernel subsystems. It does not select
program paths or fake evaluation behavior.

### BSP

Interface examples:

```rust
install_trap_vector(...)
activate_address_space(...)
return_to_user(...)
switch_context(...)
program_timer(...)
read_block_sector(...)
console_write_byte(...)
```

Unsafe hardware actions must have one narrow unsafe entry point. Safe shared
code prepares typed inputs.

### Task / Process

Interface examples:

```rust
commit_exec(...)
current_pending_user_entry(...)
prepare_user_entry_plan(...)
consume_pending_user_entry_for_final_return(...)
exit(...)
wait(...)
```

`PendingUserEntry` is process-owned. Planning, validation, or activation failure
must not consume it. It is consumed only at the real final non-returning handoff
or after a durable retryable ownership transition.

### Scheduler

Interface examples:

```rust
enqueue(...)
block_current(...)
wake(...)
on_tick(...)
pick_next(...)
commit_switch(...)
```

The scheduler owns scheduling decisions and queue membership. It does not own
process memory or architecture switch assembly.

### Memory

Interface examples:

```rust
alloc_frame(...)
map_user_page(...)
map_kernel_global(...)
hardware_root_readiness(...)
kernel_mapping_readiness(...)
user_copy_in(...)
user_copy_out(...)
```

Every user address space intended for activation must have a real hardware root
and must preserve or inherit required kernel mappings.

### VFS / FD

Interface examples:

```rust
lookup(...)
open(...)
read(...)
write(...)
fd_get(...)
```

VFS owns path resolution, inodes, dentries, fd table, and open-file-description
semantics. No syscall may implement ad-hoc path parsing.

### Storage

Interface examples:

```rust
cache_lookup(...)
plan_read_through_miss(...)
commit_completed_sector(...)
```

Block-cache hits must not call providers. Sector bytes enter the cache only
after real completion and strict validation.

### Loader

Interface examples:

```rust
parse_elf(...)
build_exec_memory_plan(...)
build_user_stack(...)
```

Loader is arch-neutral until final user return. It must not embed test programs
or historical fake data.

### Runner

Interface examples:

```rust
next_request(...)
on_process_exit(...)
```

Runner may choose a `ProgramLaunchRequest` only after real subsystem behavior is
available. Runner never implements syscall semantics or fake testcase success.

### Module Decomposition Guide

Active `source/` directory layout:

```text
source/
  build/              linker scripts, build helpers
  bin/                kernel entry binaries (riscv64, loongarch64)
  arch/
    contract/         architecture-neutral hardware boundary contracts
    riscv64/          RISC-V BSP (CSR, satp, sret, MMIO)
    loongarch64/      LoongArch BSP (CSR/TLB, ertn, PCI)
  kernel/             boot orchestration, runtime config, trap coordination
  core/
    task/             process, thread, pid, fork, exec, wait, exit
    scheduler/        run queue, wait queue, sleep, wakeup, switch admission
    mm/               frame allocator, page table, VMA, user-copy, page fault
    syscall/          dispatcher, ABI conversion, errno, syscall group routing
    fs/               VFS, fd table, OFD, inode, dentry, path, pipe, procfs, devfs, tmpfs
    storage/          block device, block cache, page cache, request/completion/writeback
    loader/           ELF, user stack, argv/envp/auxv, entry context
    time/             clocks, timer queue, sleep, timeout
    sync/             synchronization primitives, locking abstractions
    drivers/          generic driver interfaces (not BSP hardware policy)
    net/              future shared network/socket semantics
    compat/           future Linux ABI compatibility (signal, futex, poll, TLS)
  official/           judge output adapter only
  docs/               implementation documentation
```

Ownership rules:
- `arch/` owns BSP hardware and ISA mechanics only.
- `arch/contract/` owns architecture-neutral hardware boundary contracts.
- `kernel/` owns boot/runtime orchestration, not subsystem state ownership.
- `core/` owns architecture-neutral OS semantics; no `unsafe` blocks, no arch imports.
- `official/` owns judge output adapter only; subsystems must not print markers directly.

Do not create dumping-ground files (`types.rs`, `utils.rs`, `common.rs`,
`helpers.rs`) without a clear owning module and narrow scope.

New modules should be created only when the new behavior introduces a new owner,
execution tier, hardware boundary, cache-miss path, syscall group, or public
subsystem API. Small helpers and local validation stay in the existing owner
module. See `source/RULES.md` §25 for the full decomposition policy.

---

## 11. Validation Boundary

Validation is not official score unless the full official workflow ran and the
log was inspected.

Validation categories:

- compile-time Rust validation
- architecture build validation
- forbidden-pattern grep
- architecture-boundary grep
- host-side pure-logic tests
- QEMU integration tests only when explicitly requested
- official evaluation only when explicitly requested

Production validation must not rely on boot self-tests or production debug logs.

---

## 12. Build Boundary

Root `make all` must build `kernel-rv` and `kernel-la` from `source/`.

The active build path should use:

```text
source/Cargo.toml
source/build.rs
source/build/riscv64.ld
source/build/loongarch64.ld
source/bin/riscv64_kernel.rs
source/bin/loongarch64_kernel.rs
```

`source.1/` and `src/` must not participate in active `source/` builds unless
explicitly requested.

Do not create duplicate active linker-script locations. Prefer `source/build/`
as the single linker-script directory.

---

## 13. Documentation Boundary

Documentation should be operational and non-duplicative.

Recommended active documents:

```text
AGENTS.md
source/RULES.md
source/ARCHITECTURE.md
source/ROADMAP.md
source/IMPLEMENTATION_STATUS.md
source/EVALUATION_REQUIREMENTS.md
source/EVALUATION_COVERAGE.md
source/VALIDATION.md
source/PROMPT_TEMPLATES.md
```

Avoid active `.bak` policy files, duplicate roadmaps, or stale audit documents.
If historical notes must be retained, put them in an archive or rely on git
history.

---

## 14. Evaluation Mapping Summary

- `basic`: syscall ABI, task/process, mm, fd, loader, time
- `busybox`: fs, procfs, devfs, path resolution, pipes, task/process, loader
- `cyclictest`: timer source, scheduler wakeup latency, wait queues
- `iozone`: VFS, page cache, block cache, positioned/vector I/O
- `lmbench`: syscall, scheduler, pipe, fs, mm, fork/exec latency
- `libcbench` / `libctest`: mm, task, futex, signal, loader, fs, time, socket
- `lua`: fs, time, random source, mm, loader, user execution correctness
- `iperf` / `netperf`: network stack, sockets, virtio-net, readiness waits
- `ltp`: broad Linux compatibility across shared subsystems

Official cases must exercise real kernel behavior and must not be special-cased.

---

## 15. Non-Goals for Initial Implementation

The initial implementation does not need:

- full TCP/IP
- full ext4
- full SMP
- full dynamic loader
- full CFS-style scheduler
- full signal/futex/TLS compatibility
- full page-cache/writeback performance

Interfaces must still leave room for these capabilities without large rewrites.

---

## 16. Final Principle

Build the kernel around the shortest real execution path first:

```text
real boot
-> real hardware primitive
-> real memory root
-> real trap vector
-> real user entry
-> real syscall loop
-> real process/file/memory behavior
```

Do not build the kernel around permanent gates, fake readiness, official-path
shortcuts, or placeholder state machines.

Fast paths are production contracts. Slow paths may validate and recover; hot
paths must consume prepared compact state.
