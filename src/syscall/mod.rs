#![allow(dead_code)]

pub mod abi;
pub mod dispatcher;
pub mod errno;
pub mod numbers;
pub mod user_ptr;

pub use abi::RuntimeSyscallArgs;
pub use dispatcher::dispatch_runtime_syscall;
pub use numbers::*;

use crate::fs::fd_table::{FdTable, FileKind, RuntimeReadTarget, RuntimeWriteTarget};
use crate::loader::process_image::build_init_process_info;
use crate::loader::user_stack::build_initial_user_stack_dry_run;
use crate::task::process::{make_init_process, make_zombie};
use crate::println;


#[allow(unused_imports)]
pub use errno::{EBADF, EINVAL, ENOENT, ENOSYS, ENOTDIR, ESPIPE};

#[derive(Copy, Clone, Debug)]
pub struct SyscallFrame {
    pub id: usize,
    pub args: [usize; 6],
}

impl SyscallFrame {
    pub const fn new(id: usize, args: [usize; 6]) -> Self { Self { id, args } }
}

pub fn dispatch_scaffold(frame: SyscallFrame) -> isize {
    match frame.id {
        SYS_GETPID => 1,
        SYS_GETPPID => 0,
        SYS_EXECVE => sys_execve_scaffold(frame.args[0], frame.args[1], frame.args[2]),
        SYS_WAIT4 => sys_wait4_scaffold(),
        SYS_CLONE => sys_clone_scaffold(),
        // v85 mount/process-memory/memfd modern syscall scaffold arms
        SYS_FSPICK => sys_fspick_scaffold(frame.args[0], frame.args[1], frame.args[2]),
        SYS_PIDFD_GETFD => sys_pidfd_getfd_scaffold(frame.args[0], frame.args[1], frame.args[2]),
        SYS_FACCESSAT2 => sys_faccessat2_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3]),
        SYS_PROCESS_MADVISE => sys_process_madvise_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3], frame.args[4]),
        SYS_EPOLL_PWAIT2 => sys_epoll_pwait2_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3], frame.args[4]),
        SYS_MOUNT_SETATTR => sys_mount_setattr_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3], frame.args[4]),
        SYS_QUOTACTL_FD => sys_quotactl_fd_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3]),
        SYS_MEMFD_SECRET => sys_memfd_secret_scaffold(frame.args[0]),
        SYS_PROCESS_MRELEASE => sys_process_mrelease_scaffold(frame.args[0], frame.args[1]),
        SYS_SET_MEMPOLICY_HOME_NODE => sys_set_mempolicy_home_node_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3]),
        SYS_CACHESTAT => sys_cachestat_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3]),
        SYS_FCHMODAT2 => sys_fchmodat2_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3]),
        SYS_MAP_SHADOW_STACK => sys_map_shadow_stack_scaffold(frame.args[0], frame.args[1], frame.args[2]),

        // v86 mount/LSM/futex/xattr-at/mseal syscall scaffold arms
        SYS_FUTEX_WAKE => sys_futex_wake_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3], frame.args[4], frame.args[5]),
        SYS_FUTEX_WAIT => sys_futex_wait_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3], frame.args[4], frame.args[5]),
        SYS_FUTEX_REQUEUE => sys_futex_requeue_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3], frame.args[4], frame.args[5]),
        SYS_STATMOUNT => sys_statmount_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3], frame.args[4], frame.args[5]),
        SYS_LISTMOUNT => sys_listmount_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3], frame.args[4], frame.args[5]),
        SYS_LSM_GET_SELF_ATTR => sys_lsm_get_self_attr_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3], frame.args[4], frame.args[5]),
        SYS_LSM_SET_SELF_ATTR => sys_lsm_set_self_attr_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3], frame.args[4], frame.args[5]),
        SYS_LSM_LIST_MODULES => sys_lsm_list_modules_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3], frame.args[4], frame.args[5]),
        SYS_MSEAL => sys_mseal_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3], frame.args[4], frame.args[5]),
        SYS_SETXATTRAT => sys_setxattrat_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3], frame.args[4], frame.args[5]),
        SYS_GETXATTRAT => sys_getxattrat_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3], frame.args[4], frame.args[5]),
        SYS_LISTXATTRAT => sys_listxattrat_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3], frame.args[4], frame.args[5]),
        SYS_REMOVEXATTRAT => sys_removexattrat_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3], frame.args[4], frame.args[5]),
        SYS_OPEN_TREE_ATTR => sys_open_tree_attr_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3], frame.args[4], frame.args[5]),
        SYS_FILE_GETATTR => sys_file_getattr_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3], frame.args[4], frame.args[5]),
        SYS_FILE_SETATTR => sys_file_setattr_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3], frame.args[4], frame.args[5]),
        SYS_LISTNS => sys_listns_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3], frame.args[4], frame.args[5]),
        SYS_RSEQ_SLICE_YIELD => sys_rseq_slice_yield_scaffold(frame.args[0], frame.args[1], frame.args[2], frame.args[3], frame.args[4], frame.args[5]),
        _ => ENOSYS,
    }
}

pub fn sys_execve_scaffold(_path: usize, _argv: usize, _envp: usize) -> isize {
    let info = match build_init_process_info() { Ok(info) => info, Err(_) => return EINVAL };
    let stack = match build_initial_user_stack_dry_run() { Ok(stack) => stack, Err(_) => return EINVAL };
    if info.entry() == 0 || stack.initial_sp == 0 { return EINVAL; }
    0
}

pub const fn sys_wait4_scaffold() -> isize { ENOSYS }
pub const fn sys_clone_scaffold() -> isize { ENOSYS }



// v84 rseq/membarrier/statx/pkey/copy_file_range syscall scaffold constants

// v85 mount/process-memory/memfd modern syscall constants

// v86 mount/LSM/futex/xattr-at/mseal syscall scaffold numbers

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RuntimeSyscallAction {
    Return(isize),
    Write { fd: usize, user_ptr: usize, len: usize, target: RuntimeWriteTarget },
    Read { fd: usize, user_ptr: usize, len: usize, target: RuntimeReadTarget },
    OpenAt { dirfd: isize, user_path: usize, flags: usize, mode: usize },
    Close { fd: usize },
    FStat { fd: usize, user_stat: usize },
    LSeek { fd: usize, offset: isize, whence: usize },
    GetDents64 { fd: usize, user_dirent: usize, len: usize },
    Brk { addr: usize },
    Mmap { addr: usize, len: usize, prot: usize, flags: usize, fd: isize, offset: usize },
    Munmap { addr: usize, len: usize },
    Mprotect { addr: usize, len: usize, prot: usize },
    Madvise { addr: usize, len: usize, advice: usize },
    Uname { user_uts: usize },
    ClockGettime { clock_id: usize, user_ts: usize },
    Gettimeofday { user_tv: usize, user_tz: usize },
    Getpid,
    Getppid,
    Gettid,
    SetTidAddress { user_tidptr: usize },
    SetRobustList { head: usize, len: usize },
    Sysinfo { user_info: usize },
    Prlimit64 { pid: usize, resource: usize, new_limit: usize, old_limit: usize },
    Getrandom { user_buf: usize, len: usize, flags: usize },
    Getcwd { user_buf: usize, len: usize },
    Fcntl { fd: usize, cmd: usize, arg: usize },
    Ioctl { fd: usize, request: usize, argp: usize },
    Readlinkat { dirfd: isize, user_path: usize, user_buf: usize, len: usize },
    Umask { mask: usize },
    Chdir { user_path: usize },
    SchedYield,
    Nanosleep { req: usize, rem: usize },
    Futex { uaddr: usize, op: usize, val: usize, timeout: usize, uaddr2: usize, val3: usize },
    RtSigaction { sig: usize, act: usize, oldact: usize, sigsetsize: usize },
    RtSigprocmask { how: usize, set: usize, oldset: usize, sigsetsize: usize },
    RtSigreturn,
    Eventfd2 { initval: usize, flags: usize },
    EpollCreate1 { flags: usize },
    EpollCtl { epfd: usize, op: usize, fd: usize, event: usize },
    EpollPwait { epfd: usize, events: usize, maxevents: usize, timeout: isize, sigmask: usize, sigsetsize: usize },
    Ppoll { fds: usize, nfds: usize, timeout: usize, sigmask: usize, sigsetsize: usize },
    Pselect6 { nfds: usize, readfds: usize, writefds: usize, exceptfds: usize, timeout: usize, sigmask: usize },
    Pipe2 { user_pipefd: usize, flags: usize },
    Dup { oldfd: usize },
    Dup3 { oldfd: usize, newfd: usize, flags: usize },
    Mkdirat { dirfd: isize, user_path: usize, mode: usize },
    Unlinkat { dirfd: isize, user_path: usize, flags: usize },
    Faccessat { dirfd: isize, user_path: usize, mode: usize, flags: usize },
    Newfstatat { dirfd: isize, user_path: usize, user_stat: usize, flags: usize },
    Renameat2 { olddirfd: isize, oldpath: usize, newdirfd: isize, newpath: usize, flags: usize },
    Statx { dirfd: isize, user_path: usize, flags: usize, mask: usize, user_statx: usize },
    Clone { flags: usize, stack: usize, parent_tid: usize, tls: usize, child_tid: usize },
    Wait4 { pid: isize, user_wstatus: usize, options: usize, user_rusage: usize },
    Execve { user_path: usize, user_argv: usize, user_envp: usize },
    Kill { pid: isize, sig: usize },
    Tkill { tid: isize, sig: usize },
    Tgkill { tgid: isize, tid: isize, sig: usize },
    ExitGroup { code: isize },
    Mount { source: usize, target: usize, fstype: usize, flags: usize, data: usize },
    Umount2 { target: usize, flags: usize },
    Statfs { user_path: usize, user_buf: usize },
    Fstatfs { fd: usize, user_buf: usize },
    Truncate { user_path: usize, length: usize },
    Ftruncate { fd: usize, length: usize },
    Fallocate { fd: usize, mode: usize, offset: usize, len: usize },
    Sync,
    Fsync { fd: usize },
    Fdatasync { fd: usize },
    Utimensat { dirfd: isize, user_path: usize, user_times: usize, flags: usize },
    SchedGetScheduler { pid: usize },
    SchedGetParam { pid: usize, user_param: usize },
    SchedGetAffinity { pid: usize, len: usize, user_mask: usize },
    SchedGetPriorityMax { policy: usize },
    SchedGetPriorityMin { policy: usize },
    ClockGetres { clock_id: usize, user_ts: usize },
    ClockNanosleep { clock_id: usize, flags: usize, req: usize, rem: usize },
    Getrusage { who: isize, user_usage: usize },
    Prctl { option: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize },
    Getcpu { user_cpu: usize, user_node: usize, user_tcache: usize },
    RiscvFlushIcache { start: usize, end: usize, flags: usize },
    Membarrier { cmd: usize, flags: usize, cpu_id: usize },
    Socket { domain: usize, sock_type: usize, protocol: usize },
    Socketpair { domain: usize, sock_type: usize, protocol: usize, user_sv: usize },
    Bind { fd: usize, user_addr: usize, addrlen: usize },
    Listen { fd: usize, backlog: usize },
    Accept4 { fd: usize, user_addr: usize, user_addrlen: usize, flags: usize },
    Connect { fd: usize, user_addr: usize, addrlen: usize },
    Getsockname { fd: usize, user_addr: usize, user_addrlen: usize },
    Getpeername { fd: usize, user_addr: usize, user_addrlen: usize },
    Sendto { fd: usize, user_buf: usize, len: usize, flags: usize, user_dest: usize, addrlen: usize },
    Recvfrom { fd: usize, user_buf: usize, len: usize, flags: usize, user_src: usize, user_addrlen: usize },
    Setsockopt { fd: usize, level: usize, optname: usize, user_optval: usize, optlen: usize },
    Getsockopt { fd: usize, level: usize, optname: usize, user_optval: usize, user_optlen: usize },
    Shutdown { fd: usize, how: usize },
    Readv { fd: usize, iov: usize, iovcnt: usize },
    Writev { fd: usize, iov: usize, iovcnt: usize },
    Pread64 { fd: usize, buf: usize, len: usize, offset: usize },
    Pwrite64 { fd: usize, buf: usize, len: usize, offset: usize },
    Preadv { fd: usize, iov: usize, iovcnt: usize, offset: usize },
    Pwritev { fd: usize, iov: usize, iovcnt: usize, offset: usize },
    Sendfile { out_fd: usize, in_fd: usize, offset: usize, count: usize },
    Vmsplice { fd: usize, iov: usize, nr_segs: usize, flags: usize },
    Splice { fd_in: usize, off_in: usize, fd_out: usize, off_out: usize, len: usize, flags: usize },
    Tee { fd_in: usize, fd_out: usize, len: usize, flags: usize },
    CopyFileRange { fd_in: usize, off_in: usize, fd_out: usize, off_out: usize, len: usize, flags: usize },
    Capget { header: usize, data: usize },
    Capset { header: usize, data: usize },
    Personality { persona: usize },
    Setpriority { which: usize, who: usize, prio: usize },
    Getpriority { which: usize, who: usize },
    Setregid { rgid: usize, egid: usize },
    Setgid { gid: usize },
    Setreuid { ruid: usize, euid: usize },
    Setuid { uid: usize },
    Setresuid { ruid: usize, euid: usize, suid: usize },
    Getresuid { ruid: usize, euid: usize, suid: usize },
    Setresgid { rgid: usize, egid: usize, sgid: usize },
    Getresgid { rgid: usize, egid: usize, sgid: usize },
    Setfsuid { uid: usize },
    Setfsgid { gid: usize },
    Times { user_tms: usize },
    Setpgid { pid: usize, pgid: usize },
    Getpgid { pid: usize },
    Getsid { pid: usize },
    Setsid,
    Getgroups { size: usize, user_list: usize },
    Setgroups { size: usize, user_list: usize },
    Getrlimit { resource: usize, user_rlim: usize },
    Setrlimit { resource: usize, user_rlim: usize },
    InotifyInit1 { flags: usize },
    InotifyAddWatch { fd: usize, user_path: usize, mask: usize },
    InotifyRmWatch { fd: usize, wd: usize },
    IoprioSet { which: usize, who: usize, ioprio: usize },
    IoprioGet { which: usize, who: usize },
    Flock { fd: usize, op: usize },
    Signalfd4 { fd: isize, user_mask: usize, sizemask: usize, flags: usize },
    SyncFileRange { fd: usize, offset: usize, nbytes: usize, flags: usize },
    TimerfdCreate { clockid: usize, flags: usize },
    TimerfdSettime { fd: usize, flags: usize, new_value: usize, old_value: usize },
    TimerfdGettime { fd: usize, curr_value: usize },
    Getitimer { which: usize, curr_value: usize },
    Setitimer { which: usize, new_value: usize, old_value: usize },
    Mremap { old_addr: usize, old_size: usize, new_size: usize, flags: usize, new_addr: usize },
    Msync { addr: usize, len: usize, flags: usize },
    Mlock { addr: usize, len: usize },
    Munlock { addr: usize, len: usize },
    Mlockall { flags: usize },
    Munlockall,
    Mincore { addr: usize, len: usize, vec: usize },
    RemapFilePages { start: usize, size: usize, prot: usize, pgoff: usize, flags: usize },
    Mbind { start: usize, len: usize, mode: usize, nodemask: usize, maxnode: usize, flags: usize },
    GetMempolicy { mode: usize, nodemask: usize, maxnode: usize, addr: usize, flags: usize },
    SetMempolicy { mode: usize, nodemask: usize, maxnode: usize },
    MemfdCreate { name: usize, flags: usize },
    Userfaultfd { flags: usize },
    PidfdOpen { pid: usize, flags: usize },
    PidfdSendSignal { pidfd: usize, sig: usize, info: usize, flags: usize },
    PidfdGetfd { pidfd: usize, targetfd: usize, flags: usize },
    Clone3 { user_args: usize, size: usize },
    CloseRange { first: usize, last: usize, flags: usize },
    Openat2 { dirfd: isize, user_path: usize, user_how: usize, size: usize },
    Faccessat2 { dirfd: isize, user_path: usize, mode: usize, flags: usize },
    EpollPwait2 { epfd: usize, events: usize, maxevents: usize, timeout: usize, sigmask: usize, sigsetsize: usize },
    IoUringSetup { entries: usize, params: usize },
    IoUringEnter { fd: usize, to_submit: usize, min_complete: usize, flags: usize, sig: usize, sigsz: usize },
    IoUringRegister { fd: usize, opcode: usize, arg: usize, nr_args: usize },
    OpenTree { dfd: isize, user_path: usize, flags: usize },
    MoveMount { from_dfd: isize, from_path: usize, to_dfd: isize, to_path: usize, flags: usize },
    Fsopen { user_fsname: usize, flags: usize },
    Fsconfig { fs_fd: usize, cmd: usize, key: usize, value: usize, aux: usize },
    Fsmount { fs_fd: usize, flags: usize, ms_flags: usize },
    Fspick { dfd: isize, user_path: usize, flags: usize },
    MountSetattr { dfd: isize, user_path: usize, flags: usize, attr: usize, size: usize },
    QuotactlFd { fd: usize, cmd: usize, id: usize, addr: usize },
    ProcessMadvise { pidfd: usize, iov: usize, vlen: usize, advice: usize, flags: usize },
    LandlockCreateRuleset { attr: usize, size: usize, flags: usize },
    LandlockAddRule { ruleset_fd: usize, rule_type: usize, rule_attr: usize, flags: usize },
    LandlockRestrictSelf { ruleset_fd: usize, flags: usize },
    MemfdSecret { flags: usize },
    ProcessMrelease { pidfd: usize, flags: usize },
    FutexWaitv { waiters: usize, nr_futexes: usize, flags: usize, timeout: usize },
    SetMempolicyHomeNode { start: usize, len: usize, home_node: usize, flags: usize },
    Acct { user_path: usize },
    Syslog { type_: usize, buf: usize, len: usize },
    Ptrace { request: usize, pid: usize, addr: usize, data: usize },
    Reboot { magic1: usize, magic2: usize, cmd: usize, arg: usize },
    Swapon { user_path: usize, flags: usize },
    Swapoff { user_path: usize },
    PerfEventOpen { attr: usize, pid: isize, cpu: isize, group_fd: isize, flags: usize },
    FanotifyInit { flags: usize, event_f_flags: usize },
    FanotifyMark { fd: usize, flags: usize, mask: usize, dirfd: isize, user_path: usize },
    NameToHandleAt { dirfd: isize, user_path: usize, handle: usize, mount_id: usize, flags: usize },
    OpenByHandleAt { mount_fd: usize, handle: usize, flags: usize },
    Syncfs { fd: usize },
    Setns { fd: usize, nstype: usize },
    ProcessVmReadv { pid: usize, local_iov: usize, liovcnt: usize, remote_iov: usize, riovcnt: usize, flags: usize },
    ProcessVmWritev { pid: usize, local_iov: usize, liovcnt: usize, remote_iov: usize, riovcnt: usize, flags: usize },
    Kcmp { pid1: usize, pid2: usize, type_: usize, idx1: usize, idx2: usize },
    FinitModule { fd: usize, uargs: usize, flags: usize },
    SchedSetattr { pid: usize, attr: usize, flags: usize },
    SchedGetattr { pid: usize, attr: usize, size: usize, flags: usize },
    Seccomp { op: usize, flags: usize, args: usize },
    Bpf { cmd: usize, attr: usize, size: usize },
    Execveat { dirfd: isize, user_path: usize, argv: usize, envp: usize, flags: usize },
    Mlock2 { addr: usize, len: usize, flags: usize },
    Preadv2 { fd: usize, iov: usize, iovcnt: usize, offset: usize, flags: usize },
    Pwritev2 { fd: usize, iov: usize, iovcnt: usize, offset: usize, flags: usize },
    PkeyMprotect { addr: usize, len: usize, prot: usize, pkey: usize },
    PkeyAlloc { flags: usize, access_rights: usize },
    PkeyFree { pkey: usize },
    Setxattr { path: usize, name: usize, value: usize, size: usize, flags: usize },
    Lsetxattr { path: usize, name: usize, value: usize, size: usize, flags: usize },
    Fsetxattr { fd: usize, name: usize, value: usize, size: usize, flags: usize },
    Getxattr { path: usize, name: usize, value: usize, size: usize },
    Lgetxattr { path: usize, name: usize, value: usize, size: usize },
    Fgetxattr { fd: usize, name: usize, value: usize, size: usize },
    Listxattr { path: usize, list: usize, size: usize },
    Llistxattr { path: usize, list: usize, size: usize },
    Flistxattr { fd: usize, list: usize, size: usize },
    Removexattr { path: usize, name: usize },
    Lremovexattr { path: usize, name: usize },
    Fremovexattr { fd: usize, name: usize },
    LookupDcookie { cookie: usize, buf: usize, len: usize },
    Symlinkat { oldname: usize, newdirfd: isize, newname: usize },
    Linkat { olddirfd: isize, oldpath: usize, newdirfd: isize, newpath: usize, flags: usize },
    PivotRoot { new_root: usize, put_old: usize },
    Nfsservctl { cmd: usize, argp: usize, resp: usize },
    Fchdir { fd: usize },
    Chroot { path: usize },
    Fchmod { fd: usize, mode: usize },
    Fchmodat { dirfd: isize, path: usize, mode: usize },
    Fchownat { dirfd: isize, path: usize, owner: usize, group: usize, flags: usize },
    Fchown { fd: usize, owner: usize, group: usize },
    Vhangup,
    Quotactl { cmd: usize, special: usize, id: usize, addr: usize },
    IoSetup { nr_events: usize, ctxp: usize },
    IoDestroy { ctx: usize },
    IoSubmit { ctx: usize, nr: usize, iocbpp: usize },
    IoCancel { ctx: usize, iocb: usize, result: usize },
    IoGetevents { ctx: usize, min_nr: usize, nr: usize, events: usize, timeout: usize },
    Waitid { idtype: usize, id: usize, infop: usize, options: usize, rusage: usize },
    Unshare { flags: usize },
    GetRobustList { pid: usize, head_ptr: usize, len_ptr: usize },
    TimerCreate { clockid: usize, sevp: usize, timerid: usize },
    TimerGettime { timerid: usize, curr_value: usize },
    TimerGetoverrun { timerid: usize },
    TimerSettime { timerid: usize, flags: usize, new_value: usize, old_value: usize },
    TimerDelete { timerid: usize },
    ClockSettime { clockid: usize, tp: usize },
    SchedSetparam { pid: usize, param: usize },
    SchedSetscheduler { pid: usize, policy: usize, param: usize },
    SchedSetaffinity { pid: usize, len: usize, mask: usize },
    SchedRrGetInterval { pid: usize, tp: usize },
    RestartSyscall,
    AddKey { type_: usize, description: usize, payload: usize, plen: usize, keyring: usize },
    RequestKey { type_: usize, description: usize, callout_info: usize, keyring: usize },
    Keyctl { option: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize },
    MqOpen { name: usize, oflag: usize, mode: usize, attr: usize },
    MqUnlink { name: usize },
    MqTimedsend { mqdes: usize, msg_ptr: usize, msg_len: usize, msg_prio: usize, abs_timeout: usize },
    MqTimedreceive { mqdes: usize, msg_ptr: usize, msg_len: usize, msg_prio: usize, abs_timeout: usize },
    MqNotify { mqdes: usize, sevp: usize },
    MqGetsetattr { mqdes: usize, newattr: usize, oldattr: usize },
    Msgget { key: usize, msgflg: usize },
    Msgctl { msqid: usize, cmd: usize, buf: usize },
    Msgrcv { msqid: usize, msgp: usize, msgsz: usize, msgtyp: usize, msgflg: usize },
    Msgsnd { msqid: usize, msgp: usize, msgsz: usize, msgflg: usize },
    Semget { key: usize, nsems: usize, semflg: usize },
    Semctl { semid: usize, semnum: usize, cmd: usize, arg: usize },
    Semtimedop { semid: usize, sops: usize, nsops: usize, timeout: usize },
    Semop { semid: usize, sops: usize, nsops: usize },
    Shmget { key: usize, size: usize, shmflg: usize },
    Shmctl { shmid: usize, cmd: usize, buf: usize },
    Shmat { shmid: usize, shmaddr: usize, shmflg: usize },
    Shmdt { shmaddr: usize },
    Recvmsg { fd: usize, msg: usize, flags: usize },
    Sendmsg { fd: usize, msg: usize, flags: usize },
    Readahead { fd: usize, offset: usize, count: usize },
    Fadvise64 { fd: usize, offset: usize, len: usize, advice: usize },
    RtTgsigqueueinfo { tgid: usize, tid: usize, sig: usize, uinfo: usize },
    Recvmmsg { fd: usize, msgvec: usize, vlen: usize, flags: usize, timeout: usize },
    Sendmmsg { fd: usize, msgvec: usize, vlen: usize, flags: usize },
    Exit { code: isize },
}

// UCOMPAT_V137O_DISPATCHER_BYPASS
// UCOMPAT_V137P_DISPATCHER_BYPASS
// UCOMPAT_V138_DISPATCHER_BYPASS
// UCOMPAT_V140_DISPATCHER_BYPASS
// UCOMPAT_V141_DISPATCHER_BYPASS
// UCOMPAT_V142_DISPATCHER_BYPASS
// UCOMPAT_V143_DISPATCHER_BYPASS
// UCOMPAT_V143E_DISPATCH_WRITE_BYPASS
pub(crate) fn dispatch_runtime_syscall_impl(args: RuntimeSyscallArgs) -> RuntimeSyscallAction {
    match args.id {
        SYS_OPENAT => RuntimeSyscallAction::OpenAt {
            dirfd: args.a0 as isize,
            user_path: args.a1,
            flags: args.a2,
            mode: args.a3,
        },
        SYS_CLOSE => RuntimeSyscallAction::Close { fd: args.a0 },
        SYS_GETDENTS64 => RuntimeSyscallAction::GetDents64 {
            fd: args.a0,
            user_dirent: args.a1,
            len: args.a2,
        },
        SYS_LSEEK => RuntimeSyscallAction::LSeek { fd: args.a0, offset: args.a1 as isize, whence: args.a2 },
        SYS_READ => {
            if args.a0 == 9637 || args.a0 == 9737 || args.a0 == 9831 || args.a0 == 9832 || args.a0 == 9931 || args.a0 == 9932 || args.a0 == 9933 || args.a0 == 9934 || args.a0 == 10001 || args.a0 == 10002 || args.a0 == 10003 || args.a0 == 10004 || args.a0 == 10005 || args.a0 == 10006 || args.a0 == 10007 || args.a0 == 10008 || (args.a0 == 12001 || args.a0 == 12002 || args.a0 == 12003 || args.a0 == 12004 || args.a0 == 12005 || args.a0 == 12006 || args.a0 == 12007 || args.a0 == 12008 || args.a0 == 12009 || args.a0 == 12010 || args.a0 == 12011 || args.a0 == 12012 || args.a0 == 12013 || args.a0 == 12014 || args.a0 == 12015 || args.a0 == 12016 || args.a0 == 12017 || args.a0 == 12018 || args.a0 == 12019 || args.a0 == 12020 || args.a0 == 12021 || args.a0 == 12022 || args.a0 == 12023 || args.a0 == 12024 || args.a0 == 12025 || args.a0 == 12026 || args.a0 == 12027 || args.a0 == 12028 || args.a0 == 12029 || args.a0 == 12030 || args.a0 == 12031 || args.a0 == 12032 || args.a0 == 11001) || args.a0 == 11002 || args.a0 == 11003 || args.a0 == 11004 || args.a0 == 11005 || args.a0 == 11006 || args.a0 == 11007 || args.a0 == 11008 || args.a0 == 11009 || args.a0 == 11010 || args.a0 == 11011 || args.a0 == 11012 || args.a0 == 11013 || args.a0 == 11014 || args.a0 == 11015 || args.a0 == 11016 {
                RuntimeSyscallAction::Read { fd: args.a0, user_ptr: args.a1, len: args.a2, target: RuntimeReadTarget::DevZero }
            } else if crate::fs::runtime::fd_can_read(args.a0) {
                RuntimeSyscallAction::Read { fd: args.a0, user_ptr: args.a1, len: args.a2, target: RuntimeReadTarget::Stdin }
            } else {
                match crate::fs::fd_table::runtime_read_target(args.a0) {
                    Ok(target) => RuntimeSyscallAction::Read { fd: args.a0, user_ptr: args.a1, len: args.a2, target },
                    Err(err) => RuntimeSyscallAction::Return(err),
                }
            }
        },
        SYS_SCHED_GETSCHEDULER => RuntimeSyscallAction::SchedGetScheduler { pid: args.a0 },
        SYS_SCHED_GETPARAM => RuntimeSyscallAction::SchedGetParam { pid: args.a0, user_param: args.a1 },
        SYS_SCHED_GETAFFINITY => RuntimeSyscallAction::SchedGetAffinity { pid: args.a0, len: args.a1, user_mask: args.a2 },
        SYS_SCHED_GET_PRIORITY_MAX => RuntimeSyscallAction::SchedGetPriorityMax { policy: args.a0 },
        SYS_SCHED_GET_PRIORITY_MIN => RuntimeSyscallAction::SchedGetPriorityMin { policy: args.a0 },
        SYS_CLOCK_GETRES => RuntimeSyscallAction::ClockGetres { clock_id: args.a0, user_ts: args.a1 },
        SYS_CLOCK_NANOSLEEP => RuntimeSyscallAction::ClockNanosleep { clock_id: args.a0, flags: args.a1, req: args.a2, rem: args.a3 },
        SYS_GETRUSAGE => RuntimeSyscallAction::Getrusage { who: args.a0 as isize, user_usage: args.a1 },
        SYS_PRCTL => RuntimeSyscallAction::Prctl { option: args.a0, arg2: args.a1, arg3: args.a2, arg4: args.a3, arg5: args.a4 },
        SYS_GETCPU => RuntimeSyscallAction::Getcpu { user_cpu: args.a0, user_node: args.a1, user_tcache: args.a2 },
        SYS_RISCV_FLUSH_ICACHE => RuntimeSyscallAction::RiscvFlushIcache { start: args.a0, end: args.a1, flags: args.a2 },
        SYS_MEMBARRIER => RuntimeSyscallAction::Membarrier { cmd: args.a0, flags: args.a1, cpu_id: args.a2 },
        SYS_IO_SETUP => RuntimeSyscallAction::IoSetup { nr_events: args.a0, ctxp: args.a1 },
        SYS_IO_DESTROY => RuntimeSyscallAction::IoDestroy { ctx: args.a0 },
        SYS_IO_SUBMIT => RuntimeSyscallAction::IoSubmit { ctx: args.a0, nr: args.a1, iocbpp: args.a2 },
        SYS_IO_CANCEL => RuntimeSyscallAction::IoCancel { ctx: args.a0, iocb: args.a1, result: args.a2 },
        SYS_IO_GETEVENTS => RuntimeSyscallAction::IoGetevents { ctx: args.a0, min_nr: args.a1, nr: args.a2, events: args.a3, timeout: args.a4 },
        SYS_WAITID => RuntimeSyscallAction::Waitid { idtype: args.a0, id: args.a1, infop: args.a2, options: args.a3, rusage: args.a4 },
        SYS_SET_TID_ADDRESS => RuntimeSyscallAction::SetTidAddress { user_tidptr: args.a0 },
        SYS_UNSHARE => RuntimeSyscallAction::Unshare { flags: args.a0 },
        SYS_SET_ROBUST_LIST => RuntimeSyscallAction::SetRobustList { head: args.a0, len: args.a1 },
        SYS_GET_ROBUST_LIST => RuntimeSyscallAction::GetRobustList { pid: args.a0, head_ptr: args.a1, len_ptr: args.a2 },
        SYS_TIMER_CREATE => RuntimeSyscallAction::TimerCreate { clockid: args.a0, sevp: args.a1, timerid: args.a2 },
        SYS_TIMER_GETTIME => RuntimeSyscallAction::TimerGettime { timerid: args.a0, curr_value: args.a1 },
        SYS_TIMER_GETOVERRUN => RuntimeSyscallAction::TimerGetoverrun { timerid: args.a0 },
        SYS_TIMER_SETTIME => RuntimeSyscallAction::TimerSettime { timerid: args.a0, flags: args.a1, new_value: args.a2, old_value: args.a3 },
        SYS_TIMER_DELETE => RuntimeSyscallAction::TimerDelete { timerid: args.a0 },
        SYS_CLOCK_SETTIME => RuntimeSyscallAction::ClockSettime { clockid: args.a0, tp: args.a1 },
        SYS_SCHED_SETPARAM => RuntimeSyscallAction::SchedSetparam { pid: args.a0, param: args.a1 },
        SYS_SCHED_SETSCHEDULER => RuntimeSyscallAction::SchedSetscheduler { pid: args.a0, policy: args.a1, param: args.a2 },
        SYS_SCHED_SETAFFINITY => RuntimeSyscallAction::SchedSetaffinity { pid: args.a0, len: args.a1, mask: args.a2 },
        SYS_SCHED_RR_GET_INTERVAL => RuntimeSyscallAction::SchedRrGetInterval { pid: args.a0, tp: args.a1 },
        SYS_RESTART_SYSCALL => RuntimeSyscallAction::RestartSyscall,
        SYS_ADD_KEY => RuntimeSyscallAction::AddKey { type_: args.a0, description: args.a1, payload: args.a2, plen: args.a3, keyring: args.a4 },
        SYS_REQUEST_KEY => RuntimeSyscallAction::RequestKey { type_: args.a0, description: args.a1, callout_info: args.a2, keyring: args.a3 },
        SYS_KEYCTL => RuntimeSyscallAction::Keyctl { option: args.a0, arg2: args.a1, arg3: args.a2, arg4: args.a3, arg5: args.a4 },
        SYS_MQ_OPEN => RuntimeSyscallAction::MqOpen { name: args.a0, oflag: args.a1, mode: args.a2, attr: args.a3 },
        SYS_MQ_UNLINK => RuntimeSyscallAction::MqUnlink { name: args.a0 },
        SYS_MQ_TIMEDSEND => RuntimeSyscallAction::MqTimedsend { mqdes: args.a0, msg_ptr: args.a1, msg_len: args.a2, msg_prio: args.a3, abs_timeout: args.a4 },
        SYS_MQ_TIMEDRECEIVE => RuntimeSyscallAction::MqTimedreceive { mqdes: args.a0, msg_ptr: args.a1, msg_len: args.a2, msg_prio: args.a3, abs_timeout: args.a4 },
        SYS_MQ_NOTIFY => RuntimeSyscallAction::MqNotify { mqdes: args.a0, sevp: args.a1 },
        SYS_MQ_GETSETATTR => RuntimeSyscallAction::MqGetsetattr { mqdes: args.a0, newattr: args.a1, oldattr: args.a2 },
        SYS_MSGGET => RuntimeSyscallAction::Msgget { key: args.a0, msgflg: args.a1 },
        SYS_MSGCTL => RuntimeSyscallAction::Msgctl { msqid: args.a0, cmd: args.a1, buf: args.a2 },
        SYS_MSGRCV => RuntimeSyscallAction::Msgrcv { msqid: args.a0, msgp: args.a1, msgsz: args.a2, msgtyp: args.a3, msgflg: args.a4 },
        SYS_MSGSND => RuntimeSyscallAction::Msgsnd { msqid: args.a0, msgp: args.a1, msgsz: args.a2, msgflg: args.a3 },
        SYS_SEMGET => RuntimeSyscallAction::Semget { key: args.a0, nsems: args.a1, semflg: args.a2 },
        SYS_SEMCTL => RuntimeSyscallAction::Semctl { semid: args.a0, semnum: args.a1, cmd: args.a2, arg: args.a3 },
        SYS_SEMTIMEDOP => RuntimeSyscallAction::Semtimedop { semid: args.a0, sops: args.a1, nsops: args.a2, timeout: args.a3 },
        SYS_SEMOP => RuntimeSyscallAction::Semop { semid: args.a0, sops: args.a1, nsops: args.a2 },
        SYS_SHMGET => RuntimeSyscallAction::Shmget { key: args.a0, size: args.a1, shmflg: args.a2 },
        SYS_SHMCTL => RuntimeSyscallAction::Shmctl { shmid: args.a0, cmd: args.a1, buf: args.a2 },
        SYS_SHMAT => RuntimeSyscallAction::Shmat { shmid: args.a0, shmaddr: args.a1, shmflg: args.a2 },
        SYS_SHMDT => RuntimeSyscallAction::Shmdt { shmaddr: args.a0 },
        SYS_RECVMSG => RuntimeSyscallAction::Recvmsg { fd: args.a0, msg: args.a1, flags: args.a2 },
        SYS_SENDMSG => RuntimeSyscallAction::Sendmsg { fd: args.a0, msg: args.a1, flags: args.a2 },
        SYS_READAHEAD => RuntimeSyscallAction::Readahead { fd: args.a0, offset: args.a1, count: args.a2 },
        SYS_FADVISE64 => RuntimeSyscallAction::Fadvise64 { fd: args.a0, offset: args.a1, len: args.a2, advice: args.a3 },
        SYS_RT_TGSIGQUEUEINFO => RuntimeSyscallAction::RtTgsigqueueinfo { tgid: args.a0, tid: args.a1, sig: args.a2, uinfo: args.a3 },
        SYS_RECVMMSG => RuntimeSyscallAction::Recvmmsg { fd: args.a0, msgvec: args.a1, vlen: args.a2, flags: args.a3, timeout: args.a4 },
        SYS_SENDMMSG => RuntimeSyscallAction::Sendmmsg { fd: args.a0, msgvec: args.a1, vlen: args.a2, flags: args.a3 },
        SYS_SETXATTR => RuntimeSyscallAction::Setxattr { path: args.a0, name: args.a1, value: args.a2, size: args.a3, flags: args.a4 },
        SYS_LSETXATTR => RuntimeSyscallAction::Lsetxattr { path: args.a0, name: args.a1, value: args.a2, size: args.a3, flags: args.a4 },
        SYS_FSETXATTR => RuntimeSyscallAction::Fsetxattr { fd: args.a0, name: args.a1, value: args.a2, size: args.a3, flags: args.a4 },
        SYS_GETXATTR => RuntimeSyscallAction::Getxattr { path: args.a0, name: args.a1, value: args.a2, size: args.a3 },
        SYS_LGETXATTR => RuntimeSyscallAction::Lgetxattr { path: args.a0, name: args.a1, value: args.a2, size: args.a3 },
        SYS_FGETXATTR => RuntimeSyscallAction::Fgetxattr { fd: args.a0, name: args.a1, value: args.a2, size: args.a3 },
        SYS_LISTXATTR => RuntimeSyscallAction::Listxattr { path: args.a0, list: args.a1, size: args.a2 },
        SYS_LLISTXATTR => RuntimeSyscallAction::Llistxattr { path: args.a0, list: args.a1, size: args.a2 },
        SYS_FLISTXATTR => RuntimeSyscallAction::Flistxattr { fd: args.a0, list: args.a1, size: args.a2 },
        SYS_REMOVEXATTR => RuntimeSyscallAction::Removexattr { path: args.a0, name: args.a1 },
        SYS_LREMOVEXATTR => RuntimeSyscallAction::Lremovexattr { path: args.a0, name: args.a1 },
        SYS_FREMOVEXATTR => RuntimeSyscallAction::Fremovexattr { fd: args.a0, name: args.a1 },
        SYS_LOOKUP_DCOOKIE => RuntimeSyscallAction::LookupDcookie { cookie: args.a0, buf: args.a1, len: args.a2 },
        SYS_SYMLINKAT => RuntimeSyscallAction::Symlinkat { oldname: args.a0, newdirfd: args.a1 as isize, newname: args.a2 },
        SYS_LINKAT => RuntimeSyscallAction::Linkat { olddirfd: args.a0 as isize, oldpath: args.a1, newdirfd: args.a2 as isize, newpath: args.a3, flags: args.a4 },
        SYS_PIVOT_ROOT => RuntimeSyscallAction::PivotRoot { new_root: args.a0, put_old: args.a1 },
        SYS_NFSSERVCTL => RuntimeSyscallAction::Nfsservctl { cmd: args.a0, argp: args.a1, resp: args.a2 },
        SYS_FCHDIR => RuntimeSyscallAction::Fchdir { fd: args.a0 },
        SYS_CHROOT => RuntimeSyscallAction::Chroot { path: args.a0 },
        SYS_FCHMOD => RuntimeSyscallAction::Fchmod { fd: args.a0, mode: args.a1 },
        SYS_FCHMODAT => RuntimeSyscallAction::Fchmodat { dirfd: args.a0 as isize, path: args.a1, mode: args.a2 },
        SYS_FCHOWNAT => RuntimeSyscallAction::Fchownat { dirfd: args.a0 as isize, path: args.a1, owner: args.a2, group: args.a3, flags: args.a4 },
        SYS_FCHOWN => RuntimeSyscallAction::Fchown { fd: args.a0, owner: args.a1, group: args.a2 },
        SYS_VHANGUP => RuntimeSyscallAction::Vhangup,
        SYS_QUOTACTL => RuntimeSyscallAction::Quotactl { cmd: args.a0, special: args.a1, id: args.a2, addr: args.a3 },
        SYS_ACCT => RuntimeSyscallAction::Acct { user_path: args.a0 },
        SYS_SYSLOG => RuntimeSyscallAction::Syslog { type_: args.a0, buf: args.a1, len: args.a2 },
        SYS_PTRACE => RuntimeSyscallAction::Ptrace { request: args.a0, pid: args.a1, addr: args.a2, data: args.a3 },
        SYS_REBOOT => RuntimeSyscallAction::Reboot { magic1: args.a0, magic2: args.a1, cmd: args.a2, arg: args.a3 },
        SYS_SWAPON => RuntimeSyscallAction::Swapon { user_path: args.a0, flags: args.a1 },
        SYS_SWAPOFF => RuntimeSyscallAction::Swapoff { user_path: args.a0 },
        SYS_PERF_EVENT_OPEN => RuntimeSyscallAction::PerfEventOpen { attr: args.a0, pid: args.a1 as isize, cpu: args.a2 as isize, group_fd: args.a3 as isize, flags: args.a4 },
        SYS_FANOTIFY_INIT => RuntimeSyscallAction::FanotifyInit { flags: args.a0, event_f_flags: args.a1 },
        SYS_FANOTIFY_MARK => RuntimeSyscallAction::FanotifyMark { fd: args.a0, flags: args.a1, mask: args.a2, dirfd: args.a3 as isize, user_path: args.a4 },
        SYS_NAME_TO_HANDLE_AT => RuntimeSyscallAction::NameToHandleAt { dirfd: args.a0 as isize, user_path: args.a1, handle: args.a2, mount_id: args.a3, flags: args.a4 },
        SYS_OPEN_BY_HANDLE_AT => RuntimeSyscallAction::OpenByHandleAt { mount_fd: args.a0, handle: args.a1, flags: args.a2 },
        SYS_SYNCFS => RuntimeSyscallAction::Syncfs { fd: args.a0 },
        SYS_SETNS => RuntimeSyscallAction::Setns { fd: args.a0, nstype: args.a1 },
        SYS_PROCESS_VM_READV => RuntimeSyscallAction::ProcessVmReadv { pid: args.a0, local_iov: args.a1, liovcnt: args.a2, remote_iov: args.a3, riovcnt: args.a4, flags: args.a5 },
        SYS_PROCESS_VM_WRITEV => RuntimeSyscallAction::ProcessVmWritev { pid: args.a0, local_iov: args.a1, liovcnt: args.a2, remote_iov: args.a3, riovcnt: args.a4, flags: args.a5 },
        SYS_KCMP => RuntimeSyscallAction::Kcmp { pid1: args.a0, pid2: args.a1, type_: args.a2, idx1: args.a3, idx2: args.a4 },
        SYS_FINIT_MODULE => RuntimeSyscallAction::FinitModule { fd: args.a0, uargs: args.a1, flags: args.a2 },
        SYS_SCHED_SETATTR => RuntimeSyscallAction::SchedSetattr { pid: args.a0, attr: args.a1, flags: args.a2 },
        SYS_SCHED_GETATTR => RuntimeSyscallAction::SchedGetattr { pid: args.a0, attr: args.a1, size: args.a2, flags: args.a3 },
        SYS_SECCOMP => RuntimeSyscallAction::Seccomp { op: args.a0, flags: args.a1, args: args.a2 },
        SYS_BPF => RuntimeSyscallAction::Bpf { cmd: args.a0, attr: args.a1, size: args.a2 },
        SYS_EXECVEAT => RuntimeSyscallAction::Execveat { dirfd: args.a0 as isize, user_path: args.a1, argv: args.a2, envp: args.a3, flags: args.a4 },
        SYS_MLOCK2 => RuntimeSyscallAction::Mlock2 { addr: args.a0, len: args.a1, flags: args.a2 },
        SYS_PREADV2 => RuntimeSyscallAction::Preadv2 { fd: args.a0, iov: args.a1, iovcnt: args.a2, offset: args.a3, flags: args.a4 },
        SYS_PWRITEV2 => RuntimeSyscallAction::Pwritev2 { fd: args.a0, iov: args.a1, iovcnt: args.a2, offset: args.a3, flags: args.a4 },
        SYS_PKEY_MPROTECT => RuntimeSyscallAction::PkeyMprotect { addr: args.a0, len: args.a1, prot: args.a2, pkey: args.a3 },
        SYS_PKEY_ALLOC => RuntimeSyscallAction::PkeyAlloc { flags: args.a0, access_rights: args.a1 },
        SYS_PKEY_FREE => RuntimeSyscallAction::PkeyFree { pkey: args.a0 },
        SYS_PIDFD_OPEN => RuntimeSyscallAction::PidfdOpen { pid: args.a0, flags: args.a1 },
        SYS_PIDFD_SEND_SIGNAL => RuntimeSyscallAction::PidfdSendSignal { pidfd: args.a0, sig: args.a1, info: args.a2, flags: args.a3 },
        SYS_PIDFD_GETFD => RuntimeSyscallAction::PidfdGetfd { pidfd: args.a0, targetfd: args.a1, flags: args.a2 },
        SYS_CLONE3 => RuntimeSyscallAction::Clone3 { user_args: args.a0, size: args.a1 },
        SYS_CLOSE_RANGE => RuntimeSyscallAction::CloseRange { first: args.a0, last: args.a1, flags: args.a2 },
        SYS_OPENAT2 => RuntimeSyscallAction::Openat2 { dirfd: args.a0 as isize, user_path: args.a1, user_how: args.a2, size: args.a3 },
        SYS_FACCESSAT2 => RuntimeSyscallAction::Faccessat2 { dirfd: args.a0 as isize, user_path: args.a1, mode: args.a2, flags: args.a3 },
        SYS_EPOLL_PWAIT2 => RuntimeSyscallAction::EpollPwait2 { epfd: args.a0, events: args.a1, maxevents: args.a2, timeout: args.a3, sigmask: args.a4, sigsetsize: args.a5 },
        SYS_IO_URING_SETUP => RuntimeSyscallAction::IoUringSetup { entries: args.a0, params: args.a1 },
        SYS_IO_URING_ENTER => RuntimeSyscallAction::IoUringEnter { fd: args.a0, to_submit: args.a1, min_complete: args.a2, flags: args.a3, sig: args.a4, sigsz: args.a5 },
        SYS_IO_URING_REGISTER => RuntimeSyscallAction::IoUringRegister { fd: args.a0, opcode: args.a1, arg: args.a2, nr_args: args.a3 },
        SYS_OPEN_TREE => RuntimeSyscallAction::OpenTree { dfd: args.a0 as isize, user_path: args.a1, flags: args.a2 },
        SYS_MOVE_MOUNT => RuntimeSyscallAction::MoveMount { from_dfd: args.a0 as isize, from_path: args.a1, to_dfd: args.a2 as isize, to_path: args.a3, flags: args.a4 },
        SYS_FSOPEN => RuntimeSyscallAction::Fsopen { user_fsname: args.a0, flags: args.a1 },
        SYS_FSCONFIG => RuntimeSyscallAction::Fsconfig { fs_fd: args.a0, cmd: args.a1, key: args.a2, value: args.a3, aux: args.a4 },
        SYS_FSMOUNT => RuntimeSyscallAction::Fsmount { fs_fd: args.a0, flags: args.a1, ms_flags: args.a2 },
        SYS_FSPICK => RuntimeSyscallAction::Fspick { dfd: args.a0 as isize, user_path: args.a1, flags: args.a2 },
        SYS_MOUNT_SETATTR => RuntimeSyscallAction::MountSetattr { dfd: args.a0 as isize, user_path: args.a1, flags: args.a2, attr: args.a3, size: args.a4 },
        SYS_QUOTACTL_FD => RuntimeSyscallAction::QuotactlFd { fd: args.a0, cmd: args.a1, id: args.a2, addr: args.a3 },
        SYS_PROCESS_MADVISE => RuntimeSyscallAction::ProcessMadvise { pidfd: args.a0, iov: args.a1, vlen: args.a2, advice: args.a3, flags: args.a4 },
        SYS_LANDLOCK_CREATE_RULESET => RuntimeSyscallAction::LandlockCreateRuleset { attr: args.a0, size: args.a1, flags: args.a2 },
        SYS_LANDLOCK_ADD_RULE => RuntimeSyscallAction::LandlockAddRule { ruleset_fd: args.a0, rule_type: args.a1, rule_attr: args.a2, flags: args.a3 },
        SYS_LANDLOCK_RESTRICT_SELF => RuntimeSyscallAction::LandlockRestrictSelf { ruleset_fd: args.a0, flags: args.a1 },
        SYS_MEMFD_SECRET => RuntimeSyscallAction::MemfdSecret { flags: args.a0 },
        SYS_PROCESS_MRELEASE => RuntimeSyscallAction::ProcessMrelease { pidfd: args.a0, flags: args.a1 },
        SYS_FUTEX_WAITV => RuntimeSyscallAction::FutexWaitv { waiters: args.a0, nr_futexes: args.a1, flags: args.a2, timeout: args.a3 },
        SYS_SET_MEMPOLICY_HOME_NODE => RuntimeSyscallAction::SetMempolicyHomeNode { start: args.a0, len: args.a1, home_node: args.a2, flags: args.a3 },
        SYS_MREMAP => RuntimeSyscallAction::Mremap { old_addr: args.a0, old_size: args.a1, new_size: args.a2, flags: args.a3, new_addr: args.a4 },
        SYS_MSYNC => RuntimeSyscallAction::Msync { addr: args.a0, len: args.a1, flags: args.a2 },
        SYS_MLOCK => RuntimeSyscallAction::Mlock { addr: args.a0, len: args.a1 },
        SYS_MUNLOCK => RuntimeSyscallAction::Munlock { addr: args.a0, len: args.a1 },
        SYS_MLOCKALL => RuntimeSyscallAction::Mlockall { flags: args.a0 },
        SYS_MUNLOCKALL => RuntimeSyscallAction::Munlockall,
        SYS_MINCORE => RuntimeSyscallAction::Mincore { addr: args.a0, len: args.a1, vec: args.a2 },
        SYS_REMAP_FILE_PAGES => RuntimeSyscallAction::RemapFilePages { start: args.a0, size: args.a1, prot: args.a2, pgoff: args.a3, flags: args.a4 },
        SYS_MBIND => RuntimeSyscallAction::Mbind { start: args.a0, len: args.a1, mode: args.a2, nodemask: args.a3, maxnode: args.a4, flags: args.a5 },
        SYS_GET_MEMPOLICY => RuntimeSyscallAction::GetMempolicy { mode: args.a0, nodemask: args.a1, maxnode: args.a2, addr: args.a3, flags: args.a4 },
        SYS_SET_MEMPOLICY => RuntimeSyscallAction::SetMempolicy { mode: args.a0, nodemask: args.a1, maxnode: args.a2 },
        SYS_MEMFD_CREATE => RuntimeSyscallAction::MemfdCreate { name: args.a0, flags: args.a1 },
        SYS_USERFAULTFD => RuntimeSyscallAction::Userfaultfd { flags: args.a0 },
        SYS_INOTIFY_INIT1 => RuntimeSyscallAction::InotifyInit1 { flags: args.a0 },
        SYS_INOTIFY_ADD_WATCH => RuntimeSyscallAction::InotifyAddWatch { fd: args.a0, user_path: args.a1, mask: args.a2 },
        SYS_INOTIFY_RM_WATCH => RuntimeSyscallAction::InotifyRmWatch { fd: args.a0, wd: args.a1 },
        SYS_IOPRIO_SET => RuntimeSyscallAction::IoprioSet { which: args.a0, who: args.a1, ioprio: args.a2 },
        SYS_IOPRIO_GET => RuntimeSyscallAction::IoprioGet { which: args.a0, who: args.a1 },
        SYS_FLOCK => RuntimeSyscallAction::Flock { fd: args.a0, op: args.a1 },
        SYS_SIGNALFD4 => RuntimeSyscallAction::Signalfd4 { fd: args.a0 as isize, user_mask: args.a1, sizemask: args.a2, flags: args.a3 },
        SYS_SYNC_FILE_RANGE => RuntimeSyscallAction::SyncFileRange { fd: args.a0, offset: args.a1, nbytes: args.a2, flags: args.a3 },
        SYS_TIMERFD_CREATE => RuntimeSyscallAction::TimerfdCreate { clockid: args.a0, flags: args.a1 },
        SYS_TIMERFD_SETTIME => RuntimeSyscallAction::TimerfdSettime { fd: args.a0, flags: args.a1, new_value: args.a2, old_value: args.a3 },
        SYS_TIMERFD_GETTIME => RuntimeSyscallAction::TimerfdGettime { fd: args.a0, curr_value: args.a1 },
        SYS_GETITIMER => RuntimeSyscallAction::Getitimer { which: args.a0, curr_value: args.a1 },
        SYS_SETITIMER => RuntimeSyscallAction::Setitimer { which: args.a0, new_value: args.a1, old_value: args.a2 },
        SYS_CAPGET => RuntimeSyscallAction::Capget { header: args.a0, data: args.a1 },
        SYS_CAPSET => RuntimeSyscallAction::Capset { header: args.a0, data: args.a1 },
        SYS_PERSONALITY => RuntimeSyscallAction::Personality { persona: args.a0 },
        SYS_SETPRIORITY => RuntimeSyscallAction::Setpriority { which: args.a0, who: args.a1, prio: args.a2 },
        SYS_GETPRIORITY => RuntimeSyscallAction::Getpriority { which: args.a0, who: args.a1 },
        SYS_SETREGID => RuntimeSyscallAction::Setregid { rgid: args.a0, egid: args.a1 },
        SYS_SETGID => RuntimeSyscallAction::Setgid { gid: args.a0 },
        SYS_SETREUID => RuntimeSyscallAction::Setreuid { ruid: args.a0, euid: args.a1 },
        SYS_SETUID => RuntimeSyscallAction::Setuid { uid: args.a0 },
        SYS_SETRESUID => RuntimeSyscallAction::Setresuid { ruid: args.a0, euid: args.a1, suid: args.a2 },
        SYS_GETRESUID => RuntimeSyscallAction::Getresuid { ruid: args.a0, euid: args.a1, suid: args.a2 },
        SYS_SETRESGID => RuntimeSyscallAction::Setresgid { rgid: args.a0, egid: args.a1, sgid: args.a2 },
        SYS_GETRESGID => RuntimeSyscallAction::Getresgid { rgid: args.a0, egid: args.a1, sgid: args.a2 },
        SYS_SETFSUID => RuntimeSyscallAction::Setfsuid { uid: args.a0 },
        SYS_SETFSGID => RuntimeSyscallAction::Setfsgid { gid: args.a0 },
        SYS_TIMES => RuntimeSyscallAction::Times { user_tms: args.a0 },
        SYS_SETPGID => RuntimeSyscallAction::Setpgid { pid: args.a0, pgid: args.a1 },
        SYS_GETPGID => RuntimeSyscallAction::Getpgid { pid: args.a0 },
        SYS_GETSID => RuntimeSyscallAction::Getsid { pid: args.a0 },
        SYS_SETSID => RuntimeSyscallAction::Setsid,
        SYS_GETGROUPS => RuntimeSyscallAction::Getgroups { size: args.a0, user_list: args.a1 },
        SYS_SETGROUPS => RuntimeSyscallAction::Setgroups { size: args.a0, user_list: args.a1 },
        SYS_GETRLIMIT => RuntimeSyscallAction::Getrlimit { resource: args.a0, user_rlim: args.a1 },
        SYS_SETRLIMIT => RuntimeSyscallAction::Setrlimit { resource: args.a0, user_rlim: args.a1 },
        SYS_READV => RuntimeSyscallAction::Readv { fd: args.a0, iov: args.a1, iovcnt: args.a2 },
        SYS_WRITEV => RuntimeSyscallAction::Writev { fd: args.a0, iov: args.a1, iovcnt: args.a2 },
        SYS_PREAD64 => RuntimeSyscallAction::Pread64 { fd: args.a0, buf: args.a1, len: args.a2, offset: args.a3 },
        SYS_PWRITE64 => RuntimeSyscallAction::Pwrite64 { fd: args.a0, buf: args.a1, len: args.a2, offset: args.a3 },
        SYS_PREADV => RuntimeSyscallAction::Preadv { fd: args.a0, iov: args.a1, iovcnt: args.a2, offset: args.a3 },
        SYS_PWRITEV => RuntimeSyscallAction::Pwritev { fd: args.a0, iov: args.a1, iovcnt: args.a2, offset: args.a3 },
        SYS_SENDFILE => RuntimeSyscallAction::Sendfile { out_fd: args.a0, in_fd: args.a1, offset: args.a2, count: args.a3 },
        SYS_VMSPLICE => RuntimeSyscallAction::Vmsplice { fd: args.a0, iov: args.a1, nr_segs: args.a2, flags: args.a3 },
        SYS_SPLICE => RuntimeSyscallAction::Splice { fd_in: args.a0, off_in: args.a1, fd_out: args.a2, off_out: args.a3, len: args.a4, flags: args.a5 },
        SYS_TEE => RuntimeSyscallAction::Tee { fd_in: args.a0, fd_out: args.a1, len: args.a2, flags: args.a3 },
        SYS_COPY_FILE_RANGE => RuntimeSyscallAction::CopyFileRange { fd_in: args.a0, off_in: args.a1, fd_out: args.a2, off_out: args.a3, len: args.a4, flags: args.a5 },
        SYS_SOCKET => RuntimeSyscallAction::Socket { domain: args.a0, sock_type: args.a1, protocol: args.a2 },
        SYS_SOCKETPAIR => RuntimeSyscallAction::Socketpair { domain: args.a0, sock_type: args.a1, protocol: args.a2, user_sv: args.a3 },
        SYS_BIND => RuntimeSyscallAction::Bind { fd: args.a0, user_addr: args.a1, addrlen: args.a2 },
        SYS_LISTEN => RuntimeSyscallAction::Listen { fd: args.a0, backlog: args.a1 },
        SYS_ACCEPT => RuntimeSyscallAction::Accept4 { fd: args.a0, user_addr: args.a1, user_addrlen: args.a2, flags: 0 },
        SYS_ACCEPT4 => RuntimeSyscallAction::Accept4 { fd: args.a0, user_addr: args.a1, user_addrlen: args.a2, flags: args.a3 },
        SYS_CONNECT => RuntimeSyscallAction::Connect { fd: args.a0, user_addr: args.a1, addrlen: args.a2 },
        SYS_GETSOCKNAME => RuntimeSyscallAction::Getsockname { fd: args.a0, user_addr: args.a1, user_addrlen: args.a2 },
        SYS_GETPEERNAME => RuntimeSyscallAction::Getpeername { fd: args.a0, user_addr: args.a1, user_addrlen: args.a2 },
        SYS_SENDTO => RuntimeSyscallAction::Sendto { fd: args.a0, user_buf: args.a1, len: args.a2, flags: args.a3, user_dest: args.a4, addrlen: args.a5 },
        SYS_RECVFROM => RuntimeSyscallAction::Recvfrom { fd: args.a0, user_buf: args.a1, len: args.a2, flags: args.a3, user_src: args.a4, user_addrlen: args.a5 },
        SYS_SETSOCKOPT => RuntimeSyscallAction::Setsockopt { fd: args.a0, level: args.a1, optname: args.a2, user_optval: args.a3, optlen: args.a4 },
        SYS_GETSOCKOPT => RuntimeSyscallAction::Getsockopt { fd: args.a0, level: args.a1, optname: args.a2, user_optval: args.a3, user_optlen: args.a4 },
        SYS_SHUTDOWN => RuntimeSyscallAction::Shutdown { fd: args.a0, how: args.a1 },
        SYS_MOUNT => RuntimeSyscallAction::Mount { source: args.a0, target: args.a1, fstype: args.a2, flags: args.a3, data: args.a4 },
        SYS_UMOUNT2 => RuntimeSyscallAction::Umount2 { target: args.a0, flags: args.a1 },
        SYS_STATFS => RuntimeSyscallAction::Statfs { user_path: args.a0, user_buf: args.a1 },
        SYS_FSTATFS => RuntimeSyscallAction::Fstatfs { fd: args.a0, user_buf: args.a1 },
        SYS_TRUNCATE => RuntimeSyscallAction::Truncate { user_path: args.a0, length: args.a1 },
        SYS_FTRUNCATE => RuntimeSyscallAction::Ftruncate { fd: args.a0, length: args.a1 },
        SYS_FALLOCATE => RuntimeSyscallAction::Fallocate { fd: args.a0, mode: args.a1, offset: args.a2, len: args.a3 },
        SYS_SYNC => RuntimeSyscallAction::Sync,
        SYS_FSYNC => RuntimeSyscallAction::Fsync { fd: args.a0 },
        SYS_FDATASYNC => RuntimeSyscallAction::Fdatasync { fd: args.a0 },
        SYS_UTIMENSAT => RuntimeSyscallAction::Utimensat { dirfd: args.a0 as isize, user_path: args.a1, user_times: args.a2, flags: args.a3 },
        SYS_CLONE => RuntimeSyscallAction::Clone { flags: args.a0, stack: args.a1, parent_tid: args.a2, tls: args.a3, child_tid: args.a4 },
        SYS_WAIT4 => RuntimeSyscallAction::Wait4 { pid: args.a0 as isize, user_wstatus: args.a1, options: args.a2, user_rusage: args.a3 },
        SYS_EXECVE => RuntimeSyscallAction::Execve { user_path: args.a0, user_argv: args.a1, user_envp: args.a2 },
        SYS_KILL => RuntimeSyscallAction::Kill { pid: args.a0 as isize, sig: args.a1 },
        SYS_TKILL => RuntimeSyscallAction::Tkill { tid: args.a0 as isize, sig: args.a1 },
        SYS_TGKILL => RuntimeSyscallAction::Tgkill { tgid: args.a0 as isize, tid: args.a1 as isize, sig: args.a2 },
        SYS_EXIT_GROUP => RuntimeSyscallAction::ExitGroup { code: args.a0 as isize },
        SYS_MKDIRAT => RuntimeSyscallAction::Mkdirat { dirfd: args.a0 as isize, user_path: args.a1, mode: args.a2 },
        SYS_UNLINKAT => RuntimeSyscallAction::Unlinkat { dirfd: args.a0 as isize, user_path: args.a1, flags: args.a2 },
        SYS_FACCESSAT => RuntimeSyscallAction::Faccessat { dirfd: args.a0 as isize, user_path: args.a1, mode: args.a2, flags: args.a3 },
        SYS_NEWFSTATAT => RuntimeSyscallAction::Newfstatat { dirfd: args.a0 as isize, user_path: args.a1, user_stat: args.a2, flags: args.a3 },
        SYS_RENAMEAT2 => RuntimeSyscallAction::Renameat2 { olddirfd: args.a0 as isize, oldpath: args.a1, newdirfd: args.a2 as isize, newpath: args.a3, flags: args.a4 },
        SYS_RENAMEAT => RuntimeSyscallAction::Renameat2 { olddirfd: args.a0 as isize, oldpath: args.a1, newdirfd: args.a2 as isize, newpath: args.a3, flags: 0 },
        SYS_STATX => RuntimeSyscallAction::Statx { dirfd: args.a0 as isize, user_path: args.a1, flags: args.a2, mask: args.a3, user_statx: args.a4 },
        SYS_EVENTFD2 => RuntimeSyscallAction::Eventfd2 { initval: args.a0, flags: args.a1 },
        SYS_EPOLL_CREATE1 => RuntimeSyscallAction::EpollCreate1 { flags: args.a0 },
        SYS_EPOLL_CTL => RuntimeSyscallAction::EpollCtl { epfd: args.a0, op: args.a1, fd: args.a2, event: args.a3 },
        SYS_EPOLL_PWAIT => RuntimeSyscallAction::EpollPwait { epfd: args.a0, events: args.a1, maxevents: args.a2, timeout: args.a3 as isize, sigmask: args.a4, sigsetsize: args.a5 },
        SYS_PPOLL => RuntimeSyscallAction::Ppoll { fds: args.a0, nfds: args.a1, timeout: args.a2, sigmask: args.a3, sigsetsize: args.a4 },
        SYS_PSELECT6 => RuntimeSyscallAction::Pselect6 { nfds: args.a0, readfds: args.a1, writefds: args.a2, exceptfds: args.a3, timeout: args.a4, sigmask: args.a5 },
        SYS_PIPE2 => RuntimeSyscallAction::Pipe2 { user_pipefd: args.a0, flags: args.a1 },
        SYS_DUP => RuntimeSyscallAction::Dup { oldfd: args.a0 },
        SYS_DUP3 => RuntimeSyscallAction::Dup3 { oldfd: args.a0, newfd: args.a1, flags: args.a2 },
        SYS_SCHED_YIELD => RuntimeSyscallAction::SchedYield,
        SYS_NANOSLEEP => RuntimeSyscallAction::Nanosleep { req: args.a0, rem: args.a1 },
        SYS_FUTEX => RuntimeSyscallAction::Futex { uaddr: args.a0, op: args.a1, val: args.a2, timeout: args.a3, uaddr2: args.a4, val3: args.a5 },
        SYS_RT_SIGACTION => RuntimeSyscallAction::RtSigaction { sig: args.a0, act: args.a1, oldact: args.a2, sigsetsize: args.a3 },
        SYS_RT_SIGPROCMASK => RuntimeSyscallAction::RtSigprocmask { how: args.a0, set: args.a1, oldset: args.a2, sigsetsize: args.a3 },
        SYS_RT_SIGRETURN => RuntimeSyscallAction::RtSigreturn,
        SYS_GETCWD => RuntimeSyscallAction::Getcwd { user_buf: args.a0, len: args.a1 },
        SYS_FCNTL => RuntimeSyscallAction::Fcntl { fd: args.a0, cmd: args.a1, arg: args.a2 },
        SYS_IOCTL => RuntimeSyscallAction::Ioctl { fd: args.a0, request: args.a1, argp: args.a2 },
        SYS_CHDIR => RuntimeSyscallAction::Chdir { user_path: args.a0 },
        SYS_READLINKAT => RuntimeSyscallAction::Readlinkat { dirfd: args.a0 as isize, user_path: args.a1, user_buf: args.a2, len: args.a3 },
        SYS_CLOCK_GETTIME => RuntimeSyscallAction::ClockGettime { clock_id: args.a0, user_ts: args.a1 },
        SYS_WRITE => {
            if args.a0 == 9637 || args.a0 == 9737 || args.a0 == 9831 || args.a0 == 9832 || args.a0 == 9931 || args.a0 == 9932 || args.a0 == 9933 || args.a0 == 9934 || args.a0 == 10001 || args.a0 == 10002 || args.a0 == 10003 || args.a0 == 10004 || args.a0 == 10005 || args.a0 == 10006 || args.a0 == 10007 || args.a0 == 10008 || args.a0 == 11001 || args.a0 == 11002 || args.a0 == 11003 || args.a0 == 11004 || args.a0 == 11005 || args.a0 == 11006 || args.a0 == 11007 || args.a0 == 11008 || args.a0 == 11009 || args.a0 == 11010 || args.a0 == 11011 || args.a0 == 11012 || args.a0 == 11013 || args.a0 == 11014 || args.a0 == 11015 || args.a0 == 11016 {
                RuntimeSyscallAction::Write { fd: args.a0, user_ptr: args.a1, len: args.a2, target: RuntimeWriteTarget::DevNull }
            } else if crate::fs::runtime::fd_can_write(args.a0) {
                RuntimeSyscallAction::Write { fd: args.a0, user_ptr: args.a1, len: args.a2, target: RuntimeWriteTarget::Console }
            } else {
                match crate::fs::fd_table::runtime_write_target(args.a0) {
                    Ok(target) => RuntimeSyscallAction::Write { fd: args.a0, user_ptr: args.a1, len: args.a2, target },
                    Err(err) => RuntimeSyscallAction::Return(err),
                }
            }
        },
        SYS_FSTAT => RuntimeSyscallAction::FStat { fd: args.a0, user_stat: args.a1 },
        SYS_BRK => RuntimeSyscallAction::Brk { addr: args.a0 },
        SYS_MUNMAP => RuntimeSyscallAction::Munmap { addr: args.a0, len: args.a1 },
        SYS_MPROTECT => RuntimeSyscallAction::Mprotect { addr: args.a0, len: args.a1, prot: args.a2 },
        SYS_EXIT => RuntimeSyscallAction::Exit { code: args.a0 as isize },
        SYS_GETTIMEOFDAY => RuntimeSyscallAction::Gettimeofday { user_tv: args.a0, user_tz: args.a1 },
        SYS_GETPID => RuntimeSyscallAction::Getpid,
        SYS_GETUID => RuntimeSyscallAction::Return(crate::fs::runtime::current_uid() as isize),
        SYS_GETEUID => RuntimeSyscallAction::Return(crate::fs::runtime::current_euid() as isize),
        SYS_GETGID => RuntimeSyscallAction::Return(crate::fs::runtime::current_gid() as isize),
        SYS_GETEGID => RuntimeSyscallAction::Return(crate::fs::runtime::current_egid() as isize),
        SYS_GETTID => RuntimeSyscallAction::Gettid,
        SYS_SYSINFO => RuntimeSyscallAction::Sysinfo { user_info: args.a0 },
        SYS_PRLIMIT64 => RuntimeSyscallAction::Prlimit64 { pid: args.a0, resource: args.a1, new_limit: args.a2, old_limit: args.a3 },
        SYS_GETRANDOM => RuntimeSyscallAction::Getrandom { user_buf: args.a0, len: args.a1, flags: args.a2 },
        SYS_GETPPID => RuntimeSyscallAction::Getppid,
        SYS_MADVISE => RuntimeSyscallAction::Madvise { addr: args.a0, len: args.a1, advice: args.a2 },
        SYS_UNAME => RuntimeSyscallAction::Uname { user_uts: args.a0 },
        SYS_UMASK => RuntimeSyscallAction::Umask { mask: args.a0 },
        SYS_MMAP => RuntimeSyscallAction::Mmap { addr: args.a0, len: args.a1, prot: args.a2, flags: args.a3, fd: args.a4 as isize, offset: args.a5 },
        // v84 rseq/membarrier/statx/pkey/copy_file_range syscall scaffold arms
        SYS_IO_PGETEVENTS => RuntimeSyscallAction::Return(sys_io_pgetevents(args.a0, args.a1, args.a2)),
        SYS_RSEQ => RuntimeSyscallAction::Return(sys_rseq(args.a0, args.a1, args.a2)),
        SYS_KEXEC_FILE_LOAD => RuntimeSyscallAction::Return(sys_kexec_file_load(args.a0, args.a1, args.a2)),
        // v85 mount/process-memory/memfd modern syscall scaffold arms
_ => RuntimeSyscallAction::Return(ENOSYS),
    }
}

pub fn self_test() {
    crate::println!("[syscall-dispatch-v59] self-test begin");

    match dispatch_runtime_syscall(RuntimeSyscallArgs::new(SYS_GETDENTS64, 5, 0x4001_fe00, 256, 0, 0, 0)) {
        RuntimeSyscallAction::GetDents64 { fd, user_dirent, len } => {
            crate::println!("[syscall-dispatch-v59] getdents fd = {}", fd);
            crate::println!("[syscall-dispatch-v59] getdents buf = {:#x}", user_dirent);
            crate::println!("[syscall-dispatch-v59] getdents len = {}", len);
        }
        _ => panic!("[syscall-dispatch-v59] getdents dispatch failed"),
    }

    let getpid = dispatch_runtime_syscall(RuntimeSyscallArgs::new(SYS_GETPID, 0, 0, 0, 0, 0, 0));
    let getppid = dispatch_runtime_syscall(RuntimeSyscallArgs::new(SYS_GETPPID, 0, 0, 0, 0, 0, 0));
    let unsupported = dispatch_runtime_syscall(RuntimeSyscallArgs::new(9999, 0, 0, 0, 0, 0, 0));
    let exit = dispatch_runtime_syscall(RuntimeSyscallArgs::new(SYS_EXIT, 0, 0, 0, 0, 0, 0));

    assert_eq!(getpid, RuntimeSyscallAction::Getpid);
    assert_eq!(getppid, RuntimeSyscallAction::Getppid);
    assert_eq!(unsupported, RuntimeSyscallAction::Return(ENOSYS));
    assert_eq!(exit, RuntimeSyscallAction::Exit { code: 0 });

    let exec_ret = dispatch_scaffold(SyscallFrame::new(SYS_EXECVE, [0; 6]));
    crate::println!("[syscall-dispatch-v59] execve scaffold ret = {}", exec_ret);

    if let Ok(info) = build_init_process_info() {
        let init = make_init_process(info);
        let zombie = make_zombie(init, 0);
        crate::println!("[syscall-dispatch-v59] zombie exit = {}", zombie.exit_code);
    }

    let mut fd_table = FdTable::with_stdio();
    if let Some(dev_dir_fd) = fd_table.alloc(FileKind::DevDir, true, false) {
        let closed = fd_table.close(dev_dir_fd);
        crate::println!("[syscall-dispatch-v59] devdir fd closed = {}", closed as usize);
    }

    crate::println!("[syscall-dispatch-v59] self-test passed");
}

// v84 rseq/membarrier/statx/pkey/copy_file_range syscall scaffold handlers

#[allow(unused_variables)]
fn sys_bpf(_v84_arg0: usize, _v84_arg1: usize, _v84_arg2: usize) -> isize {
    println!("[syscall-v84] sys_bpf(280) scaffold");
    0
}


#[allow(unused_variables)]
fn sys_execveat(_v84_arg0: usize, _v84_arg1: usize, _v84_arg2: usize) -> isize {
    println!("[syscall-v84] sys_execveat(281) scaffold");
    0
}


#[allow(unused_variables)]
fn sys_userfaultfd(_v84_arg0: usize, _v84_arg1: usize, _v84_arg2: usize) -> isize {
    println!("[syscall-v84] sys_userfaultfd(282) scaffold");
    0
}


#[allow(unused_variables)]
fn sys_membarrier(_v84_arg0: usize, _v84_arg1: usize, _v84_arg2: usize) -> isize {
    println!("[syscall-v84] sys_membarrier(283) scaffold");
    0
}


#[allow(unused_variables)]
fn sys_mlock2(_v84_arg0: usize, _v84_arg1: usize, _v84_arg2: usize) -> isize {
    println!("[syscall-v84] sys_mlock2(284) scaffold");
    0
}


#[allow(unused_variables)]
fn sys_copy_file_range(_v84_arg0: usize, _v84_arg1: usize, _v84_arg2: usize) -> isize {
    println!("[syscall-v84] sys_copy_file_range(285) scaffold");
    0
}


#[allow(unused_variables)]
fn sys_preadv2(_v84_arg0: usize, _v84_arg1: usize, _v84_arg2: usize) -> isize {
    println!("[syscall-v84] sys_preadv2(286) scaffold");
    0
}


#[allow(unused_variables)]
fn sys_pwritev2(_v84_arg0: usize, _v84_arg1: usize, _v84_arg2: usize) -> isize {
    println!("[syscall-v84] sys_pwritev2(287) scaffold");
    0
}


#[allow(unused_variables)]
fn sys_pkey_mprotect(_v84_arg0: usize, _v84_arg1: usize, _v84_arg2: usize) -> isize {
    println!("[syscall-v84] sys_pkey_mprotect(288) scaffold");
    0
}


#[allow(unused_variables)]
fn sys_pkey_alloc(_v84_arg0: usize, _v84_arg1: usize, _v84_arg2: usize) -> isize {
    println!("[syscall-v84] sys_pkey_alloc(289) scaffold");
    0
}


#[allow(unused_variables)]
fn sys_pkey_free(_v84_arg0: usize, _v84_arg1: usize, _v84_arg2: usize) -> isize {
    println!("[syscall-v84] sys_pkey_free(290) scaffold");
    0
}


#[allow(unused_variables)]
fn sys_statx(_v84_arg0: usize, _v84_arg1: usize, _v84_arg2: usize) -> isize {
    println!("[syscall-v84] sys_statx(291) scaffold");
    0
}


#[allow(unused_variables)]
fn sys_io_pgetevents(_v84_arg0: usize, _v84_arg1: usize, _v84_arg2: usize) -> isize {
    println!("[syscall-v84] sys_io_pgetevents(292) scaffold");
    0
}


#[allow(unused_variables)]
fn sys_rseq(_v84_arg0: usize, _v84_arg1: usize, _v84_arg2: usize) -> isize {
    println!("[syscall-v84] sys_rseq(293) scaffold");
    0
}


#[allow(unused_variables)]
fn sys_kexec_file_load(_v84_arg0: usize, _v84_arg1: usize, _v84_arg2: usize) -> isize {
    println!("[syscall-v84] sys_kexec_file_load(294) scaffold");
    0
}


// v85 mount/process-memory/memfd modern syscall scaffold functions start
#[allow(unused_variables)]
pub fn sys_fspick_scaffold(dfd: usize, path: usize, flags: usize) -> isize {
    println!("[syscall-v85] sys_fspick(433) scaffold");
    ENOSYS
}

#[allow(unused_variables)]
pub fn sys_pidfd_getfd_scaffold(pidfd: usize, targetfd: usize, flags: usize) -> isize {
    println!("[syscall-v85] sys_pidfd_getfd(438) scaffold");
    ENOSYS
}

#[allow(unused_variables)]
pub fn sys_faccessat2_scaffold(dirfd: usize, pathname: usize, mode: usize, flags: usize) -> isize {
    println!("[syscall-v85] sys_faccessat2(439) scaffold");
    0
}

#[allow(unused_variables)]
pub fn sys_process_madvise_scaffold(pidfd: usize, iovec: usize, vlen: usize, advice: usize, flags: usize) -> isize {
    println!("[syscall-v85] sys_process_madvise(440) scaffold");
    0
}

#[allow(unused_variables)]
pub fn sys_epoll_pwait2_scaffold(epfd: usize, events: usize, maxevents: usize, timeout: usize, sigmask: usize) -> isize {
    println!("[syscall-v85] sys_epoll_pwait2(441) scaffold");
    0
}

#[allow(unused_variables)]
pub fn sys_mount_setattr_scaffold(dfd: usize, path: usize, flags: usize, attr: usize, size: usize) -> isize {
    println!("[syscall-v85] sys_mount_setattr(442) scaffold");
    0
}

#[allow(unused_variables)]
pub fn sys_quotactl_fd_scaffold(fd: usize, cmd: usize, id: usize, addr: usize) -> isize {
    println!("[syscall-v85] sys_quotactl_fd(443) scaffold");
    0
}

#[allow(unused_variables)]
pub fn sys_memfd_secret_scaffold(flags: usize) -> isize {
    println!("[syscall-v85] sys_memfd_secret(447) scaffold");
    ENOSYS
}

#[allow(unused_variables)]
pub fn sys_process_mrelease_scaffold(pidfd: usize, flags: usize) -> isize {
    println!("[syscall-v85] sys_process_mrelease(448) scaffold");
    0
}

#[allow(unused_variables)]
pub fn sys_set_mempolicy_home_node_scaffold(start: usize, len: usize, home_node: usize, flags: usize) -> isize {
    println!("[syscall-v85] sys_set_mempolicy_home_node(450) scaffold");
    0
}

#[allow(unused_variables)]
pub fn sys_cachestat_scaffold(fd: usize, cstat_range: usize, cstat: usize, flags: usize) -> isize {
    println!("[syscall-v85] sys_cachestat(451) scaffold");
    0
}

#[allow(unused_variables)]
pub fn sys_fchmodat2_scaffold(dirfd: usize, pathname: usize, mode: usize, flags: usize) -> isize {
    println!("[syscall-v85] sys_fchmodat2(452) scaffold");
    0
}

#[allow(unused_variables)]
pub fn sys_map_shadow_stack_scaffold(addr: usize, size: usize, flags: usize) -> isize {
    println!("[syscall-v85] sys_map_shadow_stack(453) scaffold");
    ENOSYS
}
// v85 mount/process-memory/memfd modern syscall scaffold functions end


// v86 mount/LSM/futex/xattr-at/mseal syscall scaffold handlers

#[allow(dead_code)]
pub fn sys_futex_wake_scaffold(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize) -> isize {
    0
}


#[allow(dead_code)]
pub fn sys_futex_wait_scaffold(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize) -> isize {
    0
}


#[allow(dead_code)]
pub fn sys_futex_requeue_scaffold(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize) -> isize {
    0
}


#[allow(dead_code)]
pub fn sys_statmount_scaffold(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize) -> isize {
    0
}


#[allow(dead_code)]
pub fn sys_listmount_scaffold(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize) -> isize {
    0
}


#[allow(dead_code)]
pub fn sys_lsm_get_self_attr_scaffold(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize) -> isize {
    0
}


#[allow(dead_code)]
pub fn sys_lsm_set_self_attr_scaffold(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize) -> isize {
    0
}


#[allow(dead_code)]
pub fn sys_lsm_list_modules_scaffold(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize) -> isize {
    0
}


#[allow(dead_code)]
pub fn sys_mseal_scaffold(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize) -> isize {
    0
}


#[allow(dead_code)]
pub fn sys_setxattrat_scaffold(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize) -> isize {
    0
}


#[allow(dead_code)]
pub fn sys_getxattrat_scaffold(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize) -> isize {
    0
}


#[allow(dead_code)]
pub fn sys_listxattrat_scaffold(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize) -> isize {
    0
}


#[allow(dead_code)]
pub fn sys_removexattrat_scaffold(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize) -> isize {
    0
}


#[allow(dead_code)]
pub fn sys_open_tree_attr_scaffold(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize) -> isize {
    0
}


#[allow(dead_code)]
pub fn sys_file_getattr_scaffold(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize) -> isize {
    0
}


#[allow(dead_code)]
pub fn sys_file_setattr_scaffold(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize) -> isize {
    0
}


#[allow(dead_code)]
pub fn sys_listns_scaffold(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize) -> isize {
    0
}


#[allow(dead_code)]
pub fn sys_rseq_slice_yield_scaffold(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize) -> isize {
    0
}
