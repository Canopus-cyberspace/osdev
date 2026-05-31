# Implementation Status

Current factual status of the `source/` kernel implementation. This document
reports what is, not what is planned. See `source/ROADMAP.md` for the plan.

Last updated: 2026-05-31 (L0 official no-init RISC-V musl runner default path).

---

## Current Active Tree

| Item | Value |
|------|-------|
| Active implementation | `source/` Rust kernel skeleton |
| Build artifacts | `kernel-rv` and `kernel-la` are selected from `source/` by root `make all` |
| Implemented stage family | Stage 0 complete at build level; Stage 1 runtime-validated on RISC-V; Stages 2–4 advanced with RISC-V boot-selected file-backed exec evidence through VFS open/read, `sret`, user ecall, and kernel-owned exit/exit_group |

No `source/` official evaluation score has been claimed.

---

## Closure Status

### Closure 1: Boot/MMU/Trap (RISC-V)

**Status: runtime-validated.**

RISC-V boot, trap vector, kernel MMU activation, fatal console, and halt are
all implemented and observed in QEMU/GDB. The root `make all` produces working
`kernel-rv` from `source/`.

### Closure 2: User Entry / Syscall Loop (RISC-V)

**Status: partial — not complete.**

The `sret` user entry, user ecall trap dispatch, and kernel-owned exit path
are real and runtime-observed. Last known evidence: a bootargs-selected
file-backed static ELF reached user mode and issued syscalls
`96, 174, 29, 24, 64...`; `set_tid_address`, `getuid`, minimal stdio
`ioctl`, minimal stdio `dup3`, `brk`, anonymous `mmap`, `clock_gettime`,
`gettimeofday`, `newfstatat` for observed absolute rootfs paths, `write`,
`exit_group`, and `exit` are handled by the
single-task baseline where observed.

**Earliest real syscall blocker:** syscall ABI is incomplete. `getpid` (172),
`getuid` (174), `set_tid_address` (96), observed stdio `ioctl` (29), observed
stdio `dup3` (24), `write` (64), `clock_gettime` (113), `gettimeofday` (169),
observed `newfstatat` (79), `brk` (214), minimal anonymous `mmap` (222),
`exit` (93), and `exit_group` (94) exist. User reads and writes go through
the RISC-V active page-table copy contract; `write` writes only fd 1/2
through the kernel-owned `official::user_output` sink, separate from official
marker output. `brk` grows by mapping real zeroed user pages from the boot
frame allocator; no full VMA/munmap ownership exists yet.

Syscall ABI breadth is missing. FD/VFS stat/open/read/dirent operations and
process operations (`fork`, `wait4`) are not yet dispatched.

This closure **must not be marked complete** until at least the syscall ABI
covers the full set observed during file-backed ELF execution of real user
programs.

### Closure 3: Loader / Exec / Userland Compatibility

**Status: runtime-used but incomplete.**

The ELF64 loader, user stack builder, streaming PT_LOAD materialization, and
`Process::commit_exec` are real and have been exercised through the RISC-V
file-backed exec path. PT_LOAD pages are materialized from real file bytes
directly into boot-allocator frames and mapped by the user page-table builder;
the old fixed `MAX_USER_PAGES == 16` page-image cap is no longer on the
normal loader path. The boot-selected executable path is passed as `argv[0]`
for the init stack, bootargs `init.arg=` tokens are appended as `argv[1..]`,
and the stack includes a truthful `AT_PAGESZ=4096` auxv entry observed to be
required by BusyBox heap setup.

Broader loader-userland compatibility — dynamic PT_INTERP, richer
argv/envp/auxv surface, PIE executables, and loader behavior across a wider
program set — is not yet validated. No dynamic linker or libc substrate
exists.

This closure **must not be marked complete** until a real dynamic
loader/libc substrate is available or until static-only is explicitly scoped
and validated across a representative set of executables.

### Closure 4: VFS / Block / Storage / Rootfs

**Status: RISC-V vertical path partially runtime-validated — not complete.**

The read-only ext4 rootfs, virtio-mmio block provider, block cache hit/miss
split, and VFS file-read path have been exercised end-to-end on RISC-V for
the bootargs-selected exec path. Real sector data was read and committed to
cache. Real executable bytes were read from the rootfs and fed to the loader.
Host-side inspection of the current RISC-V image shows `/bin/busybox` is not
present; the actual BusyBox executable is `/musl/busybox` (inode 55, regular
0755, 1,387,560 bytes). With bootargs selecting `/musl/busybox`, VFS open and
full-file read reach the loader, user-MMU materialization succeeds,
`Process::commit_exec` creates a process-owned `PendingUserEntry`, RISC-V
executes `sret`, user ecall traps return to the kernel, and BusyBox records a
kernel-owned exit state for pid 1 with exit code 0 under the current no-arg
init invocation. With bootargs applet arguments, `true` exits 0 and
`echo hello` writes six bytes and exits 0. `ls /` reaches real user execution
and stops at the first syscall-visible VFS blocker,
then handles `newfstatat(AT_FDCWD, "/", statbuf, 0)` truthfully through the
syscall-visible mounted rootfs handle: the user path is copied through the
user-copy contract, VFS resolves `/`, and the stat buffer receives inode 2,
directory mode `040755`, size 4096, block size 4096, and 8 512-byte blocks.
The next blocker is `openat(AT_FDCWD, "/", 0x98000, 0)`, which currently
returns `-ENOSYS` because a real FD/OFD table and directory iterator are not
implemented.

**What is missing for completeness:**
- Full VFS surface: fd table breadth, `open`/`read`/`write`/`lseek` syscall dispatch, pipe/procfs/devfs/tmpfs backends
- Full FD semantics: `dup`/`dup2`, cloexec, OFD offset tracking
- Writeback, DMA, and general provider discovery
- LoongArch storage path (typed unsupported via neutral contract)
- Broad rootfs metadata handling beyond the observed ext4 read-only path

This closure **must not be marked complete** until the full VFS/FD/storage
surface reaches the minimal breadth required for real user program execution.

---

## Completed Or Advanced Capabilities

| Capability | RISC-V | LoongArch | Shared Core | Notes |
|------------|--------|-----------|--------------|-------|
| Boot entry and stack | implemented; boot stack sized for the observed ext4/exec runtime path | implemented with matching stack reserve | boot handoff | RISC-V QEMU/GDB evidence exposed debug-build stack underflow during ext4 traversal; stack sizing was corrected |
| Fatal console primitive | implemented | implemented | `FatalConsole` contract | BSP low-level console only |
| Fatal halt | implemented | implemented | `HaltReason` contract | Halt loops use ISA idle/wfi where available |
| Trap vector contract | real `stvec` apply with user trapframe path | typed unsupported | neutral `TrapServices` | RISC-V saves user registers/CSRs and dispatches user ecall; unsupported traps still halt |
| Kernel MMU contract | real Sv39 root apply | typed unsupported | `KernelMmuRequest` / `KernelMmuState` | RISC-V maps QEMU device window and kernel identity window |
| Boot memory discovery | real FDT `/memory` parsing | `DiscoveryRequired` | `BootMemory` / `MemoryFoundation` | RISC-V usable frames begin after aligned kernel image end |
| Frame allocator readiness | fed by FDT range | not ready | `BootFrameAllocator` | Allocator is real when boot memory is discovered |
| User address-space model | contract-backed and exercised by file-backed exec | typed unsupported | `UserAddressSpacePlan`, `UserAddressSpaceLoadPlan`, segment/mapped-region ownership | RISC-V file-backed static ELF reached an applied user address space |
| User page-table builder | real Sv39 page tables from boot frames, streaming file-byte population | typed unsupported | `UserMmuState` contract | Allocates user frames, copies PT_LOAD page slices, zeroes BSS/uncovered bytes, maps stack/ELF pages, preserves kernel image mapping plus UART and virtio-block MMIO pages needed by kernel-owned syscall work |
| Minimal syscall dispatch | RISC-V live trap boundary observed for user ecall | same shared semantics available after trap support | `core::syscall` | `getpid`, `getuid`, `set_tid_address`, observed stdio `ioctl`, observed stdio `dup3`, `write`, `clock_gettime`, `gettimeofday`, observed `newfstatat`, `brk`, minimal anonymous `mmap`, `exit`, and `exit_group` are implemented; bounded trace records syscall numbers, args, return values, and handled/errno/exit status |
| Kernel-owned exit state | architecture-neutral | architecture-neutral | `Process`, `ExitState`, single-task exit cell | Exit state is owned by `core::task` |
| ELF executable substrate | caller-provided and VFS-backed bytes | typed blocker until LoongArch entry path is verified | `core::loader` | Validates ELF64 target ABI, PT_LOAD ranges/permissions/overlap/alignment, builds segment-backed load plans and stack; RISC-V streams file bytes into real mapped frames |
| Process exec commit | architecture-neutral | architecture-neutral | `core::task::Process` | Commits `LoadedUserImage` into process-owned `PendingUserEntry` only after address-space/entry/stack validation |
| Block provider | RISC-V virtio-mmio read path build-valid | typed unsupported | `core::block::BlockProvider` | Real sector data is read only through provider completion; no fake sectors |
| Block cache | architecture-neutral | architecture-neutral | `core::block::BlockCache` | Read-only hit/miss split; misses commit only after completed provider read |
| Rootfs reader | ext4 bytes over block cache | typed unsupported provider | `core::fs::MountedRootfs` | Read-only ext4 mount, path lookup, inode identity, and file read for caller-requested paths |
| File-backed exec connection | bootargs-selected path drives ext4/VFS -> loader -> user-MMU -> `Process::commit_exec` -> `PendingUserEntry` -> `sret` | typed unsupported at user-MMU/user-entry hardware | `kernel::exec` | Reads executable bytes from VFS, feeds existing loader, materializes user MMU, commits process-owned `PendingUserEntry`; when bootargs lacks `init=`, defaults to `/musl/busybox sh /musl/basic_testcode.sh` for official evaluation |
| Default no-init runner (L0) | implemented — `kernel::boot::build_default_official_init_path()` provides the default init path when no `init=` bootarg is present | `discover_boot_init_path` returns `UnsupportedBootInput`, so default is not applied; halts with `NoRunnableWork` | `kernel::boot::kernel_start()` applies default on `BootInitBlocker::NoBootInitPath` | When official QEMU provides no `-append`/bootargs, the kernel defaults to the musl BusyBox shell executing the basic test script; all output comes from real user-space `write` syscalls — no fake markers |

---

## Missing P0 Blockers

P0 = no user-mode execution is possible until resolved.

| Blocker | Status |
|---------|--------|
| Verified QEMU boot/MMU/trap log | obtained for Closure 1 RISC-V boot/MMU/trap; no official score claimed |
| Loader-backed user address space | bootargs-selected file-backed static ELF path reached loader, user-MMU materialization, and process exec commit on RISC-V |
| PendingUserEntry / UserReturnPlan | process commit implemented; pending entry is replaced only after `LoadedUserImage` validation |
| Legitimate user payload source | RISC-V ext4-over-virtio path supplied real file bytes selected by FDT `/chosen/bootargs init=...`, or the default `/musl/busybox` path when no `init=` is present |
| Controlled user return (`sret`/`ertn`) | RISC-V `sret` executed for the file-backed static ELF; LoongArch typed unsupported |
| Return-capable user trapframe save/restore | RISC-V user trapframe path observed saving user `sepc`, `sstatus`, `scause`, args, and return registers |
| Syscall trap round trip | RISC-V user ecall reached kernel; `write(64)` now copies user bytes and returns written byte counts for fd 1/2; exit/exit_group record kernel-owned exit state |
| LoongArch MMU/trap hardware execution | typed unsupported until verified hardware mechanics are implemented |

---

## RISC-V Status

| Item | Status |
|------|--------|
| Boot entry | implemented |
| Early stack | implemented |
| Console write primitive | implemented |
| Boot memory discovery | implemented from firmware FDT `/memory` node |
| Trap vector | implemented via `stvec`; user trapframe save/restore dispatches user ecall, supervisor/unsupported traps halt |
| Kernel MMU/TLB | implemented for static Sv39 kernel root + `sfence.vma` |
| User page table | implemented builder for loaded page images using real boot frames; copies page bytes, maps validated permissions, and preserves the kernel-owned output MMIO page |
| User entry (`sret`) | executed from a bootargs-selected file-backed static ELF under bounded QEMU/GDB |
| User ecall decode | live trap-vector handoff observed; user ecall advances `sepc`, writes `a0`, or records exit |
| Timer/IRQ primitives | not implemented |
| Context switch | not implemented |
| Virtio block binding | RISC-V virtio-mmio read-only provider implemented at build level |
| Halt/reboot | halt implemented; reboot not implemented |

Reference: `src/arch/riscv64/` contains a legacy implementation. Mechanisms may
be studied but must be cleanly reimplemented in `source/`.

---

## LoongArch Status

| Item | Status |
|------|--------|
| Boot entry | implemented |
| Early stack | implemented |
| Console write primitive | implemented |
| Boot memory discovery | neutral contract returns `DiscoveryRequired` |
| Trap vector | neutral contract returns typed unsupported |
| MMU/TLB primitives | neutral contract returns typed unsupported |
| User page table | neutral contract returns typed unsupported |
| Timer/IRQ primitives | not implemented |
| User entry (`ertn`) | neutral contract returns typed unsupported |
| Context switch | not implemented |
| Virtio PCI block binding | neutral block contract returns typed unsupported |
| Halt/reboot | halt implemented; reboot not implemented |

LoongArch remains at the same neutral contract level as RISC-V. Hardware
uncertainty produces typed non-ready or unsupported status, not fake success.

---

## Shared Core Status

| Subsystem | Status |
|-----------|--------|
| `core::mm` foundation/frame/page-table/address-space | partial: kernel globals, hardware root readiness, boot frame allocator, memory summary, user load plan, segment-backed load ownership, stack page-init, mapped-region ownership |
| `core::syscall` | partial: compact getpid/getuid/set_tid_address/observed-stdio-ioctl/observed-stdio-dup3/write/newfstatat/exit/exit_group dispatcher and bounded syscall number/arg/return/status trace |
| `core::task` | partial: pid, process state, kernel-owned exit state, single-task exit cell, process exec commit, pending user-entry/request types |
| `core::scheduler` | not implemented |
| `core::fs` | partial: read-only ext4 rootfs mount, path lookup, inode/file identity/stat metadata, open-file-description, file read for exec input |
| `core::loader` | partial: strict ELF64 target-ABI validation, PT_LOAD segment load planning, BSS zeroing during RISC-V frame population, user stack/auxv basics, `LoadedUserImage` handoff |
| `core::time` | not implemented |
| `core::storage` / block cache | partial: neutral block request/completion/status, provider wrapper, read-only cache hit/miss path |
| `core::official` | partial: `user_output` owns the user stdout/stderr sink used by syscall `write(64)`; `judge_output` is reserved for official marker output; no official markers or score output are emitted |

---

## L0: Official No-Init RISC-V Musl Runner Path (2026-05-31)

**Status: implemented — not yet runtime-validated.**

When the FDT `/chosen/bootargs` contains no `init=<path>` token (the official
QEMU scenario), `kernel::boot::kernel_start()` constructs a default
`BootInitPath` for `/musl/busybox` with arguments `sh` and
`/musl/basic_testcode.sh`. This flows through the same VFS → loader → user-MMU
→ exec commit → `sret` pipeline as the manual `init=` path. No fake markers are
emitted — all judge-visible output comes from the user-space script's real
`write` syscalls through the `user_output` sink.

When `init=` IS present in bootargs, the existing manual path is preserved
unchanged.

The default path lives in exactly one function,
`kernel::boot::build_default_official_init_path()`, which is called only on
`BootInitBlocker::NoBootInitPath`.

On LoongArch, `discover_boot_init_path` returns `UnsupportedBootInput` (not
`NoBootInitPath`), so the default is not applied — LoongArch halts honestly
with `NoRunnableWork`.

## Current Next Task

Earliest real blockers:

- L0 default path not yet runtime-validated in QEMU (QEMU runs only when
  explicitly requested).
- For bootargs `init=/bin/busybox`, VFS correctly fails because the current
  image has no `/bin` directory.
- For bootargs `init=/musl/busybox` (or the L0 default), VFS open/read, loader
  validation, streaming PT_LOAD materialization, `Process::commit_exec`,
  `PendingUserEntry`, `sret`, user ecall, and exit state all have bounded
  RISC-V QEMU/GDB evidence. The no-arg BusyBox path records 851 syscalls and
  exits 0 after real fd 2 writes.
- `init.arg=ls init.arg=/` handles time and heap setup, copies the user path
  `/`, resolves `/` through the mounted ext4 rootfs, writes a truthful Linux
  stat buffer for the root directory, then stops at `openat` with `-ENOSYS`.
  Truthful progress requires a real FD/OFD owner plus directory iteration.

Next work should add only syscall/process/VFS behavior required by legitimate
runtime evidence from a selected real program path, likely beginning with
truthful `openat` plus the narrow FD/OFD and directory-entry ownership needed
by the observed BusyBox `ls /` path.

Dynamic PT_INTERP executables remain a typed loader blocker until a real
dynamic loader/libc substrate exists. Do not add fake dispatchers, path
selection, or testcase-specific handlers.

## Convergence Cleanup (2026-05-28)

### Pass 1: Readiness/state layer collapse

- Removed `UserAddressSpaceReadiness` — `Planned`/`Ready` were already
  expressed by `UserAddressSpacePlan` vs `UserAddressSpace` types
- Removed `AddressSpaceReadiness` — pure forwarding wrapper around
  `HardwareRootReadiness`
- Removed `KernelGlobalMappingReadiness` — single-variant enum (`Ready`)
  that always returned the same value
- Removed `FrameAllocatorReadiness` — forwarding wrapper around
  `BootFrameAllocator` state
- Simplified `MemorySummary` to carry only `HardwareRootReadiness`
- Removed unused `UserReturnBlocker` variants (`AddressSpaceNotReady`,
  `UserMemory`, `UserPayloadMissing`)
- Removed tautological readiness checks in `UserReturnPlan::prepare` and
  `validate_root`

### Pass 2: Execution skeleton reduction

- Removed `KernelContext` — every method was `self.bsp.method(...)`;
  callers now use `BspServices` directly. File `kernel/context.rs` deleted.
- Removed `UserMmuRequest` — newtype around `UserAddressSpaceLoadPlan`
  with one constructor and one accessor; BSP MMU functions now take the
  load plan directly.
- Removed `UserReturnRequest` / `UserReturnPlan` / `UserReturnBlocker` —
  the request→plan pair validated two booleans (`trap_vector_ready`,
  `user_trap_ready`) that were already checked in `boot.rs` via
  `trap.readiness()`. `PendingUserEntry` flows directly from
  `Process::take_pending_entry()` to `BspServices::enter_user()`.
- Removed `UserEntryState::NotReady` — no longer reachable since
  validation moved to boot-time caller.
- Removed duplicate `EntrySpecMismatch` check in `Process::commit_exec`
  (already validated by `LoadedUserImage::new()`).
- Removed `ExecCommitBlocker::EntrySpecMismatch` — no longer constructed.

### Retained semantic boundaries

`HardwareRootReadiness`, `TrapInstallState`, `KernelMmuState`,
`UserMmuState`, `UserEntryState`, `HardwareReadiness`, `UserMemoryBlocker`,
`ExecCommitBlocker`, `FileExecBlocker`, `BootInitExecBlocker` — each
represents a real architecture-boundary runtime fact, distinct ownership,
or observable error.

### Normal path (after)

```
bootargs init path → bsp.install_trap_vector → bsp.activate_kernel_mmu
→ MemoryFoundation → bsp.discover_boot_init_path
→ drive_boot_init_exec(bsp, memory, init_path)
  → MountedRootfs::mount_ext4 → VfsPath → commit_file_backed_exec
    → VFS open+read → loader::prepare_executable_image
    → bsp.prepare_user_mmu(load) → UserMmuState::Applied
    → LoadedUserImage → Process::commit_exec → PendingUserEntry
  → Process::take_pending_entry → PendingUserEntry
  → bsp.enter_user(pending, ApplyUnsafe)
    → sret → ecall trap → syscall dispatch → exit
```

### Hot path audit

Trap entry (assembly) → `dispatch_user_trap` → `dispatch_single` (simple
match) → `single_record_exit` (atomic stores) → trap return (assembly +
`sret`). No VFS, ELF, block, bootarg, readiness/plan construction,
allocation, or format/log on the hot path.

---

## Explicitly Out of Scope for Now

- Official runner or official score claims.
- Testcase path selection or fake userland success.
- Scheduler, VFS, storage, dynamic loader, signal, futex, TLS, networking, or
  SMP before real boot/MMU/user-entry prerequisites are closed.
