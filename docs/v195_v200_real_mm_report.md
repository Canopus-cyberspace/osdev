# v195-v200 Real Memory Manager Report

## Files changed
- src/fs/runtime.rs
- src/mm/sv39_init_exec.rs
- apply_fix.sh
- apply_fix.bat
- docs/v195_v200_real_mm_report.md

## Scope
- Bounded batch: v195-v200 only.
- Baseline preserved: all fresh QEMU runtime markers from v151k7 through v194.
- Intentionally not implemented in this batch: scheduler blocking, ext4, virtio-blk, LoongArch64, GUI, networking, and dynamic kernel modules.

## v195 physical page allocator
- Added a 32-page aligned physical page pool used by the real Sv39 U-mode fault path.
- Tracks allocation count, free count, exhaustion count, and double-free rejection count.
- Fresh QEMU evidence shows all pages allocated, one exhaustion attempt rejected, all pages freed, and a double-free rejected.

## v196 user page-table mapping
- Added map/unmap/query/protect helpers over the live USER_L0_TABLE PTEs.
- The verifier allocates a real page, maps it with R/W/U bits, queries the PTE, downgrades it to read-only, queries again, then unmaps and frees it.
- The test confirms the physical page index behind the PTE and checks that unmap clears the valid bit.

## v197 real page-fault lazy allocation
- The U-mode program /umode/v197lazy.elf calls brk, mmap, and then writes to heap, mmap, and a lower stack page that were intentionally left unmapped.
- Store page faults are handled by the kernel page-fault path, which validates the canonical VMA, allocates a physical page, installs a real PTE, and resumes the faulting instruction.
- Fresh serial evidence includes heap, mmap, and stack fault mappings with physical addresses and PTE flags.

## v198 page permission and unmap
- mprotect now updates existing real PTE permissions after the canonical VMA update.
- munmap now clears real PTEs and frees allocator-backed pages after the canonical VMA update.
- /umode/v198ro.elf proves write-to-read-only triggers a store page fault after mprotect.
- /umode/v198unmap.elf proves access-after-munmap triggers a load page fault with no valid PTE and no covering VMA.

## v199 fork address-space copy/COW foundation
- Adds a concrete page-copy foundation tied to clone_task.
- The verifier clones a child, copies a real parent page into a child page, mutates the child page, verifies the parent byte is unchanged, exits the child, and reaps the expected status.

## v200 memory stress suite
- /umode/v200stress.elf repeatedly grows brk, faults four heap pages, mmaps three anonymous pages, faults all three, munmaps one page, mprotects another, and exits only after the accesses complete.
- Fresh serial evidence shows seven real lazy fault allocations and one freed page after munmap.

## Fresh QEMU evidence excerpts
- [ucompat-v195] allocator evidence pages=32 alloc=32 free=32 exhaust=1 double_free=1 PASS
- [ucompat-v196] mapping evidence va=<va> page=<idx> flags=RWU protect=RU unmap=1 PASS
- [ucompat-v197-user] lazy heap mmap stack faults
- [ucompat-v197] real page fault lazy allocation fault evidence ... kind=heap ... PASS
- [ucompat-v197] real page fault lazy allocation fault evidence ... kind=mmap ... PASS
- [ucompat-v197] real page fault lazy allocation fault evidence ... kind=stack ... PASS
- [ucompat-v198] readonly fault evidence ... PASS
- [ucompat-v198] munmap fault evidence ... unmapped=1 PASS
- [ucompat-v199] fork copy evidence ... parent_byte=0x91 child_byte=0x92 ... PASS
- [ucompat-v200] stress evidence faults=7 alloc=7 free=1 PASS

## New runtime markers
- [ucompat-v195] physical page allocator PASS
- [ucompat-v196] user page table mapping PASS
- [ucompat-v197] real page fault lazy allocation PASS
- [ucompat-v198] page permission unmap PASS
- [ucompat-v199] fork address space copy cow foundation PASS
- [ucompat-v200] memory stress suite PASS

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
- [ucompat-v191] real rootfs elf execution bridge PASS
- [ucompat-v192] real multi program umode matrix PASS
- [ucompat-v193] fork exec wait real path PASS
- [ucompat-v194] userland abi hardening PASS

## Build log path
.repair_logs/v195_v200_real_mm_20260509_194011/cargo_build.log

## QEMU serial log path
.repair_logs/v195_v200_real_mm_20260509_194011/qemu.serial.log

## QEMU wrapper stdout path
.repair_logs/v195_v200_real_mm_20260509_194011/run-qemu.stdout.log

## Forbidden warning gate result
- PASS: build output did not contain "matches any value", "unreachable pattern", or "warning: unused variable:".
