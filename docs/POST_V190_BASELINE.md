# Post-v190 Baseline

## Current baseline

Stable baseline: **v190**.

## What v190 proves

v190 proves canonical runtime readiness:

- multi-ELF rootfs runner at canonical runtime layer
- libc-style syscall matrix at canonical runtime layer
- fs/process/memory suite
- signal/pipe/poll/ipc suite
- stress/error-path hardening
- final readiness marker
- all v151k7-v190 markers preserved

## Remaining high-risk gap

The v185-v190 report describes the runner as an **internal canonical runner**.

The next key milestone is real rootfs ELF U-mode execution:

```text
rootfs ELF bytes
-> execve
-> user address space
-> real U-mode jump
-> user program syscalls
-> exit
-> parent/runtime records status
```
