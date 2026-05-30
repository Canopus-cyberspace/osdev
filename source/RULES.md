# RULES.md

# Next-Generation High-Performance Monolithic Kernel Rules

`source/` is the active official artifact implementation of this operating
system kernel. The root `make all` path is expected to build `kernel-rv` and
`kernel-la` from `source/`.

The old `src/` tree is legacy/reference code unless the user explicitly asks to
patch it.

This document is the architectural rulebook for `source/`. It is not a
temporary debugging guide.

---

## 1. Core Direction

`source/` must implement a high-performance monolithic kernel with:

- two architecture BSPs:
  - RISC-V BSP
  - LoongArch BSP
- one shared kernel core
- unified scheduler
- unified task/process/thread model
- unified address-space and memory-management model
- unified syscall dispatcher
- unified fd/VFS/file model
- unified loader and user-entry model
- cache-aware block/file I/O design
- quiet-by-default production behavior

The kernel may be monolithic, but subsystem boundaries must remain explicit.

Architecture-specific code handles hardware. Shared core code handles
operating-system semantics.

---

## 1A. Dual-Architecture Symmetry Rule

RISC-V and LoongArch must remain at the same OS abstraction level.

For every runtime capability advanced on RISC-V, provide the matching
LoongArch neutral contract and `KernelContext` integration in the same design
stage.

Allowed asymmetry:

- hardware transport details
- MMU/TLB details
- trap/user-return instruction details
- context-switch register layout
- device discovery mechanism
- hardware readiness status

Forbidden asymmetry:

- RISC-V has a kernel/core integration point while LoongArch is omitted
- RISC-V has provider/commit validation while LoongArch lacks the same contract
- shared core assumes only RISC-V can provide a capability
- LoongArch fakes success to match RISC-V

If RISC-V can execute but LoongArch cannot, LoongArch must still expose:

- the same neutral provider contract
- the same `KernelContext` integration point
- the same commit validation semantics
- typed non-fake status such as `DiscoveryRequired`, `NotReady`, or `Unsupported`

LoongArch hardware uncertainty must produce typed non-ready status, not fake
success.

Dual-architecture symmetry requires matching contracts and observable status,
not identical implementation progress or identical fast-path readiness.

A real fast path may land first on one architecture if the other architecture
exposes the same neutral contract and returns a typed non-fake status such as
`NotReady`, `Unsupported`, or `DiscoveryRequired`.

Symmetry must not force the fast architecture to route through slow neutral
scaffolding after its architecture-specific fast path is validated.


---

## 2. Active Artifact Rule

`source/` owns the active `kernel-rv` and `kernel-la` artifacts.

Rules:

- keep `make all` working
- keep `kernel-rv` and `kernel-la` generated from `source/`
- do not patch `src/` unless explicitly requested
- do not change official output behavior except through the isolated
  `source/official/judge_output` path
- do not claim score unless full official evaluation completed and the log was
  inspected

Development must remain subsystem-by-subsystem. No big-bang rewrite, fake
success path, or testcase-specific shortcut is allowed.

---

## 3. Forbidden Historical Patterns

`source/` must not contain historical compatibility scaffolding.

Forbidden names and patterns:

- `ucompat`
- `UCOMPAT`
- `legacy`
- `history`
- `history_bus`
- `evidence_bus`
- `run_vxxx`
- `self_test`
- `startup_regression`
- `compat regression`
- testcase fake success paths
- parser-shaped fake official output

Historical code may be studied, but must not be copied as-is.

If an old implementation contains a useful mechanism, extract the real kernel
concept and rename it according to the owning subsystem.

---

## 4. Output, Logging, and Serial Policy

The production kernel must be quiet by default.

No normal boot logs, per-syscall logs, per-trap logs, scheduler logs,
fd/VFS/mm/process debug spam, driver init spam, or BusyBox command debug spam.

Allowed serial output:

1. official group markers
2. official testcase success/fail lines
3. fatal panic or unrecoverable trap diagnostics
4. explicitly enabled debug-build diagnostics

All judge-visible output must go through a dedicated official interface such as
`source/official/judge_output`. Subsystems must not print official marker
strings directly.

Debug logging must be compile-time disabled by default. Direct UART writes are
allowed only in BSP low-level console implementation, official judge-output
path, or fatal panic path.

---

## 5. No Startup Self-tests

The production kernel must not run internal self-tests during boot.

Forbidden:

- `self_test()`
- startup regression suites
- boot-time fd/VFS/mm/task tests
- historical evidence bus
- compatibility check chains
- synthetic PASS/FAIL debug sequences

Correctness must come from compile-time interfaces, type boundaries,
host-side deterministic tests, external QEMU integration tests, subsystem
invariants, and controlled diagnostics disabled by default.

---

## 6. Fast-Path-First Performance Requirement

`source/` must be designed for high-performance evolution from the start.

`source/` must be designed around real fast paths first.

The first implementation may use simple data structures, but every
performance-sensitive subsystem must expose a fast-path contract from the
beginning.

A subsystem is not considered complete only because it is functionally correct.
It must also define its hot path, slow path, setup path, allocation behavior,
lock behavior, expected complexity, cache-hit behavior, and replacement
strategy for simple first data structures.

Correctness-only implementations are acceptable only as temporary bring-up
code. Production-facing subsystem interfaces must not force future fast paths
to pass through validation scaffolding, provider discovery, path parsing,
debug formatting, allocation, or large state-machine construction.

Required long-term capabilities:

- unified scheduler with replaceable run queue
- wait queues for blocking
- timer wakeups
- address-space abstraction and VMA manager
- lazy allocation and COW-ready fork design
- page-fault handler and centralized user-copy path
- fd table + open-file-description semantics
- inode/dentry/path resolver separation
- page cache and block cache
- block-device abstraction
- no case-specific bypasses

First implementation may be simple, but interfaces must not block
high-performance replacements.

---

## 6A. Temporary Gating, Semantic Minimalism, and Fast-Path Rule

Existing default-safe gating is transitional safety only.

Avoid adding new gating layers. Reuse `RuntimeExecutionPolicy`, provider
readiness, and existing subsystem result types whenever possible.

Any new gate must be temporary and must report:

- why existing `RuntimeExecutionPolicy` or provider readiness is insufficient
- whether it sits on init path, slow path, or hot path
- when it should be removed or collapsed
- the intended final fast-path behavior

Permanent hot-path gates are forbidden.

Preferred design:

- validate capabilities during boot, provider install, mount, open, exec, or task setup
- derive compact fast-path configuration
- keep cache-hit paths short
- keep detailed typed failures on slow/error paths
- avoid repeated policy matching inside tight loops
- avoid revalidating switch ABI layout on every switch after init validation
- avoid re-probing rootfs metadata on every lookup after mount population

Do not use gating to delay implementation indefinitely. If a path is ready to
execute under explicit policy, prefer controlled real execution over adding
another readiness layer.

---


Semantic minimalism is mandatory.

A gate, mode, state, provider result, enum variant, field, trait method, or
policy is allowed only when it represents a real runtime fact, hardware-visible
condition, control-flow boundary, architecture capability, data dependency, or
recoverable error.

Do not add safety-theater layers whose only purpose is to say that code is not
called by default, not executed by default, not confirmed, not enabled, or only
planned.

Safety must be enforced through:

- Rust `unsafe` boundaries
- typed validated inputs
- feature-gated real hardware backends
- capability/provider installation
- explicit runtime policy
- tests and invariants

Safety must not be enforced by accumulating redundant booleans, duplicate
state fields, placeholder enum variants, or nested policy layers.

---

## 6B. Real Semantics Only Rule

`source/` must model real kernel semantics, not defensive scaffolding.

Every field, enum variant, state, mode, trait method, provider contract, and
policy must correspond to one of:

- a real hardware-visible state
- a real control-flow boundary
- a real architecture capability
- a real data dependency
- a real ownership transition
- a real error that the caller can observe, report, or recover from

If a construct only documents intent, delays implementation, repeats another
construct, or says that something is "not executed by default", it must not be
added.

### Forbidden safety-theater names and patterns

Forbidden names and patterns include:

```text
*_CONFIRMED
*_ENABLE_* when another runtime policy already exists
*_BY_DEFAULT
called_by_default
executes_by_default
executes_*_by_default
hardware_executed when it does not name the exact hardware action
ExplicitOnly
PreparedButNotExecuted when Prepared<T> can express the same state
PlanOnly plus ValidateOnly when they produce the same observable result
DryRunReady when no distinct dry-run behavior exists
NotCalled when it is not externally observable
```

Allowed replacement patterns:

```rust
pub enum BoundaryMode {
    Inspect,
    Prepare,
    ApplyUnsafe,
}

pub struct Prepared<T> {
    inner: T,
}

pub struct Applied<T> {
    inner: T,
}
```

Only `Prepared<T>` may cross into an unsafe architecture boundary.

### One boundary, one mode

Each hardware or control-flow boundary may have only one mode enum.

Forbidden:

```rust
pub enum ExecutionMode {
    PlanOnly,
    ValidateOnly,
    WithResult(...),
}

pub enum CommitPolicy {
    PlanOnly,
    ValidateOnly,
    PrepareUnsafeBoundary,
    ApplyUnsafe,
}
```

Required:

```rust
pub enum BoundaryMode {
    Inspect,
    Prepare,
    ApplyUnsafe,
}
```

`Inspect` means: build and validate a neutral plan without preparing or applying
the unsafe boundary.

`Prepare` means: validate all inputs and construct the typed prepared boundary,
but do not execute the unsafe operation.

`ApplyUnsafe` means: execute the real unsafe hardware or control-flow operation.

A plan must be validated when created. A non-validated plan is not a valid
semantic object.

### No boolean dispatch for semantic modes

Do not pass multiple booleans to restate what a mode already means.

Forbidden:

```rust
apply_if_matching(step, true, false, false);
apply_if_matching(step, false, true, false);
apply_if_matching(step, true, true, true);
```

Required:

```rust
apply_step(mode, step);
```

The mapping from mode to allowed action must exist in exactly one place:

```rust
fn step_allowed(mode: BoundaryMode, step: StepKind) -> bool;
```

### One lifecycle state field per object

A struct must not contain multiple fields that describe the same lifecycle.

Forbidden:

```rust
pub struct ApplyPlan {
    readiness: ApplyReadiness,
    unsafe_status: UnsafeBoundaryStatus,
}
```

Required:

```rust
pub enum ApplyState<F> {
    NotReady(F),
    Ready,
    Prepared,
    Applied,
    Unsupported(F),
}

pub struct ApplyPlan {
    state: ApplyState<ApplyFailure>,
}
```

Do not create states that can contradict each other, such as:

```text
readiness = Ready + status = NotCalled
state = Applied + result = EnterUserReady
hardware_executed = true + PreparedButNotExecuted
```

### State vocabulary

State names must have exactly one meaning:

```text
Planned     = valid neutral plan exists; no hardware side effect occurred
Prepared    = all inputs for the unsafe boundary are ready; boundary not executed
Applied     = a real side effect occurred
NotReady    = required runtime input is missing
Unsupported = backend/architecture cannot implement this operation
Invalid     = caller supplied malformed or contradictory input
```

`Applied` must not mean simulated execution.

A real non-returning user transition such as `sret`, `ertn`, or equivalent must
not return `EnterUserReady` after execution. If the transition really executes,
the function should normally be typed as:

```rust
pub unsafe fn enter_user(prepared: Prepared<UserReturnPlan>) -> !;
```

### No placeholder production steps

Do not add production sequence steps with `None` just to reserve future design
space.

Forbidden:

```rust
sequence.push_address_space_activation(None);
sequence.push_tlb_flush(None);
sequence.push_user_return_preparation(None);
```

Required:

```rust
if let Some(plan) = address_space_plan {
    sequence.push_address_space_activation(plan);
}
```

A missing plan must represent a real runtime condition, not a future TODO slot.

### No reserved enum variants without production meaning

Do not add enum variants for imagined future failures.

An enum variant is allowed only if all are true:

- production code can construct it
- tests or integration checks cover it
- it has a distinct reporting or recovery meaning
- it cannot be represented by an existing variant

Forbidden unless actually used:

```text
UnsafeExecutionDisabled
MissingSwitchToSymbol
SwitchToAssemblyMissing
BoundaryNotApplied
```

### No duplicated capability contracts

Fine-grained capability traits own hardware operations.

Top-level traits may compose capability traits, but must not redeclare the same
methods.

Allowed:

```rust
pub trait Bsp:
    BspInterruptControl
    + BspTimer
    + BspMmu
    + BspUserEntry
{
    fn architecture(&self) -> BspArchitecture;
    fn boot_state(&self) -> BspBootState;
    fn console_write(&self, bytes: &[u8]) -> usize;
    fn halt(&self) -> !;
}
```

Forbidden:

```rust
pub trait Bsp {
    fn enable_interrupts(&self);
    fn timer_now(&self) -> u64;
    fn flush_tlb(&self);
    fn activate_address_space(&self, root: PageTableRoot);
    fn enter_user(&self, context: UserEntryContext) -> !;
}
```

if those methods already exist in narrower capability traits.

### Unsafe boundary rule

Every unsafe hardware action must have exactly one narrow unsafe entry point.

The safe side prepares typed inputs.

The unsafe side performs the real operation.

Allowed:

```rust
pub unsafe fn apply_address_space(
    root: Prepared<PageTableRoot>,
) -> AddressSpaceApplyResult;
```

Forbidden:

```rust
prepare_apply_address_space(...)
maybe_apply_address_space(...)
apply_if_enabled(...)
apply_if_confirmed(...)
apply_if_matching(...)
```

Do not stack multiple "maybe apply" layers around one hardware action.

### Policy location rule

A policy may live in one of these places, but not several at once:

- build feature
- runtime execution mode
- provider readiness
- capability trait implementation
- type-state transition

Do not encode the same permission as a feature flag, runtime enum, boolean
constant, and provider status simultaneously.

Choose the mechanism closest to the real semantic boundary.

---

## 6C. Hot Path Discipline Rule

Hot paths must be short, allocation-free, logging-free, and policy-light.

The following are hot paths:

- trap entry and trap return
- syscall dispatch and syscall return
- context switch
- scheduler enqueue/dequeue/pick-next
- wakeup path
- timer interrupt handling
- page fault fast path
- user-copy fast path
- fd lookup
- VFS dentry/inode cache hit lookup
- page-cache hit
- block-cache hit
- pipe read/write readiness check
- futex wait/wake fast path once implemented

Hot paths must not:

- allocate heap memory
- format strings
- print logs
- scan unbounded lists
- re-parse paths
- re-check static architecture capabilities
- re-run provider discovery
- perform repeated policy matching
- construct large temporary plan/result objects
- clone large structs
- take locks with unknown nesting
- sleep while holding spinlocks
- call official/judge output
- branch on testcase names, binary paths, or runner metadata

Hot paths may return compact typed errors, but detailed diagnostic construction
belongs to slow/error paths.

If a hot path needs a decision, derive a compact cached configuration during
boot, mount, open, exec, fork, task creation, or provider installation.

---

## 6D. Allocation and Object Lifetime Rule

Performance-sensitive kernel paths must make allocation ownership explicit.

Rules:

- trap, syscall, scheduler, context-switch, interrupt, and cache-hit paths must not allocate
- blocking paths may allocate only before enqueueing into wait queues
- page fault handlers may allocate only through the memory subsystem's explicit fault-resolution path
- VFS lookup may allocate dentries/inodes only on cache miss
- block/page cache may allocate cache entries only on miss or population
- `fork`, `execve`, `open`, `mount`, and task creation may allocate because they are setup/slow paths
- driver interrupt paths must not allocate unless the allocator is explicitly IRQ-safe
- no hidden allocation inside formatting, collection growth, string construction, or debug helpers

Required allocator structure:

- early boot allocator for boot-only setup
- frame allocator for physical pages
- kernel heap for general kernel objects
- slab/object cache or typed pool for high-frequency objects
- optional per-CPU or per-core cache layer when SMP is introduced

High-frequency objects should have stable ownership and reuse strategy:

- task/thread objects
- wait queue nodes
- file descriptors
- open file descriptions
- dentries
- inodes
- page-cache entries
- block-cache entries
- trap frames
- scheduler nodes

---

## 6E. Locking, Interrupt, and Critical Section Rule

Locking design must preserve fast paths and interrupt latency.

Rules:

- every lock must document what data it protects
- every lock must document whether it may be acquired in interrupt context
- every lock must document whether sleeping is allowed while held
- spinlocks must guard only short critical sections
- mutexes may guard sleepable subsystem state but must not be taken in interrupt context
- interrupt-disabled sections must be minimal and bounded
- no path may hold a spinlock across user-copy, VFS lookup, block I/O, page fault resolution, allocation, or scheduler sleep
- no lock may be acquired in inconsistent order
- nested lock order must be documented for each subsystem
- hot paths must avoid global locks when a per-object, per-cache, per-task, or per-CPU lock can express ownership
- read-mostly data should use immutable snapshots, sequence counters, RCU-style design, or cached handles where appropriate

Forbidden:

- one global kernel lock around syscall dispatch
- one global filesystem lock around all VFS operations
- one global scheduler lock around unrelated queues once multiple queues exist
- lock-protected debug logging in production paths
- disabling interrupts as a substitute for proper ownership

---

## 6F. Execution Path Tier Rule

Every performance-sensitive function must belong to exactly one execution tier:

```text
BootPath   = one-time boot or architecture initialization
SetupPath  = exec, fork, open, mount, provider install, task creation
SlowPath   = cache miss, page fault slow path, blocking setup, I/O submission
HotPath    = syscall dispatch, trap return, fd lookup, cache hit, wakeup, scheduling
IrqPath    = interrupt entry, timer tick, IRQ acknowledgement, completion handler
```

Rules:

- `BootPath`, `SetupPath`, and `SlowPath` may validate, allocate, and build plans.
- `HotPath` must consume prepared state and compact handles.
- `IrqPath` must be bounded, non-blocking, allocation-free, and logging-free.
- A `HotPath` function must not call a `SetupPath` or `SlowPath` function unless the call is explicitly named as a miss path.
- A `HotPath` function must not construct high-level plans.
- A `HotPath` function must not perform provider discovery.
- A `HotPath` function must not format diagnostics.
- A `HotPath` function must not branch on official cases, binary paths, or runner metadata.

Function names should make path tier visible when useful:

```text
*_fast
*_slow
*_miss
*_prepare
*_commit
*_irq
```

A patch that moves validation, allocation, discovery, or diagnostic construction
from setup/slow path into hot path is a performance regression unless explicitly
justified.

---

## 6G. No High-Level Plans in Hot Paths

High-level planning objects are setup-path objects, not hot-path objects.

Forbidden in hot paths:

- building pipeline objects
- rebuilding architecture boundary plans
- revalidating ABI layout
- rechecking static architecture capabilities
- recomputing provider readiness
- constructing large enum/result trees
- cloning scheduler, task, VFS, or MM metadata
- converting between multiple equivalent state representations

Allowed in hot paths:

- raw task/context pointer
- compact task id
- prepared context pointer
- prepared page-table token
- cached file/inode/dentry pointer
- cached capability bit
- compact error code
- small by-value status enum
- direct function pointer or vtable call when unavoidable

Context switch hot path must consume an already prepared saved-context pair.
Trap return hot path must consume an already prepared return frame.
Cache-hit path must consume an already resolved cache entry.
Syscall dispatch must consume decoded register arguments and dispatch directly
to the owning subsystem.

---

## 6H. Complexity Budget Rule

Every performance-sensitive subsystem must document expected complexity for
its common operations.

Required examples:

```text
scheduler.enqueue       expected O(1) or documented queue-dependent cost
scheduler.pick_next     expected O(1), O(log n), or documented policy cost
wait_queue.wake_one     must not scan unrelated wait queues
fd_table.get            expected O(1)
vma.find                simple implementation may be O(n), interface must allow O(log n)
dentry_cache.lookup     expected O(1) average or documented cache policy
page_cache.lookup       expected O(1) average or tree/radix cost
block_cache.lookup      expected O(1) average
timer.next_deadline     expected O(1), O(log n), or timer-wheel bucket cost
```

Rules:

- If the first implementation is linear, the interface must hide that fact.
- Linear scan is allowed only when explicitly marked as temporary or bounded.
- Unbounded linear scan in syscall, trap, scheduler, wakeup, fd, cache-hit, or IRQ path is forbidden.
- A patch that increases common-case complexity must explain why.

---

## 6I. Production Build Performance Rule

The default production build must optimize for real execution, not diagnostics.

Production build must not include:

- boot self-tests
- debug log formatting
- per-event diagnostic object construction
- symbolic trace strings in hot paths
- heavyweight state names in fast-path results
- statistics updates that require global locks
- debug-only provider validation in repeated hot paths

Diagnostic build may include bounded counters, trace buffers, invariant checks,
slow-path debug formatting, QEMU-only assertions, and host-test-only validation
helpers.

Diagnostic behavior must be compile-time gated and must not change official
output behavior or production hot-path structure.

---

## 6J. Replaceable Data Structure Contract

Simple containers are allowed only behind subsystem-owned interfaces.

Allowed first implementations:

```text
Vec
VecDeque
fixed array
linear list
simple bitmap
simple hash map
```

Forbidden:

- exposing `Vec` layout as cross-subsystem API
- making callers depend on index order unless order is semantic
- requiring callers to scan internal collections
- returning mutable references that prevent future tree/hash/radix replacement
- embedding container-specific behavior into syscall, scheduler, VFS, or MM logic

Required replacement-ready interfaces:

```text
RunQueue       -> supports queue/tree/priority/per-CPU replacement
WaitQueue      -> supports intrusive/per-object replacement
VmaMap         -> supports interval tree or range map replacement
FdTable        -> supports bitmap/radix/table replacement
DentryCache    -> supports hash/LRU replacement
PageCache      -> supports radix/xarray/tree replacement
BlockCache     -> supports hash/LRU/clock replacement
TimerQueue     -> supports heap/timer-wheel replacement
```

A subsystem may start simple, but callers must not know or care which container
backs it.

---

## 7. Architecture BSP Rules

There must be exactly two architecture BSP families:

- `source/arch/riscv64`
- `source/arch/loongarch64`

BSP code may implement only low-level architecture and board responsibilities:
boot entry, trap vector setup, trapframe save/restore, syscall trap handoff,
interrupt enable/disable, timer source, MMU/TLB primitives, address-space
activation primitive, low-level console write, low-level device MMIO mapping,
user entry/return, halt/reboot.

BSP code must not implement syscall semantics, VFS, fd table, process
lifecycle, scheduler policy, memory-management policy, official testcase
behavior, BusyBox behavior, path resolution, or procfs/devfs/tmpfs semantics.

BSP must translate architecture events into shared kernel events:
`SyscallTrap`, `PageFault`, `TimerInterrupt`, `ExternalInterrupt`,
`IllegalInstruction`, and `FatalTrap`.

Shared core handles semantics.

---

## 8. Shared Core Requirement

All kernel semantics must live in shared core.

Required shared subsystems:

```text
core::scheduler
core::task
core::mm
core::fs
core::syscall
core::loader
core::time
core::sync
core::drivers_interface
```

Architecture code may call shared core. Shared core may call BSP primitives
through narrow interfaces. Shared core must not depend on architecture-specific
policy.

---

## 9. Scheduler Rules

The scheduler is shared across architectures.

Required concepts include `Scheduler`, `RunQueue`, `WaitQueue`, `TaskState`,
`BlockedReason`, `TimerWakeup`, and `WakeupSource`.

Required task states:

```text
Running
Ready
Blocked
Sleeping
Zombie
Exited
```

Rules:

- no busy waiting for sleep, waitpid/wait4, pipe readiness, futex readiness, or timer waits
- blocking must use wait queues
- timer waits must use timer wakeup infrastructure
- process exit must wake parent waiters
- pipe/futex/timer/file readiness must wake relevant waiters
- scheduler must not print logs in production
- scheduler must not know official testcase names

Scheduler invariants:

```text
A Running task is not in any wait queue.
A Ready task is runnable.
A Blocked task has a wait reason.
A Sleeping task has a timer wakeup or deadline.
A Zombie task is not runnable.
An Exited task cannot become Ready again.
A wait queue entry references a valid task.
A wakeup removes the task from the corresponding wait queue before making it Ready.
```

Run queue must be abstract enough to support future tree-based fair scheduling.

---


## 9A. Scheduler Performance Rule

Scheduler design must support high-performance evolution without changing task
semantics.

Rules:

- enqueue, dequeue, wakeup, and pick-next must have explicit complexity targets
- initial run queue may be simple, but the trait must support O(1), priority, or tree-based implementations
- wait queue wakeup must not scan unrelated tasks
- sleep and timer wakeup must not busy-wait
- task state transition must be atomic with respect to run queue/wait queue membership
- scheduler must not allocate in context switch or timer interrupt path
- scheduler must not inspect filesystem paths, official cases, process names, or syscall-specific semantics
- context switch ABI validation must be done at task/context setup, not on every switch
- per-task scheduler metadata must be stored in the task or scheduler node, not reconstructed on every scheduling decision

Required separation:

```text
policy decision       -> scheduler
task state ownership  -> task/process core
timer wakeup source   -> time subsystem
context switch        -> architecture boundary
blocking condition    -> owning subsystem
```

Context switch fast path must consume an already-prepared context. It must not
build a high-level plan object on every switch.

---

## 9B. Context Switch Fast-Path Rule

Context switch must be a prepared-context operation.

The switch fast path may use only current/next task pointers, current/next
saved-context pointers, prepared kernel stack pointer, prepared address-space
token/root, and the architecture switch function.

Context switch fast path must not construct `ContextSwitchPipeline`, construct
high-level plan/result objects, validate ABI layout, validate task identity
beyond required debug assertions, allocate, print, inspect process names,
paths, official cases, or syscall numbers, perform provider discovery, or
perform VFS, loader, or official runner logic.

All validation required for switching must happen during task creation,
fork/clone, exec, context materialization, architecture context installation, or
scheduler admission.

After a task is admitted as runnable, the scheduler must be able to switch to it
through compact prepared context state.

---

## 10. Task, Process, Wait, and Exec Rules

Process lifecycle must be centralized.

Required concepts include `Task`, `Process`, `Thread`, `Pid`, `PidAllocator`,
`ParentChildRelation`, `ExitState`, `WaitTarget`, and `ExecImage`.

Rules:

- `fork` and `clone` must duplicate or share resources according to flags
- `execve` must replace address space
- `execve` must close cloexec fds
- `wait4` and `waitpid` must consume child exit state exactly once
- exit status encoding must follow syscall ABI rules
- parent-child relations must be explicit
- process success must not be faked by runners
- scheduler and wait queues must be used for blocking waits

---

## 11. Memory Management Rules

Memory management is shared policy plus BSP page-table primitives.

Required concepts include `AddressSpace`, `VmArea`, `PageTableRoot`,
`MappingFlags`, `FrameAllocator`, `UserBuffer`, `UserPtr`, `PageFault`, and
`PageFaultResult`.

Rules:

- all `brk`, `mmap`, `munmap`, and `mprotect` operations must go through `AddressSpace`
- all user pointer access must go through `mm::user_copy` or `syscall::user_ptr`
- no raw user pointer may be stored inside long-lived kernel objects
- page fault handling must be explicit
- lazy allocation and COW-ready fork design must be supported by interfaces
- TLB operations are delegated to BSP
- kernel pages must never be exposed as user mappings
- user/kernel address ranges must be separated

Typed page-fault results: `Resolved`, `InvalidAccess`, `NeedKill`, `KernelBug`.

---


## 11A. Memory and TLB Performance Rule

Memory management must separate fast address-space operations from slow
validation and fault resolution.

Rules:

- address-space metadata must support efficient VMA lookup
- first implementation may use a simple ordered list, but the interface must support tree/range-map replacement
- page fault handling must distinguish fast resolvable faults from fatal/slow faults
- lazy allocation and COW must be represented without forcing full address-space copy on fork
- fork must avoid eager page copying unless explicitly required
- exec must build a new address space and commit it atomically
- TLB flush scope must be explicit: single page, address space, or global
- full TLB flush must not be the default when a narrower flush is available
- address-space activation must use a compact architecture token/root prepared before switch
- user-copy must avoid repeated range walking when a validated user buffer can be reused within one syscall
- kernel mappings must be stable enough to avoid unnecessary TLB churn

Forbidden:

- copying all user pages during fork as the long-term design
- flushing the entire TLB for every mapping change without a scoped reason
- revalidating static page-table root format on every context switch
- storing raw user pointers in long-lived kernel objects
- page fault handlers that silently fake success

---

## 11B. Address-Space Token and TLB Fast-Path Rule

Address-space switching must use compact prepared tokens.

Rules:

- page-table root validation happens when the address space is created or committed
- context switch must use a prepared architecture root/token
- context switch must not rebuild page-table root plans
- TLB flush scope must be specific: page, range, address space, or global
- global TLB flush is forbidden as the default mapping-change behavior
- user/kernel shared mappings must be stable enough to avoid unnecessary flushes
- fork must not eagerly duplicate all user pages in the long-term design
- COW metadata must be representable without full page copying
- page fault fast path must do VMA lookup and resolution without unrelated subsystem scans

Any TLB operation must state its scope, reason, whether it is required before
user return, whether it can be deferred, and whether it is local-core or global.

---

## 12. Filesystem and FD Rules

The VFS must be shared, unified, and case-independent.

Required concepts include `FdTable`, `FileDescriptor`, `OpenFileDescription`,
`Inode`, `Dentry`, `Mount`, `PathResolver`, `FileOps`, `Pipe`, `DeviceNode`,
`ProcNode`, `TmpNode`, `PageCache`, and `BlockCache`.

Rules:

- fd table and open-file-description semantics are mandatory
- `dup`, `dup2`, and `dup3` must share open-file-description offsets
- `fork` must duplicate fd references correctly
- `execve` must close cloexec fds
- path resolution must support dirfd-relative lookup, `.`, `..`, absolute paths, and relative paths
- device files are pseudo-inodes, not syscall special cases
- procfs files are generated by procfs providers, not runner hacks
- pipe blocking must use wait queues
- filesystem code must not print debug logs in production
- filesystem code must not know official testcase names

---


## 12A. VFS, FD, and Cache Fast-Path Rule

VFS and FD operations must distinguish cache-hit fast paths from slow paths.

Rules:

- fd lookup must be O(1) or table-index based
- open-file-description offset updates must avoid global filesystem locks
- dentry/inode cache hit must not re-read rootfs metadata
- path resolution must use cached dentries/inodes where possible
- repeated lookup of the same mounted root must not re-probe the block device
- page-cache hit must not call block-device providers
- block-cache hit must not allocate or copy more than necessary
- read/write must be structured around file ops and cache ops, not syscall-specific branches
- pipe readiness must use wait queues, not polling
- procfs/devfs/tmpfs behavior must be provider-backed, not path-special-cased

Cache miss rules:

- cache miss may build a read/populate plan
- cache population must happen only after real provider completion
- failed or partial I/O must not mutate cache as success
- cache insertion must define ownership, dirty state, and eviction eligibility

Forbidden:

- per-syscall ad-hoc path parsing
- hardcoded `/bin`, `/busybox`, `/musl`, or official case paths
- rootfs metadata scan on every lookup
- block read-through on cache hit
- fake file data to satisfy official output

---

## 12B. Cache-Hit Contract Rule

A cache hit is a completed operation, not a reason to re-enter provider logic.

Page-cache hit must not call block device provider, allocate cache entries,
parse filesystem metadata, perform rootfs discovery, rebuild read-through plans,
or take global filesystem locks.

Block-cache hit must not issue device I/O, allocate sector buffers, validate
provider readiness, retry previous failed reads, or mutate cache replacement
metadata through global locks if avoidable.

Dentry/inode cache hit must not re-read directory blocks, re-run mount
discovery, re-parse absolute path from root unless required by the request, or
branch on official testcase paths.

Cache miss may enter slow path. The miss path must be named and isolated.

---

## 13. Path Resolution Rules

All path handling must go through a unified path resolver.

Required behavior: absolute path resolution, relative path resolution,
dirfd-relative resolution, `AT_FDCWD`, `.`, `..`, symlink policy, mount
traversal, and permission checks.

Forbidden: per-syscall ad-hoc path parsing, runner-specific path matches,
hardcoded BusyBox path branches, and hardcoded test path branches.

---

## 14. Syscall Rules

All syscalls must go through one shared dispatcher.

BSP extracts syscall number/arguments, calls the shared dispatcher, writes the
return value to the trap frame, and returns to user.

The shared dispatcher routes syscalls, validates arguments, copies user data,
calls subsystems, converts errors to ABI values, and handles blocking contract.

Syscall semantics must not live in BSP or official runner. Syscalls must not
directly dereference user pointers, print debug logs in production, or branch
by testcase name.

---


## 14A. Syscall and Trap Fast-Path Rule

Syscall and trap paths must be minimal architecture-to-core handoff paths.

Trap entry responsibilities:

- save required registers
- decode trap cause
- build compact shared trap event
- call shared dispatcher
- restore/return through architecture boundary

Trap entry must not:

- allocate
- print
- perform VFS semantics
- perform scheduler policy except through shared trap/syscall result
- inspect official cases
- construct large diagnostic objects in normal path

Syscall dispatch responsibilities:

- decode syscall number
- validate arguments
- perform user-copy through the central contract
- call owning subsystem
- convert result to ABI return value

Syscall dispatch must not:

- branch by binary path or testcase
- duplicate filesystem/task/mm semantics
- hold spinlocks across user-copy or blocking operations
- allocate for simple fd lookup, pid lookup, time query, or cached read
- use logs as correctness mechanism

Returning to user must consume a prepared return context. Real `sret`, `ertn`,
or equivalent non-returning boundary must be represented as a narrow unsafe
architecture operation.

---

## 14B. Syscall Dispatch Performance Rule

Syscall dispatch must be table-driven or compact-match-driven.

Rules:

- syscall number decode must be O(1) or a small bounded match
- argument extraction must be register-based and allocation-free
- common syscalls must not allocate just to build request objects
- simple syscalls such as getpid, gettid, clock_gettime fast path, fd lookup,
  read/write cache hit, and wait status check must avoid large intermediate plans
- syscall dispatch must not parse paths unless the syscall semantics require a path
- syscall dispatch must not call official runner logic
- syscall dispatch must not format syscall names in production
- syscall result must be converted to ABI return value without diagnostic allocation

Slow syscalls may enter subsystem slow paths, but the boundary must be explicit.

---

## 15. Loader and User Entry Rules

Loader must be arch-neutral until final user entry.

Required concepts: `ElfImage`, `ProgramSegment`, `ExecImage`, `UserStack`,
`AuxVector`, `UserEntryContext`.

Rules:

- ELF parser is shared core
- PT_LOAD validation must be strict
- argv/envp/auxv construction is shared core
- final user transition is BSP responsibility
- loader must return structured errors
- loader must not print debug logs in production
- loader must not embed historical test programs into production boot

---

## 16. Driver, Block I/O, and Cache Rules

Drivers are split into BSP discovery/binding, generic driver interface, and
block/cache layers.

Rules:

- block I/O must go through a block-device abstraction
- filesystem reads must be cache-aware
- writeback policy must be explicit, even if initially simple
- busy loops must have timeout policy
- driver errors must be typed
- driver init must not print normal debug lines

Required cache concepts: `BlockDevice`, `BlockCache`, `PageCache`, cache
lookup, dirty tracking, writeback placeholder, and error reporting.

Block read-through rules:

- cache hit path must remain short
- cache miss may produce a read-through plan
- read-through execution requires explicit policy permission
- sector bytes may enter `BlockCache` only after completed provider result and strict validation
- failed, timed-out, unsupported, or mismatched reads must not mutate cache
- RISC-V may use a virtio-mmio explicit read provider
- LoongArch must expose the matching provider contract but may return `DiscoveryRequired`, `NotReady`, or `Unsupported` until virtio-pci I/O is real
- no fake sector data

---

## 16A. Provider and Path Selection Separation

Provider installation and path selection are separate concepts.

Provider installation answers:

- what hardware or runtime capability exists
- what neutral provider is installed
- whether `RuntimeExecutionPolicy` permits using it

Path selection answers:

- what future runner or request wants to launch

Do not add path selection unless explicitly requested.

Forbidden behavior:

- hardcoded `/musl`
- hardcoded `/busybox`
- hardcoded `/bin`
- hardcoded `/lib`
- hardcoded `basic`
- official testcase path lists
- path-specific success branches
- path-driven provider choice
- embedded testcase ELF bytes

A future `ProgramLaunchRequest` may contain a path, but every path must go
through the same generic pipeline:

```text
ProgramPath
-> VFS lookup
-> rootfs metadata traversal
-> file inode resolution
-> file data read
-> ELF validation
-> ExecMemoryPlan
-> ProcessTable::commit_exec
-> PendingUserEntry
```

Provider installation must not execute hardware. Provider installation must not
imply policy permission. Policy permission must not imply operation success.
Success still requires real provider completion and subsystem commit validation.

---


## 16B. Block I/O Pipeline Performance Rule

Block I/O must be designed as an asynchronous-capable pipeline even if the first
implementation is synchronous.

Required structure:

```text
request
-> cache lookup
-> provider request
-> completion
-> validation
-> cache commit
-> waiter wakeup
```

Rules:

- cache hit must return without provider I/O
- cache miss must produce a bounded provider request
- block-device provider must report completion, timeout, unsupported, or error
- cache mutation happens only after successful completion validation
- waiters must sleep on wait queues, not busy-poll indefinitely
- writeback state must be represented even if writeback is initially simple
- request identity must not depend on official testcase path
- driver timeout policy must be explicit
- interrupt-driven completion should be supported by interface design even if polling is used initially

Forbidden:

- synchronous-only interface that prevents future interrupt/DMA completion
- copying sector data through unnecessary intermediate buffers on cache hit
- rootfs retry loops that reissue identical failed I/O without state change
- treating unsupported hardware as empty successful sector data

---

## 17. Official Case Rules

Official cases must exercise real kernel subsystem behavior.

Forbidden: fake success, hardcoded testcase output, hardcoded BusyBox command
success, official runner bypassing syscall semantics, case-specific filesystem
shortcuts, case-specific process shortcuts, case-specific procfs shortcuts
outside procfs provider, and parser-shaped output not backed by real execution.

Allowed: official runner chooses the case/program to run, official runner
prints official markers through judge output, kernel subsystems implement real
behavior, and official adapter may translate official case metadata into normal
kernel operations.

Subsystem ownership examples:

```text
free fails  -> procfs + sysinfo + memory accounting
df fails    -> statfs + mount table + procfs
ps fails    -> task table + procfs process provider
sleep fails -> timer + scheduler wait queue
which fails -> path + access + exec lookup
ash fails   -> fork + exec + wait + fd + pipe + tty + signal
grep fails  -> open/read/write/lseek + pipe + tmpfs
```

---

## 18. Correctness Without Logs or Boot Self-tests

Allowed validation layers:

- compile-time validation: type checking, explicit traits, exhaustive enum matching, const assertions, forbidden-symbol grep, no-warning builds
- host-side deterministic tests for pure logic such as ELF parsing, user stack builder, path resolver, fd table, wait status, VMA, page/block cache, errno, scheduler transitions
- external QEMU integration tests observing official output, syscall returns, program exit status, filesystem/process behavior
- optional bounded in-memory diagnostics disabled by default

Production kernel must not add debug logs to support integration tests.

---


## 18A. Performance Regression and Measurement Rule

Performance-sensitive changes must be reviewable without production debug logs.

Required validation methods:

- host-side microtests for pure data structures
- QEMU integration tests for syscall/trap/user-mode progress
- bounded counters or in-memory diagnostics disabled by default
- optional benchmark builds that do not change production output
- no-warning builds for performance-critical configurations

Performance-sensitive changes must state whether they affect:

- syscall dispatch
- trap return
- context switch
- scheduler enqueue/dequeue
- wait queue wakeup
- page fault resolution
- VFS lookup
- fd lookup
- page-cache hit/miss
- block-cache hit/miss
- user-copy
- block I/O provider path

Forbidden:

- adding production logs to measure performance
- adding boot self-tests as benchmark substitutes
- claiming performance improvement without identifying the affected path
- optimizing official output path while bypassing real subsystem behavior

---

## 18B. Performance Regression Gate

A change is a performance regression if it causes any of the following without
explicit justification:

- hot path begins allocating
- hot path begins formatting or logging
- cache hit begins calling provider I/O
- fd lookup becomes a process-wide or system-wide scan
- wait queue wakeup scans unrelated tasks
- context switch rebuilds high-level plans
- syscall dispatch constructs large request objects for simple syscalls
- page fault path scans unrelated VMAs or processes
- VFS cache hit re-reads filesystem metadata
- block-cache hit issues device I/O
- TLB operation widens from page/range/address-space to global by default
- lock scope expands across allocation, user-copy, I/O, page fault, or sleep

Performance-sensitive patches must identify affected path, old complexity, new
complexity, allocation behavior, locking behavior, cache-hit behavior, and the
regression check.

---

## 19. Interface Consistency Rules

Every subsystem boundary must define ownership model, input/output types, error
model, blocking model, locking model, allocation behavior, user-memory access
rules, whether it may sleep, whether it may touch BSP, and whether it may emit
output.

Cross-subsystem dependency direction:

```text
arch -> shared core handoff only
kernel boot -> core initialization
syscall -> fs/task/mm/time/signal
task -> mm/fs via explicit handles
fs -> mm through page/cache abstractions
drivers -> generic device traits
official -> judge output + normal kernel interfaces
```

Forbidden dependencies:

```text
fs -> official
mm -> official
task -> official
BSP -> official case behavior
BSP -> VFS semantics
drivers -> judge output
scheduler -> official testcase names
```

---

## 20. Synchronization and Global State Rules

Global mutable state must be wrapped by a synchronization policy.

Rules:

- no uncontrolled `static mut`
- no hidden cross-subsystem global state
- no global state without initialization ownership
- no lock ordering ambiguity
- no unbounded critical sections
- interrupt safety must be documented
- sleep while holding spinlock is forbidden

Allowed synchronization concepts: spinlock, mutex, once/init cell, wait queue,
and atomic counters.

---


## 20A. Data Layout and Cache Locality Rule

Kernel data structures must be designed for cache locality and low contention.

Rules:

- hot fields and cold diagnostic fields should be separated when structs grow
- frequently read immutable fields should be grouped together
- mutable counters shared across CPUs must avoid false sharing once SMP is introduced
- large enum/result objects must not be passed through hot paths by value if a compact status is enough
- per-task hot metadata must be directly reachable from the scheduler/task object
- fd table, dentry cache, page cache, and block cache must avoid linear scans on common operations
- cache keys must be compact and stable
- subsystem-private data should not leak into generic hot structs unless needed by the fast path

Forbidden:

- storing large optional diagnostic payloads in every hot-path result
- embedding path strings in scheduler/task hot metadata
- reconstructing lookup keys repeatedly inside loops
- using Vec scans as permanent fd, VMA, dentry, page-cache, or wait-queue design when the interface can expose indexed/tree/hash replacement

---

## 21. Source Convergence Rules

`source/` is active, but it must still converge by real subsystem behavior.

Order of implementation pressure:

1. maintain quiet dual-architecture artifact builds
2. close boot, trap, MMU, scheduler, rootfs, and user-entry blockers
3. keep shared-core semantics architecture-neutral
4. implement real minimal user-mode execution
5. implement real block/rootfs and official runner paths
6. compare behavior with verified `src/` mechanisms only as reference
7. remove remaining historical dependencies only after callers have moved

Current convergence priority:

1. provider installation and policy/provider separation
2. symmetric RISC-V/LoongArch explicit block-read provider contracts
3. RISC-V controlled virtio-mmio block read-through
4. LoongArch matching PCI-gated provider contract without fake success
5. real sector commit into `BlockCache`
6. rootfs retry from real cached sectors
7. real executable bytes into `ProgramLaunch`
8. controlled MMU activation and user return
9. controlled context switch execution
10. minimal syscall/trap loop

Do not prioritize CFS, SMP, networking, futex, signal, TLS, or official runner
before the minimal real execution path is working, unless explicitly requested.

No old historical code may be copied into `source/` unchanged. If `src/`
contains useful verified platform knowledge, extract the real kernel concept
and name it according to the owning `source/` subsystem.

---

## 22. Documentation and Competition Submission Rules

Required documentation for the active `source/` implementation:

design overview, architecture split, BSP responsibilities, scheduler design,
memory-management design, VFS/fd design, syscall design, loader/user-entry
design, testing and validation strategy, performance strategy, development
log, issue/fix log, third-party source/reference statement, AI usage statement,
and official score/test report.

Source comments may be Chinese or English, but non-team code and referenced
designs must have source statements. The Git history must show a real staged
development process. Do not generate large artifacts into documentation
directories.

---

## 23. Review Checklist

Before accepting any `source/` change:

```text
[ ] No ucompat/legacy/run_vxxx/self_test/evidence-bus names
[ ] No boot self-tests
[ ] No non-official serial output
[ ] No debug logs in hot path
[ ] No case-specific hardcoding
[ ] No fake success
[ ] No safety-theater flags such as *_CONFIRMED, *_BY_DEFAULT, called_by_default, or executes_by_default
[ ] No duplicate PlanOnly/ValidateOnly modes unless they produce different observable results
[ ] No boolean dispatch where an enum mode or type-state should be used
[ ] No object has multiple lifecycle state fields describing the same boundary
[ ] No Applied state is returned unless a real side effect occurred
[ ] No non-returning user transition returns a normal "ready" result after execution
[ ] No placeholder production sequence steps with None just to reserve future hooks
[ ] No enum variant exists only for imagined future implementation
[ ] No top-level BSP trait duplicates narrower capability trait methods
[ ] Unsafe hardware actions have one narrow unsafe entry point
[ ] Hot paths allocate no heap memory
[ ] Hot paths perform no logging, formatting, provider discovery, or path-specific branching
[ ] Cache-hit paths do not call provider I/O
[ ] Context switch consumes prepared context and does not rebuild high-level plans
[ ] Syscall/trap path performs only compact handoff and owning-subsystem dispatch
[ ] Scheduler enqueue/dequeue/wakeup/pick-next have explicit complexity expectations
[ ] Wait queues do not scan unrelated tasks
[ ] No spinlock is held across allocation, user-copy, VFS lookup, block I/O, page fault resolution, or sleep
[ ] Interrupt-disabled sections are bounded and justified
[ ] VMA, fd, dentry, inode, page-cache, and block-cache interfaces allow non-linear replacement
[ ] TLB flush scope is explicit and not globally flushed by default
[ ] Page fault path distinguishes fast resolvable faults from slow/fatal faults
[ ] Block I/O path separates request, completion, validation, and cache commit
[ ] Large diagnostic/result objects are not passed through hot paths unnecessarily
[ ] Performance-sensitive changes identify affected path and validation method
[ ] BSP contains only hardware/ISA responsibilities
[ ] Shared core owns syscall/VFS/task/mm/scheduler semantics
[ ] User pointer access goes through user-copy contract
[ ] Blocking uses wait queues
[ ] Busy loops have timeout policy
[ ] FD/OFD semantics preserved
[ ] Path resolution is dirfd-aware
[ ] Address-space/VMA rules preserved
[ ] Every performance-sensitive function is assigned to BootPath, SetupPath, SlowPath, HotPath, or IrqPath
[ ] HotPath functions do not call SetupPath/SlowPath functions except through explicitly named miss paths
[ ] Hot paths do not build high-level planning/pipeline objects
[ ] Common operation complexity is documented for scheduler, wait queue, fd table, VMA, cache, and timer structures
[ ] Linear scans in hot paths are absent, bounded, or explicitly temporary
[ ] Production build does not include debug formatting, trace strings, or diagnostic object construction in hot paths
[ ] Simple containers are hidden behind subsystem-owned replacement-ready interfaces
[ ] Context switch fast path uses only prepared context, stack, task, and address-space state
[ ] Address-space switch uses a compact prepared root/token
[ ] Cache miss paths are named and isolated from cache-hit paths
[ ] Syscall dispatch is table-driven or compact-match-driven and allocation-free for simple syscalls
[ ] Performance regressions identify affected path, complexity, allocation behavior, locking behavior, and regression check
[ ] make all still passes when relevant
[ ] src/ legacy/reference tree unchanged unless explicitly authorized
```

---

## 24. Final Principle

`source/` is not a place to accumulate patches.

`source/` is the production target and active official artifact implementation.

Every line added to `source/` must move the project toward:

```text
quiet production behavior
real subsystem semantics
high-performance monolithic architecture
two BSPs with one shared core
maintainable competition-ready documentation
```

Safety, architecture neutrality, and high performance must come from real
semantics, typed boundaries, narrow unsafe entry points, replaceable
subsystem-owned data structures, and measurable hot-path behavior — not from
accumulating placeholder states, redundant gates, symbolic "not executed by
default" flags, or hardcoded shortcut paths.

Fast paths are production contracts. A subsystem that is only functionally
correct but forces hot-path allocation, repeated validation, provider discovery,
cache-hit I/O, unbounded scans, or high-level plan reconstruction is not
production-ready.

---

## 25. Module Decomposition and File-Size Policy

### 25A. Large-File Policy

Line-count thresholds are **review triggers, not truncation rules**. Do not
truncate files, delete code, or split mechanically to satisfy a numeric limit.

Suggested thresholds:

| Lines | Meaning |
|-------|---------|
| 800 | Review trigger — inspect for ownership clarity and hot-path length |
| 1200 | Do not add substantial new behavior without explicit justification |
| 1500 | Requires explicit justification and a decomposition plan in the report |

A large file may remain large temporarily if it has one clear owner and
splitting would increase state duplication, interface noise, or ownership
ambiguity. Size alone is never a reason to delete working code.

### 25B. Semantic Decomposition Rule

A new module (file) should be created only when the new behavior introduces at
least one of:

- a new **owner** (subsystem that defines and owns the state)
- a new **execution tier**: `BootPath`, `SetupPath`, `SlowPath`, `HotPath`,
  `IrqPath` (see §6F)
- a new **hardware or unsafe boundary** (see §6B, §7)
- a new **cache-miss path** or **I/O completion path**
- a new **syscall group** with distinct argument-validation and dispatch
- a new **VFS / MM / scheduler / task object** with its own lifecycle
- a new **public subsystem API** consumed by callers in a different subsystem

Small helpers, local validation, simple error conversion, compact internal
types, and 20–50 line focused logic should usually stay in the existing owner
module. Do not create a file only because a single function is new.

### 25C. Anti-Fragmentation Rule

Do not create dumping-ground files named `types.rs`, `utils.rs`, `common.rs`,
or `helpers.rs` unless the module already has a clear owner and the file
content is narrowly scoped to that owner.

Do not split one lifecycle state machine across many files unless each file
owns a distinct phase and state ownership remains clear to a reader.

Do not introduce new trait, provider, plan, result, or readiness layers just
because code moved to a new file. A split must reduce ownership ambiguity or
hot-path complexity; otherwise do not split.

Forbidden fragmentation patterns:

- one enum variant per file
- one 30-line `impl` block per file
- trait definition in one file, single implementation in another, when the
  implementation is the only known realization
- request, response, and error types for the same operation spread across
  three files when the total is under 200 lines

### 25D. Owner-First Placement Guide

New code should be placed at the file-system location that matches its
semantic owner:

```text
hardware boundary          -> source/arch/<arch>/ or source/arch/contract/
boot orchestration         -> source/kernel/boot.rs or kernel runtime init
runtime orchestration      -> source/kernel/
process lifecycle          -> source/core/task/
scheduler policy/state     -> source/core/scheduler/
memory / page table / VMA
  / user-copy / page fault -> source/core/mm/
syscall group              -> source/core/syscall/
VFS / path / fd / inode
  / dentry / pipe / procfs
  / devfs / tmpfs          -> source/core/fs/
block / cache / provider
  / request / completion   -> source/core/block/ or source/core/drivers/
loader / ELF / user stack
  / auxv                   -> source/core/loader/
time / timer API           -> source/core/time/
official marker output     -> source/official/judge_output only
```

When a function serves two owners, place it with the owner that defines the
primary state, and expose it through a re-export or a narrow public function
in the secondary owner's module.

### 25E. Hot-Path Preservation During Splits

Splitting must not lengthen hot paths with extra wrappers, adapters, or
indirection. Hot paths must consume compact prepared state (see §6C, §6G).

When splitting a module that contains hot-path code:

- The hot path must remain in a single file or call through at most one
  direct, inline-friendly function boundary.
- Setup / SlowPath code may be extracted to a separate file; HotPath must not
  be forced to route through the new setup-file abstractions.
- Do not split code in a way that forces a hot path to call through multiple
  semantic adapters, re-validate prepared state, or rebuild compact handles.

If a proposed split would add a wrapper call, a trait-object dispatch, or an
extra `match`/`if let` on every hot-path invocation, the split is invalid
regardless of line count.

---

## 26. Route-Shortening and High-Performance Implementation Rules

### 26A. Vertical Slice First Rule

Every stage must advance one real executable or data path end-to-end before
broadening. Do not build broad subsystem scaffolding (traits, providers, plans,
state machines) unless required by the current vertical slice.

A vertical slice means: a real boot step, a real trap round-trip, a real page
fault resolution, a real sector read into cache, a real ELF load into an
address space, or a real syscall returning a real result.

Horizontal scaffolding (laying out every module, trait, and enum for a
subsystem before any path works) is forbidden. Build the shortest real path
first, then widen.

### 26B. No Interface Before First Caller Rule

Do not add public traits, provider contracts, plan objects, result types, or
subsystem APIs unless a real caller in the same patch (or the immediately
following patch) consumes them.

A trait with zero implementors outside tests is dead code. A provider contract
with no consumer is scaffolding. A plan type with no construction or commit
site is placeholder design.

The first caller proves the interface shape. Add the interface when the caller
needs it, not before.

### 26C. Setup Once, Fast Path Consumes Rule

Capability discovery, ABI validation, provider setup, trap-vector installation,
page-table root validation, and kernel-mapping validation belong in boot,
setup, or slow paths.

Hot paths must consume compact prepared state and must not rebuild plans,
re-discover providers, re-validate static ABI layout, or re-check static
architecture capabilities (see §6C, §6F, §6G).

A hot path that re-validates what setup already validated is a performance
regression regardless of whether it allocates.

### 26D. One Stage, One Main Risk Rule

Do not mix unrelated high-risk categories in a single patch or stage. Each
patch should advance at most one of:

- new hardware action (user return, MMU activation, trap entry)
- new preemption source (timer interrupt, IPI)
- new context-switch path
- new blocking path (wait-queue sleep, I/O wait)
- new user-copy path
- new address-space mutation

Combining user return, timer preemption, and context switch in one patch is
forbidden. Each introduces a distinct failure mode that must be observable and
reversible independently.

### 26E. Regression Budget Rule

Every implementation report must state whether the patch:

- shortens a real execution path (progress)
- adds scaffolding, traits, plans, or unused interfaces (budget debit)
- increases hot-path length or adds hot-path branches (regression risk)
- adds public API surface (compatibility commitment)
- adds new state, trait, result, or error types (complexity budget)

Scaffolding must include an explicit removal or collapse condition. A patch
that adds only scaffolding with no path shortening is a net regression unless
the scaffold is immediately consumed by a real path in the next patch.

---


### 26F. Closure Convergence and Anti-Stacking Rule

A closure is not complete because contracts, plans, readiness states, or
substrates exist. A closure is complete only when the normal path is driven by
real runtime data and either reaches the intended side effect or stops at the
earliest concrete blocker.

When real behavior lands, collapse previous temporary blockers/readiness layers
out of the normal path. Keep them only as error, fallback, unsupported-arch, or
diagnostic branches.

Examples:

```text
file-backed executable bytes exist
-> LegitimatePayloadMissing is no longer the normal exec blocker

user trap dispatch is real
-> fatal-only trap readiness is no longer in the user-ecall path

block-cache hit exists
-> provider discovery/readiness is not re-entered on hit
```

Do not add another readiness, plan, prepared, execution, provider, or launch
layer when an existing result can express the runtime fact.

Every new abstraction must satisfy all four:

- distinct owner
- current real caller
- distinct runtime resource, side effect, or recoverable error
- clear collapse point after the closure becomes real

Each runtime closure must have one obvious normal path:

```text
input -> existing subsystem path -> outcome
```

Forbidden unless each layer owns a distinct real runtime resource and has a
current caller:

```text
input -> policy -> readiness -> plan -> prepared -> execution -> outcome
```

Boot-time program selection is input, not a policy framework. Prefer
boot-provided input such as `init=<path>` and a linear driver such as
`drive_boot_init_exec(path)`. Evidence used for QEMU/GDB validation observes
execution; it must not gate execution.

Closure reports must list:

- abstractions added
- abstractions removed or collapsed
- remaining blockers and the real input, hardware, or runtime fact they represent

---

## 27. Rust Module Layout Rule

### 27A. No mod.rs by Default

Do not create `mod.rs` files by default. Use named gateway files instead.

A `mod.rs` file may exist only with explicit justification in the patch report
and must remain declaration/re-export only. Existing `mod.rs` files, if any,
must not accumulate implementation logic.

### 27B. Named Gateway Files

Gateway files declare the module tree:

```text
source/lib.rs
source/arch.rs
source/kernel.rs
source/core.rs
source/official.rs

source/arch/contract.rs
source/arch/riscv64.rs
source/arch/loongarch64.rs

source/core/task.rs
source/core/scheduler.rs
source/core/mm.rs
source/core/syscall.rs
source/core/fs.rs
source/core/storage.rs
source/core/loader.rs
source/core/time.rs
source/core/sync.rs
source/core/drivers.rs
source/core/net.rs
source/core/compat.rs
```

Implementation files live under matching directories:

```text
source/arch/riscv64/<feature>.rs
source/arch/loongarch64/<feature>.rs
source/core/task/<feature>.rs
source/core/mm/<feature>.rs
source/core/fs/<feature>.rs
source/core/storage/<feature>.rs
```

### 27C. Gateway File Restrictions

Gateway files may contain only:
- `pub mod` declarations
- narrow `pub use` re-exports
- short module-level documentation (`//!`)

Gateway files must not contain:
- runtime implementation
- large type definitions
- helper function collections
- readiness or gating layers
- state machines
- hardware logic
- syscall / VFS / task / MM semantics
- driver logic
- official runner or judge output logic

Gateway files should normally stay below 100–150 lines. This is a review
trigger, not a truncation rule. A gateway file over 150 lines is a signal that
implementation is leaking into the declaration layer.

### 27D. Interaction with Decomposition Policy

The module layout rules in this section extend the decomposition policy in §25:

- Do not create `types.rs`, `utils.rs`, `common.rs`, or `helpers.rs` dumping
  grounds (see §25C).
- Do not create gateway files or subdirectories only to reserve future design
  space.
- A new implementation file must have a semantic owner, execution tier, and
  first real caller (see §25B, §26B).
- Splitting must reduce ownership ambiguity or hot-path complexity; otherwise
  do not split (see §25E).
- Line limits are review triggers, not mechanical split or truncation rules
  (see §25A).
