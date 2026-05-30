# Evaluation Coverage Matrix

Capability coverage across shared core and both BSPs. States the current
situation, not the plan. See `source/ROADMAP.md` for the plan.

Last updated: 2026-05-29 (BusyBox `newfstatat` evidence pass).

---

## Closure Status Summary

| Closure | Scope | RISC-V Status | Blocker |
|---------|-------|---------------|---------|
| 1 | Boot/MMU/Trap | runtime-validated | — |
| 2 | User Entry / Syscall Loop | partial, not complete | syscall ABI incomplete; write/getuid/observed stdio ioctl/observed stdio dup3/brk/time/newfstatat are minimal only; syscall-visible open/FD/dirent ownership is missing |
| 3 | Loader / Exec / Userland | runtime-used, not complete | dynamic PT_INTERP and broader argv/envp/auxv/PIE coverage unsupported or unvalidated; bootargs argv and AT_PAGESZ are now runtime-used |
| 4 | VFS / Block / Storage / Rootfs | RISC-V vertical path partially runtime-validated, not complete | `/bin/busybox` absent in current image; real `/musl/busybox` and simple applets now run through file-backed exec to exit, but broader VFS/FD/storage syscall breadth is missing |

No closure except Closure 1 may be written as "complete". No official score
is claimed. Official evaluation has not been run.

---

## Capability Matrix

| Capability | Shared Core | RISC-V | LoongArch | Status | Blocker | Next Action |
|------------|-------------|--------|-----------|--------|---------|-------------|
| Boot | boot handoff | implemented | implemented | RISC-V runtime-validated for Closure 1 | no LoongArch QEMU boot evidence | Preserve quiet boot; run broader QEMU only when requested |
| Fatal console / halt | neutral contracts | implemented | implemented | build-valid | no runtime log | Keep fatal-only output policy |
| Boot memory / frames | `BootMemory`, frame allocator | FDT `/memory` parser | `DiscoveryRequired` | advanced | LoongArch discovery not verified | Preserve contract; verify RISC-V in QEMU |
| BSP trap | neutral contract | `stvec` apply plus user trapframe save/restore observed for user ecall | typed unsupported | partial Stage 1/2B with RISC-V runtime evidence | broader trap/page-fault classification missing | Expand only from real traps observed after file-backed user programs run |
| MMU / TLB | kernel root readiness | static Sv39 root + `sfence.vma`; user root applied for file-backed exec and preserves kernel-owned UART and virtio-block MMIO pages | typed unsupported | partial Stage 1/4 with RISC-V runtime evidence | scoped TLB and page-fault path missing | Add page-fault/user-copy behavior after syscall loop evidence |
| User address space | load plan, segment-backed load ownership, mapped-region ownership, single-task brk cursor | Sv39 builder from real boot frames and streaming file-byte population exercised by file-backed static ELF; observed brk growth maps real zeroed frames into the active root | typed unsupported | partial Stage 2/3 with RISC-V runtime evidence | full VMA/lazy/COW/munmap ownership not implemented | Extend address-space ownership after exec/syscall baseline stabilizes |
| User entry | process-owned pending-entry/request model | `sret` executed from boot-selected file-backed exec | typed unsupported | partial Stage 5 with RISC-V runtime evidence | no scheduler-backed return policy | Keep single-task path until scheduler closure starts |
| Syscall loop | getpid/getuid/set_tid_address/minimal observed stdio ioctl/minimal observed stdio dup3/write/clock_gettime/gettimeofday/newfstatat/brk/minimal anonymous mmap/exit/exit_group dispatcher, single-task exit cell, bounded syscall arg/return/status trace | live RISC-V ecall dispatch observed; user-copy reads/writes walk the active user page table; brk maps real zeroed pages; newfstatat copies user path and writes stat via user-copy | shared semantics only | partial Stage 6 with RISC-V runtime evidence | broad syscall ABI and syscall-visible open/FD/dirent context missing | Add syscall/process behavior only from legitimate runtime blockers |
| Task / process | pid/process/exit state partial, single-task baseline, exec commit | — | — | partial Stage 3 with RISC-V runtime evidence | no fork/wait/scheduler ownership | Add process lifecycle breadth after syscall baseline |
| Scheduler | not implemented | — | — | Stage 7 not started | single-task user loop only | Implement after minimal syscall/process baseline expands |
| Context switch | — | not implemented | not implemented | Stage 7 not started | no scheduler | Implement prepared switch contract later |
| VFS / FD | read-only rootfs lookup, inode/file identity/stat metadata, OFD baseline for exec input | provider-backed through neutral block path | typed unsupported provider | partial Stage 8 slice with RISC-V runtime evidence | no broad fd table or syscall-visible directory surface | Keep narrow exec path; expand fd syscalls after user loop |
| Storage / block cache | neutral block request/completion/status and read-only cache | RISC-V virtio-mmio sector read observed with real completion and cache commit | typed unsupported | partial Stage 8 slice with RISC-V runtime evidence | writeback/DMA/general provider discovery missing | Keep provider/path separation; broaden only when storage syscalls require it |
| Loader / ELF / user stack | strict ELF64 target-ABI validation, PT_LOAD segment plans, BSS zeroing, bootargs argv, `AT_PAGESZ`, `LoadedUserImage` | RISC-V streams VFS file-byte slices into mapped frames from real boot allocation; init path supplies argv[0] and `init.arg=` supplies argv[1..] | typed unsupported | partial Stage 3/4 with RISC-V runtime evidence | dynamic PT_INTERP unsupported; broader envp/auxv/PIE coverage unvalidated | Expand only when real program evidence requires it |
| Official runner | not implemented | — | — | Stage 9 not started | no exec/syscall/wait | Do not integrate before real userland closure |

Legend:
- `—` = not applicable to this layer.
- `typed unsupported` = explicit non-fake status through the neutral contract.

---

## Score-Family Readiness

No score families have been evaluated from `source/`. The `src/` tree has
prior evaluation history; those scores are not attributable to `source/`.

| Score Family | Status | Requirements Not Yet Met |
|--------------|--------|--------------------------|
| basic | not evaluated | official runner, broader syscall ABI, wait/exit reporting |
| busybox | not evaluated | basic prerequisites plus VFS/rootfs/fd/process breadth |
| cyclictest | not evaluated | timer source, scheduler, wait queues |
| iozone | not evaluated | VFS, page cache, block cache, storage provider |
| lmbench | not evaluated | syscall, scheduler, pipe, fs, mm, fork/exec latency |
| libcbench / libctest | not evaluated | mm, task, futex, signal, TLS, loader, fs, time |
| lua | not evaluated | fs, time, random source, mm, loader, user execution |
| iperf / netperf | not evaluated | network stack, sockets, virtio-net, readiness waits |
| ltp | not evaluated | broad Linux compatibility |

---

## No Official Score

No official evaluation has been completed for `source/`. No score, rank, or
verdict is claimed.

Prior attempted runs did not produce inspectable verdicts (e.g., Docker
unavailable). Do not claim a score or QEMU success for `source/` until the
full official workflow completes and produces an inspectable result.

The `src/` tree has prior evaluation history; those scores are not
attributable to `source/`.

---

## Validation Status

| Check | Status |
|-------|--------|
| Rust host check | passes |
| RISC-V target build | passes |
| LoongArch target build | passes |
| Root `make all` | passes |
| Architecture-boundary grep | clean |
| Unsafe-in-core grep | clean |
| Forbidden-pattern greps | clean |
| Hardcoded-path grep | clean |
| Log/serial grep | clean |
| Official-marker grep | clean |
| Safety-theater grep | clean |
| Bounded RISC-V QEMU/GDB | obtained: `/musl/busybox` opens/reads real bytes, validates ELF, streams PT_LOAD pages into mapped frames, commits exec, creates `PendingUserEntry`, executes `sret`, traps user ecalls, writes real BusyBox usage text through fd 2, and records pid 1 exit code 0; `init.arg=true` exits 0; `init.arg=echo init.arg=hello` writes `hello\n` and exits 0 after real brk growth; `init.arg=ls init.arg=/` now handles `newfstatat(AT_FDCWD, "/", statbuf, 0)` with inode 2, mode `040755`, size 4096, blocks 8, then stops at `openat(AT_FDCWD, "/", 0x98000, 0)` returning `-ENOSYS`; not an official score |
| Official evaluation | not run |
