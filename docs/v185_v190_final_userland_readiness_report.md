# v185-v190 Final Userland Readiness Report

## Files changed
- src/fs/runtime.rs
- src/mm/sv39_init_exec.rs
- apply_fix.sh
- apply_fix.bat
- docs/v185_v190_final_userland_readiness_report.md

## Canonical subsystem choices
- The final userland validation lives in the shared canonical runtime state in src/fs/runtime.rs.
- The live marker route remains rust_main -> mm::sv39_init_exec::run_external_init_elf_smoke -> dispatch_runtime_syscall -> RuntimeSyscallAction.
- The v185-v190 validators exercise existing rootfs, fd, procfs, devfs, task, signal, VMA, socket, IPC, scheduler wait, credential, and namespace state instead of marker-only smoke paths.

## v185 multi-ELF rootfs runner
- Added an internal canonical runner that stores multiple ELF fixtures in rootfs, clones a child task, switches to that task, calls execve_from_vfs, exits with a requested status, restores the parent, and reaps through wait4.
- The validator records entry address, mm id, exec sequence, pid, path length, exit code, and wait status for three separate rootfs-backed ELF files.

## v186 libc-style syscall matrix
- Validates filesystem and fd behavior through create/read/write/stat/getdents/dup/fcntl/proc fd links.
- Validates procfs status/maps and devfs zero/random reads.
- Validates timerfd-style time readiness, mmap/brk/munmap, pipe/poll, socketpair, mq/msg/sem/shm IPC, and credential mutation.

## v187 filesystem/process/memory suite
- Exercises create/read/write/stat/getdents/link/rename/unlink over a real runtime directory.
- Exercises clone, task snapshot, exit, wait4, and child status collection.
- Exercises execve, brk, mmap, page-fault validation, mprotect, and munmap through the VMA model.

## v188 signal/pipe/poll/ipc suite
- Exercises signal action installation, signal queueing, signal frame delivery, and rt_sigreturn restore.
- Exercises SIGCHLD queueing from child exit and wait4 reaping.
- Exercises pipe readiness through epoll and canonical mq/msg/sem/shm wait queues and wakeups.

## v189 stress and error-path hardening
- Validates EBADF, ENOENT, EEXIST, EINVAL, EAGAIN, ENOSPC, fd exhaustion, socket object exhaustion, and repeated open/close paths.
- Invalid memory ranges are checked through mmap/munmap and page-fault validation where the runtime currently models them.

## v190 final readiness
- Re-runs the v185-v189 validators without printing marker-only evidence, then verifies rootfs/mount, namespace, credential, and proc snapshots after a final reset.

## Preserved runtime markers
- [ucompat-v151k7] vfs_tree_dirfd_multiinode PASS
- [ucompat-v154] fs_core_multi_feature PASS
- [ucompat-v155] namespace_procfd_multi_feature PASS
- [ucompat-v156] procfs_fd_observability PASS
- [ucompat-v157] unified historical kernel integration PASS
- [ucompat-v158] event pipe socket readiness PASS
- [ucompat-v159] timerfd deterministic readiness PASS
- [ucompat-v160] fd lifecycle cloexec close_range PASS
- [ucompat-v161] unified iovec io path PASS
- [ucompat-v162] ipc registry lifecycle PASS
- [ucompat-v163] futex wait wake object model PASS
- [ucompat-v164] scheduler wait queue foundation PASS
- [ucompat-v165] task table process lifecycle PASS
- [ucompat-v166] fork clone child task PASS
- [ucompat-v167] exit zombie wait lifecycle PASS
- [ucompat-v168] per task runtime snapshot PASS
- [ucompat-v169] execve from canonical vfs PASS
- [ucompat-v170] execve user stack cloexec PASS
- [ucompat-v171] vma page fault foundation PASS
- [ucompat-v172] lazy mmap brk munmap mprotect PASS
- [ucompat-v173] signal frame rt_sigreturn PASS
- [ucompat-v174] sigchld process group signal PASS
- [ucompat-v175] rootfs tmpfs backend PASS
- [ucompat-v176] devfs core devices PASS
- [ucompat-v177] procfs process status maps PASS
- [ucompat-v178] mount tree statfs PASS
- [ucompat-v179] permissions credentials PASS
- [ucompat-v180] capability identity model PASS
- [ucompat-v181] unix socket loopback PASS
- [ucompat-v182] local datagram socket PASS
- [ucompat-v183] ipc blocking scheduler integration PASS
- [ucompat-v184] namespace basics PASS

## New runtime markers
- [ucompat-v185] multi elf rootfs runner PASS
- [ucompat-v186] libc syscall matrix PASS
- [ucompat-v187] fs process memory suite PASS
- [ucompat-v188] signal pipe poll ipc suite PASS
- [ucompat-v189] stress error path hardening PASS
- [ucompat-v190] final competition kernel readiness PASS

## Build log path
.repair_logs/v185_v190_final_userland_readiness_20260509_130053/cargo_build.log

## QEMU serial log path
.repair_logs/v185_v190_final_userland_readiness_20260509_130053/qemu.serial.log

## QEMU wrapper stdout path
.repair_logs/v185_v190_final_userland_readiness_20260509_130053/run-qemu.stdout.log

## Forbidden warning gate result
- PASS: build output did not contain "matches any value", "unreachable pattern", or "warning: unused variable:".
