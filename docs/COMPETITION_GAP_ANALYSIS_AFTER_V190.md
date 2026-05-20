# Competition Gap Analysis after v190

## Strong areas

- RISC-V64 QEMU boot path.
- Sv39 + U-mode smoke.
- live syscall path.
- canonical runtime state.
- VFS/FD/OFD/procfs/devfs/rootfs/mount.
- eventfd/timerfd/pipe/socket/epoll readiness.
- IPC registry and wait queue foundations.
- task table and process lifecycle metadata.
- execve metadata from canonical VFS.
- VMA metadata and lazy mapping model.
- signal delivery foundation.
- credentials/capability/namespace basics.
- canonical userland readiness matrix.

## Highest remaining risks

### Internal runner vs real user program execution

Official evaluation runs user programs. v191-v194 should prove real U-mode ELF execution.

### Metadata-only page faults

Complex programs need real page allocation and mappings. v195-v200 should address this.

### Blocking and scheduling

Poll/wait/futex/sleep tests need real blocking/wakeup. v201-v206 should address this.

### Filesystem image format

Competition may require Ext4 or a provided rootfs image. v207-v214 should address this if needed.

### Architecture requirements

Some years may require LoongArch64. v215-v224 should address this if needed.

### Submission evidence

Need syscall matrix, limitations, one-command build/run, performance evidence, and final report. v225-v230 should address this.
