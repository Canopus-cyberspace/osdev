# Linux-Compatible OS Master Plan

## Scope

This plan targets a practical Linux-compatible subset suitable for a competition/teaching OS and future userland expansion.

It does not attempt to implement every Linux feature.

## Execution model

Use bounded batches.

Do not ask Codex to implement the whole OS in one run.

## Phase 1: Real memory manager, v195-v200

Goal:

- physical page allocator;
- user page-table mapping;
- real lazy page fault;
- permission/unmap;
- fork address-space copy or COW;
- memory stress.

Markers:

```text
[ucompat-v195] physical page allocator PASS
[ucompat-v196] user page table mapping PASS
[ucompat-v197] real page fault lazy allocation PASS
[ucompat-v198] page permission unmap PASS
[ucompat-v199] fork address space copy cow foundation PASS
[ucompat-v200] memory stress suite PASS
```

## Phase 2: Scheduler and blocking, v201-v206

Goal:

- run queue;
- timer tick;
- blocking wait/wakeup;
- pipe/poll/futex blocking;
- wait/sleep/timer blocking;
- scheduler regression.

Markers:

```text
[ucompat-v201] run queue task switch PASS
[ucompat-v202] timer tick scheduling PASS
[ucompat-v203] blocking wait wakeup PASS
[ucompat-v204] pipe poll futex blocking PASS
[ucompat-v205] wait sleep timer blocking PASS
[ucompat-v206] scheduler regression suite PASS
```

## Phase 3: Block device and filesystem image, v207-v214

Goal:

- block device abstraction;
- virtio-blk or ramdisk backend;
- block cache;
- readonly Ext4 or competition image mount;
- execve from filesystem image;
- stat/getdents/read from image;
- image rootfs matrix.

Markers:

```text
[ucompat-v207] block device abstraction PASS
[ucompat-v208] virtio blk ramdisk backend PASS
[ucompat-v209] block cache PASS
[ucompat-v210] ext4 readonly mount PASS
[ucompat-v211] execve from fs image PASS
[ucompat-v212] fs image metadata io PASS
[ucompat-v213] image rootfs compatibility matrix PASS
[ucompat-v214] filesystem submission hardening PASS
```

## Phase 4: libc / init / shell compatibility, v215-v222

Goal:

- minimal libc ABI support;
- crt0/syscall wrappers;
- init process;
- shell or BusyBox-like command runner;
- basic command suite;
- rootfs layout.

Markers:

```text
[ucompat-v215] libc abi support PASS
[ucompat-v216] crt0 syscall wrappers PASS
[ucompat-v217] init process runner PASS
[ucompat-v218] shell command runner PASS
[ucompat-v219] busybox style applets PASS
[ucompat-v220] rootfs layout compatibility PASS
[ucompat-v221] userland environment variables PASS
[ucompat-v222] userland os smoke suite PASS
```

## Phase 5: Drivers and devices, v223-v230

Goal:

- device registry;
- devfs auto nodes;
- UART input;
- TTY line discipline foundation;
- console read/write;
- RTC/clocksource;
- random source hardening;
- virtio input or framebuffer preparation.

Markers:

```text
[ucompat-v223] device registry PASS
[ucompat-v224] devfs auto nodes PASS
[ucompat-v225] uart input console PASS
[ucompat-v226] tty line discipline foundation PASS
[ucompat-v227] rtc clocksource PASS
[ucompat-v228] random device hardening PASS
[ucompat-v229] framebuffer preparation PASS
[ucompat-v230] driver subsystem smoke PASS
```

## Phase 6: Networking, v231-v238

Goal:

- AF_UNIX completeness;
- loopback interface;
- UDP localhost;
- TCP subset if required;
- socket poll integration;
- virtio-net if required.

Markers:

```text
[ucompat-v231] af unix compatibility PASS
[ucompat-v232] loopback interface PASS
[ucompat-v233] udp localhost PASS
[ucompat-v234] tcp subset foundation PASS
[ucompat-v235] socket poll integration PASS
[ucompat-v236] dns resolver smoke PASS
[ucompat-v237] virtio net backend PASS
[ucompat-v238] network matrix PASS
```

## Phase 7: POSIX ABI and security deepening, v239-v248

Goal:

- POSIX signal ABI;
- siginfo/ucontext;
- sigaltstack;
- credentials/capabilities inheritance;
- mount/pid/ipc namespace deepening;
- chroot/pivot_root;
- seccomp/BPF optional subset.

Markers:

```text
[ucompat-v239] posix signal abi PASS
[ucompat-v240] siginfo ucontext PASS
[ucompat-v241] sigaltstack PASS
[ucompat-v242] credential inheritance PASS
[ucompat-v243] capability inheritance PASS
[ucompat-v244] mount namespace deepening PASS
[ucompat-v245] pid ipc namespace deepening PASS
[ucompat-v246] chroot pivot root PASS
[ucompat-v247] seccomp bpf subset PASS
[ucompat-v248] security namespace matrix PASS
```

## Phase 8: HAL / LoongArch64 if required, v249-v258

Goal:

- HAL extraction;
- architecture-independent syscall core;
- LoongArch64 boot/trap/syscall/page-table;
- dual-arch QEMU regression.

Markers:

```text
[ucompat-v249] hal trait extraction PASS
[ucompat-v250] arch independent syscall core PASS
[ucompat-v251] loongarch64 boot skeleton PASS
[ucompat-v252] loongarch64 trap syscall PASS
[ucompat-v253] loongarch64 page table PASS
[ucompat-v254] loongarch64 user smoke PASS
[ucompat-v255] dual arch build PASS
[ucompat-v256] dual arch qemu PASS
[ucompat-v257] dual arch syscall matrix PASS
[ucompat-v258] dual arch readiness PASS
```

## Phase 9: final OS / submission hardening, v259-v270

Goal:

- syscall support matrix;
- known limitations;
- one-command build/run;
- performance evidence;
- final technical report;
- competition submission audit;
- OS readiness matrix.

Markers:

```text
[ucompat-v259] syscall support matrix PASS
[ucompat-v260] known limitations report PASS
[ucompat-v261] one command clean build run PASS
[ucompat-v262] performance evidence PASS
[ucompat-v263] final technical report PASS
[ucompat-v264] competition submission audit PASS
[ucompat-v265] ltp style smoke PASS
[ucompat-v266] posix style smoke PASS
[ucompat-v267] busybox smoke PASS
[ucompat-v268] os readiness matrix PASS
[ucompat-v269] release package PASS
[ucompat-v270] linux compatible subset readiness PASS
```
