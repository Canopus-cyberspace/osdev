# Post-v190 Master Plan

## Execution principle

Long-range plan is complete; Codex should implement one bounded batch at a time.

## Phase A: real user-mode execution, v191-v194

### v191

```text
[ucompat-v191] real rootfs elf execution bridge PASS
```

Store ELF in rootfs, execve it, enter U-mode, run write/getpid/exit.

### v192

```text
[ucompat-v192] real multi program umode matrix PASS
```

Run multiple independent rootfs ELF programs and collect exit codes.

### v193

```text
[ucompat-v193] fork exec wait real path PASS
```

Parent creates/execs child and waits for status.

### v194

```text
[ucompat-v194] userland abi hardening PASS
```

argv/envp/auxv, CLOEXEC, invalid path, invalid ELF.

## Phase B: real memory manager, v195-v200

```text
[ucompat-v195] physical page allocator PASS
[ucompat-v196] user page table mapping PASS
[ucompat-v197] real page fault lazy allocation PASS
[ucompat-v198] page permission unmap PASS
[ucompat-v199] fork address space copy cow foundation PASS
[ucompat-v200] memory stress suite PASS
```

Goal: convert VMA/page-fault metadata into real page mappings.

## Phase C: scheduler and blocking, v201-v206

```text
[ucompat-v201] run queue task switch PASS
[ucompat-v202] timer tick scheduling PASS
[ucompat-v203] blocking wait wakeup PASS
[ucompat-v204] pipe poll futex blocking PASS
[ucompat-v205] wait sleep timer blocking PASS
[ucompat-v206] scheduler regression suite PASS
```

Goal: real task blocking and wakeup.

## Phase D: block device and filesystem image, v207-v214

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

Goal: competition image support if required.

## Phase E: HAL and LoongArch64 if required, v215-v224

```text
[ucompat-v215] hal trait extraction PASS
[ucompat-v216] arch independent syscall core PASS
[ucompat-v217] loongarch64 boot skeleton PASS
[ucompat-v218] loongarch64 trap syscall smoke PASS
[ucompat-v219] loongarch64 page table foundation PASS
[ucompat-v220] loongarch64 user program smoke PASS
[ucompat-v221] dual arch build scripts PASS
[ucompat-v222] dual arch qemu regression PASS
[ucompat-v223] dual arch syscall matrix PASS
[ucompat-v224] dual arch readiness report PASS
```

Goal: dual architecture if required by the competition year.

## Phase F: submission hardening, v225-v230

```text
[ucompat-v225] syscall support matrix PASS
[ucompat-v226] known limitations report PASS
[ucompat-v227] one command clean build run PASS
[ucompat-v228] performance evidence collection PASS
[ucompat-v229] final technical report package PASS
[ucompat-v230] competition submission audit PASS
```

Goal: final submission artifacts and audit.
