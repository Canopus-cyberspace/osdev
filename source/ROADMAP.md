# Roadmap

Phased implementation roadmap for the `source/` kernel. No permanent gates;
temporary gates must be removed or collapsed before the owning stage completes.

See `source/RULES.md` for subsystem rules and `source/ARCHITECTURE.md` for the
layer diagram.

---

## Stage 0 — Boot / BSP Minimal Hardware

**Goal:** both BSPs reach kernel entry with a known machine state and a
byte-level console write primitive.

**Required modules:**
- `arch/riscv64/boot` — entry, early init, stack setup
- `arch/loongarch64/boot` — entry, early init, stack setup
- `arch/*/console` — `console_write(&[u8]) -> usize`
- `arch/*/halt` — halt/reboot primitive

**Forbidden shortcuts:** fake console, serial debug spam, boot self-tests, panic
without typed reason.

**Completion criteria:** both BSPs produce a halting "hello" byte sequence
through the console primitive in QEMU.

**Next-stage blockers:** no other stage may begin real execution until boot is
signed off for both architectures.

---

## Stage 1 — Frame Allocator, Kernel Page Table, Trap Vector

**Goal:** physical frame allocator, static kernel page-table construction, trap
vectors installed.

**Required modules:**
- `core::mm::frame` — frame allocator (boot, then permanent)
- `core::mm::pagetable` — kernel page-table builder (architecture-neutral contract)
- `arch/*/mmu` — MMU/TLB primitives, page-table root materialization
- `arch/*/trap` — trap vector, minimal trapframe save/restore

**Forbidden shortcuts:** fake page tables, skipping TLB ops, page-table root
without validation, trap vector without typed cause dispatch.

**Completion criteria:** kernel boots, trap vector fires, page faults produce
typed `FatalTrap`, frame allocator returns real frames.

**Next-stage blockers:** kernel page table must be functional before user
mappings are attempted.

---

## Stage 2 — User Address Space with Kernel Global Mappings

**Goal:** `AddressSpace` abstraction with kernel-global mappings shared across
all address spaces.

**Required modules:**
- `core::mm::address_space` — `AddressSpace`, `VmArea`, `MappingFlags`
- `core::mm::pagetable` — user page-table creation, kernel-map sharing

**Forbidden shortcuts:** leaking kernel pages to user, skipping VMA tracking,
global TLB flush as default, silent mapping failures.

**Completion criteria:** address space created, kernel mappings present, VMA
manager tracks at least one user region.

**Next-stage blockers:** user mappings must be possible before ELF loading.

---

## Stage 3 — ELF Loader, User Stack, PendingUserEntry

**Goal:** shared ELF parser, user stack builder, aux vector, and
`PendingUserEntry` preparation.

**Required modules:**
- `core::loader::elf` — ELF header/program validation, PT_LOAD extraction
- `core::loader::stack` — argv/envp/auxv construction
- `core::loader::image` — `ExecImage`, segment mapping plan
- `core::task::process` — minimal `Process` identity, `PidAllocator`

**Forbidden shortcuts:** embedded test ELF bytes, fake auxv, path hardcoding,
silent segment truncation.

**Completion criteria:** PT_LOAD segments validated, user stack built, auxv
populated, `PendingUserEntry` constructed from real ELF bytes.

**Next-stage blockers:** `PendingUserEntry` must be ready before MMU activation.

---

## Stage 4 — Controlled MMU Activation

**Goal:** page-table activation and TLB operations through typed boundaries.

**Required modules:**
- `core::mm::activation` — `BoundaryMode::Prepare`, `BoundaryMode::ApplyUnsafe`
- `arch/*/mmu` — `activate_address_space(Prepared<PageTableRoot>)`

**Forbidden shortcuts:** activation without validation, skipping TLB flush
scope, "simulated" activation, `ApplyUnsafe` without real side effect.

**Completion criteria:** address space activated, TLB coherent, kernel
mappings preserved after activation.

**Next-stage blockers:** MMU must be active for user-mode execution.

---

## Stage 5 — Controlled User Return

**Goal:** narrow unsafe `enter_user(Prepared<UserReturnPlan>) -> !` per BSP.

**Required modules:**
- `arch/*/user_entry` — `sret`/`ertn` return path
- `core::task::user_entry` — `UserReturnPlan`, register state

**Forbidden shortcuts:** fake return, "EnterUserReady" after real `sret`,
partial register restore, debug logs in return path.

**Completion criteria:** real `sret`/`ertn` fires, kernel does not return to
Rust after the transition (typed `-> !`).

**Next-stage blockers:** user return must work before any syscall can be tested.

---

## Stage 6 — Minimal Syscall / Trap Loop

**Goal:** trap entry, shared syscall dispatcher, minimal `write`/`exit`/`getpid`.

**Required modules:**
- `arch/*/trap` — syscall trap decode, argument extraction, return-value write
- `core::syscall::dispatch` — table-driven dispatch, errno mapping
- `core::syscall::handlers` — `write`, `exit`, `getpid`

**Forbidden shortcuts:** fake write output, direct UART outside judge path,
testcase branches in dispatch, allocation in syscall hot path.

**Completion criteria:** round-trip trap→dispatch→return for at least 3
syscalls; `write` produces real console output through fd/VFS.

**Next-stage blockers:** minimal syscalls must work before scheduler.

---

## Stage 7 — Timer, Scheduler, `switch_to`

**Goal:** timer interrupts, run queue, wait queue, context switch.

**Required modules:**
- `arch/*/timer` — timer source, timer interrupt delivery
- `core::time` — clock, deadline tracking
- `core::scheduler` — `RunQueue`, `WaitQueue`, `TaskState`, `pick_next`
- `arch/*/context_switch` — `switch_to(&mut Current, &Next)`

**Forbidden shortcuts:** busy-wait sleep, no wait queues, spinlock across
switch, allocation in switch path, plan construction per switch.

**Completion criteria:** timer fires, scheduler picks next, `switch_to`
transfers to another task, blocking sleep wakes on timer.

**Next-stage blockers:** scheduler must work for multi-process tests and
blocking I/O.

---

## Stage 8 — Storage / Rootfs / VFS Expansion

**Goal:** block-device abstraction, block cache, rootfs mount, VFS dentry/inode
cache, path resolution, fd table, procfs/devfs/tmpfs.

**Required modules:**
- `core::drivers::block` — `BlockDevice` trait, `BlockCache`
- `core::fs::vfs` — `Dentry`, `Inode`, `Mount`, `FileOps`, `PathResolver`
- `core::fs::fd_table` — `FdTable`, `FileDescriptor`, `OpenFileDescription`
- `core::fs::procfs` — procfs provider
- `core::fs::devfs` — devfs provider
- `core::fs::tmpfs` — tmpfs provider
- `core::fs::pipe` — pipe with wait-queue blocking
- `arch/*/virtio_blk` — virtio block-device provider binding

**Forbidden shortcuts:** fake sector data, hardcoded paths, cache-hit I/O,
per-syscall path parsing, faking rootfs metadata.

**Completion criteria:** rootfs mounted, path resolution traverses dentry
cache, `open`/`read`/`write`/`close` work through VFS, pipe blocks on empty.

**Next-stage blockers:** rootfs must serve real file data before runner.

---

## Stage 9 — Runner / Official Evaluation

**Goal:** official runner selects program, launches via `fork+exec`, collects
exit status, prints official markers through `judge_output`.

**Required modules:**
- `core::official::judge_output` — isolated marker output
- `core::official::runner` — program selection, launch, result collection
- `core::task::exec` — full `execve` with address-space replacement, cloexec
- `core::task::fork` — fork with COW-ready design
- `core::task::wait` — `wait4`/`waitpid` with exit-state consumption

**Forbidden shortcuts:** fake success, hardcoded testcase output, marker
strings outside `judge_output`, runner bypassing syscall semantics.

**Completion criteria:** full official evaluation produces inspectable log
with no faked results.

**Next-stage blockers:** runner must pass before performance work.

---

## Stage 10 — Performance Convergence

**Goal:** hot-path optimization, cache-hit discipline, replaceable containers,
COW fork, lazy allocation, async-capable block I/O, SMP preparation.

**Required modules:** all subsystems — tighten hot paths per `source/RULES.md`
sections 6–6J and 18A–18B.

**Forbidden shortcuts:** production debug logs for measurement, optimizing
bypassing real behavior, global locks where per-object locks suffice.

**Completion criteria:** benchmarks converge on or exceed `src/` baseline
across all score families without regressions.

---

## General Rules for All Stages

- No permanent gates. Any temporary gate must include removal timeline in the
  stage report.
- Each stage must pass both architecture builds before it is marked complete.
- A stage is not "done" until its Completion criteria items are all true.
- RISC-V and LoongArch progress within a stage must maintain the same neutral
  contract per `source/RULES.md` section 1A.
- Do not add path selection, fake success, boot self-tests, or hardcoded paths
  in any stage.
