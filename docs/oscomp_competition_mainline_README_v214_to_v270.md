# OSComp Competition Mainline Plan: v214 → v270

## Core decision

從 v214 之後立刻進入比賽要求主線。

這不是：

```text
v214 → 隨便擴功能 → v270 → 再回比賽要求
```

而是：

```text
v214 → v215-v270 全程都是比賽要求主線 → v270 competition readiness baseline
```

## Current baseline

```text
stable baseline: v214
next version:    v215
target:          competition mainline convergence by v270
```

## Main route

```text
v215-v222:
    real virtio-blk
    disk image probing
    readonly Ext4
    execve from real Ext4
    init/rootfs handoff

v223-v230:
    official autotest adapter
    sdcard-rv smoke
    BusyBox/sh minimal compatibility
    Linux syscall matrix expansion
    proc/dev/tty compatibility
    shell process/pipeline behavior
    official RV convergence report

v231-v238:
    Linux user program syscall and compatibility hardening

v239-v246:
    LoongArch64 HAL/bootstrap/trap/syscall/page-table/context-switch

v247-v254:
    virtio-net and minimal real network path

v255-v270:
    official judge convergence
    stress regression
    technical report
    open-source attribution
    submission packaging
    clean rebuild
    final readiness baseline
```

## Non-negotiable rules

- 不再擴 marker 當主線。
- 不再 active_once-only。
- 不把 ramdisk 當成 virtio-blk 的最終替代。
- 不把 local socket/loopback 當成 virtio-net 的最終替代。
- 不硬編碼官方測例輸出。
- 所有新增功能要有 fresh runtime evidence。
- 保留 v151k7-v214 已通過行為。

## Immediate next package

```text
v215_real_virtio_blk_mmio_queue_descriptor_path
```

Required evidence:

```text
[ucompat-v215] real virtio blk queue path PASS
device=virtio-blk
not_ramdisk=true
sector read/write via virtqueue
```
