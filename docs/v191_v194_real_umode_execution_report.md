# v191-v194 Real U-Mode Execution Report

## Files changed
- src/fs/runtime.rs
- src/mm/sv39_init_exec.rs
- apply_fix.sh
- apply_fix.bat
- docs/v191_v194_real_umode_execution_report.md

## Scope
- Bounded batch: v191-v194 only.
- Baseline preserved: v151k7 through v190 fresh QEMU runtime markers.
- Intentionally not implemented in this batch: real page allocator, full scheduler, ext4, virtio-blk, LoongArch64, GUI, and dynamic kernel modules.

## v191 real rootfs ELF execution bridge
- Added a bridge from the canonical rootfs exec metadata to actual Sv39 U-mode execution after the external init ELF exits.
- The bridge writes a small RISC-V ELF to rootfs, resolves it through execve_from_vfs, validates ELF header and PT_LOAD metadata, copies the load image into a user page, builds a user stack, updates the trap context, and returns through sret into U-mode.
- The v191 user ELF performs write(1), getpid(), and exit(11); the kernel prints the PASS marker only after observing the expected exit code.

## v192 real multi-program U-mode matrix
- Stores and runs three independent rootfs ELF programs: /umode/v192a.elf, /umode/v192b.elf, and /umode/v192c.elf.
- Each program enters U-mode, writes its own matrix line, calls getpid(), and exits with a distinct expected status: 21, 22, and 23.
- The runtime collects each exit status before printing the v192 PASS marker.

## v193 fork/exec/wait real path
- Creates a canonical child task with clone_task, switches the current canonical task to the child, execs /umode/v193child.elf, and enters U-mode.
- The child writes a userland line, calls getpid(), exits with 33, and the parent restores current task state and reaps the encoded status through wait4.

## v194 userland ABI hardening
- Builds argc/argv/envp/auxv data on the real user stack.
- The v194 U-mode program verifies argc == 2, argv[0] points to the executed path, argv termination is present, envp is present, and the first auxv entry reports AT_PAGESZ.
- The kernel also validates CLOEXEC cleanup across exec and stable errors for a missing path and invalid ELF content before printing the v194 PASS marker.

## Canonical runtime integration
- Rootfs files are stored in the shared tmpfs/rootfs node model.
- DATA_MAX was raised to 1024 bytes so canonical regular files can hold the small rootfs ELF fixtures without truncation.
- A public switch_current_task(pid) wrapper was added over the existing canonical task table so the v193 path can switch to the cloned child and back to pid 1.
- The v191-v194 bridge runs from the existing live route: rust_main -> run_external_init_elf_smoke -> dispatch_runtime_syscall -> RuntimeSyscallAction.

## Fresh QEMU evidence lines
- [ucompat-v191-user] write getpid exit
- [ucompat-v192-user] program=a
- [ucompat-v192-user] program=b
- [ucompat-v192-user] program=c
- [ucompat-v193-user] child fork-exec body
- [ucompat-v194-user] abi stack verified
- [ucompat-v194] abi evidence cloexec_fd=<fd> closed=1 missing_errno=-2 bad_elf_errno=-8 PASS

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
- [ucompat-v185] multi elf rootfs runner PASS
- [ucompat-v186] libc syscall matrix PASS
- [ucompat-v187] fs process memory suite PASS
- [ucompat-v188] signal pipe poll ipc suite PASS
- [ucompat-v189] stress error path hardening PASS
- [ucompat-v190] final competition kernel readiness PASS

## New runtime markers
- [ucompat-v191] real rootfs elf execution bridge PASS
- [ucompat-v192] real multi program umode matrix PASS
- [ucompat-v193] fork exec wait real path PASS
- [ucompat-v194] userland abi hardening PASS

## Build log path
.repair_logs/v191_v194_real_umode_execution_20260509_132627/cargo_build.log

## QEMU serial log path
.repair_logs/v191_v194_real_umode_execution_20260509_132627/qemu.serial.log

## QEMU wrapper stdout path
.repair_logs/v191_v194_real_umode_execution_20260509_132627/run-qemu.stdout.log

## Forbidden warning gate result
- PASS: build output did not contain "matches any value", "unreachable pattern", or "warning: unused variable:".
