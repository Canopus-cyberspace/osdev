# v169-v172 Execve/MM Report

## Files changed
- src/fs/runtime.rs
- src/mm/sv39_init_exec.rs
- apply_fix.sh
- apply_fix.bat
- docs/v169_v172_execve_mm_report.md

## Canonical subsystem choices
- Exec image state lives in src/fs/runtime.rs beside the canonical VFS/fd/task state.
- VMA metadata is per task address space via task mm_id and VmaObj records.
- Live syscall route remains rust_main -> mm::sv39_init_exec::run_external_init_elf_smoke -> dispatch_runtime_syscall -> RuntimeSyscallAction.
- brk, mmap, munmap, mprotect, and madvise now route into the shared VMA model from the live syscall handlers.

## v169 execve from canonical VFS
- execve resolves the executable path through KernelCore::resolve_at.
- Executable bytes are read from the canonical VFS file node.
- ELF64/RISC-V header, program header table, PT_LOAD bounds, alignment, filesz/memsz, and executable entry coverage are validated.
- A prepared ExecImage snapshot records entry, phdr metadata, load bounds, file size, memory size, stack metadata, mm_id, and sequence.
- Invalid ELF files return ENOEXEC; missing files return ENOENT.

## v170 argv/envp/auxv stack and CLOEXEC
- RuntimeExecString carries bounded argv/envp strings without heap allocation.
- execve computes a downward-growing user stack layout with argc, argv pointers, envp pointers, and auxv metadata.
- auxv metadata includes the ELF entry/phdr/page-size style foundation needed by later user-mode replacement work.
- Successful execve closes canonical fds marked FD_CLOEXEC while preserving non-CLOEXEC fds.
- The live sys_execve_user path reads user path, argv, and envp pointers before calling the canonical runtime helper.

## v171 VMA/page-fault foundation
- Tasks carry an mm_id and VMA records are keyed by mm_id.
- execve installs PT_LOAD VMAs and a lazy user stack VMA for the new address space.
- RuntimeFaultAccess validates read/write/execute faults against VMA permissions.
- Lazy VMA fault validation records resident-page evidence and last-fault status.
- Empty heap faults are rejected until brk creates heap VMA coverage.

## v172 lazy brk/mmap/munmap/mprotect
- brk updates the process heap break and creates/shrinks a lazy heap VMA.
- mmap creates lazy anonymous/file-descriptor-backed VMA metadata in the mmap window.
- munmap removes, trims, or splits VMA records.
- mprotect splits VMAs at range boundaries and changes permissions for the protected range.
- madvise validates that the range is covered by existing VMA metadata.

## Out of scope
- No signal delivery, mount/security policy, network-driver work, full scheduler work, or real process image jump was added.
- Lazy page allocation is represented by VMA/page-fault validation metadata; this batch does not add a physical page allocator or TLB shootdown path.

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

## New runtime markers
- [ucompat-v169] execve from canonical vfs PASS
- [ucompat-v170] execve user stack cloexec PASS
- [ucompat-v171] vma page fault foundation PASS
- [ucompat-v172] lazy mmap brk munmap mprotect PASS

## Build log path
.repair_logs/v169_v172_execve_mm_20260508_155853/cargo_build.log

## QEMU serial log path
.repair_logs/v169_v172_execve_mm_20260508_155853/qemu.serial.log

## QEMU wrapper stdout path
.repair_logs/v169_v172_execve_mm_20260508_155853/run-qemu.stdout.log

## Forbidden warning gate result
- PASS: build output did not contain "matches any value", "unreachable pattern", or "warning: unused variable:".
