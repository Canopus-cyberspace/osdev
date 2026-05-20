# PROJECT_STATE

## Current verified milestone

- v45: Sv39 + U-mode ecall smoke passed.
- v46f: static ELF loader parser scaffold added while preserving Sv39 + U-mode smoke.

## Verified capabilities

- OpenSBI enters kernel.
- QEMU serial-file logging works.
- Kernel Sv39 activation works.
- Kernel Sv39 trap smoke works.
- Sv39 + U-mode ecall works.
- sys_write / getpid / getppid / ENOSYS / exit work in the Sv39 U-mode smoke path.
- Static ELF64 header and PT_LOAD parser scaffold exists.

## Current constraints

- ELF loader does not yet load segments into real user address space.
- execve is not implemented.
- VFS/rootfs are still scaffold/stub level.

## v46f

Static ELF loader parser scaffold passed; Sv39 + U-mode ecall smoke remains passing.

## v47 - ELF-linked user image loader scaffold

- Added static ELF parser scaffold.
- Added PT_LOAD parser scaffold.
- Added linked `.user` image metadata path.
- Kept existing Sv39 + U-mode syscall smoke as regression gate.

## v48

- v48: external user/init ELF scaffold.
- Synthetic user/init.elf is embedded via include_bytes!.
- Loader checks ELF64/RISC-V header and PT_LOAD metadata.
- Sv39 + U-mode ecall smoke remains the runtime regression path.

## v49 - External init ELF load-page scaffold

- Generates and embeds `user/init.elf`.
- Parses ELF64/RISC-V header and PT_LOAD segment.
- Copies PT_LOAD bytes into a kernel-owned page as a load dry-run.
- Keeps the already passing Sv39 + U-mode ecall smoke path as regression.

## v49c external init load-page scaffold

- ELF loader parser scaffold retained.
- External `user/init.elf` is embedded and copied into a kernel load page.
- Sv39 + U-mode smoke regression remains the required runtime check.

## v49d

- Fixed crate root config module visibility for loader init-image code.
- External init ELF load-page scaffold remains present.
- Sv39 + U-mode ecall smoke remains passing.

## v50 - External init ELF execution path

- Added generated external `user/init.elf`
- Added loader/init-image path for loading PT_LOAD into a kernel-managed page
- Added Sv39 U-mode execution path using external init ELF entry
- Expected external init syscalls:
  - write
  - getpid
  - getppid
  - unsupported -> -38
  - exit

## v50b - External init ELF execution trap fix

- Added robust external init ELF Sv39 U-mode execution path
- Replaced minimal trap frame with full TrapContext save/restore
- Restores sscratch to trap_stack_top before returning to U-mode
- Expected external init syscalls:
  - write
  - getpid
  - getppid
  - unsupported -> -38
  - exit

## v51 - Process initialization scaffold for execve

- Added `loader::process_image`
- Added `UserProgram`
- Added `ProcessInitInfo`
- Wrapped external `init.elf` load result in process initialization metadata
- Kept external init ELF Sv39 U-mode execution smoke passing

## v51 - Process initialization scaffold for execve

- Added `loader::process_image`
- Added `UserProgram`
- Added `ProcessInitInfo`
- Wrapped external `init.elf` load result in process initialization metadata
- Kept external init ELF Sv39 U-mode execution smoke passing

## v52 - Larger batch: ProcessInitInfo + initial user stack dry-run

- Added/updated `loader::process_image`
- Added `loader::user_stack`
- Added `ProcessInitInfo`
- Added `UserProgram`
- Added initial user stack dry-run with argc/argv/envp/auxv placeholders
- Kept external init ELF Sv39 U-mode execution smoke passing

## v53 - Larger batch: process/fd/syscall scaffold

- Added process metadata scaffold
- Added Process / ProcessState
- Added PID allocator scaffold
- Added fd table scaffold
- Added syscall dispatch scaffold
- Added execve scaffold that validates the embedded `/init` program metadata
- External init ELF Sv39 U-mode smoke remains the runtime regression path

## v53 - Larger batch: process/fd/syscall scaffold

- Added process metadata scaffold
- Added Process / ProcessState
- Added PID allocator scaffold
- Added fd table scaffold
- Added syscall dispatch scaffold
- Added execve scaffold that validates the embedded `/init` program metadata
- External init ELF Sv39 U-mode smoke remains the runtime regression path

## v53b - Syscall module conflict fix

- Fixed E0761 by deleting `src/syscall.rs`
- Kept `src/syscall/mod.rs` as the canonical syscall module
- Preserved process/fd/syscall scaffold self-tests
- External init ELF Sv39 U-mode smoke remains the runtime regression path

## v53c - Safe process/fd/syscall scaffold regression fix

- Fixed syscall module conflict by keeping `src/syscall/mod.rs`
- Converted process/fd/syscall scaffold tests to non-panicking runtime checks
- Kept external init ELF Sv39 U-mode smoke as the primary regression path

## v53d - Isolate scaffold from runtime smoke

- Fixed syscall module conflict by keeping only `src/syscall/mod.rs`
- Kept process/fd/syscall scaffold compiled
- Isolated v53 scaffold self-tests from runtime
- Restored external init ELF Sv39 U-mode smoke as the only QEMU regression path

## v53f - Trap entry alignment fix

- Diagnosed v53d regression: external init reached `enter user`, then rebooted before first user ecall handler output.
- Root cause: `stvec` trap entry could become 2-byte aligned after code layout changes.
- RISC-V `stvec` direct mode needs low two bits clear, so the trap entry is now explicitly `.balign 4`.
- The external init ELF Sv39 U-mode smoke remains the primary regression path.

## v54 - Central syscall dispatcher

- Added central runtime syscall dispatcher in `src/syscall/mod.rs`
- Moved runtime syscall decision out of `sv39_init_exec.rs`
- External init ELF trap handler now delegates to `syscall::dispatch_runtime_syscall`
- Preserved v53f trap entry alignment fix
- External init ELF Sv39 U-mode smoke remains passing

## v54 - Central syscall dispatcher

- Added central runtime syscall dispatcher in `src/syscall/mod.rs`
- Moved runtime syscall decision out of `sv39_init_exec.rs`
- External init ELF trap handler now delegates to `syscall::dispatch_runtime_syscall`
- Preserved v53f trap entry alignment fix
- External init ELF Sv39 U-mode smoke remains passing

## v55 - fd-backed write dispatcher

- Added fd-backed write routing
- stdout/stderr route to console
- `/dev/null` scaffold target reserved at fd 3
- bad fd returns -EBADF
- External init ELF write/getpid/getppid/ENOSYS/exit remains passing

## v56 - openat/close `/dev/null` scaffold

- Extended external init ELF to call openat and close
- Added `/dev/null` open/close runtime scaffold
- Added central syscall actions for openat and close
- Kept external init ELF Sv39 U-mode smoke passing

## v57 - read `/dev/zero` scaffold

- Extended external init ELF to call read
- Added `/dev/zero` open/read/close runtime scaffold
- Added central syscall action for read
- Kept external init ELF Sv39 U-mode smoke passing

## v58 - fstat/lseek scaffold

- Extended external init ELF to call fstat and lseek
- Added minimal stat copy-out to user buffer
- Added lseek scaffold returning -ESPIPE for character devices
- Kept external init ELF Sv39 U-mode smoke passing

## v59 - getdents64 `/dev` scaffold

- Extended external init ELF to call getdents64
- Added `/dev` directory scaffold at fd 5
- Wrote minimal linux_dirent64 entries into user buffer
- Kept external init ELF Sv39 U-mode smoke passing

## v60 - brk heap scaffold

- Added `SYS_BRK = 214`
- Added central `RuntimeSyscallAction::Brk`
- Added fixed user heap mapping at `0x40030000..0x40034000`
- Implemented `brk(0)` query and in-range update
- Extended external init ELF to exercise brk
- Kept external init ELF Sv39 U-mode smoke passing

## v61 - mmap/munmap scaffold

- Added `SYS_MMAP = 222`
- Added `SYS_MUNMAP = 215`
- Added central `RuntimeSyscallAction::Mmap`
- Added central `RuntimeSyscallAction::Munmap`
- Added fixed mmap window at `0x40040000..0x40044000`
- Extended external init ELF to exercise mmap and munmap
- Kept external init ELF Sv39 U-mode smoke passing

## v62 - mprotect/madvise scaffold

- Added `SYS_MPROTECT = 226`
- Added `SYS_MADVISE = 233`
- Added central `RuntimeSyscallAction::Mprotect`
- Added central `RuntimeSyscallAction::Madvise`
- Extended external init ELF to exercise mprotect and madvise on fixed mmap area
- Kept external init ELF Sv39 U-mode smoke passing

## v63 - uname/time scaffold

- Added `SYS_CLOCK_GETTIME = 113`
- Added `SYS_UNAME = 160`
- Added `SYS_GETTIMEOFDAY = 169`
- Added central `RuntimeSyscallAction::Uname`
- Added central `RuntimeSyscallAction::ClockGettime`
- Added central `RuntimeSyscallAction::Gettimeofday`
- Extended external init ELF to exercise uname/time syscalls
- Kept external init ELF Sv39 U-mode smoke passing

## v64 - process/resource/random scaffold

- Added set_tid_address and set_robust_list scaffolds
- Added getuid/geteuid/getgid/getegid/gettid return values
- Added sysinfo scaffold
- Added prlimit64 scaffold
- Added getrandom scaffold
- Extended external init ELF to exercise process/resource/random syscalls
- Kept external init ELF Sv39 U-mode smoke passing

## v65 - path/tty/fcntl scaffold

- Added getcwd/chdir/readlinkat/umask scaffolds
- Added fcntl scaffold
- Added ioctl TIOCGWINSZ scaffold
- Extended external init ELF to exercise path/tty/fcntl syscalls
- Kept external init ELF Sv39 U-mode smoke passing

## v66 - signal/futex/sched scaffold

- Added sched_yield scaffold
- Added nanosleep scaffold
- Added futex scaffold
- Added rt_sigaction scaffold
- Added rt_sigprocmask scaffold
- Extended external init ELF to exercise signal/futex/sched syscalls
- Kept external init ELF Sv39 U-mode smoke passing

## v67 - event/pipe/dup/poll scaffold

- Added eventfd2 scaffold
- Added epoll_create1/epoll_ctl/epoll_pwait scaffolds
- Added ppoll/pselect6 scaffolds
- Added pipe2 scaffold
- Added dup/dup3 scaffolds
- Extended external init ELF to exercise event/pipe/dup/poll syscalls
- Kept external init ELF Sv39 U-mode smoke passing

## v68 - filesystem metadata scaffold

- Added mkdirat/unlinkat/faccessat/newfstatat/renameat2/statx scaffolds
- Extended external init ELF to exercise filesystem metadata syscalls
- Kept external init ELF Sv39 U-mode smoke passing

## v69 - process lifecycle scaffold

- Added clone/wait4/execve/kill/tgkill/exit_group runtime scaffolds
- Extended external init ELF to exercise process lifecycle syscalls
- Switched final user termination test to exit_group(0)
- Kept external init ELF Sv39 U-mode smoke passing

## v70 - fs sync mount scaffold

- Added mount/umount2 scaffolds
- Added statfs/fstatfs scaffolds
- Added truncate/ftruncate/fallocate scaffolds
- Added sync/fsync/fdatasync scaffolds
- Added utimensat scaffold
- Extended external init ELF to exercise fs sync/mount syscalls
- Retained clone/wait4/execve/kill/tgkill/exit_group runtime scaffolds
- Switched final user termination test to exit_group(0)
- Kept external init ELF Sv39 U-mode smoke passing

## v70c - tgkill builder fix

- Fixed undefined `syscall_tgkill_pid1_sig0()` in `user/build_init_elf.py`
- Normalized the helper to `syscall_tgkill_pid1_tid1_sig0()`
- Kept v70 fs sync/mount syscall scaffold intact

## v71b - scheduler/resource/prctl scaffold

- Added scheduler query syscall scaffolds
- Added clock_getres/clock_nanosleep scaffolds
- Added getrusage scaffold
- Added prctl/getcpu/riscv_flush_icache/membarrier scaffolds
- Extended external init ELF to exercise these compatibility syscalls
- Kept external init ELF Sv39 U-mode smoke passing

## v72 - socket/network scaffold

- Added socket/socketpair scaffolds
- Added bind/listen/accept/connect scaffolds
- Added getsockname/getpeername scaffolds
- Added sendto/recvfrom scaffolds
- Added setsockopt/getsockopt/shutdown scaffolds
- Extended external init ELF to exercise socket compatibility syscalls
- Kept external init ELF Sv39 U-mode smoke passing

## v73 - vector/range I/O scaffold

- Added readv/writev syscall scaffolds
- Added pread64/pwrite64 syscall scaffolds
- Added preadv/pwritev syscall scaffolds
- Added sendfile/vmsplice/splice/tee/copy_file_range scaffolds
- Extended external init ELF to exercise vector/range I/O compatibility syscalls
- Kept external init ELF Sv39 U-mode smoke passing

## v74 - identity/session/capability scaffold

- Added capget/capset/personality scaffolds
- Added uid/gid mutation and query scaffolds
- Added process group/session scaffolds
- Added getgroups/setgroups scaffolds
- Added getrlimit/setrlimit scaffolds
- Added getpriority/setpriority scaffolds
- Added times scaffold
- Extended external init ELF to exercise identity/session/capability compatibility syscalls
- Kept external init ELF Sv39 U-mode smoke passing

## v75 - event/timer/misc fd scaffold

- Added inotify_init1/add_watch/rm_watch scaffolds
- Added signalfd4 scaffold
- Added timerfd_create/settime/gettime scaffolds
- Added getitimer/setitimer scaffolds
- Added ioprio_set/ioprio_get and flock scaffolds
- Added sync_file_range scaffold
- Extended external init ELF to exercise event/timer/misc compatibility syscalls
- Kept external init ELF Sv39 U-mode smoke passing

## v76 - memory/policy/fd scaffold

- Added mremap/msync/mlock/munlock/mlockall/munlockall/mincore scaffolds
- Added remap_file_pages/mbind/get_mempolicy/set_mempolicy scaffolds
- Added memfd_create/userfaultfd scaffolds
- Extended external init ELF to exercise memory/policy/fd compatibility syscalls
- Kept external init ELF Sv39 U-mode smoke passing

## v75 - event/timer/misc fd scaffold

- Added inotify_init1/add_watch/rm_watch scaffolds
- Added signalfd4 scaffold
- Added timerfd_create/settime/gettime scaffolds
- Added getitimer/setitimer scaffolds
- Added ioprio_set/ioprio_get and flock scaffolds
- Added sync_file_range scaffold
- Extended external init ELF to exercise event/timer/misc compatibility syscalls
- Kept external init ELF Sv39 U-mode smoke passing

## v76 - memory/policy/fd scaffold

- Added mremap/msync/mlock/munlock/mlockall/munlockall/mincore scaffolds
- Added remap_file_pages/mbind/get_mempolicy/set_mempolicy scaffolds
- Added memfd_create/userfaultfd scaffolds
- Extended external init ELF to exercise memory/policy/fd compatibility syscalls
- Kept external init ELF Sv39 U-mode smoke passing

## v76 - memory/policy/fd scaffold

- Added mremap/msync/mlock/munlock/mlockall/munlockall/mincore scaffolds
- Added remap_file_pages/mbind/get_mempolicy/set_mempolicy scaffolds
- Added memfd_create/userfaultfd scaffolds
- Extended external init ELF to exercise memory/policy/fd compatibility syscalls
- Kept external init ELF Sv39 U-mode smoke passing

## v77 - modern Linux syscall scaffold

- Added pidfd syscall scaffolds
- Added clone3/close_range/openat2/faccessat2 scaffolds
- Added io_uring scaffolds
- Added new mount API scaffolds
- Added landlock scaffolds
- Added memfd_secret/process_mrelease/futex_waitv/set_mempolicy_home_node scaffolds
- Extended external init ELF to exercise modern Linux compatibility syscalls
- Kept external init ELF Sv39 U-mode smoke passing

## v78 - security/observability/compat scaffold

- Added security/observability syscall scaffolds
- Added perf/fanotify/name-handle/process-vm/kcmp/module/sched-attr scaffolds
- Added seccomp/bpf/execveat/mlock2/preadv2/pwritev2/pkey scaffolds
- Extended external init ELF to exercise these compatibility syscalls
- Kept external init ELF Sv39 U-mode smoke passing

## v79 - xattr/path/permission scaffold

- Added extended-attribute syscall scaffolds
- Added symlinkat/linkat scaffolds
- Added fchmod/fchmodat/fchown/fchownat scaffolds
- Added fchdir/chroot/pivot_root scaffolds
- Added vhangup/quotactl/lookup_dcookie/nfsservctl scaffolds
- Extended external init ELF to exercise these compatibility syscalls
- Kept external init ELF Sv39 U-mode smoke passing

## v81 - AIO/timer/key/scheduler scaffold

- Added Linux AIO syscall scaffolds
- Added waitid/unshare/robust-list scaffolds
- Added POSIX timer and clock_settime scaffolds
- Added scheduler mutation scaffolds
- Added key management syscall scaffolds
- Extended external init ELF to exercise these compatibility syscalls
- Kept external init ELF Sv39 U-mode smoke passing

## v82 - IPC/message/net I/O scaffold

- Added POSIX message queue syscall scaffolds
- Added SysV message/semaphore/shared-memory syscall scaffolds
- Added recvmsg/sendmsg/recvmmsg/sendmmsg scaffolds
- Added readahead/fadvise64 scaffolds
- Added rt_tgsigqueueinfo scaffold
- Extended external init ELF to exercise these compatibility syscalls
- Kept external init ELF Sv39 U-mode smoke passing
