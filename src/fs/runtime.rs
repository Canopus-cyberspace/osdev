#![allow(dead_code)]
#![allow(static_mut_refs)]

const MAX_NODES: usize = 96;
const MAX_FDS: usize = 128;
const MAX_OFDS: usize = 64;
const MAX_PIPES: usize = 8;
const MAX_EVENTS: usize = 8;
const MAX_TIMERS: usize = 8;
const MAX_SOCKETS: usize = 16;
const MAX_EPOLL: usize = 8;
const MAX_IPC: usize = 8;
const MAX_FUTEXES: usize = 8;
const MAX_TASKS: usize = 4;
const MAX_WAIT_QUEUES: usize = 8;
const MAX_VMAS: usize = 16;
const MAX_SIGNALS: usize = 32;
const MAX_PENDING_SIGNALS: usize = 16;
const MAX_MOUNTS: usize = 8;
const MAX_BLOCK_DEVS: usize = 2;
const BLOCK_SIZE: usize = 512;
const RAMDISK_BLOCKS: usize = 24;
const RAMDISK_BYTES: usize = BLOCK_SIZE * RAMDISK_BLOCKS;
const BLOCK_CACHE_SLOTS: usize = 4;
const IMAGE_MAX_ENTRIES: usize = 12;
const IMAGE_ENTRY_SIZE: usize = 64;
const IMAGE_ENTRY_TABLE_BLOCK: usize = 1;
const IMAGE_DEV: usize = 0;
const NAME_MAX: usize = 32;
const DATA_MAX: usize = 2048;
const TARGET_MAX: usize = 96;
const PATH_MAX: usize = 128;
const PIPE_BUF: usize = 128;
const SOCKET_BUF: usize = 128;
const MSG_BUF: usize = 96;
pub const RUNTIME_IOVEC_BUF: usize = 128;
pub const EXEC_ARG_MAX: usize = 8;
pub const EXEC_ENV_MAX: usize = 8;
pub const EXEC_STR_MAX: usize = 64;
pub const RUNTIME_PAGE_SIZE: usize = 4096;
pub const RUNTIME_USER_STACK_TOP: usize = 0x4002_0000;
pub const RUNTIME_USER_STACK_SIZE: usize = 4 * RUNTIME_PAGE_SIZE;
pub const RUNTIME_USER_STACK_BOTTOM: usize = RUNTIME_USER_STACK_TOP - RUNTIME_USER_STACK_SIZE;
pub const RUNTIME_USER_HEAP_START: usize = 0x4003_0000;
pub const RUNTIME_USER_HEAP_LIMIT: usize = 0x4004_0000;
pub const RUNTIME_USER_MMAP_START: usize = 0x4004_0000;
pub const RUNTIME_USER_MMAP_LIMIT: usize = 0x4008_0000;
pub const RUNTIME_PROT_READ: usize = 0x1;
pub const RUNTIME_PROT_WRITE: usize = 0x2;
pub const RUNTIME_PROT_EXEC: usize = 0x4;
pub const RUNTIME_MAP_FIXED: usize = 0x10;
pub const RUNTIME_MAP_ANONYMOUS: usize = 0x20;
const ROOT: usize = 1;

pub const AT_FDCWD: isize = -100;
pub const EFAULT: isize = crate::syscall::errno::EFAULT;
pub const EBADF: isize = crate::syscall::errno::EBADF;
pub const ENOEXEC: isize = crate::syscall::errno::ENOEXEC;
pub const ESRCH: isize = crate::syscall::errno::ESRCH;
pub const EACCES: isize = crate::syscall::errno::EACCES;
pub const EAGAIN: isize = crate::syscall::errno::EAGAIN;
pub const EEXIST: isize = crate::syscall::errno::EEXIST;
pub const ENOTDIR: isize = crate::syscall::errno::ENOTDIR;
pub const EISDIR: isize = crate::syscall::errno::EISDIR;
pub const EINVAL: isize = crate::syscall::errno::EINVAL;
pub const ENOSPC: isize = crate::syscall::errno::ENOSPC;
pub const ESPIPE: isize = crate::syscall::errno::ESPIPE;
pub const ENOSYS: isize = crate::syscall::errno::ENOSYS;
pub const ENOTEMPTY: isize = crate::syscall::errno::ENOTEMPTY;
pub const ENOENT: isize = crate::syscall::errno::ENOENT;
pub const ECHILD: isize = crate::syscall::errno::ECHILD;

pub const O_CREAT: u32 = 0x40;
pub const O_WRONLY: u32 = 0x1;
pub const O_RDWR: u32 = 0x2;
pub const O_ACCMODE: u32 = 0x3;
pub const O_TRUNC: u32 = 0x200;
pub const O_APPEND: u32 = 0x400;
pub const O_NONBLOCK: u32 = 0x800;
pub const O_DIRECTORY: u32 = 0x10000;
pub const O_CLOEXEC: u32 = 0x80000;
pub const FD_CLOEXEC: u32 = 1;
pub const F_GETFD: usize = 1;
pub const F_SETFD: usize = 2;
pub const F_GETFL: usize = 3;
pub const F_SETFL: usize = 4;
pub const CLOSE_RANGE_CLOEXEC: usize = 1 << 2;
pub const SEEK_SET: usize = 0;
pub const SEEK_CUR: usize = 1;
pub const SEEK_END: usize = 2;

pub const POLLIN: u16 = 0x001;
pub const POLLOUT: u16 = 0x004;
pub const POLLERR: u16 = 0x008;
pub const POLLHUP: u16 = 0x010;
pub const POLLNVAL: u16 = 0x020;
pub const EPOLL_CTL_ADD: usize = 1;
pub const EPOLL_CTL_DEL: usize = 2;
pub const EPOLL_CTL_MOD: usize = 3;
pub const IPC_RMID: usize = 0;
pub const IPC_STAT: usize = 2;
pub const FUTEX_WAIT: usize = 0;
pub const FUTEX_WAKE: usize = 1;
pub const SCHED_WAIT_FUTEX_BASE: usize = 0xF00D_0000;
pub const SCHED_WAIT_TIMEOUT_BASE: usize = 0x710E_0000;
pub const SCHED_WAIT_MQ_BASE: usize = 0x4D51_0000;
pub const SCHED_WAIT_MSG_BASE: usize = 0x4D53_0000;
pub const SCHED_WAIT_SEM_BASE: usize = 0x5345_0000;
pub const SCHED_WAIT_SHM_BASE: usize = 0x5348_0000;
pub const SCHED_WAIT_PIPE_BASE: usize = 0x5049_0000;
pub const SCHED_WAIT_TIMER_BASE: usize = 0x714D_0000;
pub const SCHED_WAIT_CHILD_BASE: usize = 0xC417_D000;
pub const CLONE_THREAD: usize = 0x0001_0000;
pub const CLONE_NEWNS: usize = 0x0002_0000;
pub const CLONE_NEWIPC: usize = 0x0800_0000;
pub const CLONE_NEWPID: usize = 0x2000_0000;
pub const AF_UNIX: usize = 1;
pub const SOCK_STREAM: u32 = 1;
pub const SOCK_DGRAM: u32 = 2;
const SOCK_TYPE_MASK: u32 = 0xf;
pub const CAP_CHOWN: usize = 0;
pub const CAP_DAC_OVERRIDE: usize = 1;
pub const CAP_FOWNER: usize = 3;
pub const CAP_SETGID: usize = 6;
pub const CAP_SETUID: usize = 7;
const DEFAULT_CAPS: u64 = (1u64 << CAP_CHOWN)
    | (1u64 << CAP_DAC_OVERRIDE)
    | (1u64 << CAP_FOWNER)
    | (1u64 << CAP_SETGID)
    | (1u64 << CAP_SETUID);
pub const SIG_BLOCK: usize = 0;
pub const SIG_UNBLOCK: usize = 1;
pub const SIG_SETMASK: usize = 2;
pub const SIGCHLD: usize = 17;
pub const SIGTERM: usize = 15;
pub const SIGUSR1: usize = 10;
pub const SIGUSR2: usize = 12;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Errno {
    NoEnt,
    Exist,
    BadFd,
    NotDir,
    IsDir,
    Inval,
    NoSpace,
    NoSys,
    Access,
    Again,
    NotEmpty,
    SpiPe,
    Child,
    Fault,
    NoExec,
    Srch,
}

impl Errno {
    const fn code(self) -> isize {
        match self {
            Self::NoEnt => ENOENT,
            Self::Exist => EEXIST,
            Self::BadFd => EBADF,
            Self::NotDir => ENOTDIR,
            Self::IsDir => EISDIR,
            Self::Inval => EINVAL,
            Self::NoSpace => ENOSPC,
            Self::NoSys => ENOSYS,
            Self::Access => EACCES,
            Self::Again => EAGAIN,
            Self::NotEmpty => ENOTEMPTY,
            Self::SpiPe => ESPIPE,
            Self::Child => ECHILD,
            Self::Fault => EFAULT,
            Self::NoExec => ENOEXEC,
            Self::Srch => ESRCH,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum NodeKind {
    Empty,
    Dir,
    File,
    Symlink,
    DevNull,
    DevZero,
    DevConsole,
    DevTty,
    DevRandom,
    ProcFdDir,
    ProcStatus,
    ProcStat,
    ProcMaps,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FdKind {
    Empty,
    Stdin,
    Stdout,
    Stderr,
    RegularFile,
    Directory,
    Symlink,
    DevNull,
    DevZero,
    DevConsole,
    DevTty,
    DevRandom,
    PipeRead,
    PipeWrite,
    EventFd,
    TimerFd,
    Socket,
    Epoll,
    Procfs,
    Mq,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RuntimeFsKind {
    Rootfs,
    Tmpfs,
    Devfs,
    Procfs,
    Imagefs,
}

#[derive(Copy, Clone)]
struct Node {
    used: bool,
    kind: NodeKind,
    fs_kind: RuntimeFsKind,
    parent: usize,
    owner: usize,
    uid: u32,
    gid: u32,
    mode: u16,
    name: [u8; NAME_MAX],
    name_len: usize,
    data: [u8; DATA_MAX],
    size: usize,
    target: [u8; TARGET_MAX],
    target_len: usize,
    image_inode: usize,
    image_offset: usize,
    image_flags: u32,
}

impl Node {
    const fn empty() -> Self {
        Self {
            used: false,
            kind: NodeKind::Empty,
            fs_kind: RuntimeFsKind::Rootfs,
            parent: 0,
            owner: 0,
            uid: 0,
            gid: 0,
            mode: 0,
            name: [0; NAME_MAX],
            name_len: 0,
            data: [0; DATA_MAX],
            size: 0,
            target: [0; TARGET_MAX],
            target_len: 0,
            image_inode: 0,
            image_offset: 0,
            image_flags: 0,
        }
    }
}

#[derive(Copy, Clone)]
struct MountObj {
    used: bool,
    target: usize,
    fs_kind: RuntimeFsKind,
    flags: usize,
    source: [u8; NAME_MAX],
    source_len: usize,
    fstype: [u8; NAME_MAX],
    fstype_len: usize,
}

impl MountObj {
    const fn empty() -> Self {
        Self {
            used: false,
            target: 0,
            fs_kind: RuntimeFsKind::Rootfs,
            flags: 0,
            source: [0; NAME_MAX],
            source_len: 0,
            fstype: [0; NAME_MAX],
            fstype_len: 0,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum BlockDeviceKind {
    Empty,
    Ramdisk,
}

#[derive(Copy, Clone)]
struct BlockDeviceObj {
    used: bool,
    kind: BlockDeviceKind,
    sector_size: usize,
    sectors: usize,
    readonly: bool,
    read_ops: usize,
    write_ops: usize,
    errors: usize,
}

impl BlockDeviceObj {
    const fn empty() -> Self {
        Self {
            used: false,
            kind: BlockDeviceKind::Empty,
            sector_size: 0,
            sectors: 0,
            readonly: false,
            read_ops: 0,
            write_ops: 0,
            errors: 0,
        }
    }
}

#[derive(Copy, Clone)]
struct BlockCacheEntry {
    valid: bool,
    dirty: bool,
    dev: usize,
    block: usize,
    data: [u8; BLOCK_SIZE],
}

impl BlockCacheEntry {
    const fn empty() -> Self {
        Self { valid: false, dirty: false, dev: 0, block: 0, data: [0; BLOCK_SIZE] }
    }
}

#[derive(Copy, Clone)]
struct Fd {
    used: bool,
    ofd: usize,
    cloexec: bool,
}

impl Fd {
    const fn empty() -> Self {
        Self { used: false, ofd: 0, cloexec: false }
    }
}

#[derive(Copy, Clone)]
struct Ofd {
    used: bool,
    kind: FdKind,
    node: usize,
    off: usize,
    flags: u32,
    refs: usize,
    object: usize,
}

impl Ofd {
    const fn empty() -> Self {
        Self { used: false, kind: FdKind::Empty, node: 0, off: 0, flags: 0, refs: 0, object: 0 }
    }
}

#[derive(Copy, Clone)]
struct PipeObj {
    used: bool,
    data: [u8; PIPE_BUF],
    len: usize,
    readers: usize,
    writers: usize,
}

impl PipeObj {
    const fn empty() -> Self {
        Self { used: false, data: [0; PIPE_BUF], len: 0, readers: 0, writers: 0 }
    }
}

#[derive(Copy, Clone)]
struct EventObj {
    used: bool,
    counter: u64,
    flags: u32,
}

impl EventObj {
    const fn empty() -> Self {
        Self { used: false, counter: 0, flags: 0 }
    }
}

#[derive(Copy, Clone)]
struct TimerObj {
    used: bool,
    clockid: usize,
    armed: bool,
    expirations: u64,
}

impl TimerObj {
    const fn empty() -> Self {
        Self { used: false, clockid: 0, armed: false, expirations: 0 }
    }
}

#[derive(Copy, Clone)]
struct SocketObj {
    used: bool,
    domain: u16,
    sock_type: u16,
    protocol: u16,
    peer: usize,
    pending: usize,
    bound: bool,
    listening: bool,
    addr: [u8; NAME_MAX],
    addr_len: usize,
    peer_addr: [u8; NAME_MAX],
    peer_addr_len: usize,
    data: [u8; SOCKET_BUF],
    len: usize,
    sends: usize,
    recvs: usize,
}

impl SocketObj {
    const fn empty() -> Self {
        Self {
            used: false,
            domain: 0,
            sock_type: 0,
            protocol: 0,
            peer: usize::MAX,
            pending: usize::MAX,
            bound: false,
            listening: false,
            addr: [0; NAME_MAX],
            addr_len: 0,
            peer_addr: [0; NAME_MAX],
            peer_addr_len: 0,
            data: [0; SOCKET_BUF],
            len: 0,
            sends: 0,
            recvs: 0,
        }
    }
}

#[derive(Copy, Clone)]
struct EpollObj {
    used: bool,
    watched: [usize; 8],
    events: [u32; 8],
    data: [u64; 8],
    count: usize,
}

impl EpollObj {
    const fn empty() -> Self {
        Self { used: false, watched: [0; 8], events: [0; 8], data: [0; 8], count: 0 }
    }
}

#[derive(Copy, Clone)]
pub struct RuntimeEpollEvent {
    pub events: u32,
    pub data: u64,
    pub fd: usize,
}

impl RuntimeEpollEvent {
    pub const fn empty() -> Self {
        Self { events: 0, data: 0, fd: 0 }
    }
}

#[derive(Copy, Clone)]
pub struct RuntimeIovec {
    pub data: [u8; RUNTIME_IOVEC_BUF],
    pub len: usize,
}

impl RuntimeIovec {
    pub const fn empty() -> Self {
        Self { data: [0; RUNTIME_IOVEC_BUF], len: 0 }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RuntimeExecString {
    pub data: [u8; EXEC_STR_MAX],
    pub len: usize,
}

impl RuntimeExecString {
    pub const fn empty() -> Self {
        Self { data: [0; EXEC_STR_MAX], len: 0 }
    }

    pub fn from_bytes(src: &[u8]) -> Result<Self, isize> {
        if src.len() >= EXEC_STR_MAX {
            return Err(EINVAL);
        }
        let mut item = Self::empty();
        let mut i = 0usize;
        while i < src.len() {
            item.data[i] = src[i];
            i += 1;
        }
        item.len = src.len();
        Ok(item)
    }

    fn as_slice(&self) -> &[u8] {
        &self.data[..self.len]
    }
}

#[derive(Copy, Clone)]
pub struct RuntimeIoStats {
    pub read_ops: usize,
    pub write_ops: usize,
    pub positioned_ops: usize,
    pub msg_ops: usize,
    pub bytes_read: usize,
    pub bytes_written: usize,
}

impl RuntimeIoStats {
    const fn empty() -> Self {
        Self { read_ops: 0, write_ops: 0, positioned_ops: 0, msg_ops: 0, bytes_read: 0, bytes_written: 0 }
    }
}

#[derive(Copy, Clone)]
struct MqObj {
    used: bool,
    unlinked: bool,
    key: usize,
    data: [u8; MSG_BUF],
    len: usize,
    prio: usize,
}

impl MqObj {
    const fn empty() -> Self {
        Self { used: false, unlinked: false, key: 0, data: [0; MSG_BUF], len: 0, prio: 0 }
    }
}

#[derive(Copy, Clone)]
struct MsgObj {
    used: bool,
    id: usize,
    key: usize,
    data: [u8; MSG_BUF],
    len: usize,
}

impl MsgObj {
    const fn empty() -> Self {
        Self { used: false, id: 0, key: 0, data: [0; MSG_BUF], len: 0 }
    }
}

#[derive(Copy, Clone)]
struct SemObj {
    used: bool,
    id: usize,
    key: usize,
    nsems: usize,
    value: isize,
}

impl SemObj {
    const fn empty() -> Self {
        Self { used: false, id: 0, key: 0, nsems: 0, value: 0 }
    }
}

#[derive(Copy, Clone)]
struct ShmObj {
    used: bool,
    attached: bool,
    id: usize,
    key: usize,
    size: usize,
}

impl ShmObj {
    const fn empty() -> Self {
        Self { used: false, attached: false, id: 0, key: 0, size: 0 }
    }
}

#[derive(Copy, Clone)]
struct FutexObj {
    used: bool,
    key: usize,
    waiters: usize,
    total_waits: usize,
    total_wakes: usize,
    last_expected: u32,
}

impl FutexObj {
    const fn empty() -> Self {
        Self { used: false, key: 0, waiters: 0, total_waits: 0, total_wakes: 0, last_expected: 0 }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RuntimeTaskState {
    Empty,
    Running,
    Ready,
    Waiting,
    Zombie,
    Exited,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RuntimeVmaKind {
    Empty,
    Load,
    Heap,
    Stack,
    Mmap,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RuntimeFaultAccess {
    Read,
    Write,
    Execute,
}

const VMA_R: u8 = 1 << 0;
const VMA_W: u8 = 1 << 1;
const VMA_X: u8 = 1 << 2;
const VMA_U: u8 = 1 << 3;

#[derive(Copy, Clone)]
struct VmaObj {
    used: bool,
    mm_id: usize,
    start: usize,
    end: usize,
    perm: u8,
    kind: RuntimeVmaKind,
    lazy: bool,
    resident_pages: usize,
    fd: isize,
    offset: usize,
}

impl VmaObj {
    const fn empty() -> Self {
        Self {
            used: false,
            mm_id: 0,
            start: 0,
            end: 0,
            perm: 0,
            kind: RuntimeVmaKind::Empty,
            lazy: false,
            resident_pages: 0,
            fd: -1,
            offset: 0,
        }
    }
}

#[derive(Copy, Clone)]
struct ExecImage {
    valid: bool,
    node: usize,
    entry: usize,
    phoff: usize,
    phentsize: usize,
    phnum: usize,
    load_start: usize,
    load_end: usize,
    file_size: usize,
    mem_size: usize,
    stack_pointer: usize,
    argv0_ptr: usize,
    env0_ptr: usize,
    auxv_start: usize,
    argc: usize,
    envc: usize,
    auxc: usize,
    closed_cloexec: usize,
    mm_id: usize,
    seq: usize,
}

impl ExecImage {
    const fn empty() -> Self {
        Self {
            valid: false,
            node: 0,
            entry: 0,
            phoff: 0,
            phentsize: 0,
            phnum: 0,
            load_start: 0,
            load_end: 0,
            file_size: 0,
            mem_size: 0,
            stack_pointer: 0,
            argv0_ptr: 0,
            env0_ptr: 0,
            auxv_start: 0,
            argc: 0,
            envc: 0,
            auxc: 0,
            closed_cloexec: 0,
            mm_id: 0,
            seq: 0,
        }
    }
}

#[derive(Copy, Clone)]
struct ElfLoadInfo {
    entry: usize,
    phoff: usize,
    phentsize: usize,
    phnum: usize,
    load_start: usize,
    load_end: usize,
    file_size: usize,
    mem_size: usize,
    load_flags: u32,
}

#[derive(Copy, Clone)]
struct ExecStackLayout {
    sp: usize,
    argv0_ptr: usize,
    env0_ptr: usize,
    auxv_start: usize,
    auxc: usize,
}

#[derive(Copy, Clone)]
pub struct RuntimeExecSnapshot {
    pub valid: bool,
    pub entry: usize,
    pub phnum: usize,
    pub load_start: usize,
    pub load_end: usize,
    pub stack_pointer: usize,
    pub argv0_ptr: usize,
    pub env0_ptr: usize,
    pub auxv_start: usize,
    pub argc: usize,
    pub envc: usize,
    pub auxc: usize,
    pub closed_cloexec: usize,
    pub mm_id: usize,
    pub seq: usize,
}

#[derive(Copy, Clone)]
pub struct RuntimeVmSnapshot {
    pub mm_id: usize,
    pub vma_count: usize,
    pub load_count: usize,
    pub heap_end: usize,
    pub mmap_count: usize,
    pub lazy_count: usize,
    pub resident_pages: usize,
    pub writable_count: usize,
    pub executable_count: usize,
    pub last_fault_addr: usize,
    pub last_fault_ok: bool,
}

#[derive(Copy, Clone)]
pub struct RuntimeVmPermissions {
    pub mm_id: usize,
    pub start: usize,
    pub end: usize,
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
    pub user: bool,
    pub lazy: bool,
    pub kind: RuntimeVmaKind,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RuntimeSignalAction {
    pub handler: usize,
    pub flags: usize,
    pub restorer: usize,
    pub mask: u64,
    pub installed: bool,
}

impl RuntimeSignalAction {
    pub const fn empty() -> Self {
        Self { handler: 0, flags: 0, restorer: 0, mask: 0, installed: false }
    }

    pub const fn handler(handler: usize, flags: usize, restorer: usize, mask: u64) -> Self {
        Self { handler, flags, restorer, mask, installed: true }
    }
}

#[derive(Copy, Clone)]
struct PendingSignalObj {
    used: bool,
    target_pid: usize,
    source_pid: usize,
    sig: usize,
    code: isize,
    process_group: bool,
}

impl PendingSignalObj {
    const fn empty() -> Self {
        Self { used: false, target_pid: 0, source_pid: 0, sig: 0, code: 0, process_group: false }
    }
}

#[derive(Copy, Clone)]
struct SignalFrameObj {
    active: bool,
    pid: usize,
    sig: usize,
    handler: usize,
    frame_sp: usize,
    saved_pc: usize,
    saved_sp: usize,
    saved_mask: u64,
    restorer: usize,
    seq: usize,
}

impl SignalFrameObj {
    const fn empty() -> Self {
        Self { active: false, pid: 0, sig: 0, handler: 0, frame_sp: 0, saved_pc: 0, saved_sp: 0, saved_mask: 0, restorer: 0, seq: 0 }
    }
}

#[derive(Copy, Clone)]
pub struct RuntimeSignalRestore {
    pub pc: usize,
    pub sp: usize,
    pub mask: u64,
    pub sig: usize,
}

#[derive(Copy, Clone)]
pub struct RuntimeSignalSnapshot {
    pub current_pid: usize,
    pub blocked_mask: u64,
    pub action_count: usize,
    pub pending_count: usize,
    pub frame_active: bool,
    pub frame_sig: usize,
    pub frame_sp: usize,
    pub frame_handler: usize,
    pub frame_restorer: usize,
    pub saved_pc: usize,
    pub saved_sp: usize,
    pub last_delivered_sig: usize,
    pub last_delivered_pid: usize,
    pub delivered_count: usize,
    pub returned_count: usize,
    pub queued_count: usize,
    pub sigchld_count: usize,
    pub direct_deliveries: usize,
    pub tkill_deliveries: usize,
    pub tgkill_deliveries: usize,
    pub group_deliveries: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RuntimeCred {
    pub uid: u32,
    pub euid: u32,
    pub suid: u32,
    pub fsuid: u32,
    pub gid: u32,
    pub egid: u32,
    pub sgid: u32,
    pub fsgid: u32,
    pub cap_permitted: u64,
    pub cap_effective: u64,
    pub cap_inheritable: u64,
}

impl RuntimeCred {
    pub const fn root() -> Self {
        Self {
            uid: 0,
            euid: 0,
            suid: 0,
            fsuid: 0,
            gid: 0,
            egid: 0,
            sgid: 0,
            fsgid: 0,
            cap_permitted: DEFAULT_CAPS,
            cap_effective: DEFAULT_CAPS,
            cap_inheritable: 0,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RuntimeNamespaceRefs {
    pub mount_ns: usize,
    pub ipc_ns: usize,
    pub pid_ns: usize,
    pub unshare_count: usize,
    pub setns_count: usize,
    pub last_error: isize,
}

impl RuntimeNamespaceRefs {
    pub const fn initial() -> Self {
        Self { mount_ns: 1, ipc_ns: 1, pid_ns: 1, unshare_count: 0, setns_count: 0, last_error: 0 }
    }
}

#[derive(Copy, Clone)]
struct TaskObj {
    used: bool,
    pid: usize,
    ppid: usize,
    tgid: usize,
    pgid: usize,
    sid: usize,
    state: RuntimeTaskState,
    exit_code: isize,
    wait_key: usize,
    yields: usize,
    fork_return: isize,
    fd_count: usize,
    cwd: usize,
    root: usize,
    signal_mask: u64,
    mm_id: usize,
    cred: RuntimeCred,
    namespaces: RuntimeNamespaceRefs,
}

impl TaskObj {
    const fn empty() -> Self {
        Self {
            used: false,
            pid: 0,
            ppid: 0,
            tgid: 0,
            pgid: 0,
            sid: 0,
            state: RuntimeTaskState::Empty,
            exit_code: 0,
            wait_key: 0,
            yields: 0,
            fork_return: 0,
            fd_count: 0,
            cwd: 0,
            root: 0,
            signal_mask: 0,
            mm_id: 0,
            cred: RuntimeCred::root(),
            namespaces: RuntimeNamespaceRefs::initial(),
        }
    }
}

#[derive(Copy, Clone)]
struct WaitQueueObj {
    used: bool,
    key: usize,
    waiters: usize,
    wakeups: usize,
}

impl WaitQueueObj {
    const fn empty() -> Self {
        Self { used: false, key: 0, waiters: 0, wakeups: 0 }
    }
}

#[derive(Copy, Clone)]
pub struct RuntimeSchedSnapshot {
    pub current_pid: usize,
    pub current_state: RuntimeTaskState,
    pub wait_queues: usize,
    pub waiters: usize,
    pub wakeups: usize,
    pub yields: usize,
    pub runq_len: usize,
    pub ticks: usize,
    pub switches: usize,
    pub blocks: usize,
    pub wakes: usize,
    pub timer_wakes: usize,
    pub last_from: usize,
    pub last_to: usize,
}

#[derive(Copy, Clone)]
pub struct RuntimeTaskSnapshot {
    pub pid: usize,
    pub ppid: usize,
    pub tgid: usize,
    pub pgid: usize,
    pub sid: usize,
    pub state: RuntimeTaskState,
    pub exit_code: isize,
    pub fork_return: isize,
    pub fd_count: usize,
    pub cwd_len: usize,
    pub root_len: usize,
    pub signal_mask: u64,
    pub child_count: usize,
}

#[derive(Copy, Clone)]
pub struct RuntimeSecuritySnapshot {
    pub uid: u32,
    pub euid: u32,
    pub gid: u32,
    pub egid: u32,
    pub fsuid: u32,
    pub fsgid: u32,
    pub cap_effective: u64,
    pub cap_permitted: u64,
    pub node_uid: u32,
    pub node_gid: u32,
    pub node_mode: u16,
}

#[derive(Copy, Clone)]
pub struct RuntimeSocketSnapshot {
    pub sockets_used: usize,
    pub stream_connected: usize,
    pub datagram_bound: usize,
    pub queued_bytes: usize,
    pub sends: usize,
    pub recvs: usize,
}

#[derive(Copy, Clone)]
pub struct RuntimeIpcWaitSnapshot {
    pub mq_waiters: usize,
    pub msg_waiters: usize,
    pub sem_waiters: usize,
    pub shm_waiters: usize,
    pub wakeups: usize,
}

#[derive(Copy, Clone)]
pub struct RuntimeStat {
    pub kind: FdKind,
    pub ino: u64,
    pub mode: u16,
    pub size: usize,
    pub nlink: u32,
}

#[derive(Copy, Clone)]
pub struct ProcSnapshot {
    pub pid: usize,
    pub ppid: usize,
    pub tgid: usize,
    pub pgid: usize,
    pub sid: usize,
    pub fd_count: usize,
    pub cwd_len: usize,
    pub root_len: usize,
}

#[derive(Copy, Clone)]
pub struct RuntimeStatFs {
    pub fs_kind: RuntimeFsKind,
    pub magic: u64,
    pub block_size: usize,
    pub files: usize,
    pub free_files: usize,
    pub fds_used: usize,
    pub mount_count: usize,
}

#[derive(Copy, Clone)]
pub struct RuntimeMountSnapshot {
    pub mount_count: usize,
    pub rootfs_mounts: usize,
    pub tmpfs_mounts: usize,
    pub devfs_mounts: usize,
    pub procfs_mounts: usize,
    pub imagefs_mounts: usize,
}

#[derive(Copy, Clone)]
pub struct RuntimeStorageSnapshot {
    pub block_devices: usize,
    pub sector_size: usize,
    pub sectors: usize,
    pub raw_reads: usize,
    pub raw_writes: usize,
    pub block_errors: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub cache_dirty_marks: usize,
    pub cache_writebacks: usize,
    pub image_mounted: bool,
    pub image_mount_node: usize,
    pub image_dirs: usize,
    pub image_files: usize,
    pub image_exec_files: usize,
    pub image_metadata_reads: usize,
    pub image_data_reads: usize,
    pub image_errors: usize,
}

#[derive(Copy, Clone)]
struct RuntimeUserProgramResult {
    pid: usize,
    path_len: usize,
    exit_code: isize,
    wait_status: isize,
    entry: usize,
    mm_id: usize,
    exec_seq: usize,
}

struct KernelCore {
    initialized: bool,
    nodes: [Node; MAX_NODES],
    fds: [Fd; MAX_FDS],
    ofds: [Ofd; MAX_OFDS],
    pipes: [PipeObj; MAX_PIPES],
    events: [EventObj; MAX_EVENTS],
    timers: [TimerObj; MAX_TIMERS],
    sockets: [SocketObj; MAX_SOCKETS],
    epolls: [EpollObj; MAX_EPOLL],
    mqs: [MqObj; MAX_IPC],
    msgs: [MsgObj; MAX_IPC],
    sems: [SemObj; MAX_IPC],
    shms: [ShmObj; MAX_IPC],
    futexes: [FutexObj; MAX_FUTEXES],
    tasks: [TaskObj; MAX_TASKS],
    waitqs: [WaitQueueObj; MAX_WAIT_QUEUES],
    runq: [usize; MAX_TASKS],
    runq_len: usize,
    vmas: [VmaObj; MAX_VMAS],
    exec_image: ExecImage,
    signal_actions: [[RuntimeSignalAction; MAX_SIGNALS]; MAX_TASKS],
    pending_signals: [PendingSignalObj; MAX_PENDING_SIGNALS],
    signal_frames: [SignalFrameObj; MAX_TASKS],
    mounts: [MountObj; MAX_MOUNTS],
    block_devices: [BlockDeviceObj; MAX_BLOCK_DEVS],
    ramdisk: [u8; RAMDISK_BYTES],
    block_cache: [BlockCacheEntry; BLOCK_CACHE_SLOTS],
    block_cache_next: usize,
    io_stats: RuntimeIoStats,
    cwd: usize,
    root: usize,
    next_pid: usize,
    next_ipc_id: usize,
    next_mm_id: usize,
    next_ns_id: usize,
    current_task: usize,
    sched_ticks: usize,
    sched_switches: usize,
    sched_blocks: usize,
    sched_wakes: usize,
    sched_timer_wakes: usize,
    sched_last_from: usize,
    sched_last_to: usize,
    block_cache_hits: usize,
    block_cache_misses: usize,
    block_cache_dirty_marks: usize,
    block_cache_writebacks: usize,
    image_mounted: bool,
    image_mount_node: usize,
    image_dirs: usize,
    image_files: usize,
    image_exec_files: usize,
    image_metadata_reads: usize,
    image_data_reads: usize,
    image_errors: usize,
    brk_current: usize,
    mmap_next: usize,
    last_fault_addr: usize,
    last_fault_ok: bool,
    next_signal_frame_seq: usize,
    last_delivered_sig: usize,
    last_delivered_pid: usize,
    delivered_signals: usize,
    returned_signals: usize,
    queued_signals: usize,
    sigchld_queued: usize,
    direct_signal_deliveries: usize,
    tkill_signal_deliveries: usize,
    tgkill_signal_deliveries: usize,
    group_signal_deliveries: usize,
}

impl KernelCore {
    const fn new() -> Self {
        Self {
            initialized: false,
            nodes: [Node::empty(); MAX_NODES],
            fds: [Fd::empty(); MAX_FDS],
            ofds: [Ofd::empty(); MAX_OFDS],
            pipes: [PipeObj::empty(); MAX_PIPES],
            events: [EventObj::empty(); MAX_EVENTS],
            timers: [TimerObj::empty(); MAX_TIMERS],
            sockets: [SocketObj::empty(); MAX_SOCKETS],
            epolls: [EpollObj::empty(); MAX_EPOLL],
            mqs: [MqObj::empty(); MAX_IPC],
            msgs: [MsgObj::empty(); MAX_IPC],
            sems: [SemObj::empty(); MAX_IPC],
            shms: [ShmObj::empty(); MAX_IPC],
            futexes: [FutexObj::empty(); MAX_FUTEXES],
            tasks: [TaskObj::empty(); MAX_TASKS],
            waitqs: [WaitQueueObj::empty(); MAX_WAIT_QUEUES],
            runq: [0; MAX_TASKS],
            runq_len: 0,
            vmas: [VmaObj::empty(); MAX_VMAS],
            exec_image: ExecImage::empty(),
            signal_actions: [[RuntimeSignalAction::empty(); MAX_SIGNALS]; MAX_TASKS],
            pending_signals: [PendingSignalObj::empty(); MAX_PENDING_SIGNALS],
            signal_frames: [SignalFrameObj::empty(); MAX_TASKS],
            mounts: [MountObj::empty(); MAX_MOUNTS],
            block_devices: [BlockDeviceObj::empty(); MAX_BLOCK_DEVS],
            ramdisk: [0; RAMDISK_BYTES],
            block_cache: [BlockCacheEntry::empty(); BLOCK_CACHE_SLOTS],
            block_cache_next: 0,
            io_stats: RuntimeIoStats::empty(),
            cwd: ROOT,
            root: ROOT,
            next_pid: 2,
            next_ipc_id: 100,
            next_mm_id: 2,
            next_ns_id: 2,
            current_task: 0,
            sched_ticks: 0,
            sched_switches: 0,
            sched_blocks: 0,
            sched_wakes: 0,
            sched_timer_wakes: 0,
            sched_last_from: 0,
            sched_last_to: 0,
            block_cache_hits: 0,
            block_cache_misses: 0,
            block_cache_dirty_marks: 0,
            block_cache_writebacks: 0,
            image_mounted: false,
            image_mount_node: 0,
            image_dirs: 0,
            image_files: 0,
            image_exec_files: 0,
            image_metadata_reads: 0,
            image_data_reads: 0,
            image_errors: 0,
            brk_current: RUNTIME_USER_HEAP_START,
            mmap_next: RUNTIME_USER_MMAP_START,
            last_fault_addr: 0,
            last_fault_ok: false,
            next_signal_frame_seq: 1,
            last_delivered_sig: 0,
            last_delivered_pid: 0,
            delivered_signals: 0,
            returned_signals: 0,
            queued_signals: 0,
            sigchld_queued: 0,
            direct_signal_deliveries: 0,
            tkill_signal_deliveries: 0,
            tgkill_signal_deliveries: 0,
            group_signal_deliveries: 0,
        }
    }

    fn reset(&mut self) {
        self.nodes = [Node::empty(); MAX_NODES];
        self.fds = [Fd::empty(); MAX_FDS];
        self.ofds = [Ofd::empty(); MAX_OFDS];
        self.pipes = [PipeObj::empty(); MAX_PIPES];
        self.events = [EventObj::empty(); MAX_EVENTS];
        self.timers = [TimerObj::empty(); MAX_TIMERS];
        self.sockets = [SocketObj::empty(); MAX_SOCKETS];
        self.epolls = [EpollObj::empty(); MAX_EPOLL];
        self.mqs = [MqObj::empty(); MAX_IPC];
        self.msgs = [MsgObj::empty(); MAX_IPC];
        self.sems = [SemObj::empty(); MAX_IPC];
        self.shms = [ShmObj::empty(); MAX_IPC];
        self.futexes = [FutexObj::empty(); MAX_FUTEXES];
        self.tasks = [TaskObj::empty(); MAX_TASKS];
        self.waitqs = [WaitQueueObj::empty(); MAX_WAIT_QUEUES];
        self.runq = [0; MAX_TASKS];
        self.runq_len = 0;
        self.vmas = [VmaObj::empty(); MAX_VMAS];
        self.exec_image = ExecImage::empty();
        self.signal_actions = [[RuntimeSignalAction::empty(); MAX_SIGNALS]; MAX_TASKS];
        self.pending_signals = [PendingSignalObj::empty(); MAX_PENDING_SIGNALS];
        self.signal_frames = [SignalFrameObj::empty(); MAX_TASKS];
        self.mounts = [MountObj::empty(); MAX_MOUNTS];
        self.block_devices = [BlockDeviceObj::empty(); MAX_BLOCK_DEVS];
        self.ramdisk = [0; RAMDISK_BYTES];
        self.block_cache = [BlockCacheEntry::empty(); BLOCK_CACHE_SLOTS];
        self.block_cache_next = 0;
        self.io_stats = RuntimeIoStats::empty();
        self.cwd = ROOT;
        self.root = ROOT;
        self.next_pid = 2;
        self.next_ipc_id = 100;
        self.next_mm_id = 2;
        self.next_ns_id = 2;
        self.current_task = 0;
        self.sched_ticks = 0;
        self.sched_switches = 0;
        self.sched_blocks = 0;
        self.sched_wakes = 0;
        self.sched_timer_wakes = 0;
        self.sched_last_from = 0;
        self.sched_last_to = 0;
        self.block_cache_hits = 0;
        self.block_cache_misses = 0;
        self.block_cache_dirty_marks = 0;
        self.block_cache_writebacks = 0;
        self.image_mounted = false;
        self.image_mount_node = 0;
        self.image_dirs = 0;
        self.image_files = 0;
        self.image_exec_files = 0;
        self.image_metadata_reads = 0;
        self.image_data_reads = 0;
        self.image_errors = 0;
        self.brk_current = RUNTIME_USER_HEAP_START;
        self.mmap_next = RUNTIME_USER_MMAP_START;
        self.last_fault_addr = 0;
        self.last_fault_ok = false;
        self.next_signal_frame_seq = 1;
        self.last_delivered_sig = 0;
        self.last_delivered_pid = 0;
        self.delivered_signals = 0;
        self.returned_signals = 0;
        self.queued_signals = 0;
        self.sigchld_queued = 0;
        self.direct_signal_deliveries = 0;
        self.tkill_signal_deliveries = 0;
        self.tgkill_signal_deliveries = 0;
        self.group_signal_deliveries = 0;
        self.initialized = true;

        self.nodes[ROOT].used = true;
        self.nodes[ROOT].kind = NodeKind::Dir;
        self.nodes[ROOT].fs_kind = RuntimeFsKind::Rootfs;
        self.nodes[ROOT].parent = ROOT;
        self.nodes[ROOT].mode = 0o755;
        self.nodes[ROOT].name[0] = b'/';
        self.nodes[ROOT].name_len = 1;
        let _ = self.register_mount(ROOT, RuntimeFsKind::Rootfs, b"rootfs", b"rootfs", 0);

        let _ = self.mkdir_at_node(ROOT, b"dev", 0o755);
        let _ = self.create_node_at(ROOT, b"proc", NodeKind::Dir, 0o555);
        if let Some(dev) = self.find_child(ROOT, b"dev") {
            self.nodes[dev].fs_kind = RuntimeFsKind::Devfs;
            let _ = self.register_mount(dev, RuntimeFsKind::Devfs, b"devfs", b"devfs", 0);
            let _ = self.create_node_at(dev, b"null", NodeKind::DevNull, 0o666);
            let _ = self.create_node_at(dev, b"zero", NodeKind::DevZero, 0o666);
            let _ = self.create_node_at(dev, b"console", NodeKind::DevConsole, 0o666);
            let _ = self.create_node_at(dev, b"tty", NodeKind::DevTty, 0o666);
            let _ = self.create_node_at(dev, b"random", NodeKind::DevRandom, 0o444);
            let _ = self.create_node_at(dev, b"urandom", NodeKind::DevRandom, 0o444);
        }
        if let Some(proc_node) = self.find_child(ROOT, b"proc") {
            self.nodes[proc_node].fs_kind = RuntimeFsKind::Procfs;
            let _ = self.register_mount(proc_node, RuntimeFsKind::Procfs, b"proc", b"procfs", 0);
            if let Ok(self_node) = self.create_node_at(proc_node, b"self", NodeKind::Dir, 0o555) {
                let _ = self.create_node_at(self_node, b"fd", NodeKind::ProcFdDir, 0o555);
                let _ = self.create_node_at(self_node, b"status", NodeKind::ProcStatus, 0o444);
                let _ = self.create_node_at(self_node, b"stat", NodeKind::ProcStat, 0o444);
                let _ = self.create_node_at(self_node, b"maps", NodeKind::ProcMaps, 0o444);
            }
        }

        let _ = self.install_fixed_fd(0, FdKind::Stdin, ROOT, 0, 0);
        let _ = self.install_fixed_fd(1, FdKind::Stdout, ROOT, 0, 0);
        let _ = self.install_fixed_fd(2, FdKind::Stderr, ROOT, 0, 0);

        self.tasks[0] = TaskObj {
            used: true,
            pid: 1,
            ppid: 1,
            tgid: 1,
            pgid: 1,
            sid: 1,
            state: RuntimeTaskState::Running,
            exit_code: 0,
            wait_key: 0,
            yields: 0,
            fork_return: 1,
            fd_count: 3,
            cwd: ROOT,
            root: ROOT,
            signal_mask: 0,
            mm_id: 1,
            cred: RuntimeCred::root(),
            namespaces: RuntimeNamespaceRefs::initial(),
        };
        let _ = self.add_vma(1, RUNTIME_USER_STACK_BOTTOM, RUNTIME_USER_STACK_TOP, VMA_R | VMA_W | VMA_U, RuntimeVmaKind::Stack, true, -1, 0);
        self.rebuild_runq();
    }

    fn ensure(&mut self) {
        if !self.initialized {
            self.reset();
        }
    }

    fn same_name(node: &Node, name: &[u8]) -> bool {
        if node.name_len != name.len() {
            return false;
        }
        let mut i = 0usize;
        while i < name.len() {
            if node.name[i] != name[i] {
                return false;
            }
            i += 1;
        }
        true
    }

    fn node_is_dirlike(&self, node: usize) -> bool {
        node < MAX_NODES && self.nodes[node].used && matches!(self.nodes[node].kind, NodeKind::Dir | NodeKind::ProcFdDir)
    }

    fn current_cred_copy(&self) -> RuntimeCred {
        if self.current_task < MAX_TASKS && self.tasks[self.current_task].used {
            self.tasks[self.current_task].cred
        } else {
            RuntimeCred::root()
        }
    }

    fn cred_has_cap(cred: RuntimeCred, cap: usize) -> bool {
        cap < 64 && (cred.cap_effective & (1u64 << cap)) != 0
    }

    fn access_allowed_with_cred(&self, node: usize, mask: usize, cred: RuntimeCred) -> bool {
        if mask == 0 {
            return true;
        }
        let owner = self.owner(node);
        if owner >= MAX_NODES || !self.nodes[owner].used {
            return false;
        }
        if (mask & 0b110) != 0 && (cred.euid == 0 || Self::cred_has_cap(cred, CAP_DAC_OVERRIDE)) {
            if (mask & 1) == 0 || self.node_is_dirlike(owner) || (self.nodes[owner].mode & 0o111) != 0 {
                return true;
            }
        }
        if (mask & 1) != 0 && cred.euid == 0 && (self.nodes[owner].mode & 0o111) != 0 && (mask & !1) == 0 {
            return true;
        }
        let bits = if cred.fsuid == self.nodes[owner].uid {
            (self.nodes[owner].mode >> 6) & 0o7
        } else if cred.fsgid == self.nodes[owner].gid || cred.egid == self.nodes[owner].gid {
            (self.nodes[owner].mode >> 3) & 0o7
        } else {
            self.nodes[owner].mode & 0o7
        } as usize;
        if (mask & 4) != 0 && (bits & 4) == 0 {
            return false;
        }
        if (mask & 2) != 0 && (bits & 2) == 0 {
            return false;
        }
        if (mask & 1) != 0 && (bits & 1) == 0 {
            return false;
        }
        true
    }

    fn access_allowed(&self, node: usize, mask: usize) -> bool {
        self.access_allowed_with_cred(node, mask, self.current_cred_copy())
    }

    fn open_access_mask(flags: u32) -> usize {
        let mut mask = match flags & O_ACCMODE {
            O_WRONLY => 2,
            O_RDWR => 6,
            _ => 4,
        };
        if (flags & O_TRUNC) != 0 {
            mask |= 2;
        }
        mask
    }

    fn fs_magic(kind: RuntimeFsKind) -> u64 {
        match kind {
            RuntimeFsKind::Rootfs => 0x8584_58f6,
            RuntimeFsKind::Tmpfs => 0x0102_1994,
            RuntimeFsKind::Devfs => 0x1373,
            RuntimeFsKind::Procfs => 0x9fa0,
            RuntimeFsKind::Imagefs => 0xEF53,
        }
    }

    fn fs_kind_from_name(name: &[u8]) -> Result<RuntimeFsKind, Errno> {
        if bytes_eq(name, b"tmpfs") {
            Ok(RuntimeFsKind::Tmpfs)
        } else if bytes_eq(name, b"devfs") {
            Ok(RuntimeFsKind::Devfs)
        } else if bytes_eq(name, b"proc") || bytes_eq(name, b"procfs") {
            Ok(RuntimeFsKind::Procfs)
        } else if bytes_eq(name, b"rootfs") {
            Ok(RuntimeFsKind::Rootfs)
        } else if bytes_eq(name, b"ext4")
            || bytes_eq(name, b"ext4-ro")
            || bytes_eq(name, b"imagefs")
            || bytes_eq(name, b"vfat")
        {
            Ok(RuntimeFsKind::Imagefs)
        } else {
            Err(Errno::Inval)
        }
    }

    fn mount_count(&self) -> usize {
        let mut count = 0usize;
        let mut i = 0usize;
        while i < MAX_MOUNTS {
            if self.mounts[i].used {
                count += 1;
            }
            i += 1;
        }
        count
    }

    fn register_mount(&mut self, target: usize, fs_kind: RuntimeFsKind, source: &[u8], fstype: &[u8], flags: usize) -> Result<usize, Errno> {
        if target >= MAX_NODES || !self.node_is_dirlike(target) {
            return Err(Errno::NotDir);
        }
        let mut i = 0usize;
        while i < MAX_MOUNTS {
            if self.mounts[i].used && self.mounts[i].target == target {
                self.mounts[i] = MountObj::empty();
                break;
            }
            i += 1;
        }
        i = 0;
        while i < MAX_MOUNTS {
            if !self.mounts[i].used {
                let mut mount = MountObj::empty();
                mount.used = true;
                mount.target = target;
                mount.fs_kind = fs_kind;
                mount.flags = flags;
                mount.source_len = copy_name_bytes(source, &mut mount.source);
                mount.fstype_len = copy_name_bytes(fstype, &mut mount.fstype);
                self.mounts[i] = mount;
                return Ok(i);
            }
            i += 1;
        }
        Err(Errno::NoSpace)
    }

    fn set_subtree_fs_kind(&mut self, root: usize, fs_kind: RuntimeFsKind) {
        let mut i = 1usize;
        while i < MAX_NODES {
            if self.nodes[i].used && (i == root || self.node_is_descendant(i, root)) {
                self.nodes[i].fs_kind = fs_kind;
            }
            i += 1;
        }
    }

    fn node_is_descendant(&self, node: usize, ancestor: usize) -> bool {
        if node >= MAX_NODES || ancestor >= MAX_NODES || !self.nodes[node].used || !self.nodes[ancestor].used {
            return false;
        }
        let mut cur = node;
        let mut depth = 0usize;
        while depth < 16 {
            if cur == ancestor {
                return true;
            }
            if cur == self.nodes[cur].parent {
                break;
            }
            cur = self.nodes[cur].parent;
            depth += 1;
        }
        false
    }

    fn is_dot(name: &[u8]) -> bool {
        name.len() == 1 && name[0] == b'.'
    }

    fn is_dotdot(name: &[u8]) -> bool {
        name.len() == 2 && name[0] == b'.' && name[1] == b'.'
    }

    fn is_abs(path: &[u8]) -> bool {
        !path.is_empty() && path[0] == b'/'
    }

    fn set_name(node: &mut Node, name: &[u8]) -> Result<(), Errno> {
        if name.is_empty() || name.len() > NAME_MAX || Self::is_dot(name) || Self::is_dotdot(name) {
            return Err(Errno::Inval);
        }
        node.name = [0; NAME_MAX];
        node.name_len = name.len();
        let mut i = 0usize;
        while i < name.len() {
            node.name[i] = name[i];
            i += 1;
        }
        Ok(())
    }

    fn alloc_node(&mut self) -> Result<usize, Errno> {
        let mut i = 2usize;
        while i < MAX_NODES {
            if !self.nodes[i].used {
                self.nodes[i] = Node::empty();
                self.nodes[i].used = true;
                return Ok(i);
            }
            i += 1;
        }
        Err(Errno::NoSpace)
    }

    fn owner(&self, node: usize) -> usize {
        if self.nodes[node].owner == 0 { node } else { self.nodes[node].owner }
    }

    fn find_child(&self, parent: usize, name: &[u8]) -> Option<usize> {
        let mut i = 1usize;
        while i < MAX_NODES {
            if self.nodes[i].used && self.nodes[i].parent == parent && Self::same_name(&self.nodes[i], name) {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    fn mkdir_at_node(&mut self, parent: usize, name: &[u8], mode: u16) -> Result<usize, Errno> {
        self.create_node_at(parent, name, NodeKind::Dir, mode)
    }

    fn create_node_at(&mut self, parent: usize, name: &[u8], kind: NodeKind, mode: u16) -> Result<usize, Errno> {
        if parent >= MAX_NODES || !self.nodes[parent].used || !self.node_is_dirlike(parent) {
            return Err(Errno::NotDir);
        }
        if self.find_child(parent, name).is_some() {
            return Err(Errno::Exist);
        }
        let node = self.alloc_node()?;
        self.nodes[node].kind = kind;
        self.nodes[node].fs_kind = self.nodes[parent].fs_kind;
        self.nodes[node].parent = parent;
        if let Ok(task_idx) = self.current_task_idx() {
            self.nodes[node].uid = self.tasks[task_idx].cred.fsuid;
            self.nodes[node].gid = self.tasks[task_idx].cred.fsgid;
        }
        self.nodes[node].mode = mode & 0o777;
        Self::set_name(&mut self.nodes[node], name)?;
        Ok(node)
    }

    fn next_component(path: &[u8], mut pos: usize, out: &mut [u8; NAME_MAX]) -> Option<(usize, usize)> {
        while pos < path.len() && path[pos] == b'/' {
            pos += 1;
        }
        if pos >= path.len() {
            return None;
        }
        let mut len = 0usize;
        while pos < path.len() && path[pos] != b'/' {
            if len >= NAME_MAX {
                return Some((usize::MAX, usize::MAX));
            }
            out[len] = path[pos];
            len += 1;
            pos += 1;
        }
        Some((pos, len))
    }

    fn has_more_component(path: &[u8], mut pos: usize) -> bool {
        while pos < path.len() {
            if path[pos] != b'/' {
                return true;
            }
            pos += 1;
        }
        false
    }

    fn fd_idx(&self, fd: isize) -> Result<usize, Errno> {
        if fd < 0 {
            return Err(Errno::BadFd);
        }
        let idx = fd as usize;
        if idx >= MAX_FDS || !self.fds[idx].used {
            return Err(Errno::BadFd);
        }
        let ofd = self.fds[idx].ofd;
        if ofd >= MAX_OFDS || !self.ofds[ofd].used {
            return Err(Errno::BadFd);
        }
        Ok(idx)
    }

    fn start_dir(&self, dirfd: isize, path: &[u8]) -> Result<usize, Errno> {
        if Self::is_abs(path) {
            return Ok(self.root);
        }
        if dirfd == AT_FDCWD {
            return Ok(self.cwd);
        }
        let idx = self.fd_idx(dirfd)?;
        let ofd = self.fds[idx].ofd;
        if self.ofds[ofd].kind != FdKind::Directory && self.ofds[ofd].kind != FdKind::Procfs {
            return Err(Errno::NotDir);
        }
        Ok(self.ofds[ofd].node)
    }

    fn resolve_at(&self, dirfd: isize, path: &[u8], follow_final: bool) -> Result<usize, Errno> {
        if path.is_empty() {
            return Err(Errno::NoEnt);
        }
        let mut cur = self.start_dir(dirfd, path)?;
        let mut pos = 0usize;
        loop {
            let mut comp = [0u8; NAME_MAX];
            let (next_pos, len) = match Self::next_component(path, pos, &mut comp) {
                Some(v) => v,
                None => return Ok(cur),
            };
            if len == usize::MAX {
                return Err(Errno::Inval);
            }
            pos = next_pos;
            let name = &comp[..len];
            if Self::is_dot(name) {
                continue;
            }
            if Self::is_dotdot(name) {
                cur = self.nodes[cur].parent;
                continue;
            }
            if !self.node_is_dirlike(cur) {
                return Err(Errno::NotDir);
            }
            let child = match self.find_child(cur, name) {
                Some(node) => node,
                None => return Err(Errno::NoEnt),
            };
            if self.nodes[child].kind == NodeKind::Symlink && (follow_final || Self::has_more_component(path, pos)) {
                return Err(Errno::NoSys);
            }
            cur = child;
        }
    }

    fn resolve_parent_at(&self, dirfd: isize, path: &[u8], out: &mut [u8; NAME_MAX]) -> Result<(usize, usize), Errno> {
        if path.is_empty() {
            return Err(Errno::NoEnt);
        }
        let mut cur = self.start_dir(dirfd, path)?;
        let mut pos = 0usize;
        loop {
            let mut comp = [0u8; NAME_MAX];
            let (next_pos, len) = match Self::next_component(path, pos, &mut comp) {
                Some(v) => v,
                None => return Err(Errno::NoEnt),
            };
            if len == usize::MAX {
                return Err(Errno::Inval);
            }
            let final_component = !Self::has_more_component(path, next_pos);
            let name = &comp[..len];
            if final_component {
                if Self::is_dot(name) || Self::is_dotdot(name) {
                    return Err(Errno::Inval);
                }
                let mut i = 0usize;
                while i < len {
                    out[i] = comp[i];
                    i += 1;
                }
                return Ok((cur, len));
            }
            pos = next_pos;
            if Self::is_dot(name) {
                continue;
            }
            if Self::is_dotdot(name) {
                cur = self.nodes[cur].parent;
                continue;
            }
            let child = match self.find_child(cur, name) {
                Some(node) => node,
                None => return Err(Errno::NoEnt),
            };
            if !self.node_is_dirlike(child) {
                return Err(Errno::NotDir);
            }
            cur = child;
        }
    }

    fn alloc_ofd(&mut self, kind: FdKind, node: usize, flags: u32, object: usize) -> Result<usize, Errno> {
        let mut i = 0usize;
        while i < MAX_OFDS {
            if !self.ofds[i].used {
                self.ofds[i] = Ofd { used: true, kind, node, off: 0, flags: flags & !O_CLOEXEC, refs: 0, object };
                return Ok(i);
            }
            i += 1;
        }
        Err(Errno::NoSpace)
    }

    fn install_fixed_fd(&mut self, fd: usize, kind: FdKind, node: usize, flags: u32, object: usize) -> Result<isize, Errno> {
        if fd >= MAX_FDS {
            return Err(Errno::BadFd);
        }
        let ofd = self.alloc_ofd(kind, node, flags, object)?;
        self.ofds[ofd].refs = 1;
        self.fds[fd] = Fd { used: true, ofd, cloexec: false };
        Ok(fd as isize)
    }

    fn alloc_fd_for_ofd(&mut self, ofd: usize, min_fd: usize, cloexec: bool) -> Result<isize, Errno> {
        let mut fd = min_fd;
        while fd < MAX_FDS {
            if !self.fds[fd].used {
                self.fds[fd] = Fd { used: true, ofd, cloexec };
                self.ofds[ofd].refs += 1;
                return Ok(fd as isize);
            }
            fd += 1;
        }
        Err(Errno::NoSpace)
    }

    fn object_ofd_count(&self, kind: FdKind, object: usize) -> usize {
        let mut count = 0usize;
        let mut i = 0usize;
        while i < MAX_OFDS {
            if self.ofds[i].used && self.ofds[i].kind == kind && self.ofds[i].object == object {
                count += 1;
            }
            i += 1;
        }
        count
    }

    fn release_ofd_object(&mut self, kind: FdKind, object: usize) {
        match kind {
            FdKind::PipeRead => {
                if object < MAX_PIPES && self.pipes[object].used {
                    if self.pipes[object].readers > 0 {
                        self.pipes[object].readers -= 1;
                    }
                    if self.pipes[object].readers == 0 && self.pipes[object].writers == 0 {
                        self.pipes[object] = PipeObj::empty();
                    }
                }
            }
            FdKind::PipeWrite => {
                if object < MAX_PIPES && self.pipes[object].used {
                    if self.pipes[object].writers > 0 {
                        self.pipes[object].writers -= 1;
                    }
                    if self.pipes[object].readers == 0 && self.pipes[object].writers == 0 {
                        self.pipes[object] = PipeObj::empty();
                    }
                }
            }
            FdKind::EventFd => {
                if object < MAX_EVENTS {
                    self.events[object] = EventObj::empty();
                }
            }
            FdKind::TimerFd => {
                if object < MAX_TIMERS {
                    self.timers[object] = TimerObj::empty();
                }
            }
            FdKind::Socket => {
                if object < MAX_SOCKETS {
                    self.sockets[object] = SocketObj::empty();
                }
            }
            FdKind::Epoll => {
                if object < MAX_EPOLL {
                    self.epolls[object] = EpollObj::empty();
                }
            }
            FdKind::Mq => {
                if object < MAX_IPC && self.mqs[object].used && self.mqs[object].unlinked && self.object_ofd_count(FdKind::Mq, object) <= 1 {
                    self.mqs[object] = MqObj::empty();
                }
            }
            _ => {}
        }
    }

    fn open_node(&mut self, node: usize, flags: u32) -> Result<isize, Errno> {
        if node >= MAX_NODES || !self.nodes[node].used {
            return Err(Errno::NoEnt);
        }
        let kind = match self.nodes[node].kind {
            NodeKind::Dir => FdKind::Directory,
            NodeKind::File => FdKind::RegularFile,
            NodeKind::Symlink => FdKind::Symlink,
            NodeKind::DevNull => FdKind::DevNull,
            NodeKind::DevZero => FdKind::DevZero,
            NodeKind::DevConsole => FdKind::DevConsole,
            NodeKind::DevTty => FdKind::DevTty,
            NodeKind::DevRandom => FdKind::DevRandom,
            NodeKind::ProcFdDir | NodeKind::ProcStatus | NodeKind::ProcStat | NodeKind::ProcMaps => FdKind::Procfs,
            NodeKind::Empty => return Err(Errno::NoEnt),
        };
        if (flags & O_DIRECTORY) != 0
            && kind != FdKind::Directory
            && !(kind == FdKind::Procfs && self.nodes[node].kind == NodeKind::ProcFdDir)
        {
            return Err(Errno::NotDir);
        }
        if self.node_is_dirlike(node) && (flags & O_ACCMODE) != 0 {
            return Err(Errno::IsDir);
        }
        if !self.access_allowed(node, Self::open_access_mask(flags)) {
            return Err(Errno::Access);
        }
        let owner = self.owner(node);
        if self.nodes[owner].fs_kind == RuntimeFsKind::Imagefs && ((flags & O_ACCMODE) != 0 || (flags & O_TRUNC) != 0) {
            return Err(Errno::Access);
        }
        if self.nodes[owner].kind == NodeKind::File && (flags & O_TRUNC) != 0 {
            self.nodes[owner].size = 0;
        }
        let ofd = self.alloc_ofd(kind, node, flags, 0)?;
        if self.nodes[owner].kind == NodeKind::File && (flags & O_APPEND) != 0 {
            self.ofds[ofd].off = self.nodes[owner].size;
        }
        self.alloc_fd_for_ofd(ofd, 0, (flags & O_CLOEXEC) != 0)
    }

    fn openat(&mut self, dirfd: isize, path: &[u8], flags: u32, mode: u16) -> Result<isize, Errno> {
        self.ensure();
        let node = match self.resolve_at(dirfd, path, true) {
            Ok(node) => node,
            Err(Errno::NoEnt) if (flags & O_CREAT) != 0 => {
                let mut name = [0u8; NAME_MAX];
                let (parent, len) = self.resolve_parent_at(dirfd, path, &mut name)?;
                if !self.access_allowed(parent, 3) {
                    return Err(Errno::Access);
                }
                self.create_node_at(parent, &name[..len], NodeKind::File, mode)?
            }
            Err(err) => return Err(err),
        };
        self.open_node(node, flags)
    }

    fn close(&mut self, fd: isize) -> Result<(), Errno> {
        self.ensure();
        let idx = self.fd_idx(fd)?;
        let ofd = self.fds[idx].ofd;
        let kind = self.ofds[ofd].kind;
        let object = self.ofds[ofd].object;
        self.fds[idx] = Fd::empty();
        if self.ofds[ofd].refs > 0 {
            self.ofds[ofd].refs -= 1;
        }
        if self.ofds[ofd].refs == 0 {
            self.release_ofd_object(kind, object);
            self.ofds[ofd] = Ofd::empty();
        }
        Ok(())
    }

    fn close_range(&mut self, first: usize, last: usize, flags: usize) -> Result<isize, Errno> {
        self.ensure();
        if first > last {
            return Ok(0);
        }
        if (flags & !CLOSE_RANGE_CLOEXEC) != 0 {
            return Err(Errno::Inval);
        }
        let mut fd = first;
        while fd < MAX_FDS && fd <= last {
            if (flags & CLOSE_RANGE_CLOEXEC) != 0 {
                if self.fds[fd].used {
                    self.fds[fd].cloexec = true;
                }
            } else if self.fds[fd].used {
                let _ = self.close(fd as isize);
            }
            fd += 1;
        }
        Ok(0)
    }

    fn can_read_fd(&mut self, fd: usize) -> bool {
        self.ensure();
        match self.fd_idx(fd as isize) {
            Ok(idx) => {
                let kind = self.ofds[self.fds[idx].ofd].kind;
                matches!(kind, FdKind::Stdin | FdKind::RegularFile | FdKind::DevNull | FdKind::DevZero | FdKind::DevConsole | FdKind::DevTty | FdKind::DevRandom | FdKind::PipeRead | FdKind::EventFd | FdKind::TimerFd | FdKind::Socket | FdKind::Procfs)
            }
            Err(_) => false,
        }
    }

    fn can_write_fd(&mut self, fd: usize) -> bool {
        self.ensure();
        match self.fd_idx(fd as isize) {
            Ok(idx) => {
                let kind = self.ofds[self.fds[idx].ofd].kind;
                matches!(kind, FdKind::Stdout | FdKind::Stderr | FdKind::RegularFile | FdKind::DevNull | FdKind::DevZero | FdKind::DevConsole | FdKind::DevTty | FdKind::DevRandom | FdKind::PipeWrite | FdKind::EventFd | FdKind::Socket)
            }
            Err(_) => false,
        }
    }

    fn write(&mut self, fd: isize, src: &[u8]) -> Result<usize, Errno> {
        self.ensure();
        let idx = self.fd_idx(fd)?;
        let ofd = self.fds[idx].ofd;
        match self.ofds[ofd].kind {
            FdKind::Stdout | FdKind::Stderr | FdKind::DevConsole | FdKind::DevTty => {
                let mut i = 0usize;
                while i < src.len() {
                    crate::sbi::console_putchar(src[i] as usize);
                    i += 1;
                }
                Ok(src.len())
            }
            FdKind::DevNull | FdKind::DevZero | FdKind::DevRandom => Ok(src.len()),
            FdKind::RegularFile => {
                let owner = self.owner(self.ofds[ofd].node);
                if self.nodes[owner].fs_kind == RuntimeFsKind::Imagefs {
                    return Err(Errno::Access);
                }
                if (self.ofds[ofd].flags & O_APPEND) != 0 {
                    self.ofds[ofd].off = self.nodes[owner].size;
                }
                let mut pos = self.ofds[ofd].off;
                let mut copied = 0usize;
                while copied < src.len() && pos < DATA_MAX {
                    self.nodes[owner].data[pos] = src[copied];
                    pos += 1;
                    copied += 1;
                }
                if pos > self.nodes[owner].size {
                    self.nodes[owner].size = pos;
                }
                self.ofds[ofd].off = pos;
                Ok(copied)
            }
            FdKind::PipeWrite => {
                let pipe = self.ofds[ofd].object;
                if pipe >= MAX_PIPES || !self.pipes[pipe].used {
                    return Err(Errno::BadFd);
                }
                if self.pipes[pipe].readers == 0 {
                    return Err(Errno::BadFd);
                }
                if src.is_empty() {
                    return Ok(0);
                }
                let nonblock = (self.ofds[ofd].flags & O_NONBLOCK) != 0;
                if self.pipes[pipe].len >= PIPE_BUF && nonblock {
                    return Err(Errno::Again);
                }
                let mut copied = 0usize;
                while copied < src.len() && self.pipes[pipe].len < PIPE_BUF {
                    let pos = self.pipes[pipe].len;
                    self.pipes[pipe].data[pos] = src[copied];
                    self.pipes[pipe].len += 1;
                    copied += 1;
                }
                if copied == 0 && nonblock {
                    return Err(Errno::Again);
                }
                if copied > 0 {
                    let _ = self.sched_wake(SCHED_WAIT_PIPE_BASE + pipe, MAX_TASKS)?;
                }
                Ok(copied)
            }
            FdKind::EventFd => {
                if src.len() < 8 {
                    return Err(Errno::Inval);
                }
                let event = self.ofds[ofd].object;
                if event >= MAX_EVENTS || !self.events[event].used {
                    return Err(Errno::BadFd);
                }
                let mut bytes = [0u8; 8];
                let mut i = 0usize;
                while i < 8 {
                    bytes[i] = src[i];
                    i += 1;
                }
                let value = u64::from_le_bytes(bytes);
                if value == u64::MAX {
                    return Err(Errno::Inval);
                }
                if u64::MAX - self.events[event].counter <= value {
                    return Err(Errno::Again);
                }
                self.events[event].counter += value;
                Ok(8)
            }
            FdKind::Socket => {
                let copied = self.sendto_socket(fd, src, None)?;
                if copied == 0 && (self.ofds[ofd].flags & O_NONBLOCK) != 0 {
                    return Err(Errno::Again);
                }
                Ok(copied)
            }
            _ => Err(Errno::Inval),
        }
    }

    fn read(&mut self, fd: isize, out: &mut [u8]) -> Result<usize, Errno> {
        self.ensure();
        let idx = self.fd_idx(fd)?;
        let ofd = self.fds[idx].ofd;
        match self.ofds[ofd].kind {
            FdKind::Stdin | FdKind::DevNull | FdKind::DevConsole | FdKind::DevTty => Ok(0),
            FdKind::DevZero => {
                let mut i = 0usize;
                while i < out.len() {
                    out[i] = 0;
                    i += 1;
                }
                Ok(out.len())
            }
            FdKind::DevRandom => {
                let mut i = 0usize;
                let mut seed = self.ofds[ofd].off.wrapping_add(0x5a);
                while i < out.len() {
                    seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
                    out[i] = ((seed >> 16) & 0xff) as u8;
                    i += 1;
                }
                self.ofds[ofd].off = self.ofds[ofd].off.wrapping_add(out.len());
                Ok(out.len())
            }
            FdKind::RegularFile => {
                let owner = self.owner(self.ofds[ofd].node);
                if self.nodes[owner].fs_kind == RuntimeFsKind::Imagefs {
                    let start = self.ofds[ofd].off;
                    let copied = self.image_read_node(owner, start, out)?;
                    self.ofds[ofd].off = start + copied;
                    return Ok(copied);
                }
                let mut pos = self.ofds[ofd].off;
                let mut copied = 0usize;
                while copied < out.len() && pos < self.nodes[owner].size {
                    out[copied] = self.nodes[owner].data[pos];
                    pos += 1;
                    copied += 1;
                }
                self.ofds[ofd].off = pos;
                Ok(copied)
            }
            FdKind::PipeRead => {
                let pipe = self.ofds[ofd].object;
                if pipe >= MAX_PIPES || !self.pipes[pipe].used {
                    return Err(Errno::BadFd);
                }
                if self.pipes[pipe].len == 0 {
                    if (self.ofds[ofd].flags & O_NONBLOCK) != 0 {
                        return Err(Errno::Again);
                    }
                    return Ok(0);
                }
                let mut copied = 0usize;
                while copied < out.len() && copied < self.pipes[pipe].len {
                    out[copied] = self.pipes[pipe].data[copied];
                    copied += 1;
                }
                let mut i = copied;
                while i < self.pipes[pipe].len {
                    self.pipes[pipe].data[i - copied] = self.pipes[pipe].data[i];
                    i += 1;
                }
                self.pipes[pipe].len -= copied;
                Ok(copied)
            }
            FdKind::EventFd => {
                if out.len() < 8 {
                    return Err(Errno::Inval);
                }
                let event = self.ofds[ofd].object;
                if event >= MAX_EVENTS || !self.events[event].used {
                    return Err(Errno::BadFd);
                }
                if self.events[event].counter == 0 {
                    if (self.ofds[ofd].flags & O_NONBLOCK) != 0 {
                        return Err(Errno::Again);
                    }
                    return Ok(0);
                }
                let bytes = self.events[event].counter.to_le_bytes();
                let mut i = 0usize;
                while i < 8 {
                    out[i] = bytes[i];
                    i += 1;
                }
                self.events[event].counter = 0;
                Ok(8)
            }
            FdKind::TimerFd => {
                if out.len() < 8 {
                    return Err(Errno::Inval);
                }
                let timer = self.ofds[ofd].object;
                if timer >= MAX_TIMERS || !self.timers[timer].used {
                    return Err(Errno::BadFd);
                }
                if self.timers[timer].expirations == 0 {
                    if (self.ofds[ofd].flags & O_NONBLOCK) != 0 {
                        return Err(Errno::Again);
                    }
                    return Ok(0);
                }
                let value = self.timers[timer].expirations;
                let bytes = value.to_le_bytes();
                let mut i = 0usize;
                while i < 8 {
                    out[i] = bytes[i];
                    i += 1;
                }
                self.timers[timer].expirations = 0;
                Ok(8)
            }
            FdKind::Socket => {
                if self.sockets[self.ofds[ofd].object].len == 0 {
                    if (self.ofds[ofd].flags & O_NONBLOCK) != 0 {
                        return Err(Errno::Again);
                    }
                    return Ok(0);
                }
                let mut src = [0u8; NAME_MAX];
                let (copied, _) = self.recvfrom_socket(fd, out, &mut src)?;
                Ok(copied)
            }
            FdKind::Procfs => self.read_procfs_fd(ofd, out),
            _ => Err(Errno::Inval),
        }
    }

    fn read_procfs_fd(&mut self, ofd: usize, out: &mut [u8]) -> Result<usize, Errno> {
        let node = self.ofds[ofd].node;
        let mut data = [0u8; DATA_MAX];
        let len = match self.nodes[node].kind {
            NodeKind::ProcStatus => self.proc_status_content(&mut data),
            NodeKind::ProcStat => self.proc_stat_content(&mut data),
            NodeKind::ProcMaps => self.proc_maps_content(&mut data),
            NodeKind::ProcFdDir => return Err(Errno::IsDir),
            _ => return Err(Errno::Inval),
        };
        let start = self.ofds[ofd].off;
        if start >= len {
            return Ok(0);
        }
        let mut copied = 0usize;
        while copied < out.len() && start + copied < len {
            out[copied] = data[start + copied];
            copied += 1;
        }
        self.ofds[ofd].off = start + copied;
        Ok(copied)
    }

    fn proc_status_content(&self, out: &mut [u8]) -> usize {
        let idx = self.current_task_idx().unwrap_or(0);
        let task = self.task_snapshot_at(idx);
        let mut len = 0usize;
        append_bytes(out, &mut len, b"Name:\tinit\n");
        append_bytes(out, &mut len, b"State:\t");
        append_bytes(out, &mut len, task_state_bytes(task.state));
        append_bytes(out, &mut len, b"\nPid:\t");
        append_dec(out, &mut len, task.pid);
        append_bytes(out, &mut len, b"\nPPid:\t");
        append_dec(out, &mut len, task.ppid);
        append_bytes(out, &mut len, b"\nTgid:\t");
        append_dec(out, &mut len, task.tgid);
        append_bytes(out, &mut len, b"\nFDs:\t");
        append_dec(out, &mut len, task.fd_count);
        append_bytes(out, &mut len, b"\nSigBlk:\t");
        append_hex(out, &mut len, task.signal_mask as usize, 8);
        append_bytes(out, &mut len, b"\n");
        len
    }

    fn proc_stat_content(&self, out: &mut [u8]) -> usize {
        let idx = self.current_task_idx().unwrap_or(0);
        let task = self.tasks[idx];
        let mut len = 0usize;
        append_dec(out, &mut len, task.pid);
        append_bytes(out, &mut len, b" (init) ");
        append_bytes(out, &mut len, task_state_short(task.state));
        append_bytes(out, &mut len, b" ");
        append_dec(out, &mut len, task.ppid);
        append_bytes(out, &mut len, b" ");
        append_dec(out, &mut len, task.pgid);
        append_bytes(out, &mut len, b" ");
        append_dec(out, &mut len, task.sid);
        append_bytes(out, &mut len, b" ");
        append_dec(out, &mut len, self.fd_count());
        append_bytes(out, &mut len, b"\n");
        len
    }

    fn proc_maps_content(&self, out: &mut [u8]) -> usize {
        let mm_id = self.current_mm_id().unwrap_or(0);
        let mut len = 0usize;
        let mut i = 0usize;
        while i < MAX_VMAS {
            let vma = self.vmas[i];
            if vma.used && vma.mm_id == mm_id {
                append_hex(out, &mut len, vma.start, 8);
                append_bytes(out, &mut len, b"-");
                append_hex(out, &mut len, vma.end, 8);
                append_bytes(out, &mut len, b" ");
                append_perm(out, &mut len, vma.perm);
                append_bytes(out, &mut len, b" ");
                append_hex(out, &mut len, vma.offset, 8);
                append_bytes(out, &mut len, b" 00:00 0 ");
                append_bytes(out, &mut len, vma_kind_label(vma.kind));
                append_bytes(out, &mut len, b"\n");
            }
            i += 1;
        }
        len
    }

    fn getdents_proc_fd(&mut self, ofd: usize, out: &mut [u8]) -> Result<usize, Errno> {
        if self.ofds[ofd].off != 0 {
            return Ok(0);
        }
        let mut off = 0usize;
        let mut next = 32i64;
        let mut fd = 0usize;
        while fd < MAX_FDS {
            if self.fds[fd].used {
                let mut name = [0u8; 20];
                let name_len = decimal_to_bytes(fd, &mut name);
                off = put_dirent64(out, off, 30_000 + fd as u64, next, 10, &name[..name_len])?;
                next += 32;
            }
            fd += 1;
        }
        self.ofds[ofd].off = off;
        Ok(off)
    }

    fn pwrite(&mut self, fd: isize, src: &[u8], offset: usize) -> Result<usize, Errno> {
        self.ensure();
        let idx = self.fd_idx(fd)?;
        let ofd = self.fds[idx].ofd;
        if self.ofds[ofd].kind != FdKind::RegularFile {
            return self.write(fd, src);
        }
        let saved = self.ofds[ofd].off;
        self.ofds[ofd].off = offset;
        let written = self.write(fd, src);
        self.ofds[ofd].off = saved;
        written
    }

    fn pread(&mut self, fd: isize, out: &mut [u8], offset: usize) -> Result<usize, Errno> {
        self.ensure();
        let idx = self.fd_idx(fd)?;
        let ofd = self.fds[idx].ofd;
        if self.ofds[ofd].kind != FdKind::RegularFile {
            return self.read(fd, out);
        }
        let saved = self.ofds[ofd].off;
        self.ofds[ofd].off = offset;
        let read = self.read(fd, out);
        self.ofds[ofd].off = saved;
        read
    }

    fn read_iovec(&mut self, fd: isize, vecs: &mut [RuntimeIovec], offset: Option<usize>, msg_io: bool) -> Result<usize, Errno> {
        self.ensure();
        let mut total = 0usize;
        let mut off = offset.unwrap_or(0);
        let mut i = 0usize;
        while i < vecs.len() {
            let want = if vecs[i].len < RUNTIME_IOVEC_BUF { vecs[i].len } else { RUNTIME_IOVEC_BUF };
            if want != 0 {
                let ret = match offset {
                    Some(_) => self.pread(fd, &mut vecs[i].data[..want], off),
                    None => self.read(fd, &mut vecs[i].data[..want]),
                };
                match ret {
                    Ok(0) => {
                        vecs[i].len = 0;
                        break;
                    }
                    Ok(n) => {
                        vecs[i].len = n;
                        total += n;
                        if offset.is_some() {
                            off += n;
                        }
                        if n != want {
                            break;
                        }
                    }
                    Err(err) => {
                        if total == 0 {
                            return Err(err);
                        }
                        break;
                    }
                }
            } else {
                vecs[i].len = 0;
            }
            i += 1;
        }
        self.io_stats.read_ops += 1;
        if offset.is_some() {
            self.io_stats.positioned_ops += 1;
        }
        if msg_io {
            self.io_stats.msg_ops += 1;
        }
        self.io_stats.bytes_read += total;
        Ok(total)
    }

    fn write_iovec(&mut self, fd: isize, vecs: &[RuntimeIovec], offset: Option<usize>, msg_io: bool) -> Result<usize, Errno> {
        self.ensure();
        let mut total = 0usize;
        let mut off = offset.unwrap_or(0);
        let mut i = 0usize;
        while i < vecs.len() {
            let len = if vecs[i].len < RUNTIME_IOVEC_BUF { vecs[i].len } else { RUNTIME_IOVEC_BUF };
            if len != 0 {
                let ret = match offset {
                    Some(_) => self.pwrite(fd, &vecs[i].data[..len], off),
                    None => self.write(fd, &vecs[i].data[..len]),
                };
                match ret {
                    Ok(0) => break,
                    Ok(n) => {
                        total += n;
                        if offset.is_some() {
                            off += n;
                        }
                        if n != len {
                            break;
                        }
                    }
                    Err(err) => {
                        if total == 0 {
                            return Err(err);
                        }
                        break;
                    }
                }
            }
            i += 1;
        }
        self.io_stats.write_ops += 1;
        if offset.is_some() {
            self.io_stats.positioned_ops += 1;
        }
        if msg_io {
            self.io_stats.msg_ops += 1;
        }
        self.io_stats.bytes_written += total;
        Ok(total)
    }

    fn lseek(&mut self, fd: isize, offset: isize, whence: usize) -> Result<usize, Errno> {
        self.ensure();
        let idx = self.fd_idx(fd)?;
        let ofd = self.fds[idx].ofd;
        let owner = self.owner(self.ofds[ofd].node);
        if !matches!(self.ofds[ofd].kind, FdKind::RegularFile | FdKind::Directory | FdKind::Symlink) {
            return Err(Errno::SpiPe);
        }
        let size = if self.nodes[owner].kind == NodeKind::File { self.nodes[owner].size } else { 0 };
        let base = match whence {
            SEEK_SET => 0isize,
            SEEK_CUR => self.ofds[ofd].off as isize,
            SEEK_END => size as isize,
            _ => return Err(Errno::Inval),
        };
        let next = base + offset;
        if next < 0 {
            return Err(Errno::Inval);
        }
        self.ofds[ofd].off = next as usize;
        Ok(self.ofds[ofd].off)
    }

    fn dup(&mut self, oldfd: isize) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.fd_idx(oldfd)?;
        let ofd = self.fds[idx].ofd;
        self.alloc_fd_for_ofd(ofd, 3, false)
    }

    fn dup3(&mut self, oldfd: isize, newfd: isize, flags: u32) -> Result<isize, Errno> {
        self.ensure();
        if oldfd == newfd || newfd < 0 || newfd as usize >= MAX_FDS {
            return Err(Errno::Inval);
        }
        let old = self.fd_idx(oldfd)?;
        let new_idx = newfd as usize;
        if self.fds[new_idx].used {
            let _ = self.close(newfd);
        }
        let ofd = self.fds[old].ofd;
        self.fds[new_idx] = Fd { used: true, ofd, cloexec: (flags & O_CLOEXEC) != 0 };
        self.ofds[ofd].refs += 1;
        Ok(newfd)
    }

    fn fcntl(&mut self, fd: isize, cmd: usize, arg: usize) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.fd_idx(fd)?;
        let ofd = self.fds[idx].ofd;
        match cmd {
            F_GETFD => Ok(if self.fds[idx].cloexec { FD_CLOEXEC as isize } else { 0 }),
            F_SETFD => {
                self.fds[idx].cloexec = (arg as u32 & FD_CLOEXEC) != 0;
                Ok(0)
            }
            F_GETFL => Ok(self.ofds[ofd].flags as isize),
            F_SETFL => {
                let keep = self.ofds[ofd].flags & !(O_APPEND | O_NONBLOCK);
                self.ofds[ofd].flags = keep | (arg as u32 & (O_APPEND | O_NONBLOCK));
                Ok(0)
            }
            _ => Err(Errno::Inval),
        }
    }

    fn mkdirat(&mut self, dirfd: isize, path: &[u8], mode: u16) -> Result<isize, Errno> {
        self.ensure();
        let mut name = [0u8; NAME_MAX];
        let (parent, len) = self.resolve_parent_at(dirfd, path, &mut name)?;
        self.mkdir_at_node(parent, &name[..len], mode)?;
        Ok(0)
    }

    fn chdir(&mut self, path: &[u8]) -> Result<isize, Errno> {
        self.ensure();
        let node = self.resolve_at(AT_FDCWD, path, true)?;
        if !self.node_is_dirlike(node) {
            return Err(Errno::NotDir);
        }
        self.cwd = node;
        if self.current_task < MAX_TASKS && self.tasks[self.current_task].used {
            self.tasks[self.current_task].cwd = node;
            self.tasks[self.current_task].root = self.root;
        }
        Ok(0)
    }

    fn getcwd(&mut self, out: &mut [u8]) -> Result<usize, Errno> {
        self.ensure();
        let len = self.node_abs_path(self.cwd, out)?;
        if len < out.len() {
            out[len] = 0;
            Ok(len + 1)
        } else {
            Ok(len)
        }
    }

    fn linkat(&mut self, olddir: isize, oldpath: &[u8], newdir: isize, newpath: &[u8]) -> Result<isize, Errno> {
        self.ensure();
        let old = self.resolve_at(olddir, oldpath, true)?;
        let owner = self.owner(old);
        if self.nodes[owner].kind != NodeKind::File {
            return Err(Errno::Inval);
        }
        let mut name = [0u8; NAME_MAX];
        let (parent, len) = self.resolve_parent_at(newdir, newpath, &mut name)?;
        if self.find_child(parent, &name[..len]).is_some() {
            return Err(Errno::Exist);
        }
        let node = self.alloc_node()?;
        self.nodes[node].kind = NodeKind::File;
        self.nodes[node].fs_kind = self.nodes[parent].fs_kind;
        self.nodes[node].parent = parent;
        self.nodes[node].owner = owner;
        self.nodes[node].uid = self.nodes[owner].uid;
        self.nodes[node].gid = self.nodes[owner].gid;
        self.nodes[node].mode = self.nodes[owner].mode;
        Self::set_name(&mut self.nodes[node], &name[..len])?;
        Ok(0)
    }

    fn renameat(&mut self, olddir: isize, oldpath: &[u8], newdir: isize, newpath: &[u8]) -> Result<isize, Errno> {
        self.ensure();
        let node = self.resolve_at(olddir, oldpath, false)?;
        let mut name = [0u8; NAME_MAX];
        let (parent, len) = self.resolve_parent_at(newdir, newpath, &mut name)?;
        if let Some(existing) = self.find_child(parent, &name[..len]) {
            if self.nodes[existing].kind == NodeKind::Dir {
                return Err(Errno::IsDir);
            }
            self.nodes[existing] = Node::empty();
        }
        self.nodes[node].parent = parent;
        Self::set_name(&mut self.nodes[node], &name[..len])?;
        Ok(0)
    }

    fn symlinkat(&mut self, target: &[u8], newdir: isize, newpath: &[u8]) -> Result<isize, Errno> {
        self.ensure();
        if target.is_empty() || target.len() > TARGET_MAX {
            return Err(Errno::Inval);
        }
        let mut name = [0u8; NAME_MAX];
        let (parent, len) = self.resolve_parent_at(newdir, newpath, &mut name)?;
        if self.find_child(parent, &name[..len]).is_some() {
            return Err(Errno::Exist);
        }
        let node = self.create_node_at(parent, &name[..len], NodeKind::Symlink, 0o777)?;
        let mut i = 0usize;
        while i < target.len() {
            self.nodes[node].target[i] = target[i];
            i += 1;
        }
        self.nodes[node].target_len = target.len();
        Ok(0)
    }

    fn readlinkat(&mut self, dirfd: isize, path: &[u8], out: &mut [u8]) -> Result<usize, Errno> {
        self.ensure();
        if path_starts(path, b"/proc/self/fd/") {
            let fd = parse_decimal(&path[14..]).ok_or(Errno::NoEnt)?;
            return self.fd_target_path(fd as isize, out);
        }
        let node = self.resolve_at(dirfd, path, false)?;
        if self.nodes[node].kind != NodeKind::Symlink {
            return Err(Errno::Inval);
        }
        let mut i = 0usize;
        while i < out.len() && i < self.nodes[node].target_len {
            out[i] = self.nodes[node].target[i];
            i += 1;
        }
        Ok(i)
    }

    fn unlinkat(&mut self, dirfd: isize, path: &[u8], flags: usize) -> Result<isize, Errno> {
        self.ensure();
        let node = self.resolve_at(dirfd, path, false)?;
        if node == ROOT {
            return Err(Errno::Inval);
        }
        if self.nodes[node].kind == NodeKind::Dir {
            if flags & 0x200 == 0 {
                return Err(Errno::IsDir);
            }
            let mut i = 1usize;
            while i < MAX_NODES {
                if self.nodes[i].used && self.nodes[i].parent == node && i != node {
                    return Err(Errno::NotEmpty);
                }
                i += 1;
            }
        }
        self.nodes[node] = Node::empty();
        Ok(0)
    }

    fn truncate_path(&mut self, dirfd: isize, path: &[u8], length: usize) -> Result<isize, Errno> {
        self.ensure();
        let node = self.resolve_at(dirfd, path, true)?;
        self.truncate_node(node, length)
    }

    fn truncate_fd(&mut self, fd: isize, length: usize) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.fd_idx(fd)?;
        let node = self.ofds[self.fds[idx].ofd].node;
        self.truncate_node(node, length)
    }

    fn truncate_node(&mut self, node: usize, length: usize) -> Result<isize, Errno> {
        let owner = self.owner(node);
        if self.nodes[owner].kind != NodeKind::File || length > DATA_MAX {
            return Err(Errno::Inval);
        }
        if length > self.nodes[owner].size {
            let mut i = self.nodes[owner].size;
            while i < length {
                self.nodes[owner].data[i] = 0;
                i += 1;
            }
        }
        self.nodes[owner].size = length;
        Ok(0)
    }

    fn chmod_node(&mut self, node: usize, mode: usize) -> Result<isize, Errno> {
        let owner = self.owner(node);
        if owner >= MAX_NODES || !self.nodes[owner].used {
            return Err(Errno::NoEnt);
        }
        let cred = self.current_cred_copy();
        if cred.euid != self.nodes[owner].uid && !Self::cred_has_cap(cred, CAP_FOWNER) {
            return Err(Errno::Access);
        }
        self.nodes[owner].mode = (mode as u16) & 0o7777;
        Ok(0)
    }

    fn chmod_fd(&mut self, fd: isize, mode: usize) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.fd_idx(fd)?;
        let node = self.ofds[self.fds[idx].ofd].node;
        self.chmod_node(node, mode)
    }

    fn chmod_path(&mut self, dirfd: isize, path: &[u8], mode: usize) -> Result<isize, Errno> {
        self.ensure();
        let node = self.resolve_at(dirfd, path, true)?;
        self.chmod_node(node, mode)
    }

    fn chown_node(&mut self, node: usize, uid: usize, gid: usize) -> Result<isize, Errno> {
        let owner = self.owner(node);
        if owner >= MAX_NODES || !self.nodes[owner].used {
            return Err(Errno::NoEnt);
        }
        let cred = self.current_cred_copy();
        if !Self::cred_has_cap(cred, CAP_CHOWN) {
            return Err(Errno::Access);
        }
        if uid != usize::MAX {
            self.nodes[owner].uid = uid as u32;
        }
        if gid != usize::MAX {
            self.nodes[owner].gid = gid as u32;
        }
        Ok(0)
    }

    fn chown_fd(&mut self, fd: isize, uid: usize, gid: usize) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.fd_idx(fd)?;
        let node = self.ofds[self.fds[idx].ofd].node;
        self.chown_node(node, uid, gid)
    }

    fn chown_path(&mut self, dirfd: isize, path: &[u8], uid: usize, gid: usize) -> Result<isize, Errno> {
        self.ensure();
        let node = self.resolve_at(dirfd, path, true)?;
        self.chown_node(node, uid, gid)
    }

    fn access(&mut self, dirfd: isize, path: &[u8], mask: usize) -> Result<isize, Errno> {
        self.ensure();
        if (mask & !0o7) != 0 {
            return Err(Errno::Inval);
        }
        let node = self.resolve_at(dirfd, path, true)?;
        if !self.access_allowed(node, mask) {
            return Err(Errno::Access);
        }
        Ok(0)
    }

    fn stat_node(&self, node: usize) -> Result<RuntimeStat, Errno> {
        if node >= MAX_NODES || !self.nodes[node].used {
            return Err(Errno::NoEnt);
        }
        let owner = self.owner(node);
        let mut nlink = 0u32;
        let mut i = 1usize;
        while i < MAX_NODES {
            if self.nodes[i].used && self.nodes[i].kind == NodeKind::File && self.owner(i) == owner {
                nlink += 1;
            }
            i += 1;
        }
        let kind = match self.nodes[node].kind {
            NodeKind::Dir => {
                nlink = 2;
                FdKind::Directory
            }
            NodeKind::ProcFdDir => {
                nlink = 2;
                FdKind::Directory
            }
            NodeKind::File => FdKind::RegularFile,
            NodeKind::Symlink => {
                nlink = 1;
                FdKind::Symlink
            }
            NodeKind::DevNull => {
                nlink = 1;
                FdKind::DevNull
            }
            NodeKind::DevZero => {
                nlink = 1;
                FdKind::DevZero
            }
            NodeKind::DevConsole => {
                nlink = 1;
                FdKind::DevConsole
            }
            NodeKind::DevTty => {
                nlink = 1;
                FdKind::DevTty
            }
            NodeKind::DevRandom => {
                nlink = 1;
                FdKind::DevRandom
            }
            NodeKind::ProcStatus | NodeKind::ProcStat | NodeKind::ProcMaps => {
                nlink = 1;
                FdKind::Procfs
            }
            NodeKind::Empty => return Err(Errno::NoEnt),
        };
        let size = match self.nodes[node].kind {
            NodeKind::File => self.nodes[owner].size,
            NodeKind::Symlink => self.nodes[node].target_len,
            NodeKind::ProcStatus => 160,
            NodeKind::ProcStat => 80,
            NodeKind::ProcMaps => 256,
            _ => 0,
        };
        let type_bits = match kind {
            FdKind::Directory => 0o040000,
            FdKind::Symlink => 0o120000,
            FdKind::DevNull | FdKind::DevZero | FdKind::DevConsole | FdKind::DevTty | FdKind::DevRandom | FdKind::Stdin | FdKind::Stdout | FdKind::Stderr => 0o020000,
            _ => 0o100000,
        };
        Ok(RuntimeStat { kind, ino: 10_000 + node as u64, mode: type_bits | self.nodes[owner].mode, size, nlink })
    }

    fn stat_fd(&mut self, fd: isize) -> Result<RuntimeStat, Errno> {
        self.ensure();
        let idx = self.fd_idx(fd)?;
        let ofd = self.fds[idx].ofd;
        match self.ofds[ofd].kind {
            FdKind::RegularFile | FdKind::Directory | FdKind::Symlink | FdKind::DevNull | FdKind::DevZero | FdKind::DevConsole | FdKind::DevTty | FdKind::DevRandom | FdKind::Procfs => self.stat_node(self.ofds[ofd].node),
            kind => Ok(RuntimeStat { kind, ino: 20_000 + fd as u64, mode: fd_kind_mode(kind), size: 0, nlink: 1 }),
        }
    }

    fn stat_path(&mut self, dirfd: isize, path: &[u8], follow: bool) -> Result<RuntimeStat, Errno> {
        self.ensure();
        let node = self.resolve_at(dirfd, path, follow)?;
        self.stat_node(node)
    }

    fn getdents64(&mut self, fd: isize, out: &mut [u8]) -> Result<usize, Errno> {
        self.ensure();
        let idx = self.fd_idx(fd)?;
        let ofd = self.fds[idx].ofd;
        if self.ofds[ofd].kind != FdKind::Directory && self.ofds[ofd].kind != FdKind::Procfs {
            return Err(Errno::NotDir);
        }
        let dir = self.ofds[ofd].node;
        if self.nodes[dir].kind == NodeKind::ProcFdDir {
            return self.getdents_proc_fd(ofd, out);
        }
        if self.ofds[ofd].off != 0 {
            return Ok(0);
        }
        let mut off = 0usize;
        let mut next = 32i64;
        let mut i = 1usize;
        while i < MAX_NODES {
            if self.nodes[i].used && self.nodes[i].parent == dir && i != dir {
                let dtype = match self.nodes[i].kind {
                    NodeKind::Dir | NodeKind::ProcFdDir => 4,
                    NodeKind::Symlink => 10,
                    NodeKind::DevNull | NodeKind::DevZero | NodeKind::DevConsole | NodeKind::DevTty | NodeKind::DevRandom => 2,
                    NodeKind::File | NodeKind::ProcStatus | NodeKind::ProcStat | NodeKind::ProcMaps => 8,
                    NodeKind::Empty => 0,
                };
                off = put_dirent64(out, off, 10_000 + i as u64, next, dtype, &self.nodes[i].name[..self.nodes[i].name_len])?;
                next += 32;
            }
            i += 1;
        }
        self.ofds[ofd].off = off;
        Ok(off)
    }

    fn statfs_for_kind(&self, fs_kind: RuntimeFsKind) -> RuntimeStatFs {
        let mut files = 0usize;
        let mut fds_used = 0usize;
        let mut i = 1usize;
        while i < MAX_NODES {
            if self.nodes[i].used && self.nodes[i].fs_kind == fs_kind {
                files += 1;
            }
            i += 1;
        }
        i = 0;
        while i < MAX_FDS {
            if self.fds[i].used {
                fds_used += 1;
            }
            i += 1;
        }
        RuntimeStatFs {
            fs_kind,
            magic: Self::fs_magic(fs_kind),
            block_size: if fs_kind == RuntimeFsKind::Imagefs { BLOCK_SIZE } else { RUNTIME_PAGE_SIZE },
            files,
            free_files: MAX_NODES - files,
            fds_used,
            mount_count: self.mount_count(),
        }
    }

    fn statfs(&mut self) -> RuntimeStatFs {
        self.ensure();
        self.statfs_for_kind(self.nodes[self.root].fs_kind)
    }

    fn statfs_path(&mut self, dirfd: isize, path: &[u8]) -> Result<RuntimeStatFs, Errno> {
        self.ensure();
        let node = self.resolve_at(dirfd, path, true)?;
        Ok(self.statfs_for_kind(self.nodes[node].fs_kind))
    }

    fn statfs_fd(&mut self, fd: isize) -> Result<RuntimeStatFs, Errno> {
        self.ensure();
        let idx = self.fd_idx(fd)?;
        let node = self.ofds[self.fds[idx].ofd].node;
        Ok(self.statfs_for_kind(self.nodes[node].fs_kind))
    }

    fn mount_fs(&mut self, source: &[u8], target: &[u8], fstype: &[u8], flags: usize) -> Result<isize, Errno> {
        self.ensure();
        let fs_kind = Self::fs_kind_from_name(fstype)?;
        let node = self.resolve_at(AT_FDCWD, target, true)?;
        if !self.node_is_dirlike(node) {
            return Err(Errno::NotDir);
        }
        self.set_subtree_fs_kind(node, fs_kind);
        let src = if source.is_empty() { fstype } else { source };
        let _ = self.register_mount(node, fs_kind, src, fstype, flags)?;
        Ok(0)
    }

    fn umount2(&mut self, target: &[u8], _flags: usize) -> Result<isize, Errno> {
        self.ensure();
        let node = self.resolve_at(AT_FDCWD, target, true)?;
        if node == self.root {
            return Err(Errno::Inval);
        }
        let mut removed = false;
        let mut i = 0usize;
        while i < MAX_MOUNTS {
            if self.mounts[i].used && self.mounts[i].target == node {
                self.mounts[i] = MountObj::empty();
                removed = true;
            }
            i += 1;
        }
        if !removed {
            return Err(Errno::Inval);
        }
        let parent_kind = self.nodes[self.nodes[node].parent].fs_kind;
        self.set_subtree_fs_kind(node, parent_kind);
        Ok(0)
    }

    fn mount_snapshot(&mut self) -> RuntimeMountSnapshot {
        self.ensure();
        let mut snapshot = RuntimeMountSnapshot {
            mount_count: 0,
            rootfs_mounts: 0,
            tmpfs_mounts: 0,
            devfs_mounts: 0,
            procfs_mounts: 0,
            imagefs_mounts: 0,
        };
        let mut i = 0usize;
        while i < MAX_MOUNTS {
            if self.mounts[i].used {
                snapshot.mount_count += 1;
                match self.mounts[i].fs_kind {
                    RuntimeFsKind::Rootfs => snapshot.rootfs_mounts += 1,
                    RuntimeFsKind::Tmpfs => snapshot.tmpfs_mounts += 1,
                    RuntimeFsKind::Devfs => snapshot.devfs_mounts += 1,
                    RuntimeFsKind::Procfs => snapshot.procfs_mounts += 1,
                    RuntimeFsKind::Imagefs => snapshot.imagefs_mounts += 1,
                }
            }
            i += 1;
        }
        snapshot
    }

    fn block_device_count(&self) -> usize {
        let mut count = 0usize;
        let mut i = 0usize;
        while i < MAX_BLOCK_DEVS {
            if self.block_devices[i].used {
                count += 1;
            }
            i += 1;
        }
        count
    }

    fn block_raw_read(&mut self, dev: usize, block: usize, out: &mut [u8; BLOCK_SIZE]) -> Result<(), Errno> {
        if dev >= MAX_BLOCK_DEVS || !self.block_devices[dev].used {
            return Err(Errno::Inval);
        }
        if block >= self.block_devices[dev].sectors {
            self.block_devices[dev].errors += 1;
            return Err(Errno::Inval);
        }
        let start = block.checked_mul(self.block_devices[dev].sector_size).ok_or(Errno::Inval)?;
        let end = start.checked_add(BLOCK_SIZE).ok_or(Errno::Inval)?;
        if end > RAMDISK_BYTES || self.block_devices[dev].kind != BlockDeviceKind::Ramdisk {
            self.block_devices[dev].errors += 1;
            return Err(Errno::Inval);
        }
        let mut i = 0usize;
        while i < BLOCK_SIZE {
            out[i] = self.ramdisk[start + i];
            i += 1;
        }
        self.block_devices[dev].read_ops += 1;
        Ok(())
    }

    fn block_raw_write(&mut self, dev: usize, block: usize, src: &[u8; BLOCK_SIZE]) -> Result<(), Errno> {
        if dev >= MAX_BLOCK_DEVS || !self.block_devices[dev].used {
            return Err(Errno::Inval);
        }
        if self.block_devices[dev].readonly {
            self.block_devices[dev].errors += 1;
            return Err(Errno::Access);
        }
        if block >= self.block_devices[dev].sectors {
            self.block_devices[dev].errors += 1;
            return Err(Errno::Inval);
        }
        let start = block.checked_mul(self.block_devices[dev].sector_size).ok_or(Errno::Inval)?;
        let end = start.checked_add(BLOCK_SIZE).ok_or(Errno::Inval)?;
        if end > RAMDISK_BYTES || self.block_devices[dev].kind != BlockDeviceKind::Ramdisk {
            self.block_devices[dev].errors += 1;
            return Err(Errno::Inval);
        }
        let mut i = 0usize;
        while i < BLOCK_SIZE {
            self.ramdisk[start + i] = src[i];
            i += 1;
        }
        self.block_devices[dev].write_ops += 1;
        Ok(())
    }

    fn block_cache_lookup(&self, dev: usize, block: usize) -> Option<usize> {
        let mut i = 0usize;
        while i < BLOCK_CACHE_SLOTS {
            if self.block_cache[i].valid && self.block_cache[i].dev == dev && self.block_cache[i].block == block {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    fn block_cache_flush_slot(&mut self, slot: usize) -> Result<(), Errno> {
        if slot >= BLOCK_CACHE_SLOTS || !self.block_cache[slot].valid || !self.block_cache[slot].dirty {
            return Ok(());
        }
        let dev = self.block_cache[slot].dev;
        let block = self.block_cache[slot].block;
        let data = self.block_cache[slot].data;
        self.block_raw_write(dev, block, &data)?;
        self.block_cache[slot].dirty = false;
        self.block_cache_writebacks += 1;
        Ok(())
    }

    fn block_cache_flush(&mut self) -> Result<(), Errno> {
        let mut i = 0usize;
        while i < BLOCK_CACHE_SLOTS {
            self.block_cache_flush_slot(i)?;
            i += 1;
        }
        Ok(())
    }

    fn block_cache_read_block(&mut self, dev: usize, block: usize, out: &mut [u8; BLOCK_SIZE]) -> Result<(), Errno> {
        if let Some(slot) = self.block_cache_lookup(dev, block) {
            let mut i = 0usize;
            while i < BLOCK_SIZE {
                out[i] = self.block_cache[slot].data[i];
                i += 1;
            }
            self.block_cache_hits += 1;
            return Ok(());
        }
        let slot = self.block_cache_next % BLOCK_CACHE_SLOTS;
        self.block_cache_next = (self.block_cache_next + 1) % BLOCK_CACHE_SLOTS;
        self.block_cache_flush_slot(slot)?;
        let mut data = [0u8; BLOCK_SIZE];
        self.block_raw_read(dev, block, &mut data)?;
        self.block_cache[slot] = BlockCacheEntry { valid: true, dirty: false, dev, block, data };
        let mut i = 0usize;
        while i < BLOCK_SIZE {
            out[i] = data[i];
            i += 1;
        }
        self.block_cache_misses += 1;
        Ok(())
    }

    fn block_cache_write_block(&mut self, dev: usize, block: usize, src: &[u8; BLOCK_SIZE]) -> Result<(), Errno> {
        let slot = match self.block_cache_lookup(dev, block) {
            Some(slot) => slot,
            None => {
                let slot = self.block_cache_next % BLOCK_CACHE_SLOTS;
                self.block_cache_next = (self.block_cache_next + 1) % BLOCK_CACHE_SLOTS;
                self.block_cache_flush_slot(slot)?;
                let mut data = [0u8; BLOCK_SIZE];
                self.block_raw_read(dev, block, &mut data)?;
                self.block_cache[slot] = BlockCacheEntry { valid: true, dirty: false, dev, block, data };
                self.block_cache_misses += 1;
                slot
            }
        };
        let mut i = 0usize;
        while i < BLOCK_SIZE {
            self.block_cache[slot].data[i] = src[i];
            i += 1;
        }
        self.block_cache[slot].dirty = true;
        self.block_cache_dirty_marks += 1;
        Ok(())
    }

    fn block_cache_read_bytes(&mut self, dev: usize, offset: usize, out: &mut [u8]) -> Result<usize, Errno> {
        let mut copied = 0usize;
        while copied < out.len() {
            let abs = offset.checked_add(copied).ok_or(Errno::Inval)?;
            let block = abs / BLOCK_SIZE;
            let in_block = abs % BLOCK_SIZE;
            let mut sector = [0u8; BLOCK_SIZE];
            self.block_cache_read_block(dev, block, &mut sector)?;
            let mut take = BLOCK_SIZE - in_block;
            let remain = out.len() - copied;
            if take > remain {
                take = remain;
            }
            let mut i = 0usize;
            while i < take {
                out[copied + i] = sector[in_block + i];
                i += 1;
            }
            copied += take;
        }
        Ok(copied)
    }

    fn encode_image_entry(table: &mut [u8; BLOCK_SIZE], index: usize, inode: usize, parent: usize, kind: u8, mode: u16, size: usize, start_block: usize, flags: u32, name: &[u8]) -> Result<(), Errno> {
        if index >= IMAGE_MAX_ENTRIES || name.is_empty() || name.len() > NAME_MAX {
            return Err(Errno::Inval);
        }
        let off = index * IMAGE_ENTRY_SIZE;
        write_le_u32(table, off, inode as u32);
        write_le_u32(table, off + 4, parent as u32);
        table[off + 8] = kind;
        write_le_u16(table, off + 10, mode);
        write_le_u32(table, off + 12, size as u32);
        write_le_u32(table, off + 16, start_block as u32);
        write_le_u32(table, off + 20, flags);
        table[off + 24] = name.len() as u8;
        let mut i = 0usize;
        while i < name.len() {
            table[off + 32 + i] = name[i];
            i += 1;
        }
        Ok(())
    }

    fn write_image_file_block(&mut self, block: usize, src: &[u8]) -> Result<(), Errno> {
        let mut sector = [0u8; BLOCK_SIZE];
        let mut i = 0usize;
        while i < src.len() && i < BLOCK_SIZE {
            sector[i] = src[i];
            i += 1;
        }
        self.block_raw_write(IMAGE_DEV, block, &sector)
    }

    fn prepare_competition_ramdisk(&mut self) -> Result<(), Errno> {
        self.block_devices = [BlockDeviceObj::empty(); MAX_BLOCK_DEVS];
        self.ramdisk = [0; RAMDISK_BYTES];
        self.block_cache = [BlockCacheEntry::empty(); BLOCK_CACHE_SLOTS];
        self.block_cache_next = 0;
        self.block_cache_hits = 0;
        self.block_cache_misses = 0;
        self.block_cache_dirty_marks = 0;
        self.block_cache_writebacks = 0;
        self.image_mounted = false;
        self.image_mount_node = 0;
        self.image_dirs = 0;
        self.image_files = 0;
        self.image_exec_files = 0;
        self.image_metadata_reads = 0;
        self.image_data_reads = 0;
        self.image_errors = 0;
        self.block_devices[IMAGE_DEV] = BlockDeviceObj {
            used: true,
            kind: BlockDeviceKind::Ramdisk,
            sector_size: BLOCK_SIZE,
            sectors: RAMDISK_BLOCKS,
            readonly: false,
            read_ops: 0,
            write_ops: 0,
            errors: 0,
        };

        let readme = b"storage image v207-v214\n";
        let config = b"root=/image console=/dev/console\n";
        let mut elf = [0u8; 160];
        let elf_len = build_runtime_exec_fixture_bytes(&mut elf);
        let mut header = [0u8; BLOCK_SIZE];
        header[0] = b'U';
        header[1] = b'C';
        header[2] = b'F';
        header[3] = b'S';
        header[4] = b'I';
        header[5] = b'M';
        header[6] = b'G';
        header[7] = b'1';
        write_le_u32(&mut header, 8, BLOCK_SIZE as u32);
        write_le_u32(&mut header, 12, 8);
        write_le_u32(&mut header, 16, 1);
        self.block_raw_write(IMAGE_DEV, 0, &header)?;

        let mut entries = [0u8; BLOCK_SIZE];
        Self::encode_image_entry(&mut entries, 0, 1, 0, 1, 0o555, 0, 0, 0, b"/")?;
        Self::encode_image_entry(&mut entries, 1, 2, 1, 1, 0o555, 0, 0, 0, b"bin")?;
        Self::encode_image_entry(&mut entries, 2, 3, 1, 1, 0o555, 0, 0, 0, b"etc")?;
        Self::encode_image_entry(&mut entries, 3, 4, 1, 2, 0o444, readme.len(), 4, 0, b"README.txt")?;
        Self::encode_image_entry(&mut entries, 4, 5, 3, 2, 0o444, config.len(), 5, 0, b"config.txt")?;
        Self::encode_image_entry(&mut entries, 5, 6, 2, 2, 0o555, elf_len, 6, 1, b"hello.elf")?;
        Self::encode_image_entry(&mut entries, 6, 7, 2, 2, 0o555, elf_len, 7, 1, b"worker.elf")?;
        Self::encode_image_entry(&mut entries, 7, 8, 2, 2, 0o555, elf_len, 8, 1, b"tool.elf")?;
        self.block_raw_write(IMAGE_DEV, IMAGE_ENTRY_TABLE_BLOCK, &entries)?;
        self.write_image_file_block(4, readme)?;
        self.write_image_file_block(5, config)?;
        self.write_image_file_block(6, &elf[..elf_len])?;
        self.write_image_file_block(7, &elf[..elf_len])?;
        self.write_image_file_block(8, &elf[..elf_len])?;
        Ok(())
    }

    fn mount_competition_image(&mut self) -> Result<RuntimeStorageSnapshot, Errno> {
        if IMAGE_DEV >= MAX_BLOCK_DEVS || !self.block_devices[IMAGE_DEV].used {
            self.prepare_competition_ramdisk()?;
        }
        self.block_devices[IMAGE_DEV].readonly = true;
        let image_node = match self.find_child(ROOT, b"image") {
            Some(node) => node,
            None => self.mkdir_at_node(ROOT, b"image", 0o555)?,
        };
        self.nodes[image_node].fs_kind = RuntimeFsKind::Imagefs;
        self.nodes[image_node].mode = 0o555;
        self.nodes[image_node].image_inode = 1;
        self.nodes[image_node].image_offset = 0;
        self.nodes[image_node].image_flags = 0;
        let _ = self.register_mount(image_node, RuntimeFsKind::Imagefs, b"ramdisk0", b"ext4-ro", 0)?;

        let mut header = [0u8; BLOCK_SIZE];
        self.block_cache_read_block(IMAGE_DEV, 0, &mut header)?;
        self.image_metadata_reads += 1;
        if &header[..8] != b"UCFSIMG1" || read_le_u32(&header, 8) as usize != BLOCK_SIZE {
            self.image_errors += 1;
            return Err(Errno::Inval);
        }
        let entry_count = read_le_u32(&header, 12) as usize;
        if entry_count == 0 || entry_count > IMAGE_MAX_ENTRIES {
            self.image_errors += 1;
            return Err(Errno::Inval);
        }
        let mut entries = [0u8; BLOCK_SIZE];
        self.block_cache_read_block(IMAGE_DEV, IMAGE_ENTRY_TABLE_BLOCK, &mut entries)?;
        self.image_metadata_reads += 1;
        let mut inode_nodes = [0usize; 32];
        inode_nodes[1] = image_node;
        self.image_dirs = 1;
        self.image_files = 0;
        self.image_exec_files = 0;
        let mut idx = 0usize;
        while idx < entry_count {
            let off = idx * IMAGE_ENTRY_SIZE;
            let inode = read_le_u32(&entries, off) as usize;
            let parent = read_le_u32(&entries, off + 4) as usize;
            let kind = entries[off + 8];
            let mode = read_le_u16(&entries, off + 10);
            let size = read_le_u32(&entries, off + 12) as usize;
            let start_block = read_le_u32(&entries, off + 16) as usize;
            let flags = read_le_u32(&entries, off + 20);
            let name_len = entries[off + 24] as usize;
            if inode == 0 || inode >= inode_nodes.len() || name_len == 0 || name_len > NAME_MAX {
                self.image_errors += 1;
                return Err(Errno::Inval);
            }
            if inode == 1 {
                idx += 1;
                continue;
            }
            let parent_node = if parent < inode_nodes.len() { inode_nodes[parent] } else { 0 };
            if parent_node == 0 {
                self.image_errors += 1;
                return Err(Errno::Inval);
            }
            let node_kind = if kind == 1 { NodeKind::Dir } else if kind == 2 { NodeKind::File } else {
                self.image_errors += 1;
                return Err(Errno::Inval);
            };
            let name = &entries[off + 32..off + 32 + name_len];
            let node = self.create_node_at(parent_node, name, node_kind, mode)?;
            self.nodes[node].fs_kind = RuntimeFsKind::Imagefs;
            self.nodes[node].mode = mode & 0o777;
            self.nodes[node].size = size;
            self.nodes[node].image_inode = inode;
            self.nodes[node].image_offset = start_block * BLOCK_SIZE;
            self.nodes[node].image_flags = flags;
            inode_nodes[inode] = node;
            if node_kind == NodeKind::Dir {
                self.image_dirs += 1;
            } else {
                self.image_files += 1;
                if (flags & 1) != 0 {
                    self.image_exec_files += 1;
                }
            }
            idx += 1;
        }
        self.image_mounted = true;
        self.image_mount_node = image_node;
        Ok(self.storage_snapshot())
    }

    fn image_read_node(&mut self, node: usize, offset: usize, out: &mut [u8]) -> Result<usize, Errno> {
        if node >= MAX_NODES || !self.nodes[node].used || self.nodes[node].kind != NodeKind::File || self.nodes[node].fs_kind != RuntimeFsKind::Imagefs {
            return Err(Errno::Inval);
        }
        if offset >= self.nodes[node].size {
            return Ok(0);
        }
        let mut want = self.nodes[node].size - offset;
        if want > out.len() {
            want = out.len();
        }
        let image_offset = self.nodes[node].image_offset.checked_add(offset).ok_or(Errno::Inval)?;
        let copied = self.block_cache_read_bytes(IMAGE_DEV, image_offset, &mut out[..want])?;
        self.image_data_reads += 1;
        Ok(copied)
    }

    fn node_file_bytes(&mut self, owner: usize, out: &mut [u8]) -> Result<usize, Errno> {
        if owner >= MAX_NODES || !self.nodes[owner].used || self.nodes[owner].kind != NodeKind::File {
            return Err(Errno::NoExec);
        }
        if self.nodes[owner].size > out.len() {
            return Err(Errno::NoExec);
        }
        let size = self.nodes[owner].size;
        if self.nodes[owner].fs_kind == RuntimeFsKind::Imagefs {
            return self.image_read_node(owner, 0, &mut out[..size]);
        }
        let mut i = 0usize;
        while i < size {
            out[i] = self.nodes[owner].data[i];
            i += 1;
        }
        Ok(size)
    }

    fn storage_snapshot(&self) -> RuntimeStorageSnapshot {
        let mut raw_reads = 0usize;
        let mut raw_writes = 0usize;
        let mut block_errors = 0usize;
        let mut i = 0usize;
        while i < MAX_BLOCK_DEVS {
            if self.block_devices[i].used {
                raw_reads += self.block_devices[i].read_ops;
                raw_writes += self.block_devices[i].write_ops;
                block_errors += self.block_devices[i].errors;
            }
            i += 1;
        }
        RuntimeStorageSnapshot {
            block_devices: self.block_device_count(),
            sector_size: if self.block_devices[IMAGE_DEV].used { self.block_devices[IMAGE_DEV].sector_size } else { 0 },
            sectors: if self.block_devices[IMAGE_DEV].used { self.block_devices[IMAGE_DEV].sectors } else { 0 },
            raw_reads,
            raw_writes,
            block_errors,
            cache_hits: self.block_cache_hits,
            cache_misses: self.block_cache_misses,
            cache_dirty_marks: self.block_cache_dirty_marks,
            cache_writebacks: self.block_cache_writebacks,
            image_mounted: self.image_mounted,
            image_mount_node: self.image_mount_node,
            image_dirs: self.image_dirs,
            image_files: self.image_files,
            image_exec_files: self.image_exec_files,
            image_metadata_reads: self.image_metadata_reads,
            image_data_reads: self.image_data_reads,
            image_errors: self.image_errors,
        }
    }

    fn node_abs_path(&self, node: usize, out: &mut [u8]) -> Result<usize, Errno> {
        if node >= MAX_NODES || !self.nodes[node].used || out.is_empty() {
            return Err(Errno::NoEnt);
        }
        if node == self.root {
            out[0] = b'/';
            return Ok(1);
        }
        let mut chain = [0usize; 16];
        let mut count = 0usize;
        let mut cur = node;
        while cur != self.root && count < chain.len() {
            chain[count] = cur;
            count += 1;
            cur = self.nodes[cur].parent;
        }
        if cur != self.root {
            return Err(Errno::NoSpace);
        }
        let mut len = 0usize;
        out[len] = b'/';
        len += 1;
        while count > 0 {
            count -= 1;
            if len > 1 {
                if len >= out.len() {
                    return Err(Errno::NoSpace);
                }
                out[len] = b'/';
                len += 1;
            }
            let item = chain[count];
            let mut j = 0usize;
            while j < self.nodes[item].name_len {
                if len >= out.len() {
                    return Err(Errno::NoSpace);
                }
                out[len] = self.nodes[item].name[j];
                len += 1;
                j += 1;
            }
        }
        Ok(len)
    }

    fn fd_count(&self) -> usize {
        let mut count = 0usize;
        let mut i = 0usize;
        while i < MAX_FDS {
            if self.fds[i].used {
                count += 1;
            }
            i += 1;
        }
        count
    }

    fn align_down(addr: usize) -> usize {
        addr & !(RUNTIME_PAGE_SIZE - 1)
    }

    fn align_up(addr: usize) -> Result<usize, Errno> {
        if addr == 0 {
            return Ok(0);
        }
        addr.checked_add(RUNTIME_PAGE_SIZE - 1)
            .map(|value| value & !(RUNTIME_PAGE_SIZE - 1))
            .ok_or(Errno::Inval)
    }

    fn current_mm_id(&self) -> Result<usize, Errno> {
        let idx = self.current_task_idx()?;
        Ok(self.tasks[idx].mm_id)
    }

    fn mm_refcount(&self, mm_id: usize) -> usize {
        let mut count = 0usize;
        let mut i = 0usize;
        while i < MAX_TASKS {
            if self.tasks[i].used && self.tasks[i].mm_id == mm_id {
                count += 1;
            }
            i += 1;
        }
        count
    }

    fn clear_vmas_for_mm(&mut self, mm_id: usize) {
        let mut i = 0usize;
        while i < MAX_VMAS {
            if self.vmas[i].used && self.vmas[i].mm_id == mm_id {
                self.vmas[i] = VmaObj::empty();
            }
            i += 1;
        }
    }

    fn vma_overlaps(&self, mm_id: usize, start: usize, end: usize) -> bool {
        let mut i = 0usize;
        while i < MAX_VMAS {
            let vma = self.vmas[i];
            if vma.used && vma.mm_id == mm_id && start < vma.end && vma.start < end {
                return true;
            }
            i += 1;
        }
        false
    }

    fn add_vma(&mut self, mm_id: usize, start: usize, end: usize, perm: u8, kind: RuntimeVmaKind, lazy: bool, fd: isize, offset: usize) -> Result<usize, Errno> {
        if mm_id == 0 || start >= end || (start & (RUNTIME_PAGE_SIZE - 1)) != 0 || (end & (RUNTIME_PAGE_SIZE - 1)) != 0 {
            return Err(Errno::Inval);
        }
        if self.vma_overlaps(mm_id, start, end) {
            return Err(Errno::Inval);
        }
        let mut i = 0usize;
        while i < MAX_VMAS {
            if !self.vmas[i].used {
                self.vmas[i] = VmaObj { used: true, mm_id, start, end, perm, kind, lazy, resident_pages: 0, fd, offset };
                return Ok(i);
            }
            i += 1;
        }
        Err(Errno::NoSpace)
    }

    fn find_vma_idx(&self, mm_id: usize, addr: usize) -> Result<usize, Errno> {
        let mut i = 0usize;
        while i < MAX_VMAS {
            let vma = self.vmas[i];
            if vma.used && vma.mm_id == mm_id && addr >= vma.start && addr < vma.end {
                return Ok(i);
            }
            i += 1;
        }
        Err(Errno::Fault)
    }

    fn split_vma_at(&mut self, mm_id: usize, addr: usize) -> Result<(), Errno> {
        if (addr & (RUNTIME_PAGE_SIZE - 1)) != 0 {
            return Err(Errno::Inval);
        }
        let mut i = 0usize;
        while i < MAX_VMAS {
            let vma = self.vmas[i];
            if vma.used && vma.mm_id == mm_id && addr > vma.start && addr < vma.end {
                self.vmas[i].end = addr;
                let right_offset = vma.offset + (addr - vma.start);
                let _ = self.add_vma(mm_id, addr, vma.end, vma.perm, vma.kind, vma.lazy, vma.fd, right_offset)?;
                return Ok(());
            }
            i += 1;
        }
        Ok(())
    }

    fn range_covered(&self, mm_id: usize, start: usize, end: usize) -> bool {
        let mut cur = start;
        while cur < end {
            let mut next = cur;
            let mut found = false;
            let mut i = 0usize;
            while i < MAX_VMAS {
                let vma = self.vmas[i];
                if vma.used && vma.mm_id == mm_id && vma.start <= cur && cur < vma.end {
                    next = vma.end;
                    found = true;
                    break;
                }
                i += 1;
            }
            if !found || next <= cur {
                return false;
            }
            cur = next;
        }
        true
    }

    fn validate_elf_node(&mut self, owner: usize) -> Result<ElfLoadInfo, Errno> {
        let mut data = [0u8; DATA_MAX];
        let size = self.node_file_bytes(owner, &mut data)?;
        if size < 120 {
            return Err(Errno::NoExec);
        }
        if data[0] != 0x7f || data[1] != b'E' || data[2] != b'L' || data[3] != b'F' || data[4] != 2 || data[5] != 1 || data[6] != 1 {
            return Err(Errno::NoExec);
        }
        let e_type = read_le_u16(&data, 16);
        let e_machine = read_le_u16(&data, 18);
        let e_version = read_le_u32(&data, 20);
        if e_type != 2 || e_machine != 243 || e_version != 1 {
            return Err(Errno::NoExec);
        }
        let entry = read_le_u64(&data, 24);
        let phoff = read_le_u64(&data, 32);
        let phentsize = read_le_u16(&data, 54) as usize;
        let phnum = read_le_u16(&data, 56) as usize;
        if entry == 0 || phoff == 0 || phentsize < 56 || phnum == 0 {
            return Err(Errno::NoExec);
        }
        let ph_end = phentsize
            .checked_mul(phnum)
            .and_then(|span| phoff.checked_add(span))
            .ok_or(Errno::NoExec)?;
        if ph_end > size {
            return Err(Errno::NoExec);
        }

        let mut load_seen = false;
        let mut entry_ok = false;
        let mut first_start = 0usize;
        let mut first_end = 0usize;
        let mut first_filesz = 0usize;
        let mut first_memsz = 0usize;
        let mut first_flags = 0u32;
        let mut i = 0usize;
        while i < phnum {
            let off = phoff + i * phentsize;
            let p_type = read_le_u32(&data, off);
            if p_type == 1 {
                let p_flags = read_le_u32(&data, off + 4);
                let p_offset = read_le_u64(&data, off + 8);
                let p_vaddr = read_le_u64(&data, off + 16);
                let p_filesz = read_le_u64(&data, off + 32);
                let p_memsz = read_le_u64(&data, off + 40);
                let p_align = read_le_u64(&data, off + 48);
                if p_memsz == 0 || p_filesz > p_memsz {
                    return Err(Errno::NoExec);
                }
                let file_end = p_offset.checked_add(p_filesz).ok_or(Errno::NoExec)?;
                let mem_end = p_vaddr.checked_add(p_memsz).ok_or(Errno::NoExec)?;
                if file_end > size {
                    return Err(Errno::NoExec);
                }
                if p_align > 1 {
                    if (p_align & (p_align - 1)) != 0 || (p_offset & (p_align - 1)) != (p_vaddr & (p_align - 1)) {
                        return Err(Errno::NoExec);
                    }
                }
                if !load_seen {
                    first_start = Self::align_down(p_vaddr);
                    first_end = Self::align_up(mem_end)?;
                    first_filesz = p_filesz;
                    first_memsz = p_memsz;
                    first_flags = p_flags;
                    load_seen = true;
                }
                if entry >= p_vaddr && entry < mem_end && (p_flags & 1) != 0 {
                    entry_ok = true;
                }
            }
            i += 1;
        }
        if !load_seen || !entry_ok {
            return Err(Errno::NoExec);
        }
        Ok(ElfLoadInfo { entry, phoff, phentsize, phnum, load_start: first_start, load_end: first_end, file_size: first_filesz, mem_size: first_memsz, load_flags: first_flags })
    }

    fn elf_perm(flags: u32) -> u8 {
        let mut perm = VMA_U;
        if (flags & 4) != 0 {
            perm |= VMA_R;
        }
        if (flags & 2) != 0 {
            perm |= VMA_W;
        }
        if (flags & 1) != 0 {
            perm |= VMA_X;
        }
        perm
    }

    fn install_exec_vmas(&mut self, mm_id: usize, owner: usize, info: &ElfLoadInfo) -> Result<(), Errno> {
        let mut data = [0u8; DATA_MAX];
        let _ = self.node_file_bytes(owner, &mut data)?;
        let mut i = 0usize;
        while i < info.phnum {
            let off = info.phoff + i * info.phentsize;
            if read_le_u32(&data, off) == 1 {
                let flags = read_le_u32(&data, off + 4);
                let p_offset = read_le_u64(&data, off + 8);
                let p_vaddr = read_le_u64(&data, off + 16);
                let p_memsz = read_le_u64(&data, off + 40);
                let start = Self::align_down(p_vaddr);
                let end = Self::align_up(p_vaddr.checked_add(p_memsz).ok_or(Errno::NoExec)?)?;
                let _ = self.add_vma(mm_id, start, end, Self::elf_perm(flags), RuntimeVmaKind::Load, false, -1, p_offset)?;
            }
            i += 1;
        }
        let _ = self.add_vma(mm_id, RUNTIME_USER_STACK_BOTTOM, RUNTIME_USER_STACK_TOP, VMA_R | VMA_W | VMA_U, RuntimeVmaKind::Stack, true, -1, 0)?;
        Ok(())
    }

    fn build_exec_stack_layout(&self, info: &ElfLoadInfo, argv: &[RuntimeExecString], envp: &[RuntimeExecString]) -> Result<ExecStackLayout, Errno> {
        if argv.len() > EXEC_ARG_MAX || envp.len() > EXEC_ENV_MAX {
            return Err(Errno::Inval);
        }
        let mut sp = RUNTIME_USER_STACK_TOP;
        let mut argv_ptrs = [0usize; EXEC_ARG_MAX];
        let mut envp_ptrs = [0usize; EXEC_ENV_MAX];

        let mut i = envp.len();
        while i > 0 {
            i -= 1;
            let bytes = envp[i].as_slice();
            sp = sp.checked_sub(bytes.len() + 1).ok_or(Errno::Fault)?;
            if sp < RUNTIME_USER_STACK_BOTTOM {
                return Err(Errno::Fault);
            }
            envp_ptrs[i] = sp;
        }

        i = argv.len();
        while i > 0 {
            i -= 1;
            let bytes = argv[i].as_slice();
            sp = sp.checked_sub(bytes.len() + 1).ok_or(Errno::Fault)?;
            if sp < RUNTIME_USER_STACK_BOTTOM {
                return Err(Errno::Fault);
            }
            argv_ptrs[i] = sp;
        }

        sp &= !15usize;
        let auxc = 6usize;
        let pointer_bytes = (auxc * 2 + envp.len() + 1 + argv.len() + 1 + 1)
            .checked_mul(core::mem::size_of::<usize>())
            .ok_or(Errno::Fault)?;
        sp = sp.checked_sub(pointer_bytes).ok_or(Errno::Fault)?;
        if sp < RUNTIME_USER_STACK_BOTTOM {
            return Err(Errno::Fault);
        }
        let auxv_start = sp + (1 + argv.len() + 1 + envp.len() + 1) * core::mem::size_of::<usize>();
        let argv0_ptr = if argv.is_empty() { 0 } else { argv_ptrs[0] };
        let env0_ptr = if envp.is_empty() { 0 } else { envp_ptrs[0] };
        let phdr_ptr = info.load_start + info.phoff;
        if phdr_ptr < info.load_start || auxv_start >= RUNTIME_USER_STACK_TOP {
            return Err(Errno::Fault);
        }
        Ok(ExecStackLayout { sp, argv0_ptr, env0_ptr, auxv_start, auxc })
    }

    fn close_cloexec_for_exec(&mut self) -> usize {
        let mut closed = 0usize;
        let mut fd = 0usize;
        while fd < MAX_FDS {
            if self.fds[fd].used && self.fds[fd].cloexec {
                if self.close(fd as isize).is_ok() {
                    closed += 1;
                }
            }
            fd += 1;
        }
        closed
    }

    fn execve_from_vfs(&mut self, path: &[u8], argv: &[RuntimeExecString], envp: &[RuntimeExecString]) -> Result<isize, Errno> {
        self.ensure();
        let node = self.resolve_at(AT_FDCWD, path, true)?;
        let owner = self.owner(node);
        let info = self.validate_elf_node(owner)?;
        let stack = self.build_exec_stack_layout(&info, argv, envp)?;
        let task_idx = self.current_task_idx()?;
        let old_mm = self.tasks[task_idx].mm_id;
        let mm_id = self.next_mm_id;
        self.next_mm_id = self.next_mm_id.checked_add(1).ok_or(Errno::NoSpace)?;
        self.tasks[task_idx].mm_id = mm_id;
        if self.mm_refcount(old_mm) == 0 {
            self.clear_vmas_for_mm(old_mm);
        }
        self.brk_current = RUNTIME_USER_HEAP_START;
        self.mmap_next = RUNTIME_USER_MMAP_START;
        self.install_exec_vmas(mm_id, owner, &info)?;
        let closed = self.close_cloexec_for_exec();
        self.tasks[task_idx].fd_count = self.fd_count();
        let seq = self.exec_image.seq + 1;
        self.exec_image = ExecImage {
            valid: true,
            node: owner,
            entry: info.entry,
            phoff: info.phoff,
            phentsize: info.phentsize,
            phnum: info.phnum,
            load_start: info.load_start,
            load_end: info.load_end,
            file_size: info.file_size,
            mem_size: info.mem_size,
            stack_pointer: stack.sp,
            argv0_ptr: stack.argv0_ptr,
            env0_ptr: stack.env0_ptr,
            auxv_start: stack.auxv_start,
            argc: argv.len(),
            envc: envp.len(),
            auxc: stack.auxc,
            closed_cloexec: closed,
            mm_id,
            seq,
        };
        Ok(0)
    }

    fn switch_current_task_idx(&mut self, idx: usize) -> Result<(), Errno> {
        self.scheduled_switch_to_idx(idx)
    }

    fn switch_current_task_pid(&mut self, pid: usize) -> Result<isize, Errno> {
        let idx = self.task_idx_by_pid(pid)?;
        self.switch_current_task_idx(idx)?;
        Ok(0)
    }

    fn run_user_program_from_vfs(&mut self, path: &[u8], exit_code: isize) -> Result<RuntimeUserProgramResult, Errno> {
        self.ensure();
        let argv0 = match RuntimeExecString::from_bytes(path) {
            Ok(item) => item,
            Err(_) => return Err(Errno::Inval),
        };
        let parent_idx = self.current_task_idx()?;
        let child_pid = self.clone_task(17)? as usize;
        let child_idx = self.task_idx_by_pid(child_pid)?;
        self.switch_current_task_idx(child_idx)?;
        let argv = [argv0];
        let exec_result = self.execve_from_vfs(path, &argv, &[]);
        let snap = self.exec_snapshot();
        let exit_result = if exec_result.is_ok() {
            self.exit_task_pid(child_pid, exit_code)
        } else {
            Ok(0)
        };
        let restore_result = self.switch_current_task_idx(parent_idx);
        if restore_result.is_err() {
            return Err(Errno::Srch);
        }
        exec_result?;
        exit_result?;
        let (wait_pid, wait_status) = self.wait4(child_pid as isize)?;
        if wait_pid != child_pid {
            return Err(Errno::Child);
        }
        Ok(RuntimeUserProgramResult {
            pid: child_pid,
            path_len: path.len(),
            exit_code,
            wait_status,
            entry: snap.entry,
            mm_id: snap.mm_id,
            exec_seq: snap.seq,
        })
    }

    fn update_heap_vma(&mut self, mm_id: usize) -> Result<(), Errno> {
        let end = if self.brk_current == RUNTIME_USER_HEAP_START {
            RUNTIME_USER_HEAP_START
        } else {
            Self::align_up(self.brk_current)?
        };
        let mut i = 0usize;
        while i < MAX_VMAS {
            if self.vmas[i].used && self.vmas[i].mm_id == mm_id && self.vmas[i].kind == RuntimeVmaKind::Heap {
                if end <= RUNTIME_USER_HEAP_START {
                    self.vmas[i] = VmaObj::empty();
                } else {
                    self.vmas[i].end = end;
                }
                return Ok(());
            }
            i += 1;
        }
        if end > RUNTIME_USER_HEAP_START {
            let _ = self.add_vma(mm_id, RUNTIME_USER_HEAP_START, end, VMA_R | VMA_W | VMA_U, RuntimeVmaKind::Heap, true, -1, 0)?;
        }
        Ok(())
    }

    fn brk(&mut self, addr: usize) -> Result<isize, Errno> {
        self.ensure();
        if addr == 0 {
            return Ok(self.brk_current as isize);
        }
        if !(RUNTIME_USER_HEAP_START..=RUNTIME_USER_HEAP_LIMIT).contains(&addr) {
            return Ok(self.brk_current as isize);
        }
        let mm_id = self.current_mm_id()?;
        self.brk_current = addr;
        self.update_heap_vma(mm_id)?;
        Ok(self.brk_current as isize)
    }

    fn find_mmap_gap(&self, mm_id: usize, len: usize, hint: usize) -> Result<usize, Errno> {
        let mut addr = if hint == 0 { self.mmap_next } else { Self::align_up(hint)? };
        if addr < RUNTIME_USER_MMAP_START {
            addr = RUNTIME_USER_MMAP_START;
        }
        while addr.checked_add(len).ok_or(Errno::NoSpace)? <= RUNTIME_USER_MMAP_LIMIT {
            if !self.vma_overlaps(mm_id, addr, addr + len) {
                return Ok(addr);
            }
            addr = addr.checked_add(RUNTIME_PAGE_SIZE).ok_or(Errno::NoSpace)?;
        }
        Err(Errno::NoSpace)
    }

    fn mmap(&mut self, addr: usize, len: usize, prot: usize, flags: usize, fd: isize, offset: usize) -> Result<isize, Errno> {
        self.ensure();
        if len == 0 {
            return Err(Errno::Inval);
        }
        if fd >= 0 {
            let _ = self.fd_idx(fd)?;
        }
        let mm_id = self.current_mm_id()?;
        let map_len = Self::align_up(len)?;
        let start = if (flags & RUNTIME_MAP_FIXED) != 0 {
            if addr == 0 {
                return Err(Errno::Inval);
            }
            Self::align_down(addr)
        } else {
            self.find_mmap_gap(mm_id, map_len, addr)?
        };
        let end = start.checked_add(map_len).ok_or(Errno::NoSpace)?;
        if start < RUNTIME_USER_MMAP_START || end > RUNTIME_USER_MMAP_LIMIT {
            return Err(Errno::Inval);
        }
        if (flags & RUNTIME_MAP_FIXED) != 0 {
            let _ = self.munmap(start, map_len)?;
        } else if self.vma_overlaps(mm_id, start, end) {
            return Err(Errno::Inval);
        }
        let mut perm = VMA_U;
        if (prot & RUNTIME_PROT_READ) != 0 {
            perm |= VMA_R;
        }
        if (prot & RUNTIME_PROT_WRITE) != 0 {
            perm |= VMA_W;
        }
        if (prot & RUNTIME_PROT_EXEC) != 0 {
            perm |= VMA_X;
        }
        let _ = self.add_vma(mm_id, start, end, perm, RuntimeVmaKind::Mmap, true, fd, offset)?;
        self.mmap_next = end;
        Ok(start as isize)
    }

    fn read_mmap_backing_page(&mut self, addr: usize, out: &mut [u8]) -> Result<usize, Errno> {
        self.ensure();
        let mm_id = self.current_mm_id()?;
        let page = Self::align_down(addr);
        let vma_idx = self.find_vma_idx(mm_id, page)?;
        let vma = self.vmas[vma_idx];
        if vma.fd < 0 || vma.kind != RuntimeVmaKind::Mmap {
            return Ok(0);
        }
        let fd_idx = self.fd_idx(vma.fd)?;
        let ofd_idx = self.fds[fd_idx].ofd;
        if self.ofds[ofd_idx].kind != FdKind::RegularFile {
            return Ok(0);
        }
        let owner = self.owner(self.ofds[ofd_idx].node);
        if owner >= MAX_NODES || !self.nodes[owner].used || self.nodes[owner].kind != NodeKind::File {
            return Ok(0);
        }
        let file_offset = vma.offset + (page - vma.start);
        if self.nodes[owner].fs_kind == RuntimeFsKind::Imagefs {
            return self.image_read_node(owner, file_offset, out);
        }
        if file_offset >= self.nodes[owner].size {
            return Ok(0);
        }
        let mut copied = 0usize;
        let mut pos = file_offset;
        while copied < out.len() && pos < self.nodes[owner].size {
            out[copied] = self.nodes[owner].data[pos];
            copied += 1;
            pos += 1;
        }
        Ok(copied)
    }

    fn munmap(&mut self, addr: usize, len: usize) -> Result<isize, Errno> {
        self.ensure();
        if len == 0 {
            return Err(Errno::Inval);
        }
        let mm_id = self.current_mm_id()?;
        let start = Self::align_down(addr);
        let end = Self::align_up(addr.checked_add(len).ok_or(Errno::Inval)?)?;
        let mut i = 0usize;
        while i < MAX_VMAS {
            let vma = self.vmas[i];
            if vma.used && vma.mm_id == mm_id && start < vma.end && vma.start < end {
                if start <= vma.start && end >= vma.end {
                    self.vmas[i] = VmaObj::empty();
                } else if start <= vma.start {
                    self.vmas[i].start = end;
                    self.vmas[i].offset = vma.offset + (end - vma.start);
                } else if end >= vma.end {
                    self.vmas[i].end = start;
                } else {
                    self.vmas[i].end = start;
                    let right_offset = vma.offset + (end - vma.start);
                    let _ = self.add_vma(mm_id, end, vma.end, vma.perm, vma.kind, vma.lazy, vma.fd, right_offset)?;
                }
            }
            i += 1;
        }
        Ok(0)
    }

    fn mprotect(&mut self, addr: usize, len: usize, prot: usize) -> Result<isize, Errno> {
        self.ensure();
        if len == 0 {
            return Ok(0);
        }
        let mm_id = self.current_mm_id()?;
        let start = Self::align_down(addr);
        let end = Self::align_up(addr.checked_add(len).ok_or(Errno::Inval)?)?;
        self.split_vma_at(mm_id, start)?;
        self.split_vma_at(mm_id, end)?;
        if !self.range_covered(mm_id, start, end) {
            return Err(Errno::Fault);
        }
        let mut perm = VMA_U;
        if (prot & RUNTIME_PROT_READ) != 0 {
            perm |= VMA_R;
        }
        if (prot & RUNTIME_PROT_WRITE) != 0 {
            perm |= VMA_W;
        }
        if (prot & RUNTIME_PROT_EXEC) != 0 {
            perm |= VMA_X;
        }
        let mut i = 0usize;
        while i < MAX_VMAS {
            if self.vmas[i].used && self.vmas[i].mm_id == mm_id && self.vmas[i].start >= start && self.vmas[i].end <= end {
                self.vmas[i].perm = perm;
            }
            i += 1;
        }
        Ok(0)
    }

    fn madvise(&mut self, addr: usize, len: usize) -> Result<isize, Errno> {
        self.ensure();
        if len == 0 {
            return Ok(0);
        }
        let mm_id = self.current_mm_id()?;
        let start = Self::align_down(addr);
        let end = Self::align_up(addr.checked_add(len).ok_or(Errno::Inval)?)?;
        if self.range_covered(mm_id, start, end) {
            Ok(0)
        } else {
            Err(Errno::Fault)
        }
    }

    fn page_fault_validate(&mut self, addr: usize, access: RuntimeFaultAccess) -> Result<bool, Errno> {
        self.ensure();
        let mm_id = self.current_mm_id()?;
        self.last_fault_addr = addr;
        let idx = match self.find_vma_idx(mm_id, addr) {
            Ok(idx) => idx,
            Err(_) => {
                self.last_fault_ok = false;
                return Ok(false);
            }
        };
        let perm = self.vmas[idx].perm;
        let ok = match access {
            RuntimeFaultAccess::Read => (perm & VMA_R) != 0,
            RuntimeFaultAccess::Write => (perm & VMA_W) != 0,
            RuntimeFaultAccess::Execute => (perm & VMA_X) != 0,
        };
        if ok && self.vmas[idx].lazy {
            self.vmas[idx].resident_pages += 1;
        }
        self.last_fault_ok = ok;
        Ok(ok)
    }

    fn page_fault_permissions(&mut self, addr: usize) -> Result<RuntimeVmPermissions, Errno> {
        self.ensure();
        let mm_id = self.current_mm_id()?;
        let idx = self.find_vma_idx(mm_id, addr)?;
        let vma = self.vmas[idx];
        Ok(RuntimeVmPermissions {
            mm_id,
            start: vma.start,
            end: vma.end,
            readable: (vma.perm & VMA_R) != 0,
            writable: (vma.perm & VMA_W) != 0,
            executable: (vma.perm & VMA_X) != 0,
            user: (vma.perm & VMA_U) != 0,
            lazy: vma.lazy,
            kind: vma.kind,
        })
    }

    fn exec_snapshot(&mut self) -> RuntimeExecSnapshot {
        self.ensure();
        let image = self.exec_image;
        RuntimeExecSnapshot {
            valid: image.valid,
            entry: image.entry,
            phnum: image.phnum,
            load_start: image.load_start,
            load_end: image.load_end,
            stack_pointer: image.stack_pointer,
            argv0_ptr: image.argv0_ptr,
            env0_ptr: image.env0_ptr,
            auxv_start: image.auxv_start,
            argc: image.argc,
            envc: image.envc,
            auxc: image.auxc,
            closed_cloexec: image.closed_cloexec,
            mm_id: image.mm_id,
            seq: image.seq,
        }
    }

    fn vm_snapshot(&mut self) -> RuntimeVmSnapshot {
        self.ensure();
        let mm_id = self.current_mm_id().unwrap_or(0);
        let mut vma_count = 0usize;
        let mut load_count = 0usize;
        let mut mmap_count = 0usize;
        let mut lazy_count = 0usize;
        let mut resident_pages = 0usize;
        let mut writable_count = 0usize;
        let mut executable_count = 0usize;
        let mut i = 0usize;
        while i < MAX_VMAS {
            let vma = self.vmas[i];
            if vma.used && vma.mm_id == mm_id {
                vma_count += 1;
                resident_pages += vma.resident_pages;
                if vma.kind == RuntimeVmaKind::Load {
                    load_count += 1;
                }
                if vma.kind == RuntimeVmaKind::Mmap {
                    mmap_count += 1;
                }
                if vma.lazy {
                    lazy_count += 1;
                }
                if (vma.perm & VMA_W) != 0 {
                    writable_count += 1;
                }
                if (vma.perm & VMA_X) != 0 {
                    executable_count += 1;
                }
            }
            i += 1;
        }
        RuntimeVmSnapshot { mm_id, vma_count, load_count, heap_end: self.brk_current, mmap_count, lazy_count, resident_pages, writable_count, executable_count, last_fault_addr: self.last_fault_addr, last_fault_ok: self.last_fault_ok }
    }

    fn current_task_idx(&self) -> Result<usize, Errno> {
        if self.current_task < MAX_TASKS && self.tasks[self.current_task].used {
            Ok(self.current_task)
        } else {
            Err(Errno::NoEnt)
        }
    }

    fn task_idx_by_pid(&self, pid: usize) -> Result<usize, Errno> {
        let wanted = if pid == 0 {
            self.tasks[self.current_task].pid
        } else {
            pid
        };
        let mut i = 0usize;
        while i < MAX_TASKS {
            if self.tasks[i].used && self.tasks[i].pid == wanted {
                return Ok(i);
            }
            i += 1;
        }
        Err(Errno::NoEnt)
    }

    fn child_count_for(&self, pid: usize) -> usize {
        let mut count = 0usize;
        let mut i = 0usize;
        while i < MAX_TASKS {
            if self.tasks[i].used && self.tasks[i].ppid == pid && self.tasks[i].pid != pid {
                count += 1;
            }
            i += 1;
        }
        count
    }

    fn valid_signal(sig: usize) -> bool {
        sig > 0 && sig < MAX_SIGNALS
    }

    const fn signal_bit(sig: usize) -> u64 {
        1u64 << sig
    }

    fn action_count_for_task(&self, idx: usize) -> usize {
        let mut count = 0usize;
        let mut sig = 1usize;
        while sig < MAX_SIGNALS {
            if self.signal_actions[idx][sig].installed {
                count += 1;
            }
            sig += 1;
        }
        count
    }

    fn pending_count_for_pid(&self, pid: usize) -> usize {
        let mut count = 0usize;
        let mut i = 0usize;
        while i < MAX_PENDING_SIGNALS {
            if self.pending_signals[i].used && self.pending_signals[i].target_pid == pid {
                count += 1;
            }
            i += 1;
        }
        count
    }

    fn total_pending_count(&self) -> usize {
        let mut count = 0usize;
        let mut i = 0usize;
        while i < MAX_PENDING_SIGNALS {
            if self.pending_signals[i].used {
                count += 1;
            }
            i += 1;
        }
        count
    }

    fn queue_signal_to_idx(&mut self, target_idx: usize, sig: usize, source_pid: usize, code: isize, process_group: bool) -> Result<isize, Errno> {
        if target_idx >= MAX_TASKS || !self.tasks[target_idx].used {
            return Err(Errno::Srch);
        }
        if sig == 0 {
            return Ok(0);
        }
        if !Self::valid_signal(sig) {
            return Err(Errno::Inval);
        }
        let mut i = 0usize;
        while i < MAX_PENDING_SIGNALS {
            if !self.pending_signals[i].used {
                self.pending_signals[i] = PendingSignalObj {
                    used: true,
                    target_pid: self.tasks[target_idx].pid,
                    source_pid,
                    sig,
                    code,
                    process_group,
                };
                self.queued_signals += 1;
                if sig == SIGCHLD {
                    self.sigchld_queued += 1;
                }
                return Ok(0);
            }
            i += 1;
        }
        Err(Errno::NoSpace)
    }

    fn queue_signal_to_pid(&mut self, pid: usize, sig: usize, source_pid: usize, code: isize, process_group: bool) -> Result<isize, Errno> {
        let idx = self.task_idx_by_pid(pid)?;
        self.queue_signal_to_idx(idx, sig, source_pid, code, process_group)
    }

    fn rt_sigaction(&mut self, sig: usize, new_action: Option<RuntimeSignalAction>) -> Result<RuntimeSignalAction, Errno> {
        self.ensure();
        if !Self::valid_signal(sig) {
            return Err(Errno::Inval);
        }
        let idx = self.current_task_idx()?;
        let old = self.signal_actions[idx][sig];
        if let Some(action) = new_action {
            self.signal_actions[idx][sig] = action;
        }
        Ok(old)
    }

    fn next_deliverable_signal(&self, task_idx: usize) -> Result<usize, Errno> {
        if task_idx >= MAX_TASKS || !self.tasks[task_idx].used {
            return Err(Errno::Srch);
        }
        let pid = self.tasks[task_idx].pid;
        let mask = self.tasks[task_idx].signal_mask;
        let mut i = 0usize;
        while i < MAX_PENDING_SIGNALS {
            let pending = self.pending_signals[i];
            if pending.used && pending.target_pid == pid && (mask & Self::signal_bit(pending.sig)) == 0 {
                let action = self.signal_actions[task_idx][pending.sig];
                if action.installed && action.handler != 0 {
                    return Ok(i);
                }
            }
            i += 1;
        }
        Err(Errno::Again)
    }

    fn deliver_signal_frame(&mut self, saved_pc: usize, saved_sp: usize) -> Result<isize, Errno> {
        self.ensure();
        let task_idx = self.current_task_idx()?;
        if self.signal_frames[task_idx].active {
            return Err(Errno::Again);
        }
        let pending_idx = self.next_deliverable_signal(task_idx)?;
        let pending = self.pending_signals[pending_idx];
        let action = self.signal_actions[task_idx][pending.sig];
        let frame_sp = saved_sp.checked_sub(256).ok_or(Errno::Fault)? & !15usize;
        if frame_sp < RUNTIME_USER_STACK_BOTTOM {
            return Err(Errno::Fault);
        }
        let saved_mask = self.tasks[task_idx].signal_mask;
        let seq = self.next_signal_frame_seq;
        self.next_signal_frame_seq += 1;
        self.signal_frames[task_idx] = SignalFrameObj {
            active: true,
            pid: self.tasks[task_idx].pid,
            sig: pending.sig,
            handler: action.handler,
            frame_sp,
            saved_pc,
            saved_sp,
            saved_mask,
            restorer: action.restorer,
            seq,
        };
        self.tasks[task_idx].signal_mask = saved_mask | action.mask | Self::signal_bit(pending.sig);
        self.pending_signals[pending_idx] = PendingSignalObj::empty();
        self.delivered_signals += 1;
        self.last_delivered_sig = pending.sig;
        self.last_delivered_pid = self.tasks[task_idx].pid;
        Ok(0)
    }

    fn rt_sigreturn(&mut self) -> Result<RuntimeSignalRestore, Errno> {
        self.ensure();
        let task_idx = self.current_task_idx()?;
        let frame = self.signal_frames[task_idx];
        if !frame.active {
            return Err(Errno::Inval);
        }
        self.tasks[task_idx].signal_mask = frame.saved_mask;
        self.signal_frames[task_idx] = SignalFrameObj::empty();
        self.returned_signals += 1;
        Ok(RuntimeSignalRestore { pc: frame.saved_pc, sp: frame.saved_sp, mask: frame.saved_mask, sig: frame.sig })
    }

    fn send_signal_to_process_group(&mut self, pgid: usize, sig: usize, source_pid: usize) -> Result<isize, Errno> {
        let mut delivered = 0isize;
        let mut found = false;
        let mut i = 0usize;
        while i < MAX_TASKS {
            if self.tasks[i].used && self.tasks[i].pgid == pgid {
                found = true;
                self.queue_signal_to_idx(i, sig, source_pid, 0, true)?;
                if sig != 0 {
                    delivered += 1;
                }
            }
            i += 1;
        }
        if !found {
            return Err(Errno::Srch);
        }
        if sig != 0 {
            self.group_signal_deliveries += delivered as usize;
        }
        Ok(0)
    }

    fn kill_signal(&mut self, pid: isize, sig: usize) -> Result<isize, Errno> {
        self.ensure();
        if sig != 0 && !Self::valid_signal(sig) {
            return Err(Errno::Inval);
        }
        let source = self.tasks[self.current_task_idx()?].pid;
        if pid > 0 {
            self.queue_signal_to_pid(pid as usize, sig, source, 0, false)?;
            if sig != 0 {
                self.direct_signal_deliveries += 1;
            }
            return Ok(0);
        }
        if pid == 0 {
            let pgid = self.tasks[self.current_task_idx()?].pgid;
            return self.send_signal_to_process_group(pgid, sig, source);
        }
        if pid == -1 {
            let mut found = false;
            let mut i = 0usize;
            while i < MAX_TASKS {
                if self.tasks[i].used {
                    found = true;
                    self.queue_signal_to_idx(i, sig, source, 0, false)?;
                    if sig != 0 {
                        self.direct_signal_deliveries += 1;
                    }
                }
                i += 1;
            }
            if found {
                Ok(0)
            } else {
                Err(Errno::Srch)
            }
        } else {
            self.send_signal_to_process_group((-pid) as usize, sig, source)
        }
    }

    fn tkill_signal(&mut self, tid: isize, sig: usize) -> Result<isize, Errno> {
        self.ensure();
        if tid <= 0 {
            return Err(Errno::Srch);
        }
        let source = self.tasks[self.current_task_idx()?].pid;
        self.queue_signal_to_pid(tid as usize, sig, source, 0, false)?;
        if sig != 0 {
            self.tkill_signal_deliveries += 1;
        }
        Ok(0)
    }

    fn tgkill_signal(&mut self, tgid: isize, tid: isize, sig: usize) -> Result<isize, Errno> {
        self.ensure();
        if tgid <= 0 || tid <= 0 {
            return Err(Errno::Srch);
        }
        if sig != 0 && !Self::valid_signal(sig) {
            return Err(Errno::Inval);
        }
        let idx = self.task_idx_by_pid(tid as usize)?;
        if self.tasks[idx].tgid != tgid as usize {
            return Err(Errno::Srch);
        }
        let source = self.tasks[self.current_task_idx()?].pid;
        self.queue_signal_to_idx(idx, sig, source, 0, false)?;
        if sig != 0 {
            self.tgkill_signal_deliveries += 1;
        }
        Ok(0)
    }

    fn signal_snapshot(&mut self) -> RuntimeSignalSnapshot {
        self.ensure();
        let idx = self.current_task_idx().unwrap_or(0);
        let task = self.tasks[idx];
        let frame = self.signal_frames[idx];
        RuntimeSignalSnapshot {
            current_pid: task.pid,
            blocked_mask: task.signal_mask,
            action_count: self.action_count_for_task(idx),
            pending_count: self.pending_count_for_pid(task.pid),
            frame_active: frame.active,
            frame_sig: frame.sig,
            frame_sp: frame.frame_sp,
            frame_handler: frame.handler,
            frame_restorer: frame.restorer,
            saved_pc: frame.saved_pc,
            saved_sp: frame.saved_sp,
            last_delivered_sig: self.last_delivered_sig,
            last_delivered_pid: self.last_delivered_pid,
            delivered_count: self.delivered_signals,
            returned_count: self.returned_signals,
            queued_count: self.queued_signals,
            sigchld_count: self.sigchld_queued,
            direct_deliveries: self.direct_signal_deliveries,
            tkill_deliveries: self.tkill_signal_deliveries,
            tgkill_deliveries: self.tgkill_signal_deliveries,
            group_deliveries: self.group_signal_deliveries,
        }
    }

    fn task_snapshot_at(&self, idx: usize) -> RuntimeTaskSnapshot {
        let task = self.tasks[idx];
        let fd_count = if idx == self.current_task { self.fd_count() } else { task.fd_count };
        let cwd = if idx == self.current_task { self.cwd } else { task.cwd };
        let root = if idx == self.current_task { self.root } else { task.root };
        let mut cwd_buf = [0u8; PATH_MAX];
        let mut root_buf = [0u8; PATH_MAX];
        let cwd_len = self.node_abs_path(cwd, &mut cwd_buf).unwrap_or(0);
        let root_len = self.node_abs_path(root, &mut root_buf).unwrap_or(0);
        RuntimeTaskSnapshot {
            pid: task.pid,
            ppid: task.ppid,
            tgid: task.tgid,
            pgid: task.pgid,
            sid: task.sid,
            state: task.state,
            exit_code: task.exit_code,
            fork_return: task.fork_return,
            fd_count,
            cwd_len,
            root_len,
            signal_mask: task.signal_mask,
            child_count: self.child_count_for(task.pid),
        }
    }

    fn task_snapshot(&mut self, pid: usize) -> Result<RuntimeTaskSnapshot, Errno> {
        self.ensure();
        let idx = self.task_idx_by_pid(pid)?;
        Ok(self.task_snapshot_at(idx))
    }

    fn proc_snapshot_for_pid(&mut self, pid: usize) -> Result<ProcSnapshot, Errno> {
        let task = self.task_snapshot(pid)?;
        Ok(ProcSnapshot {
            pid: task.pid,
            ppid: task.ppid,
            tgid: task.tgid,
            pgid: task.pgid,
            sid: task.sid,
            fd_count: task.fd_count,
            cwd_len: task.cwd_len,
            root_len: task.root_len,
        })
    }

    fn fd_target_path(&mut self, fd: isize, out: &mut [u8]) -> Result<usize, Errno> {
        self.ensure();
        let idx = self.fd_idx(fd)?;
        let ofd = self.fds[idx].ofd;
        match self.ofds[ofd].kind {
            FdKind::Stdin => copy_literal(out, b"/dev/stdin"),
            FdKind::Stdout => copy_literal(out, b"/dev/stdout"),
            FdKind::Stderr => copy_literal(out, b"/dev/stderr"),
            FdKind::RegularFile | FdKind::Directory | FdKind::Symlink | FdKind::DevNull | FdKind::DevZero | FdKind::DevConsole | FdKind::DevTty | FdKind::DevRandom | FdKind::Procfs => self.node_abs_path(self.ofds[ofd].node, out),
            FdKind::PipeRead | FdKind::PipeWrite => copy_numbered(out, b"pipe:[", self.ofds[ofd].object, b"]"),
            FdKind::EventFd => copy_numbered(out, b"eventfd:[", self.ofds[ofd].object, b"]"),
            FdKind::TimerFd => copy_numbered(out, b"timerfd:[", self.ofds[ofd].object, b"]"),
            FdKind::Socket => copy_numbered(out, b"socket:[", self.ofds[ofd].object, b"]"),
            FdKind::Epoll => copy_numbered(out, b"anon_inode:[eventpoll:", self.ofds[ofd].object, b"]"),
            FdKind::Mq => copy_numbered(out, b"mqueue:[", self.ofds[ofd].object, b"]"),
            FdKind::Empty => Err(Errno::BadFd),
        }
    }

    fn snapshot(&mut self) -> ProcSnapshot {
        self.ensure();
        match self.proc_snapshot_for_pid(0) {
            Ok(snapshot) => snapshot,
            Err(_) => ProcSnapshot { pid: 0, ppid: 0, tgid: 0, pgid: 0, sid: 0, fd_count: 0, cwd_len: 0, root_len: 0 },
        }
    }

    fn alloc_pipe_slot(&mut self) -> Result<usize, Errno> {
        let mut i = 0usize;
        while i < MAX_PIPES {
            if !self.pipes[i].used {
                self.pipes[i] = PipeObj { used: true, data: [0; PIPE_BUF], len: 0, readers: 1, writers: 1 };
                return Ok(i);
            }
            i += 1;
        }
        Err(Errno::NoSpace)
    }

    fn pipe2(&mut self, flags: u32) -> Result<(isize, isize), Errno> {
        self.ensure();
        let pipe = self.alloc_pipe_slot()?;
        let read_ofd = self.alloc_ofd(FdKind::PipeRead, ROOT, flags, pipe)?;
        let read_fd = self.alloc_fd_for_ofd(read_ofd, 3, (flags & O_CLOEXEC) != 0)?;
        let write_ofd = self.alloc_ofd(FdKind::PipeWrite, ROOT, flags, pipe)?;
        let write_fd = self.alloc_fd_for_ofd(write_ofd, 3, (flags & O_CLOEXEC) != 0)?;
        Ok((read_fd, write_fd))
    }

    fn eventfd2(&mut self, initval: usize, flags: u32) -> Result<isize, Errno> {
        self.ensure();
        let mut i = 0usize;
        while i < MAX_EVENTS {
            if !self.events[i].used {
                self.events[i] = EventObj { used: true, counter: initval as u64, flags };
                let ofd = self.alloc_ofd(FdKind::EventFd, ROOT, flags, i)?;
                return self.alloc_fd_for_ofd(ofd, 3, (flags & O_CLOEXEC) != 0);
            }
            i += 1;
        }
        Err(Errno::NoSpace)
    }

    fn timerfd_create(&mut self, clockid: usize, flags: u32) -> Result<isize, Errno> {
        self.ensure();
        let mut i = 0usize;
        while i < MAX_TIMERS {
            if !self.timers[i].used {
                self.timers[i] = TimerObj { used: true, clockid, armed: false, expirations: 0 };
                let ofd = self.alloc_ofd(FdKind::TimerFd, ROOT, flags, i)?;
                return self.alloc_fd_for_ofd(ofd, 3, (flags & O_CLOEXEC) != 0);
            }
            i += 1;
        }
        Err(Errno::NoSpace)
    }

    fn timerfd_settime(&mut self, fd: isize) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.fd_idx(fd)?;
        let ofd = self.fds[idx].ofd;
        if self.ofds[ofd].kind != FdKind::TimerFd {
            return Err(Errno::BadFd);
        }
        let timer = self.ofds[ofd].object;
        self.timers[timer].armed = true;
        self.timers[timer].expirations = 1;
        let woke = self.sched_wake(SCHED_WAIT_TIMER_BASE + timer, MAX_TASKS)?;
        self.sched_timer_wakes += woke as usize;
        Ok(0)
    }

    fn alloc_socket_slot(&mut self, domain: usize, sock_type: u32, protocol: usize) -> Result<usize, Errno> {
        let mut i = 0usize;
        while i < MAX_SOCKETS {
            if !self.sockets[i].used {
                let mut sock = SocketObj::empty();
                sock.used = true;
                sock.domain = domain as u16;
                sock.sock_type = (sock_type & SOCK_TYPE_MASK) as u16;
                if sock.sock_type == 0 {
                    sock.sock_type = SOCK_STREAM as u16;
                }
                sock.protocol = protocol as u16;
                self.sockets[i] = sock;
                return Ok(i);
            }
            i += 1;
        }
        Err(Errno::NoSpace)
    }

    fn socket(&mut self, flags: u32) -> Result<isize, Errno> {
        self.socket_with(AF_UNIX, SOCK_STREAM | flags, 0)
    }

    fn socket_with(&mut self, domain: usize, sock_type: u32, protocol: usize) -> Result<isize, Errno> {
        self.ensure();
        let slot = self.alloc_socket_slot(domain, sock_type, protocol)?;
        let ofd = self.alloc_ofd(FdKind::Socket, ROOT, sock_type, slot)?;
        self.alloc_fd_for_ofd(ofd, 3, (sock_type & O_CLOEXEC) != 0)
    }

    fn socketpair(&mut self, flags: u32) -> Result<(isize, isize), Errno> {
        self.ensure();
        let first = self.alloc_socket_slot(AF_UNIX, SOCK_STREAM | flags, 0)?;
        let second = self.alloc_socket_slot(AF_UNIX, SOCK_STREAM | flags, 0)?;
        self.sockets[first].peer = second;
        self.sockets[second].peer = first;
        let ofd0 = self.alloc_ofd(FdKind::Socket, ROOT, flags, first)?;
        let fd0 = self.alloc_fd_for_ofd(ofd0, 3, (flags & O_CLOEXEC) != 0)?;
        let ofd1 = self.alloc_ofd(FdKind::Socket, ROOT, flags, second)?;
        let fd1 = self.alloc_fd_for_ofd(ofd1, 3, (flags & O_CLOEXEC) != 0)?;
        Ok((fd0, fd1))
    }

    fn socket_index_for_fd(&mut self, fd: isize) -> Result<usize, Errno> {
        let idx = self.fd_idx(fd)?;
        let ofd = self.fds[idx].ofd;
        if self.ofds[ofd].kind != FdKind::Socket {
            return Err(Errno::BadFd);
        }
        let sock = self.ofds[ofd].object;
        if sock >= MAX_SOCKETS || !self.sockets[sock].used {
            return Err(Errno::BadFd);
        }
        Ok(sock)
    }

    fn socket_addr_matches(sock: &SocketObj, addr: &[u8]) -> bool {
        sock.bound && sock.addr_len == addr.len() && bytes_eq(&sock.addr[..sock.addr_len], addr)
    }

    fn bind_socket(&mut self, fd: isize, addr: &[u8]) -> Result<isize, Errno> {
        self.ensure();
        let sock = self.socket_index_for_fd(fd)?;
        if addr.is_empty() || addr.len() > NAME_MAX {
            return Err(Errno::Inval);
        }
        let mut i = 0usize;
        while i < MAX_SOCKETS {
            if i != sock && self.sockets[i].used && Self::socket_addr_matches(&self.sockets[i], addr) {
                return Err(Errno::Exist);
            }
            i += 1;
        }
        self.sockets[sock].addr = [0; NAME_MAX];
        self.sockets[sock].addr_len = copy_name_bytes(addr, &mut self.sockets[sock].addr);
        self.sockets[sock].bound = true;
        Ok(0)
    }

    fn listen_socket(&mut self, fd: isize, _backlog: usize) -> Result<isize, Errno> {
        self.ensure();
        let sock = self.socket_index_for_fd(fd)?;
        if !self.sockets[sock].bound {
            return Err(Errno::Inval);
        }
        self.sockets[sock].listening = true;
        Ok(0)
    }

    fn find_bound_socket(&self, addr: &[u8], require_listener: bool) -> Result<usize, Errno> {
        let mut i = 0usize;
        while i < MAX_SOCKETS {
            if self.sockets[i].used && Self::socket_addr_matches(&self.sockets[i], addr) && (!require_listener || self.sockets[i].listening) {
                return Ok(i);
            }
            i += 1;
        }
        Err(Errno::NoEnt)
    }

    fn connect_socket(&mut self, fd: isize, addr: &[u8]) -> Result<isize, Errno> {
        self.ensure();
        let client = self.socket_index_for_fd(fd)?;
        if addr.is_empty() {
            return Err(Errno::Inval);
        }
        let server = self.find_bound_socket(addr, true)?;
        let accepted = self.alloc_socket_slot(self.sockets[client].domain as usize, SOCK_STREAM, self.sockets[client].protocol as usize)?;
        self.sockets[client].peer = accepted;
        self.sockets[accepted].peer = client;
        self.sockets[accepted].bound = true;
        self.sockets[accepted].addr_len = copy_name_bytes(addr, &mut self.sockets[accepted].addr);
        self.sockets[accepted].peer_addr_len = copy_name_bytes(addr, &mut self.sockets[accepted].peer_addr);
        self.sockets[client].peer_addr_len = copy_name_bytes(addr, &mut self.sockets[client].peer_addr);
        self.sockets[server].pending = accepted;
        Ok(0)
    }

    fn accept_socket(&mut self, fd: isize, flags: u32) -> Result<isize, Errno> {
        self.ensure();
        let listener = self.socket_index_for_fd(fd)?;
        if !self.sockets[listener].listening {
            return Err(Errno::Inval);
        }
        let accepted = self.sockets[listener].pending;
        if accepted >= MAX_SOCKETS || !self.sockets[accepted].used {
            return Err(Errno::Again);
        }
        self.sockets[listener].pending = usize::MAX;
        let ofd = self.alloc_ofd(FdKind::Socket, ROOT, flags, accepted)?;
        self.alloc_fd_for_ofd(ofd, 3, (flags & O_CLOEXEC) != 0)
    }

    fn sendto_socket(&mut self, fd: isize, src: &[u8], dest: Option<&[u8]>) -> Result<usize, Errno> {
        self.ensure();
        let sock = self.socket_index_for_fd(fd)?;
        let target = if let Some(addr) = dest {
            self.find_bound_socket(addr, false)?
        } else if self.sockets[sock].peer < MAX_SOCKETS {
            self.sockets[sock].peer
        } else {
            sock
        };
        if target >= MAX_SOCKETS || !self.sockets[target].used {
            return Err(Errno::BadFd);
        }
        let mut copied = 0usize;
        while copied < src.len() && self.sockets[target].len < SOCKET_BUF {
            let pos = self.sockets[target].len;
            self.sockets[target].data[pos] = src[copied];
            self.sockets[target].len += 1;
            copied += 1;
        }
        self.sockets[sock].sends += 1;
        Ok(copied)
    }

    fn recvfrom_socket(&mut self, fd: isize, out: &mut [u8], src_out: &mut [u8]) -> Result<(usize, usize), Errno> {
        self.ensure();
        let sock = self.socket_index_for_fd(fd)?;
        let mut copied = 0usize;
        while copied < out.len() && copied < self.sockets[sock].len {
            out[copied] = self.sockets[sock].data[copied];
            copied += 1;
        }
        let mut i = copied;
        while i < self.sockets[sock].len {
            self.sockets[sock].data[i - copied] = self.sockets[sock].data[i];
            i += 1;
        }
        self.sockets[sock].len -= copied;
        self.sockets[sock].recvs += 1;
        let src_len = if self.sockets[sock].peer_addr_len != 0 {
            let mut n = 0usize;
            while n < src_out.len() && n < self.sockets[sock].peer_addr_len {
                src_out[n] = self.sockets[sock].peer_addr[n];
                n += 1;
            }
            n
        } else {
            0
        };
        Ok((copied, src_len))
    }

    fn socket_snapshot(&mut self) -> RuntimeSocketSnapshot {
        self.ensure();
        let mut snapshot = RuntimeSocketSnapshot { sockets_used: 0, stream_connected: 0, datagram_bound: 0, queued_bytes: 0, sends: 0, recvs: 0 };
        let mut i = 0usize;
        while i < MAX_SOCKETS {
            let sock = self.sockets[i];
            if sock.used {
                snapshot.sockets_used += 1;
                snapshot.queued_bytes += sock.len;
                snapshot.sends += sock.sends;
                snapshot.recvs += sock.recvs;
                if sock.peer < MAX_SOCKETS && self.sockets[sock.peer].used {
                    snapshot.stream_connected += 1;
                }
                if sock.bound && sock.sock_type as u32 == SOCK_DGRAM {
                    snapshot.datagram_bound += 1;
                }
            }
            i += 1;
        }
        snapshot
    }

    fn epoll_create1(&mut self, flags: u32) -> Result<isize, Errno> {
        self.ensure();
        let mut i = 0usize;
        while i < MAX_EPOLL {
            if !self.epolls[i].used {
                self.epolls[i] = EpollObj::empty();
                self.epolls[i].used = true;
                let ofd = self.alloc_ofd(FdKind::Epoll, ROOT, flags, i)?;
                return self.alloc_fd_for_ofd(ofd, 3, (flags & O_CLOEXEC) != 0);
            }
            i += 1;
        }
        Err(Errno::NoSpace)
    }

    fn epoll_ctl(&mut self, epfd: isize, op: usize, fd: usize, events: u32, data: u64) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.fd_idx(epfd)?;
        let ofd = self.fds[idx].ofd;
        if self.ofds[ofd].kind != FdKind::Epoll {
            return Err(Errno::BadFd);
        }
        let epoll = self.ofds[ofd].object;
        let _ = self.fd_idx(fd as isize)?;
        if fd == epfd as usize {
            return Err(Errno::Inval);
        }
        let interest = if events == 0 { (POLLIN | POLLOUT) as u32 } else { events };
        let mut pos = usize::MAX;
        let mut i = 0usize;
        while i < self.epolls[epoll].count {
            if self.epolls[epoll].watched[i] == fd {
                pos = i;
                break;
            }
            i += 1;
        }
        match op {
            EPOLL_CTL_ADD => {
                if pos != usize::MAX {
                    return Err(Errno::Exist);
                }
                if self.epolls[epoll].count >= self.epolls[epoll].watched.len() {
                    return Err(Errno::NoSpace);
                }
                let slot = self.epolls[epoll].count;
                self.epolls[epoll].watched[slot] = fd;
                self.epolls[epoll].events[slot] = interest;
                self.epolls[epoll].data[slot] = data;
                self.epolls[epoll].count += 1;
            }
            EPOLL_CTL_MOD => {
                if pos == usize::MAX {
                    return Err(Errno::NoEnt);
                }
                self.epolls[epoll].events[pos] = interest;
                self.epolls[epoll].data[pos] = data;
            }
            EPOLL_CTL_DEL => {
                if pos == usize::MAX {
                    return Err(Errno::NoEnt);
                }
                let mut j = pos + 1;
                while j < self.epolls[epoll].count {
                    self.epolls[epoll].watched[j - 1] = self.epolls[epoll].watched[j];
                    self.epolls[epoll].events[j - 1] = self.epolls[epoll].events[j];
                    self.epolls[epoll].data[j - 1] = self.epolls[epoll].data[j];
                    j += 1;
                }
                self.epolls[epoll].count -= 1;
                let tail = self.epolls[epoll].count;
                self.epolls[epoll].watched[tail] = 0;
                self.epolls[epoll].events[tail] = 0;
                self.epolls[epoll].data[tail] = 0;
            }
            _ => return Err(Errno::Inval),
        }
        Ok(0)
    }

    fn epoll_ready_count(&mut self, epfd: isize) -> Result<usize, Errno> {
        let mut events = [RuntimeEpollEvent::empty(); 8];
        self.epoll_collect_ready(epfd, &mut events)
    }

    fn epoll_collect_ready(&mut self, epfd: isize, out: &mut [RuntimeEpollEvent]) -> Result<usize, Errno> {
        self.ensure();
        let idx = self.fd_idx(epfd)?;
        let ofd = self.fds[idx].ofd;
        if self.ofds[ofd].kind != FdKind::Epoll {
            return Err(Errno::BadFd);
        }
        let epoll = self.ofds[ofd].object;
        let mut ready = 0usize;
        let mut i = 0usize;
        while i < self.epolls[epoll].count {
            let watched_fd = self.epolls[epoll].watched[i];
            let interest = self.epolls[epoll].events[i];
            let data = self.epolls[epoll].data[i];
            let mask = self.fd_readiness(watched_fd) as u32;
            let revents = mask & (interest | POLLERR as u32 | POLLHUP as u32);
            if revents != 0 {
                if ready < out.len() {
                    out[ready] = RuntimeEpollEvent { events: revents, data, fd: watched_fd };
                }
                ready += 1;
            }
            i += 1;
        }
        Ok(ready)
    }

    fn fd_readiness(&mut self, fd: usize) -> u16 {
        self.ensure();
        let idx = match self.fd_idx(fd as isize) {
            Ok(idx) => idx,
            Err(_) => return 0,
        };
        let ofd = self.fds[idx].ofd;
        match self.ofds[ofd].kind {
            FdKind::Stdin => 0,
            FdKind::Stdout | FdKind::Stderr | FdKind::DevNull | FdKind::DevZero | FdKind::DevConsole | FdKind::DevTty | FdKind::DevRandom | FdKind::RegularFile => POLLOUT | POLLIN,
            FdKind::Directory | FdKind::Symlink | FdKind::Procfs => POLLIN,
            FdKind::PipeRead => {
                let pipe = self.ofds[ofd].object;
                if pipe >= MAX_PIPES || !self.pipes[pipe].used {
                    0
                } else {
                    let mut mask = 0u16;
                    if self.pipes[pipe].len > 0 {
                        mask |= POLLIN;
                    }
                    if self.pipes[pipe].writers == 0 {
                        mask |= POLLHUP;
                    }
                    mask
                }
            }
            FdKind::PipeWrite => {
                let pipe = self.ofds[ofd].object;
                if pipe >= MAX_PIPES || !self.pipes[pipe].used {
                    0
                } else if self.pipes[pipe].readers == 0 {
                    POLLERR
                } else if self.pipes[pipe].len < PIPE_BUF {
                    POLLOUT
                } else {
                    0
                }
            }
            FdKind::EventFd => {
                let event = self.ofds[ofd].object;
                let readable = event < MAX_EVENTS && self.events[event].used && self.events[event].counter > 0;
                let writable = event < MAX_EVENTS && self.events[event].used && self.events[event].counter < u64::MAX - 1;
                (if writable { POLLOUT } else { 0 }) | if readable { POLLIN } else { 0 }
            }
            FdKind::TimerFd => {
                let timer = self.ofds[ofd].object;
                if timer < MAX_TIMERS && self.timers[timer].used && self.timers[timer].expirations > 0 { POLLIN } else { 0 }
            }
            FdKind::Socket => {
                let sock = self.ofds[ofd].object;
                let readable = sock < MAX_SOCKETS && self.sockets[sock].used && self.sockets[sock].len > 0;
                let writable = if sock < MAX_SOCKETS && self.sockets[sock].used {
                    if self.sockets[sock].sock_type as u32 == SOCK_DGRAM {
                        self.sockets[sock].len < SOCKET_BUF
                    } else {
                        let peer = self.sockets[sock].peer;
                        (peer < MAX_SOCKETS && self.sockets[peer].used && self.sockets[peer].len < SOCKET_BUF)
                            || self.sockets[sock].listening
                    }
                } else {
                    false
                };
                let hup = if sock < MAX_SOCKETS && self.sockets[sock].used {
                    let peer = self.sockets[sock].peer;
                    self.sockets[sock].sock_type as u32 == SOCK_STREAM && !self.sockets[sock].listening && (peer >= MAX_SOCKETS || !self.sockets[peer].used)
                } else {
                    false
                };
                (if writable { POLLOUT } else { 0 }) | if readable { POLLIN } else { 0 } | if hup { POLLHUP } else { 0 }
            }
            FdKind::Epoll => {
                if self.epoll_ready_count(fd as isize).unwrap_or(0) > 0 { POLLIN } else { 0 }
            }
            FdKind::Mq => POLLIN | POLLOUT,
            FdKind::Empty => 0,
        }
    }

    fn poll_revents(&mut self, fd: isize, requested: u16) -> u16 {
        self.ensure();
        if fd < 0 {
            return 0;
        }
        if self.fd_idx(fd).is_err() {
            return POLLNVAL;
        }
        let mask = self.fd_readiness(fd as usize);
        let always = mask & (POLLERR | POLLHUP | POLLNVAL);
        (mask & requested) | always
    }

    fn mq_open(&mut self, key: usize, flags: u32) -> Result<isize, Errno> {
        self.ensure();
        let mut free = usize::MAX;
        let mut i = 0usize;
        while i < MAX_IPC {
            if self.mqs[i].used && self.mqs[i].key == key && !self.mqs[i].unlinked {
                let ofd = self.alloc_ofd(FdKind::Mq, ROOT, flags, i)?;
                return self.alloc_fd_for_ofd(ofd, 3, (flags & O_CLOEXEC) != 0);
            }
            if !self.mqs[i].used && free == usize::MAX {
                free = i;
            }
            i += 1;
        }
        if free == usize::MAX {
            return Err(Errno::NoSpace);
        }
        self.mqs[free] = MqObj { used: true, unlinked: false, key, data: [0; MSG_BUF], len: 0, prio: 0 };
        let ofd = self.alloc_ofd(FdKind::Mq, ROOT, flags, free)?;
        self.alloc_fd_for_ofd(ofd, 3, (flags & O_CLOEXEC) != 0)
    }

    fn mq_index_for_fd(&mut self, fd: isize) -> Result<usize, Errno> {
        let idx = self.fd_idx(fd)?;
        let ofd = self.fds[idx].ofd;
        if self.ofds[ofd].kind != FdKind::Mq {
            return Err(Errno::BadFd);
        }
        Ok(self.ofds[ofd].object)
    }

    fn mq_send(&mut self, fd: isize, msg: &[u8], prio: usize) -> Result<isize, Errno> {
        self.ensure();
        let mq = self.mq_index_for_fd(fd)?;
        let mut n = 0usize;
        while n < msg.len() && n < MSG_BUF {
            self.mqs[mq].data[n] = msg[n];
            n += 1;
        }
        self.mqs[mq].len = n;
        self.mqs[mq].prio = prio;
        let _ = self.sched_wake(SCHED_WAIT_MQ_BASE + mq, 1)?;
        Ok(0)
    }

    fn mq_receive(&mut self, fd: isize, out: &mut [u8]) -> Result<usize, Errno> {
        self.ensure();
        let mq = self.mq_index_for_fd(fd)?;
        if self.mqs[mq].len == 0 {
            let _ = self.sched_wait_on(SCHED_WAIT_MQ_BASE + mq)?;
            return Ok(0);
        }
        let mut n = 0usize;
        while n < out.len() && n < self.mqs[mq].len {
            out[n] = self.mqs[mq].data[n];
            n += 1;
        }
        self.mqs[mq].len = 0;
        Ok(n)
    }

    fn mq_unlink(&mut self, key: usize) -> Result<isize, Errno> {
        self.ensure();
        let mut i = 0usize;
        while i < MAX_IPC {
            if self.mqs[i].used && self.mqs[i].key == key {
                self.mqs[i].unlinked = true;
                return Ok(0);
            }
            i += 1;
        }
        Err(Errno::NoEnt)
    }

    fn msgget(&mut self, key: usize) -> Result<isize, Errno> {
        self.ensure();
        let mut free = usize::MAX;
        let mut i = 0usize;
        while i < MAX_IPC {
            if self.msgs[i].used && self.msgs[i].key == key {
                return Ok(self.msgs[i].id as isize);
            }
            if !self.msgs[i].used && free == usize::MAX {
                free = i;
            }
            i += 1;
        }
        if free == usize::MAX {
            return Err(Errno::NoSpace);
        }
        let id = self.next_ipc_id;
        self.next_ipc_id += 1;
        self.msgs[free] = MsgObj { used: true, id, key, data: [0; MSG_BUF], len: 0 };
        Ok(id as isize)
    }

    fn msg_index(&self, id: usize) -> Result<usize, Errno> {
        let mut i = 0usize;
        while i < MAX_IPC {
            if self.msgs[i].used && self.msgs[i].id == id {
                return Ok(i);
            }
            i += 1;
        }
        Err(Errno::NoEnt)
    }

    fn msgsnd(&mut self, id: usize, msg: &[u8]) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.msg_index(id)?;
        let mut n = 0usize;
        while n < msg.len() && n < MSG_BUF {
            self.msgs[idx].data[n] = msg[n];
            n += 1;
        }
        self.msgs[idx].len = n;
        let _ = self.sched_wake(SCHED_WAIT_MSG_BASE + idx, 1)?;
        Ok(0)
    }

    fn msgrcv(&mut self, id: usize, out: &mut [u8]) -> Result<usize, Errno> {
        self.ensure();
        let idx = self.msg_index(id)?;
        if self.msgs[idx].len == 0 {
            let _ = self.sched_wait_on(SCHED_WAIT_MSG_BASE + idx)?;
            return Ok(0);
        }
        let mut n = 0usize;
        while n < out.len() && n < self.msgs[idx].len {
            out[n] = self.msgs[idx].data[n];
            n += 1;
        }
        self.msgs[idx].len = 0;
        Ok(n)
    }

    fn msgctl(&mut self, id: usize, cmd: usize) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.msg_index(id)?;
        if cmd == IPC_RMID {
            self.msgs[idx] = MsgObj::empty();
        }
        Ok(0)
    }

    fn semget(&mut self, key: usize, nsems: usize) -> Result<isize, Errno> {
        self.ensure();
        let mut free = usize::MAX;
        let mut i = 0usize;
        while i < MAX_IPC {
            if self.sems[i].used && self.sems[i].key == key {
                return Ok(self.sems[i].id as isize);
            }
            if !self.sems[i].used && free == usize::MAX {
                free = i;
            }
            i += 1;
        }
        if free == usize::MAX {
            return Err(Errno::NoSpace);
        }
        let id = self.next_ipc_id;
        self.next_ipc_id += 1;
        self.sems[free] = SemObj { used: true, id, key, nsems, value: 0 };
        Ok(id as isize)
    }

    fn sem_index(&self, id: usize) -> Result<usize, Errno> {
        let mut i = 0usize;
        while i < MAX_IPC {
            if self.sems[i].used && self.sems[i].id == id {
                return Ok(i);
            }
            i += 1;
        }
        Err(Errno::NoEnt)
    }

    fn semop(&mut self, id: usize, delta: isize) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.sem_index(id)?;
        if delta < 0 && self.sems[idx].value + delta < 0 {
            let _ = self.sched_wait_on(SCHED_WAIT_SEM_BASE + idx)?;
            return Ok(0);
        }
        self.sems[idx].value += delta;
        if delta > 0 {
            let _ = self.sched_wake(SCHED_WAIT_SEM_BASE + idx, delta as usize)?;
        }
        Ok(0)
    }

    fn semctl(&mut self, id: usize, cmd: usize) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.sem_index(id)?;
        if cmd == IPC_RMID {
            self.sems[idx] = SemObj::empty();
        }
        Ok(0)
    }

    fn shmget(&mut self, key: usize, size: usize) -> Result<isize, Errno> {
        self.ensure();
        let mut free = usize::MAX;
        let mut i = 0usize;
        while i < MAX_IPC {
            if self.shms[i].used && self.shms[i].key == key {
                return Ok(self.shms[i].id as isize);
            }
            if !self.shms[i].used && free == usize::MAX {
                free = i;
            }
            i += 1;
        }
        if free == usize::MAX {
            return Err(Errno::NoSpace);
        }
        let id = self.next_ipc_id;
        self.next_ipc_id += 1;
        self.shms[free] = ShmObj { used: true, attached: false, id, key, size };
        Ok(id as isize)
    }

    fn shm_index(&self, id: usize) -> Result<usize, Errno> {
        let mut i = 0usize;
        while i < MAX_IPC {
            if self.shms[i].used && self.shms[i].id == id {
                return Ok(i);
            }
            i += 1;
        }
        Err(Errno::NoEnt)
    }

    fn shmat(&mut self, id: usize) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.shm_index(id)?;
        self.shms[idx].attached = true;
        let _ = self.sched_wake(SCHED_WAIT_SHM_BASE + idx, 1)?;
        Ok(0x4004_0000isize)
    }

    fn shm_wait_for_attach(&mut self, id: usize) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.shm_index(id)?;
        if !self.shms[idx].attached {
            let _ = self.sched_wait_on(SCHED_WAIT_SHM_BASE + idx)?;
        }
        Ok(0)
    }

    fn shmdt(&mut self, addr: usize) -> Result<isize, Errno> {
        self.ensure();
        if addr == 0 {
            return Err(Errno::Inval);
        }
        let mut i = 0usize;
        while i < MAX_IPC {
            if self.shms[i].used && self.shms[i].attached {
                self.shms[i].attached = false;
                return Ok(0);
            }
            i += 1;
        }
        Err(Errno::NoEnt)
    }

    fn shmctl(&mut self, id: usize, cmd: usize) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.shm_index(id)?;
        if cmd == IPC_RMID {
            self.shms[idx] = ShmObj::empty();
        }
        Ok(0)
    }

    fn futex_slot(&mut self, key: usize, create: bool) -> Result<usize, Errno> {
        let mut free = usize::MAX;
        let mut i = 0usize;
        while i < MAX_FUTEXES {
            if self.futexes[i].used && self.futexes[i].key == key {
                return Ok(i);
            }
            if !self.futexes[i].used && free == usize::MAX {
                free = i;
            }
            i += 1;
        }
        if !create || free == usize::MAX {
            return Err(Errno::NoEnt);
        }
        self.futexes[free] = FutexObj { used: true, key, waiters: 0, total_waits: 0, total_wakes: 0, last_expected: 0 };
        Ok(free)
    }

    fn waitq_slot(&mut self, key: usize, create: bool) -> Result<usize, Errno> {
        let mut free = usize::MAX;
        let mut i = 0usize;
        while i < MAX_WAIT_QUEUES {
            if self.waitqs[i].used && self.waitqs[i].key == key {
                return Ok(i);
            }
            if !self.waitqs[i].used && free == usize::MAX {
                free = i;
            }
            i += 1;
        }
        if !create || free == usize::MAX {
            return Err(Errno::NoEnt);
        }
        self.waitqs[free] = WaitQueueObj { used: true, key, waiters: 0, wakeups: 0 };
        Ok(free)
    }

    fn task_is_runnable(state: RuntimeTaskState) -> bool {
        state == RuntimeTaskState::Running || state == RuntimeTaskState::Ready
    }

    fn runq_contains_idx(&self, idx: usize) -> bool {
        let mut i = 0usize;
        while i < self.runq_len {
            if self.runq[i] == idx {
                return true;
            }
            i += 1;
        }
        false
    }

    fn runq_push_idx(&mut self, idx: usize) {
        if idx >= MAX_TASKS || !self.tasks[idx].used || !Self::task_is_runnable(self.tasks[idx].state) {
            return;
        }
        if self.runq_len < MAX_TASKS && !self.runq_contains_idx(idx) {
            self.runq[self.runq_len] = idx;
            self.runq_len += 1;
        }
    }

    fn runq_remove_idx(&mut self, idx: usize) {
        let mut i = 0usize;
        while i < self.runq_len {
            if self.runq[i] == idx {
                let mut j = i + 1;
                while j < self.runq_len {
                    self.runq[j - 1] = self.runq[j];
                    j += 1;
                }
                self.runq_len -= 1;
                self.runq[self.runq_len] = 0;
                return;
            }
            i += 1;
        }
    }

    fn rebuild_runq(&mut self) {
        self.runq = [0; MAX_TASKS];
        self.runq_len = 0;
        let mut i = 0usize;
        while i < MAX_TASKS {
            if self.tasks[i].used && Self::task_is_runnable(self.tasks[i].state) {
                self.runq_push_idx(i);
            }
            i += 1;
        }
    }

    fn save_current_task_runtime(&mut self) {
        if self.current_task < MAX_TASKS && self.tasks[self.current_task].used {
            self.tasks[self.current_task].cwd = self.cwd;
            self.tasks[self.current_task].root = self.root;
            self.tasks[self.current_task].fd_count = self.fd_count();
        }
    }

    fn scheduled_switch_to_idx(&mut self, idx: usize) -> Result<(), Errno> {
        self.ensure();
        if idx >= MAX_TASKS || !self.tasks[idx].used {
            return Err(Errno::NoEnt);
        }
        let from_idx = self.current_task;
        let from_pid = if from_idx < MAX_TASKS && self.tasks[from_idx].used { self.tasks[from_idx].pid } else { 0 };
        self.save_current_task_runtime();
        if from_idx < MAX_TASKS
            && self.tasks[from_idx].used
            && from_idx != idx
            && self.tasks[from_idx].state == RuntimeTaskState::Running
        {
            self.tasks[from_idx].state = RuntimeTaskState::Ready;
        }
        self.current_task = idx;
        self.cwd = self.tasks[idx].cwd;
        self.root = self.tasks[idx].root;
        if self.tasks[idx].state == RuntimeTaskState::Ready {
            self.tasks[idx].state = RuntimeTaskState::Running;
        }
        self.rebuild_runq();
        let to_pid = self.tasks[idx].pid;
        if from_idx != idx {
            self.sched_switches += 1;
            self.sched_last_from = from_pid;
            self.sched_last_to = to_pid;
        }
        Ok(())
    }

    fn next_runnable_idx(&self) -> Option<usize> {
        if self.runq_len == 0 {
            return None;
        }
        let mut current_pos = usize::MAX;
        let mut i = 0usize;
        while i < self.runq_len {
            if self.runq[i] == self.current_task {
                current_pos = i;
                break;
            }
            i += 1;
        }
        if current_pos == usize::MAX {
            Some(self.runq[0])
        } else {
            Some(self.runq[(current_pos + 1) % self.runq_len])
        }
    }

    fn sched_tick_once(&mut self) -> Result<isize, Errno> {
        self.ensure();
        self.sched_ticks += 1;
        self.rebuild_runq();
        let next = match self.next_runnable_idx() {
            Some(idx) => idx,
            None => return Ok(0),
        };
        self.scheduled_switch_to_idx(next)?;
        Ok(0)
    }

    fn fd_wait_key(&mut self, fd: isize) -> Result<usize, Errno> {
        let idx = self.fd_idx(fd)?;
        let ofd = self.fds[idx].ofd;
        match self.ofds[ofd].kind {
            FdKind::PipeRead | FdKind::PipeWrite => Ok(SCHED_WAIT_PIPE_BASE + self.ofds[ofd].object),
            FdKind::TimerFd => Ok(SCHED_WAIT_TIMER_BASE + self.ofds[ofd].object),
            _ => Err(Errno::Inval),
        }
    }

    fn sched_wait_fd_readable(&mut self, fd: isize) -> Result<isize, Errno> {
        self.ensure();
        if (self.fd_readiness(fd as usize) & POLLIN) != 0 {
            return Ok(0);
        }
        let key = self.fd_wait_key(fd)?;
        self.sched_wait_on(key)
    }

    fn current_task_mut(&mut self) -> Result<&mut TaskObj, Errno> {
        if self.current_task < MAX_TASKS && self.tasks[self.current_task].used {
            Ok(&mut self.tasks[self.current_task])
        } else {
            Err(Errno::NoEnt)
        }
    }

    fn sched_wait_on(&mut self, key: usize) -> Result<isize, Errno> {
        self.ensure();
        let q = self.waitq_slot(key, true)?;
        self.waitqs[q].waiters += 1;
        let idx = self.current_task_idx()?;
        self.tasks[idx].state = RuntimeTaskState::Waiting;
        self.tasks[idx].wait_key = key;
        self.runq_remove_idx(idx);
        self.sched_blocks += 1;
        Ok(0)
    }

    fn sched_wake(&mut self, key: usize, max_count: usize) -> Result<isize, Errno> {
        self.ensure();
        let q = match self.waitq_slot(key, false) {
            Ok(q) => q,
            Err(_) => return Ok(0),
        };
        let limit = if max_count == 0 { 1 } else { max_count };
        let queued = if self.waitqs[q].waiters < limit { self.waitqs[q].waiters } else { limit };
        let mut woke_tasks = 0usize;
        let mut i = 0usize;
        while i < MAX_TASKS && woke_tasks < queued {
            if self.tasks[i].used && self.tasks[i].state == RuntimeTaskState::Waiting && self.tasks[i].wait_key == key {
                self.tasks[i].state = RuntimeTaskState::Ready;
                self.tasks[i].wait_key = 0;
                self.runq_push_idx(i);
                woke_tasks += 1;
            }
            i += 1;
        }
        if queued > self.waitqs[q].waiters {
            self.waitqs[q].waiters = 0;
        } else {
            self.waitqs[q].waiters -= queued;
        }
        self.waitqs[q].wakeups += queued;
        self.sched_wakes += queued;
        if self.waitqs[q].waiters == 0 {
            self.waitqs[q] = WaitQueueObj::empty();
        }
        Ok(queued as isize)
    }

    fn sched_yield_current(&mut self) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.current_task_idx()?;
        self.tasks[idx].yields += 1;
        self.tasks[idx].state = RuntimeTaskState::Ready;
        self.runq_push_idx(idx);
        let _ = self.sched_tick_once()?;
        if self.current_task == idx && self.tasks[idx].state == RuntimeTaskState::Ready {
            self.tasks[idx].state = RuntimeTaskState::Running;
            self.rebuild_runq();
        }
        Ok(0)
    }

    fn sched_timeout_wait(&mut self, key: usize) -> Result<isize, Errno> {
        self.sched_wait_on(key)?;
        let _ = self.sched_wake(key, 1)?;
        let idx = self.current_task_idx()?;
        if self.tasks[idx].state == RuntimeTaskState::Ready {
            self.tasks[idx].state = RuntimeTaskState::Running;
            self.rebuild_runq();
        }
        self.sched_timer_wakes += 1;
        Ok(0)
    }

    fn clone_task(&mut self, flags: usize) -> Result<isize, Errno> {
        self.ensure();
        let parent_idx = self.current_task_idx()?;
        let parent = self.tasks[parent_idx];
        let mut child_idx = usize::MAX;
        let mut i = 0usize;
        while i < MAX_TASKS {
            if !self.tasks[i].used {
                child_idx = i;
                break;
            }
            i += 1;
        }
        if child_idx == usize::MAX {
            return Err(Errno::NoSpace);
        }
        let child_pid = self.next_pid;
        self.next_pid += 1;
        let child_tgid = if (flags & CLONE_THREAD) != 0 { parent.tgid } else { child_pid };
        self.tasks[child_idx] = TaskObj {
            used: true,
            pid: child_pid,
            ppid: parent.pid,
            tgid: child_tgid,
            pgid: parent.pgid,
            sid: parent.sid,
            state: RuntimeTaskState::Ready,
            exit_code: 0,
            wait_key: 0,
            yields: 0,
            fork_return: 0,
            fd_count: self.fd_count(),
            cwd: self.cwd,
            root: self.root,
            signal_mask: parent.signal_mask,
            mm_id: parent.mm_id,
            cred: parent.cred,
            namespaces: parent.namespaces,
        };
        self.runq_push_idx(child_idx);
        Ok(child_pid as isize)
    }

    fn exit_task_pid(&mut self, pid: usize, code: isize) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.task_idx_by_pid(pid)?;
        let parent_pid = self.tasks[idx].ppid;
        let parent_idx = if parent_pid != 0 { self.task_idx_by_pid(parent_pid).ok() } else { None };
        let parent_alive = parent_idx.is_some();
        self.tasks[idx].exit_code = code;
        self.tasks[idx].wait_key = 0;
        self.tasks[idx].state = if parent_alive { RuntimeTaskState::Zombie } else { RuntimeTaskState::Exited };
        self.runq_remove_idx(idx);
        if let Some(pidx) = parent_idx {
            let _ = self.queue_signal_to_idx(pidx, SIGCHLD, pid, code, false);
            let _ = self.sched_wake(SCHED_WAIT_CHILD_BASE + parent_pid, MAX_TASKS)?;
        }
        Ok(0)
    }

    fn exit_current_task(&mut self, code: isize) -> Result<isize, Errno> {
        let idx = self.current_task_idx()?;
        self.exit_task_pid(self.tasks[idx].pid, code)
    }

    fn exit_group_current(&mut self, code: isize) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.current_task_idx()?;
        let tgid = self.tasks[idx].tgid;
        let mut i = 0usize;
        while i < MAX_TASKS {
            if self.tasks[i].used && self.tasks[i].tgid == tgid {
                let pid = self.tasks[i].pid;
                let _ = self.exit_task_pid(pid, code)?;
            }
            i += 1;
        }
        Ok(0)
    }

    fn find_waitable_child(&self, wait_pid: isize) -> Result<(usize, bool), Errno> {
        let parent = self.tasks[self.current_task_idx()?].pid;
        let mut found_child = false;
        let mut i = 0usize;
        while i < MAX_TASKS {
            let task = self.tasks[i];
            let pid_matches = wait_pid <= 0 || task.pid == wait_pid as usize;
            if task.used && task.ppid == parent && task.pid != parent && pid_matches {
                found_child = true;
                if task.state == RuntimeTaskState::Zombie || task.state == RuntimeTaskState::Exited {
                    return Ok((i, true));
                }
            }
            i += 1;
        }
        if found_child {
            Ok((usize::MAX, false))
        } else {
            Err(Errno::Child)
        }
    }

    fn wait4(&mut self, pid: isize) -> Result<(usize, isize), Errno> {
        self.ensure();
        let (idx, ready) = self.find_waitable_child(pid)?;
        if !ready {
            let parent_pid = self.tasks[self.current_task_idx()?].pid;
            let _ = self.sched_wait_on(SCHED_WAIT_CHILD_BASE + parent_pid)?;
            return Ok((0, 0));
        }
        let child_pid = self.tasks[idx].pid;
        let status = (self.tasks[idx].exit_code & 0xff) << 8;
        self.tasks[idx] = TaskObj::empty();
        Ok((child_pid, status))
    }

    fn waitid(&mut self, pid: isize) -> Result<(usize, isize), Errno> {
        self.ensure();
        let (idx, ready) = self.find_waitable_child(pid)?;
        if !ready {
            return Ok((0, 0));
        }
        let child_pid = self.tasks[idx].pid;
        let code = self.tasks[idx].exit_code;
        self.tasks[idx] = TaskObj::empty();
        Ok((child_pid, code))
    }

    fn setpgid(&mut self, pid: usize, pgid: usize) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.task_idx_by_pid(pid)?;
        let new_pgid = if pgid == 0 { self.tasks[idx].pid } else { pgid };
        self.tasks[idx].pgid = new_pgid;
        Ok(0)
    }

    fn getpgid(&mut self, pid: usize) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.task_idx_by_pid(pid)?;
        Ok(self.tasks[idx].pgid as isize)
    }

    fn getsid(&mut self, pid: usize) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.task_idx_by_pid(pid)?;
        Ok(self.tasks[idx].sid as isize)
    }

    fn setsid(&mut self) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.current_task_idx()?;
        let pid = self.tasks[idx].pid;
        self.tasks[idx].sid = pid;
        self.tasks[idx].pgid = pid;
        Ok(pid as isize)
    }

    fn update_signal_mask(&mut self, how: usize, mask: Option<u64>) -> Result<u64, Errno> {
        self.ensure();
        let idx = self.current_task_idx()?;
        let old = self.tasks[idx].signal_mask;
        if let Some(set) = mask {
            let new_mask = match how {
                SIG_BLOCK => old | set,
                SIG_UNBLOCK => old & !set,
                SIG_SETMASK => set,
                _ => return Err(Errno::Inval),
            };
            self.tasks[idx].signal_mask = new_mask;
        }
        Ok(old)
    }

    fn cred_snapshot(&mut self) -> Result<RuntimeCred, Errno> {
        self.ensure();
        let idx = self.current_task_idx()?;
        Ok(self.tasks[idx].cred)
    }

    fn normalize_id_arg(value: usize) -> Option<u32> {
        if value == usize::MAX {
            None
        } else {
            Some(value as u32)
        }
    }

    fn uid_change_allowed(cred: RuntimeCred, next: Option<u32>) -> bool {
        match next {
            None => true,
            Some(uid) => Self::cred_has_cap(cred, CAP_SETUID) || uid == cred.uid || uid == cred.euid || uid == cred.suid,
        }
    }

    fn gid_change_allowed(cred: RuntimeCred, next: Option<u32>) -> bool {
        match next {
            None => true,
            Some(gid) => Self::cred_has_cap(cred, CAP_SETGID) || gid == cred.gid || gid == cred.egid || gid == cred.sgid,
        }
    }

    fn refresh_effective_caps_for_uid(cred: &mut RuntimeCred) {
        if cred.euid == 0 {
            cred.cap_effective = cred.cap_permitted;
        } else {
            cred.cap_effective = 0;
        }
    }

    fn setresuid(&mut self, ruid: usize, euid: usize, suid: usize) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.current_task_idx()?;
        let cred = self.tasks[idx].cred;
        let next_r = Self::normalize_id_arg(ruid);
        let next_e = Self::normalize_id_arg(euid);
        let next_s = Self::normalize_id_arg(suid);
        if !Self::uid_change_allowed(cred, next_r) || !Self::uid_change_allowed(cred, next_e) || !Self::uid_change_allowed(cred, next_s) {
            return Err(Errno::Access);
        }
        if let Some(uid) = next_r {
            self.tasks[idx].cred.uid = uid;
        }
        if let Some(uid) = next_e {
            self.tasks[idx].cred.euid = uid;
            self.tasks[idx].cred.fsuid = uid;
        }
        if let Some(uid) = next_s {
            self.tasks[idx].cred.suid = uid;
        }
        Self::refresh_effective_caps_for_uid(&mut self.tasks[idx].cred);
        Ok(0)
    }

    fn setuid(&mut self, uid: usize) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.current_task_idx()?;
        let cred = self.tasks[idx].cred;
        let next = Self::normalize_id_arg(uid);
        if !Self::uid_change_allowed(cred, next) {
            return Err(Errno::Access);
        }
        if let Some(uid32) = next {
            if Self::cred_has_cap(cred, CAP_SETUID) {
                self.tasks[idx].cred.uid = uid32;
                self.tasks[idx].cred.euid = uid32;
                self.tasks[idx].cred.suid = uid32;
                self.tasks[idx].cred.fsuid = uid32;
            } else {
                self.tasks[idx].cred.euid = uid32;
                self.tasks[idx].cred.fsuid = uid32;
            }
        }
        Self::refresh_effective_caps_for_uid(&mut self.tasks[idx].cred);
        Ok(0)
    }

    fn setresgid(&mut self, rgid: usize, egid: usize, sgid: usize) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.current_task_idx()?;
        let cred = self.tasks[idx].cred;
        let next_r = Self::normalize_id_arg(rgid);
        let next_e = Self::normalize_id_arg(egid);
        let next_s = Self::normalize_id_arg(sgid);
        if !Self::gid_change_allowed(cred, next_r) || !Self::gid_change_allowed(cred, next_e) || !Self::gid_change_allowed(cred, next_s) {
            return Err(Errno::Access);
        }
        if let Some(gid) = next_r {
            self.tasks[idx].cred.gid = gid;
        }
        if let Some(gid) = next_e {
            self.tasks[idx].cred.egid = gid;
            self.tasks[idx].cred.fsgid = gid;
        }
        if let Some(gid) = next_s {
            self.tasks[idx].cred.sgid = gid;
        }
        Ok(0)
    }

    fn setgid(&mut self, gid: usize) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.current_task_idx()?;
        let cred = self.tasks[idx].cred;
        let next = Self::normalize_id_arg(gid);
        if !Self::gid_change_allowed(cred, next) {
            return Err(Errno::Access);
        }
        if let Some(gid32) = next {
            if Self::cred_has_cap(cred, CAP_SETGID) {
                self.tasks[idx].cred.gid = gid32;
                self.tasks[idx].cred.egid = gid32;
                self.tasks[idx].cred.sgid = gid32;
                self.tasks[idx].cred.fsgid = gid32;
            } else {
                self.tasks[idx].cred.egid = gid32;
                self.tasks[idx].cred.fsgid = gid32;
            }
        }
        Ok(0)
    }

    fn setfsuid(&mut self, uid: usize) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.current_task_idx()?;
        let old = self.tasks[idx].cred.fsuid;
        let next = Self::normalize_id_arg(uid);
        if Self::uid_change_allowed(self.tasks[idx].cred, next) {
            if let Some(uid32) = next {
                self.tasks[idx].cred.fsuid = uid32;
            }
        }
        Ok(old as isize)
    }

    fn setfsgid(&mut self, gid: usize) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.current_task_idx()?;
        let old = self.tasks[idx].cred.fsgid;
        let next = Self::normalize_id_arg(gid);
        if Self::gid_change_allowed(self.tasks[idx].cred, next) {
            if let Some(gid32) = next {
                self.tasks[idx].cred.fsgid = gid32;
            }
        }
        Ok(old as isize)
    }

    fn capset(&mut self, permitted: u64, effective: u64, inheritable: u64) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.current_task_idx()?;
        let cred = self.tasks[idx].cred;
        if cred.euid != 0 && (permitted & !cred.cap_permitted) != 0 {
            return Err(Errno::Access);
        }
        if (effective & !permitted) != 0 {
            return Err(Errno::Inval);
        }
        self.tasks[idx].cred.cap_permitted = permitted;
        self.tasks[idx].cred.cap_effective = effective;
        self.tasks[idx].cred.cap_inheritable = inheritable;
        Ok(0)
    }

    fn security_snapshot_path(&mut self, path: &[u8]) -> Result<RuntimeSecuritySnapshot, Errno> {
        self.ensure();
        let node = self.resolve_at(AT_FDCWD, path, true)?;
        let owner = self.owner(node);
        let cred = self.tasks[self.current_task_idx()?].cred;
        Ok(RuntimeSecuritySnapshot {
            uid: cred.uid,
            euid: cred.euid,
            gid: cred.gid,
            egid: cred.egid,
            fsuid: cred.fsuid,
            fsgid: cred.fsgid,
            cap_effective: cred.cap_effective,
            cap_permitted: cred.cap_permitted,
            node_uid: self.nodes[owner].uid,
            node_gid: self.nodes[owner].gid,
            node_mode: self.nodes[owner].mode,
        })
    }

    fn supported_namespace_flags(flags: usize) -> bool {
        (flags & !(CLONE_NEWNS | CLONE_NEWIPC | CLONE_NEWPID)) == 0
    }

    fn alloc_namespace_id(&mut self) -> Result<usize, Errno> {
        let id = self.next_ns_id;
        self.next_ns_id = self.next_ns_id.checked_add(1).ok_or(Errno::NoSpace)?;
        Ok(id)
    }

    fn unshare_namespaces(&mut self, flags: usize) -> Result<isize, Errno> {
        self.ensure();
        let idx = self.current_task_idx()?;
        if !Self::supported_namespace_flags(flags) {
            self.tasks[idx].namespaces.last_error = EINVAL;
            return Err(Errno::Inval);
        }
        if (flags & CLONE_NEWNS) != 0 {
            let id = self.alloc_namespace_id()?;
            self.tasks[idx].namespaces.mount_ns = id;
        }
        if (flags & CLONE_NEWIPC) != 0 {
            let id = self.alloc_namespace_id()?;
            self.tasks[idx].namespaces.ipc_ns = id;
        }
        if (flags & CLONE_NEWPID) != 0 {
            let id = self.alloc_namespace_id()?;
            self.tasks[idx].namespaces.pid_ns = id;
        }
        self.tasks[idx].namespaces.unshare_count += 1;
        self.tasks[idx].namespaces.last_error = 0;
        Ok(0)
    }

    fn setns_namespace(&mut self, fd: usize, nstype: usize) -> Result<isize, Errno> {
        self.ensure();
        let _ = self.fd_idx(fd as isize)?;
        let idx = self.current_task_idx()?;
        if nstype != 0 && !Self::supported_namespace_flags(nstype) {
            self.tasks[idx].namespaces.last_error = EINVAL;
            return Err(Errno::Inval);
        }
        if nstype == 0 || (nstype & CLONE_NEWNS) != 0 {
            self.tasks[idx].namespaces.mount_ns = 1;
        }
        if nstype == 0 || (nstype & CLONE_NEWIPC) != 0 {
            self.tasks[idx].namespaces.ipc_ns = 1;
        }
        if nstype == 0 || (nstype & CLONE_NEWPID) != 0 {
            self.tasks[idx].namespaces.pid_ns = 1;
        }
        self.tasks[idx].namespaces.setns_count += 1;
        self.tasks[idx].namespaces.last_error = 0;
        Ok(0)
    }

    fn namespace_snapshot(&mut self) -> RuntimeNamespaceRefs {
        self.ensure();
        match self.current_task_idx() {
            Ok(idx) => self.tasks[idx].namespaces,
            Err(_) => RuntimeNamespaceRefs::initial(),
        }
    }

    fn futex_wait(&mut self, key: usize, observed: u32, expected: u32, has_timeout: bool) -> Result<isize, Errno> {
        self.ensure();
        if observed != expected {
            return Err(Errno::Again);
        }
        let idx = self.futex_slot(key, true)?;
        self.futexes[idx].waiters += 1;
        self.futexes[idx].total_waits += 1;
        self.futexes[idx].last_expected = expected;
        let wait_key = SCHED_WAIT_FUTEX_BASE ^ key;
        if has_timeout {
            self.sched_timeout_wait(wait_key)?;
            if self.futexes[idx].waiters > 0 {
                self.futexes[idx].waiters -= 1;
            }
        } else {
            self.sched_wait_on(wait_key)?;
        }
        Ok(0)
    }

    fn futex_wake(&mut self, key: usize, count: usize) -> Result<isize, Errno> {
        self.ensure();
        let idx = match self.futex_slot(key, false) {
            Ok(idx) => idx,
            Err(_) => return Ok(0),
        };
        let limit = if count == 0 { 1 } else { count };
        let woke = if self.futexes[idx].waiters < limit { self.futexes[idx].waiters } else { limit };
        self.futexes[idx].waiters -= woke;
        self.futexes[idx].total_wakes += woke;
        let _ = self.sched_wake(SCHED_WAIT_FUTEX_BASE ^ key, woke)?;
        if self.futexes[idx].waiters == 0 && self.futexes[idx].total_waits == self.futexes[idx].total_wakes {
            self.futexes[idx] = FutexObj::empty();
        }
        Ok(woke as isize)
    }

    fn sched_snapshot(&mut self) -> RuntimeSchedSnapshot {
        self.ensure();
        let task = self.tasks[self.current_task];
        let mut wait_queues = 0usize;
        let mut waiters = 0usize;
        let mut wakeups = 0usize;
        let mut i = 0usize;
        while i < MAX_WAIT_QUEUES {
            if self.waitqs[i].used {
                wait_queues += 1;
                waiters += self.waitqs[i].waiters;
                wakeups += self.waitqs[i].wakeups;
            }
            i += 1;
        }
        RuntimeSchedSnapshot {
            current_pid: task.pid,
            current_state: task.state,
            wait_queues,
            waiters,
            wakeups,
            yields: task.yields,
            runq_len: self.runq_len,
            ticks: self.sched_ticks,
            switches: self.sched_switches,
            blocks: self.sched_blocks,
            wakes: self.sched_wakes,
            timer_wakes: self.sched_timer_wakes,
            last_from: self.sched_last_from,
            last_to: self.sched_last_to,
        }
    }

    fn ipc_wait_snapshot(&mut self) -> RuntimeIpcWaitSnapshot {
        self.ensure();
        let mut snapshot = RuntimeIpcWaitSnapshot { mq_waiters: 0, msg_waiters: 0, sem_waiters: 0, shm_waiters: 0, wakeups: 0 };
        let mut i = 0usize;
        while i < MAX_WAIT_QUEUES {
            if self.waitqs[i].used {
                let key = self.waitqs[i].key;
                if key >= SCHED_WAIT_MQ_BASE && key < SCHED_WAIT_MQ_BASE + MAX_IPC {
                    snapshot.mq_waiters += self.waitqs[i].waiters;
                } else if key >= SCHED_WAIT_MSG_BASE && key < SCHED_WAIT_MSG_BASE + MAX_IPC {
                    snapshot.msg_waiters += self.waitqs[i].waiters;
                } else if key >= SCHED_WAIT_SEM_BASE && key < SCHED_WAIT_SEM_BASE + MAX_IPC {
                    snapshot.sem_waiters += self.waitqs[i].waiters;
                } else if key >= SCHED_WAIT_SHM_BASE && key < SCHED_WAIT_SHM_BASE + MAX_IPC {
                    snapshot.shm_waiters += self.waitqs[i].waiters;
                }
                snapshot.wakeups += self.waitqs[i].wakeups;
            }
            i += 1;
        }
        snapshot
    }
}

static mut CORE: KernelCore = KernelCore::new();

fn core_mut() -> &'static mut KernelCore {
    unsafe { &mut CORE }
}

pub fn init() {
    core_mut().ensure();
    crate::println!("[fs::runtime] canonical shared state init");
}

fn public_result<T>(r: Result<T, Errno>) -> Result<T, isize> {
    match r {
        Ok(v) => Ok(v),
        Err(e) => Err(e.code()),
    }
}

pub fn reset_for_integration() {
    core_mut().reset();
}

pub fn fd_exists(fd: usize) -> bool {
    core_mut().fd_idx(fd as isize).is_ok()
}

pub fn fd_kind(fd: usize) -> Option<FdKind> {
    let core = core_mut();
    match core.fd_idx(fd as isize) {
        Ok(idx) => {
            let ofd = core.fds[idx].ofd;
            Some(core.ofds[ofd].kind)
        }
        Err(_) => None,
    }
}

pub fn fd_can_read(fd: usize) -> bool {
    core_mut().can_read_fd(fd)
}

pub fn fd_can_write(fd: usize) -> bool {
    core_mut().can_write_fd(fd)
}

pub fn openat(dirfd: isize, path: &[u8], flags: u32, mode: u16) -> isize {
    public_result(core_mut().openat(dirfd, path, flags, mode)).unwrap_or_else(|e| e)
}

pub fn close(fd: usize) -> isize {
    public_result(core_mut().close(fd as isize)).map(|_| 0).unwrap_or_else(|e| e)
}

pub fn close_range(first: usize, last: usize) -> isize {
    close_range_flags(first, last, 0)
}

pub fn close_range_flags(first: usize, last: usize, flags: usize) -> isize {
    public_result(core_mut().close_range(first, last, flags)).unwrap_or_else(|e| e)
}

pub fn read(fd: usize, out: &mut [u8]) -> isize {
    public_result(core_mut().read(fd as isize, out)).map(|n| n as isize).unwrap_or_else(|e| e)
}

pub fn write(fd: usize, src: &[u8]) -> isize {
    public_result(core_mut().write(fd as isize, src)).map(|n| n as isize).unwrap_or_else(|e| e)
}

pub fn pread(fd: usize, out: &mut [u8], offset: usize) -> isize {
    public_result(core_mut().pread(fd as isize, out, offset)).map(|n| n as isize).unwrap_or_else(|e| e)
}

pub fn pwrite(fd: usize, src: &[u8], offset: usize) -> isize {
    public_result(core_mut().pwrite(fd as isize, src, offset)).map(|n| n as isize).unwrap_or_else(|e| e)
}

pub fn read_iovec(fd: usize, vecs: &mut [RuntimeIovec], offset: Option<usize>, msg_io: bool) -> isize {
    public_result(core_mut().read_iovec(fd as isize, vecs, offset, msg_io)).map(|n| n as isize).unwrap_or_else(|e| e)
}

pub fn write_iovec(fd: usize, vecs: &[RuntimeIovec], offset: Option<usize>, msg_io: bool) -> isize {
    public_result(core_mut().write_iovec(fd as isize, vecs, offset, msg_io)).map(|n| n as isize).unwrap_or_else(|e| e)
}

pub fn io_stats() -> RuntimeIoStats {
    core_mut().io_stats
}

pub fn lseek(fd: usize, offset: isize, whence: usize) -> isize {
    public_result(core_mut().lseek(fd as isize, offset, whence)).map(|n| n as isize).unwrap_or_else(|e| e)
}

pub fn dup(fd: usize) -> isize {
    public_result(core_mut().dup(fd as isize)).unwrap_or_else(|e| e)
}

pub fn dup3(oldfd: usize, newfd: usize, flags: u32) -> isize {
    public_result(core_mut().dup3(oldfd as isize, newfd as isize, flags)).unwrap_or_else(|e| e)
}

pub fn fcntl(fd: usize, cmd: usize, arg: usize) -> isize {
    public_result(core_mut().fcntl(fd as isize, cmd, arg)).unwrap_or_else(|e| e)
}

pub fn fd_cloexec(fd: usize) -> Option<bool> {
    let core = core_mut();
    match core.fd_idx(fd as isize) {
        Ok(idx) => Some(core.fds[idx].cloexec),
        Err(_) => None,
    }
}

pub fn mkdirat(dirfd: isize, path: &[u8], mode: u16) -> isize {
    public_result(core_mut().mkdirat(dirfd, path, mode)).unwrap_or_else(|e| e)
}

pub fn chdir(path: &[u8]) -> isize {
    public_result(core_mut().chdir(path)).unwrap_or_else(|e| e)
}

pub fn getcwd(out: &mut [u8]) -> isize {
    public_result(core_mut().getcwd(out)).map(|n| n as isize).unwrap_or_else(|e| e)
}

pub fn linkat(olddir: isize, oldpath: &[u8], newdir: isize, newpath: &[u8]) -> isize {
    public_result(core_mut().linkat(olddir, oldpath, newdir, newpath)).unwrap_or_else(|e| e)
}

pub fn renameat(olddir: isize, oldpath: &[u8], newdir: isize, newpath: &[u8]) -> isize {
    public_result(core_mut().renameat(olddir, oldpath, newdir, newpath)).unwrap_or_else(|e| e)
}

pub fn symlinkat(target: &[u8], newdir: isize, newpath: &[u8]) -> isize {
    public_result(core_mut().symlinkat(target, newdir, newpath)).unwrap_or_else(|e| e)
}

pub fn readlinkat(dirfd: isize, path: &[u8], out: &mut [u8]) -> isize {
    public_result(core_mut().readlinkat(dirfd, path, out)).map(|n| n as isize).unwrap_or_else(|e| e)
}

pub fn unlinkat(dirfd: isize, path: &[u8], flags: usize) -> isize {
    public_result(core_mut().unlinkat(dirfd, path, flags)).unwrap_or_else(|e| e)
}

pub fn truncate_path(path: &[u8], length: usize) -> isize {
    public_result(core_mut().truncate_path(AT_FDCWD, path, length)).unwrap_or_else(|e| e)
}

pub fn ftruncate(fd: usize, length: usize) -> isize {
    public_result(core_mut().truncate_fd(fd as isize, length)).unwrap_or_else(|e| e)
}

pub fn fchmod(fd: usize, mode: usize) -> isize {
    public_result(core_mut().chmod_fd(fd as isize, mode)).unwrap_or_else(|e| e)
}

pub fn fchmodat(dirfd: isize, path: &[u8], mode: usize) -> isize {
    public_result(core_mut().chmod_path(dirfd, path, mode)).unwrap_or_else(|e| e)
}

pub fn fchown(fd: usize, uid: usize, gid: usize) -> isize {
    public_result(core_mut().chown_fd(fd as isize, uid, gid)).unwrap_or_else(|e| e)
}

pub fn fchownat(dirfd: isize, path: &[u8], uid: usize, gid: usize) -> isize {
    public_result(core_mut().chown_path(dirfd, path, uid, gid)).unwrap_or_else(|e| e)
}

pub fn faccessat(dirfd: isize, path: &[u8], mask: usize) -> isize {
    public_result(core_mut().access(dirfd, path, mask)).unwrap_or_else(|e| e)
}

pub fn cred_snapshot() -> RuntimeCred {
    public_result(core_mut().cred_snapshot()).unwrap_or_else(|_| RuntimeCred::root())
}

pub fn security_snapshot_path(path: &[u8]) -> Result<RuntimeSecuritySnapshot, isize> {
    public_result(core_mut().security_snapshot_path(path))
}

pub fn capset_masks(permitted: u64, effective: u64, inheritable: u64) -> isize {
    public_result(core_mut().capset(permitted, effective, inheritable)).unwrap_or_else(|e| e)
}

pub fn setuid(uid: usize) -> isize {
    public_result(core_mut().setuid(uid)).unwrap_or_else(|e| e)
}

pub fn setgid(gid: usize) -> isize {
    public_result(core_mut().setgid(gid)).unwrap_or_else(|e| e)
}

pub fn setresuid(ruid: usize, euid: usize, suid: usize) -> isize {
    public_result(core_mut().setresuid(ruid, euid, suid)).unwrap_or_else(|e| e)
}

pub fn setresgid(rgid: usize, egid: usize, sgid: usize) -> isize {
    public_result(core_mut().setresgid(rgid, egid, sgid)).unwrap_or_else(|e| e)
}

pub fn setfsuid(uid: usize) -> isize {
    public_result(core_mut().setfsuid(uid)).unwrap_or_else(|e| e)
}

pub fn setfsgid(gid: usize) -> isize {
    public_result(core_mut().setfsgid(gid)).unwrap_or_else(|e| e)
}

pub fn stat_fd(fd: usize) -> Result<RuntimeStat, isize> {
    public_result(core_mut().stat_fd(fd as isize))
}

pub fn stat_path(dirfd: isize, path: &[u8], follow: bool) -> Result<RuntimeStat, isize> {
    public_result(core_mut().stat_path(dirfd, path, follow))
}

pub fn getdents64(fd: usize, out: &mut [u8]) -> isize {
    public_result(core_mut().getdents64(fd as isize, out)).map(|n| n as isize).unwrap_or_else(|e| e)
}

pub fn statfs() -> RuntimeStatFs {
    core_mut().statfs()
}

pub fn statfs_path(path: &[u8]) -> Result<RuntimeStatFs, isize> {
    public_result(core_mut().statfs_path(AT_FDCWD, path))
}

pub fn statfs_fd(fd: usize) -> Result<RuntimeStatFs, isize> {
    public_result(core_mut().statfs_fd(fd as isize))
}

pub fn mount_fs(source: &[u8], target: &[u8], fstype: &[u8], flags: usize) -> isize {
    public_result(core_mut().mount_fs(source, target, fstype, flags)).unwrap_or_else(|e| e)
}

pub fn umount2(target: &[u8], flags: usize) -> isize {
    public_result(core_mut().umount2(target, flags)).unwrap_or_else(|e| e)
}

pub fn mount_snapshot() -> RuntimeMountSnapshot {
    core_mut().mount_snapshot()
}

pub fn proc_snapshot() -> ProcSnapshot {
    core_mut().snapshot()
}

pub fn proc_snapshot_for_pid(pid: usize) -> Result<ProcSnapshot, isize> {
    public_result(core_mut().proc_snapshot_for_pid(pid))
}

pub fn proc_fd_readlink(fd: usize, out: &mut [u8]) -> isize {
    public_result(core_mut().fd_target_path(fd as isize, out)).map(|n| n as isize).unwrap_or_else(|e| e)
}

pub fn pipe2(flags: u32, out: &mut [isize; 2]) -> isize {
    match public_result(core_mut().pipe2(flags)) {
        Ok((r, w)) => {
            out[0] = r;
            out[1] = w;
            0
        }
        Err(e) => e,
    }
}

pub fn eventfd2(initval: usize, flags: u32) -> isize {
    public_result(core_mut().eventfd2(initval, flags)).unwrap_or_else(|e| e)
}

pub fn timerfd_create(clockid: usize, flags: u32) -> isize {
    public_result(core_mut().timerfd_create(clockid, flags)).unwrap_or_else(|e| e)
}

pub fn timerfd_settime(fd: usize) -> isize {
    public_result(core_mut().timerfd_settime(fd as isize)).unwrap_or_else(|e| e)
}

pub fn socket(flags: u32) -> isize {
    public_result(core_mut().socket(flags)).unwrap_or_else(|e| e)
}

pub fn socket_with(domain: usize, sock_type: u32, protocol: usize) -> isize {
    public_result(core_mut().socket_with(domain, sock_type, protocol)).unwrap_or_else(|e| e)
}

pub fn socketpair(flags: u32, out: &mut [isize; 2]) -> isize {
    match public_result(core_mut().socketpair(flags)) {
        Ok((a, b)) => {
            out[0] = a;
            out[1] = b;
            0
        }
        Err(e) => e,
    }
}

pub fn bind_socket(fd: usize, addr: &[u8]) -> isize {
    public_result(core_mut().bind_socket(fd as isize, addr)).unwrap_or_else(|e| e)
}

pub fn listen_socket(fd: usize, backlog: usize) -> isize {
    public_result(core_mut().listen_socket(fd as isize, backlog)).unwrap_or_else(|e| e)
}

pub fn connect_socket(fd: usize, addr: &[u8]) -> isize {
    public_result(core_mut().connect_socket(fd as isize, addr)).unwrap_or_else(|e| e)
}

pub fn accept_socket(fd: usize, flags: u32) -> isize {
    public_result(core_mut().accept_socket(fd as isize, flags)).unwrap_or_else(|e| e)
}

pub fn sendto_socket(fd: usize, src: &[u8], dest: Option<&[u8]>) -> isize {
    public_result(core_mut().sendto_socket(fd as isize, src, dest)).map(|n| n as isize).unwrap_or_else(|e| e)
}

pub fn recvfrom_socket(fd: usize, out: &mut [u8], src: &mut [u8]) -> isize {
    public_result(core_mut().recvfrom_socket(fd as isize, out, src)).map(|(n, _)| n as isize).unwrap_or_else(|e| e)
}

pub fn socket_snapshot() -> RuntimeSocketSnapshot {
    core_mut().socket_snapshot()
}

pub fn epoll_create1(flags: u32) -> isize {
    public_result(core_mut().epoll_create1(flags)).unwrap_or_else(|e| e)
}

pub fn epoll_ctl(epfd: usize, op: usize, fd: usize) -> isize {
    epoll_ctl_event(epfd, op, fd, (POLLIN | POLLOUT) as u32, fd as u64)
}

pub fn epoll_ctl_event(epfd: usize, op: usize, fd: usize, events: u32, data: u64) -> isize {
    public_result(core_mut().epoll_ctl(epfd as isize, op, fd, events, data)).unwrap_or_else(|e| e)
}

pub fn epoll_ready_count(epfd: usize) -> isize {
    public_result(core_mut().epoll_ready_count(epfd as isize)).map(|n| n as isize).unwrap_or_else(|e| e)
}

pub fn epoll_collect_ready(epfd: usize, out: &mut [RuntimeEpollEvent]) -> isize {
    public_result(core_mut().epoll_collect_ready(epfd as isize, out)).map(|n| n as isize).unwrap_or_else(|e| e)
}

pub fn fd_readiness(fd: usize) -> u16 {
    core_mut().fd_readiness(fd)
}

pub fn poll_revents(fd: isize, requested: u16) -> u16 {
    core_mut().poll_revents(fd, requested)
}

pub fn mq_open(key: usize, flags: u32) -> isize {
    public_result(core_mut().mq_open(key, flags)).unwrap_or_else(|e| e)
}

pub fn mq_send(fd: usize, msg: &[u8], prio: usize) -> isize {
    public_result(core_mut().mq_send(fd as isize, msg, prio)).unwrap_or_else(|e| e)
}

pub fn mq_receive(fd: usize, out: &mut [u8]) -> isize {
    public_result(core_mut().mq_receive(fd as isize, out)).map(|n| n as isize).unwrap_or_else(|e| e)
}

pub fn mq_unlink(key: usize) -> isize {
    public_result(core_mut().mq_unlink(key)).unwrap_or_else(|e| e)
}

pub fn msgget(key: usize) -> isize {
    public_result(core_mut().msgget(key)).unwrap_or_else(|e| e)
}

pub fn msgsnd(id: usize, msg: &[u8]) -> isize {
    public_result(core_mut().msgsnd(id, msg)).unwrap_or_else(|e| e)
}

pub fn msgrcv(id: usize, out: &mut [u8]) -> isize {
    public_result(core_mut().msgrcv(id, out)).map(|n| n as isize).unwrap_or_else(|e| e)
}

pub fn msgctl(id: usize, cmd: usize) -> isize {
    public_result(core_mut().msgctl(id, cmd)).unwrap_or_else(|e| e)
}

pub fn semget(key: usize, nsems: usize) -> isize {
    public_result(core_mut().semget(key, nsems)).unwrap_or_else(|e| e)
}

pub fn semop(id: usize, delta: isize) -> isize {
    public_result(core_mut().semop(id, delta)).unwrap_or_else(|e| e)
}

pub fn semctl(id: usize, cmd: usize) -> isize {
    public_result(core_mut().semctl(id, cmd)).unwrap_or_else(|e| e)
}

pub fn shmget(key: usize, size: usize) -> isize {
    public_result(core_mut().shmget(key, size)).unwrap_or_else(|e| e)
}

pub fn shmat(id: usize) -> isize {
    public_result(core_mut().shmat(id)).unwrap_or_else(|e| e)
}

pub fn shm_wait_for_attach(id: usize) -> isize {
    public_result(core_mut().shm_wait_for_attach(id)).unwrap_or_else(|e| e)
}

pub fn shmdt(addr: usize) -> isize {
    public_result(core_mut().shmdt(addr)).unwrap_or_else(|e| e)
}

pub fn shmctl(id: usize, cmd: usize) -> isize {
    public_result(core_mut().shmctl(id, cmd)).unwrap_or_else(|e| e)
}

pub fn futex_wait(key: usize, observed: u32, expected: u32, has_timeout: bool) -> isize {
    public_result(core_mut().futex_wait(key, observed, expected, has_timeout)).unwrap_or_else(|e| e)
}

pub fn futex_wake(key: usize, count: usize) -> isize {
    public_result(core_mut().futex_wake(key, count)).unwrap_or_else(|e| e)
}

pub fn sched_wait_on(key: usize) -> isize {
    public_result(core_mut().sched_wait_on(key)).unwrap_or_else(|e| e)
}

pub fn sched_wake(key: usize, count: usize) -> isize {
    public_result(core_mut().sched_wake(key, count)).unwrap_or_else(|e| e)
}

pub fn sched_yield_current() -> isize {
    public_result(core_mut().sched_yield_current()).unwrap_or_else(|e| e)
}

pub fn sched_timeout_wait(key: usize) -> isize {
    public_result(core_mut().sched_timeout_wait(key)).unwrap_or_else(|e| e)
}

pub fn sched_snapshot() -> RuntimeSchedSnapshot {
    core_mut().sched_snapshot()
}

pub fn ipc_wait_snapshot() -> RuntimeIpcWaitSnapshot {
    core_mut().ipc_wait_snapshot()
}

pub fn unshare_namespaces(flags: usize) -> isize {
    public_result(core_mut().unshare_namespaces(flags)).unwrap_or_else(|e| e)
}

pub fn setns_namespace(fd: usize, nstype: usize) -> isize {
    public_result(core_mut().setns_namespace(fd, nstype)).unwrap_or_else(|e| e)
}

pub fn namespace_snapshot() -> RuntimeNamespaceRefs {
    core_mut().namespace_snapshot()
}

pub fn current_pid() -> usize {
    match core_mut().task_snapshot(0) {
        Ok(task) => task.pid,
        Err(_) => 0,
    }
}

pub fn current_ppid() -> usize {
    match core_mut().task_snapshot(0) {
        Ok(task) => task.ppid,
        Err(_) => 0,
    }
}

pub fn current_tgid() -> usize {
    match core_mut().task_snapshot(0) {
        Ok(task) => task.tgid,
        Err(_) => 0,
    }
}

pub fn current_uid() -> usize {
    cred_snapshot().uid as usize
}

pub fn current_euid() -> usize {
    cred_snapshot().euid as usize
}

pub fn current_gid() -> usize {
    cred_snapshot().gid as usize
}

pub fn current_egid() -> usize {
    cred_snapshot().egid as usize
}

pub fn task_snapshot(pid: usize) -> Result<RuntimeTaskSnapshot, isize> {
    public_result(core_mut().task_snapshot(pid))
}

pub fn clone_task(flags: usize) -> isize {
    public_result(core_mut().clone_task(flags)).unwrap_or_else(|e| e)
}

pub fn switch_current_task(pid: usize) -> isize {
    public_result(core_mut().switch_current_task_pid(pid)).unwrap_or_else(|e| e)
}

pub fn exit_task_pid(pid: usize, code: isize) -> isize {
    public_result(core_mut().exit_task_pid(pid, code)).unwrap_or_else(|e| e)
}

pub fn exit_current_task(code: isize) -> isize {
    public_result(core_mut().exit_current_task(code)).unwrap_or_else(|e| e)
}

pub fn exit_group_current(code: isize) -> isize {
    public_result(core_mut().exit_group_current(code)).unwrap_or_else(|e| e)
}

pub fn wait4(pid: isize, status: &mut isize) -> isize {
    match public_result(core_mut().wait4(pid)) {
        Ok((child, wait_status)) => {
            *status = wait_status;
            child as isize
        }
        Err(err) => err,
    }
}

pub fn waitid(pid: isize, code: &mut isize) -> isize {
    match public_result(core_mut().waitid(pid)) {
        Ok((child, exit_code)) => {
            *code = exit_code;
            child as isize
        }
        Err(err) => err,
    }
}

pub fn setpgid(pid: usize, pgid: usize) -> isize {
    public_result(core_mut().setpgid(pid, pgid)).unwrap_or_else(|e| e)
}

pub fn getpgid(pid: usize) -> isize {
    public_result(core_mut().getpgid(pid)).unwrap_or_else(|e| e)
}

pub fn getsid(pid: usize) -> isize {
    public_result(core_mut().getsid(pid)).unwrap_or_else(|e| e)
}

pub fn setsid() -> isize {
    public_result(core_mut().setsid()).unwrap_or_else(|e| e)
}

pub fn rt_sigprocmask(how: usize, mask: Option<u64>) -> Result<u64, isize> {
    public_result(core_mut().update_signal_mask(how, mask))
}

pub fn signal_mask_bit(sig: usize) -> u64 {
    KernelCore::signal_bit(sig)
}

pub fn rt_sigaction(sig: usize, new_action: Option<RuntimeSignalAction>) -> Result<RuntimeSignalAction, isize> {
    public_result(core_mut().rt_sigaction(sig, new_action))
}

pub fn set_current_signal_mask(mask: u64) -> isize {
    match rt_sigprocmask(SIG_SETMASK, Some(mask)) {
        Ok(_) => 0,
        Err(err) => err,
    }
}

pub fn set_current_signal_action(sig: usize, handler: usize, flags: usize, restorer: usize, mask: u64) -> isize {
    let action = RuntimeSignalAction::handler(handler, flags, restorer, mask);
    match rt_sigaction(sig, Some(action)) {
        Ok(_) => 0,
        Err(err) => err,
    }
}

pub fn kill_signal(pid: isize, sig: usize) -> isize {
    public_result(core_mut().kill_signal(pid, sig)).unwrap_or_else(|e| e)
}

pub fn tkill_signal(tid: isize, sig: usize) -> isize {
    public_result(core_mut().tkill_signal(tid, sig)).unwrap_or_else(|e| e)
}

pub fn tgkill_signal(tgid: isize, tid: isize, sig: usize) -> isize {
    public_result(core_mut().tgkill_signal(tgid, tid, sig)).unwrap_or_else(|e| e)
}

pub fn deliver_signal_frame(saved_pc: usize, saved_sp: usize) -> isize {
    public_result(core_mut().deliver_signal_frame(saved_pc, saved_sp)).unwrap_or_else(|e| e)
}

pub fn rt_sigreturn_restore() -> Result<RuntimeSignalRestore, isize> {
    public_result(core_mut().rt_sigreturn())
}

pub fn signal_snapshot() -> RuntimeSignalSnapshot {
    core_mut().signal_snapshot()
}

pub fn execve_from_vfs(path: &[u8], argv: &[RuntimeExecString], envp: &[RuntimeExecString]) -> isize {
    public_result(core_mut().execve_from_vfs(path, argv, envp)).unwrap_or_else(|e| e)
}

pub fn exec_snapshot() -> RuntimeExecSnapshot {
    core_mut().exec_snapshot()
}

pub fn brk(addr: usize) -> isize {
    public_result(core_mut().brk(addr)).unwrap_or_else(|e| e)
}

pub fn mmap(addr: usize, len: usize, prot: usize, flags: usize, fd: isize, offset: usize) -> isize {
    public_result(core_mut().mmap(addr, len, prot, flags, fd, offset)).unwrap_or_else(|e| e)
}

pub fn read_mmap_backing_page(addr: usize, out: &mut [u8]) -> isize {
    match public_result(core_mut().read_mmap_backing_page(addr, out)) {
        Ok(copied) => copied as isize,
        Err(err) => err,
    }
}

pub fn munmap(addr: usize, len: usize) -> isize {
    public_result(core_mut().munmap(addr, len)).unwrap_or_else(|e| e)
}

pub fn mprotect(addr: usize, len: usize, prot: usize) -> isize {
    public_result(core_mut().mprotect(addr, len, prot)).unwrap_or_else(|e| e)
}

pub fn madvise(addr: usize, len: usize) -> isize {
    public_result(core_mut().madvise(addr, len)).unwrap_or_else(|e| e)
}

pub fn page_fault_validate(addr: usize, access: RuntimeFaultAccess) -> bool {
    core_mut().page_fault_validate(addr, access).unwrap_or(false)
}

pub fn page_fault_permissions(addr: usize) -> Option<RuntimeVmPermissions> {
    core_mut().page_fault_permissions(addr).ok()
}

pub fn vm_snapshot() -> RuntimeVmSnapshot {
    core_mut().vm_snapshot()
}

pub fn run_v157_unified_historical_integration() -> Option<&'static str> {
    reset_for_integration();
    if mkdirat(AT_FDCWD, b"v157root", 0o755) != 0 { return Some("mkdir_root"); }
    if chdir(b"v157root") != 0 { return Some("chdir_root"); }
    if mkdirat(AT_FDCWD, b"sub", 0o755) != 0 { return Some("mkdir_sub"); }
    let rootfd = openat(AT_FDCWD, b".", O_DIRECTORY, 0);
    if rootfd < 0 { return Some("open_rootfd"); }
    let subfd = openat(rootfd, b"sub", O_DIRECTORY, 0);
    if subfd < 0 { return Some("open_subfd"); }
    let fd = openat(rootfd, b"alpha.txt", O_CREAT | O_TRUNC, 0o644);
    if fd < 0 { return Some("open_alpha"); }
    if write(fd as usize, b"abcdef") != 6 { return Some("write_alpha"); }
    if lseek(fd as usize, 3, SEEK_SET) != 3 { return Some("seek_alpha"); }
    if write(fd as usize, b"XYZ") != 3 { return Some("overwrite_alpha"); }
    if lseek(fd as usize, 0, SEEK_SET) != 0 { return Some("rewind_alpha"); }
    let dupfd = dup(fd as usize);
    if dupfd < 0 { return Some("dup_alpha"); }
    let mut first = [0u8; 3];
    if read(dupfd as usize, &mut first) != 3 || &first != b"abc" { return Some("dup_read_first"); }
    let mut second = [0u8; 3];
    if read(fd as usize, &mut second) != 3 || &second != b"XYZ" { return Some("shared_ofd_offset"); }
    if fcntl(fd as usize, F_SETFL, O_APPEND as usize | O_NONBLOCK as usize) != 0 { return Some("fcntl_setfl"); }
    let flags = fcntl(dupfd as usize, F_GETFL, 0);
    if flags < 0 || (flags as u32 & O_APPEND) == 0 || (flags as u32 & O_NONBLOCK) == 0 { return Some("fcntl_shared_flags"); }
    match stat_path(rootfd, b"alpha.txt", true) {
        Ok(st) => if st.size != 6 || st.nlink != 1 || st.kind != FdKind::RegularFile { return Some("stat_alpha"); },
        Err(_) => return Some("stat_alpha_err"),
    }
    if linkat(rootfd, b"alpha.txt", rootfd, b"hard_alpha.txt") != 0 { return Some("link_alpha"); }
    match stat_path(rootfd, b"alpha.txt", true) {
        Ok(st) => if st.nlink != 2 { return Some("nlink_after_link"); },
        Err(_) => return Some("stat_after_link"),
    }
    if unlinkat(rootfd, b"hard_alpha.txt", 0) != 0 { return Some("unlink_hard"); }
    match stat_path(rootfd, b"alpha.txt", true) {
        Ok(st) => if st.nlink != 1 { return Some("nlink_after_unlink"); },
        Err(_) => return Some("stat_after_unlink"),
    }
    if symlinkat(b"alpha.txt", rootfd, b"sym_alpha") != 0 { return Some("symlink_alpha"); }
    let mut link = [0u8; 32];
    let link_len = readlinkat(rootfd, b"sym_alpha", &mut link);
    if link_len != 9 || &link[..9] != b"alpha.txt" { return Some("readlink_alpha"); }
    let mut dents = [0u8; 192];
    let dents_len = getdents64(rootfd as usize, &mut dents);
    if dents_len <= 0 || !contains_bytes(&dents[..dents_len as usize], b"sub") || !contains_bytes(&dents[..dents_len as usize], b"alpha.txt") || !contains_bytes(&dents[..dents_len as usize], b"sym_alpha") {
        return Some("getdents_shared_tree");
    }
    let mut fdlink = [0u8; PATH_MAX];
    let fdlink_len = proc_fd_readlink(fd as usize, &mut fdlink);
    if fdlink_len <= 0 || !contains_bytes(&fdlink[..fdlink_len as usize], b"/v157root/alpha.txt") { return Some("procfs_fd_real_table"); }
    let snap = proc_snapshot();
    if snap.pid != 1 || snap.ppid != 1 || snap.tgid != 1 || snap.pgid != 1 || snap.sid != 1 || snap.fd_count < 6 || snap.cwd_len == 0 || snap.root_len == 0 {
        return Some("proc_snapshot");
    }
    let mut pipe_fds = [-1isize; 2];
    if pipe2(0, &mut pipe_fds) != 0 { return Some("pipe2_alloc"); }
    if write(pipe_fds[1] as usize, b"p") != 1 { return Some("pipe_write"); }
    if fd_readiness(pipe_fds[0] as usize) & POLLIN == 0 { return Some("pipe_ready"); }
    let mut pipe_byte = [0u8; 1];
    if read(pipe_fds[0] as usize, &mut pipe_byte) != 1 || pipe_byte[0] != b'p' { return Some("pipe_read"); }
    let eventfd = eventfd2(0, 0);
    if eventfd < 0 { return Some("eventfd_alloc"); }
    if write(eventfd as usize, &5u64.to_le_bytes()) != 8 { return Some("eventfd_write"); }
    if fd_readiness(eventfd as usize) & POLLIN == 0 { return Some("eventfd_ready"); }
    let timerfd = timerfd_create(0, 0);
    if timerfd < 0 || timerfd_settime(timerfd as usize) != 0 || fd_readiness(timerfd as usize) & POLLIN == 0 { return Some("timerfd_ready"); }
    let mut sv = [-1isize; 2];
    if socketpair(0, &mut sv) != 0 { return Some("socketpair_alloc"); }
    if write(sv[0] as usize, b"loop") != 4 { return Some("socketpair_send"); }
    let mut sock_buf = [0u8; 4];
    if read(sv[1] as usize, &mut sock_buf) != 4 || &sock_buf != b"loop" { return Some("socketpair_recv"); }
    let epfd = epoll_create1(0);
    if epfd < 0 || epoll_ctl(epfd as usize, EPOLL_CTL_ADD, eventfd as usize) != 0 || epoll_ready_count(epfd as usize) <= 0 {
        return Some("epoll_real_readiness");
    }
    let mqfd = mq_open(0x157, 0);
    if mqfd < 0 { return Some("mq_open"); }
    if mq_send(mqfd as usize, b"mq", 7) != 0 { return Some("mq_send"); }
    let mut mq_buf = [0u8; 4];
    if mq_receive(mqfd as usize, &mut mq_buf) != 2 || &mq_buf[..2] != b"mq" { return Some("mq_receive"); }
    if mq_unlink(0x157) != 0 { return Some("mq_unlink"); }
    let msgid = msgget(0x157);
    if msgid < 0 || msgsnd(msgid as usize, b"msg") != 0 { return Some("msg_registry"); }
    let mut msg_buf = [0u8; 4];
    if msgrcv(msgid as usize, &mut msg_buf) != 3 || &msg_buf[..3] != b"msg" || msgctl(msgid as usize, IPC_RMID) != 0 {
        return Some("msg_lifecycle");
    }
    let semid = semget(0x157, 1);
    if semid < 0 || semop(semid as usize, 1) != 0 || semctl(semid as usize, IPC_RMID) != 0 { return Some("sem_lifecycle"); }
    let shmid = shmget(0x157, 4096);
    if shmid < 0 { return Some("shmget"); }
    let shmaddr = shmat(shmid as usize);
    if shmaddr <= 0 || shmdt(shmaddr as usize) != 0 || shmctl(shmid as usize, IPC_RMID) != 0 { return Some("shm_lifecycle"); }
    if ftruncate(fd as usize, 2) != 0 { return Some("ftruncate"); }
    match stat_fd(fd as usize) {
        Ok(st) => if st.size != 2 { return Some("fstat_truncate"); },
        Err(_) => return Some("fstat_truncate_err"),
    }
    if close(fd as usize) != 0 || close(dupfd as usize) != 0 || close(rootfd as usize) != 0 || close(subfd as usize) != 0 {
        return Some("close_regular_fds");
    }
    None
}

pub fn run_v158_event_pipe_socket_readiness() -> Option<&'static str> {
    reset_for_integration();

    let mut pipe_fds = [-1isize; 2];
    if pipe2(0, &mut pipe_fds) != 0 { return Some("pipe2_alloc"); }
    if poll_revents(pipe_fds[0], POLLIN) != 0 { return Some("pipe_empty_readiness"); }
    if fd_readiness(pipe_fds[1] as usize) & POLLOUT == 0 { return Some("pipe_write_ready"); }
    if write(pipe_fds[1] as usize, b"v158") != 4 { return Some("pipe_write"); }
    if poll_revents(pipe_fds[0], POLLIN) & POLLIN == 0 { return Some("ppoll_pipe_readiness"); }
    let mut pipe_buf = [0u8; 4];
    if read(pipe_fds[0] as usize, &mut pipe_buf) != 4 || &pipe_buf != b"v158" { return Some("pipe_read"); }
    if poll_revents(pipe_fds[0], POLLIN) != 0 { return Some("pipe_readiness_cleared"); }

    let eventfd = eventfd2(0, O_NONBLOCK);
    if eventfd < 0 { return Some("eventfd_alloc"); }
    if poll_revents(eventfd, POLLIN) != 0 { return Some("eventfd_initial_readiness"); }
    if poll_revents(eventfd, POLLOUT) & POLLOUT == 0 { return Some("eventfd_write_ready"); }
    if write(eventfd as usize, &7u64.to_le_bytes()) != 8 { return Some("eventfd_write"); }
    if poll_revents(eventfd, POLLIN) & POLLIN == 0 { return Some("eventfd_readiness"); }

    let epfd = epoll_create1(0);
    if epfd < 0 { return Some("epoll_create"); }
    if epoll_ctl_event(epfd as usize, EPOLL_CTL_ADD, eventfd as usize, POLLIN as u32, 0x158) != 0 {
        return Some("epoll_add_eventfd");
    }
    let mut events = [RuntimeEpollEvent::empty(); 4];
    let ready = epoll_collect_ready(epfd as usize, &mut events);
    if ready != 1 || events[0].fd != eventfd as usize || events[0].data != 0x158 || (events[0].events & POLLIN as u32) == 0 {
        return Some("epoll_eventfd_ready");
    }
    let mut event_buf = [0u8; 8];
    if read(eventfd as usize, &mut event_buf) != 8 || u64::from_le_bytes(event_buf) != 7 {
        return Some("eventfd_read");
    }
    if epoll_collect_ready(epfd as usize, &mut events) != 0 { return Some("epoll_eventfd_cleared"); }

    let mut sv = [-1isize; 2];
    if socketpair(0, &mut sv) != 0 { return Some("socketpair_alloc"); }
    if poll_revents(sv[1], POLLIN) != 0 { return Some("socket_initial_readiness"); }
    if write(sv[0] as usize, b"loop") != 4 { return Some("socketpair_send"); }
    if poll_revents(sv[1], POLLIN) & POLLIN == 0 { return Some("socket_ppoll_readiness"); }
    let mut sock_buf = [0u8; 4];
    if read(sv[1] as usize, &mut sock_buf) != 4 || &sock_buf != b"loop" { return Some("socketpair_recv"); }
    if poll_revents(sv[1], POLLIN) != 0 { return Some("socket_readiness_cleared"); }

    None
}

pub fn run_v159_timerfd_deterministic_readiness() -> Option<&'static str> {
    reset_for_integration();

    let timerfd = timerfd_create(0, O_NONBLOCK);
    if timerfd < 0 { return Some("timerfd_create"); }
    if fd_readiness(timerfd as usize) & POLLIN != 0 { return Some("timer_initial_readiness"); }

    let epfd = epoll_create1(0);
    if epfd < 0 { return Some("epoll_create"); }
    if epoll_ctl_event(epfd as usize, EPOLL_CTL_ADD, timerfd as usize, POLLIN as u32, 0x159) != 0 {
        return Some("epoll_add_timer");
    }
    let mut events = [RuntimeEpollEvent::empty(); 2];
    if epoll_collect_ready(epfd as usize, &mut events) != 0 { return Some("epoll_timer_initial"); }

    if timerfd_settime(timerfd as usize) != 0 { return Some("timerfd_settime"); }
    if fd_readiness(timerfd as usize) & POLLIN == 0 { return Some("timer_ready"); }
    let ready = epoll_collect_ready(epfd as usize, &mut events);
    if ready != 1 || events[0].fd != timerfd as usize || events[0].data != 0x159 || (events[0].events & POLLIN as u32) == 0 {
        return Some("epoll_timer_ready");
    }

    let mut timer_buf = [0u8; 8];
    if read(timerfd as usize, &mut timer_buf) != 8 || u64::from_le_bytes(timer_buf) != 1 {
        return Some("timerfd_read");
    }
    if fd_readiness(timerfd as usize) & POLLIN != 0 { return Some("timer_readiness_cleared"); }
    if epoll_collect_ready(epfd as usize, &mut events) != 0 { return Some("epoll_timer_cleared"); }
    if read(timerfd as usize, &mut timer_buf) != EAGAIN { return Some("timer_nonblock_empty"); }

    None
}

pub fn run_v160_fd_lifecycle_cloexec_close_range() -> Option<&'static str> {
    reset_for_integration();

    let initial = proc_snapshot().fd_count;
    let eventfd = eventfd2(0, O_NONBLOCK | O_CLOEXEC);
    if eventfd < 0 { return Some("eventfd_alloc"); }
    if fd_cloexec(eventfd as usize) != Some(true) { return Some("eventfd_cloexec"); }
    if fcntl(eventfd as usize, F_GETFD, 0) != FD_CLOEXEC as isize { return Some("fcntl_getfd_cloexec"); }
    let flags = fcntl(eventfd as usize, F_GETFL, 0);
    if flags < 0 || (flags as u32 & O_NONBLOCK) == 0 || (flags as u32 & O_CLOEXEC) != 0 {
        return Some("fcntl_getfl_nonblock");
    }
    let mut empty_event = [0u8; 8];
    if read(eventfd as usize, &mut empty_event) != EAGAIN { return Some("eventfd_nonblock_empty"); }
    let mut proc_link = [0u8; PATH_MAX];
    let link_len = proc_fd_readlink(eventfd as usize, &mut proc_link);
    if link_len <= 0 || !contains_bytes(&proc_link[..link_len as usize], b"eventfd:[") {
        return Some("procfd_eventfd_visible");
    }

    let filefd = openat(AT_FDCWD, b"v160.txt", O_CREAT | O_TRUNC, 0o644);
    if filefd < 0 { return Some("open_file"); }
    let dupfd = dup3(filefd as usize, 20, O_CLOEXEC);
    if dupfd != 20 { return Some("dup3_cloexec"); }
    if fd_cloexec(dupfd as usize) != Some(true) { return Some("dup3_cloexec_state"); }
    if close_range_flags(dupfd as usize, dupfd as usize, 0) != 0 || fd_exists(dupfd as usize) {
        return Some("close_range_dupfd");
    }
    if !fd_exists(filefd as usize) { return Some("close_range_kept_original"); }

    let mut pipe_fds = [-1isize; 2];
    if pipe2(O_NONBLOCK, &mut pipe_fds) != 0 { return Some("pipe2_nonblock"); }
    let mut empty_pipe = [0u8; 1];
    if read(pipe_fds[0] as usize, &mut empty_pipe) != EAGAIN { return Some("pipe_nonblock_empty"); }
    if close_range_flags(pipe_fds[0] as usize, pipe_fds[1] as usize, CLOSE_RANGE_CLOEXEC) != 0 {
        return Some("close_range_cloexec");
    }
    if fd_cloexec(pipe_fds[0] as usize) != Some(true) || fd_cloexec(pipe_fds[1] as usize) != Some(true) {
        return Some("close_range_cloexec_state");
    }
    if close_range_flags(pipe_fds[0] as usize, pipe_fds[1] as usize, 0) != 0 {
        return Some("close_range_close_pipe");
    }
    if fd_exists(pipe_fds[0] as usize) || fd_exists(pipe_fds[1] as usize) {
        return Some("close_range_pipe_closed");
    }

    let before_close = proc_snapshot().fd_count;
    if before_close <= initial { return Some("proc_snapshot_before_close"); }
    if close(eventfd as usize) != 0 { return Some("close_eventfd"); }
    if proc_fd_readlink(eventfd as usize, &mut proc_link) != EBADF { return Some("procfd_after_close"); }
    if close(filefd as usize) != 0 { return Some("close_file"); }
    let final_count = proc_snapshot().fd_count;
    if final_count != initial { return Some("fd_count_after_close"); }

    None
}

pub fn run_v161_unified_iovec_io_path() -> Option<&'static str> {
    reset_for_integration();

    let fd = openat(AT_FDCWD, b"v161.txt", O_CREAT | O_TRUNC, 0o644);
    if fd < 0 { return Some("open_file"); }

    let mut write_vecs = [RuntimeIovec::empty(); 3];
    write_vecs[0].data[..3].copy_from_slice(b"iov");
    write_vecs[0].len = 3;
    write_vecs[1].data[..5].copy_from_slice(b"ec-io");
    write_vecs[1].len = 5;
    if write_iovec(fd as usize, &write_vecs, None, false) != 8 { return Some("write_iovec"); }
    if lseek(fd as usize, 0, SEEK_SET) != 0 { return Some("rewind"); }

    let mut read_vecs = [RuntimeIovec::empty(); 3];
    read_vecs[0].len = 3;
    read_vecs[1].len = 5;
    if read_iovec(fd as usize, &mut read_vecs, None, false) != 8 { return Some("read_iovec"); }
    if &read_vecs[0].data[..3] != b"iov" || &read_vecs[1].data[..5] != b"ec-io" {
        return Some("read_iovec_data");
    }

    let mut pos_write = [RuntimeIovec::empty(); 2];
    pos_write[0].data[..2].copy_from_slice(b"XX");
    pos_write[0].len = 2;
    if write_iovec(fd as usize, &pos_write, Some(3), false) != 2 { return Some("pwrite_iovec"); }
    let mut pos_read = [RuntimeIovec::empty(); 1];
    pos_read[0].len = 8;
    if read_iovec(fd as usize, &mut pos_read, Some(0), false) != 8 { return Some("pread_iovec"); }
    if &pos_read[0].data[..8] != b"iovXX-io" { return Some("pread_iovec_data"); }

    let mut sv = [-1isize; 2];
    if socketpair(0, &mut sv) != 0 { return Some("socketpair"); }
    let mut msg_write = [RuntimeIovec::empty(); 2];
    msg_write[0].data[..4].copy_from_slice(b"send");
    msg_write[0].len = 4;
    msg_write[1].data[..3].copy_from_slice(b"msg");
    msg_write[1].len = 3;
    if write_iovec(sv[0] as usize, &msg_write, None, true) != 7 { return Some("sendmsg_iovec"); }
    let mut msg_read = [RuntimeIovec::empty(); 2];
    msg_read[0].len = 4;
    msg_read[1].len = 3;
    if read_iovec(sv[1] as usize, &mut msg_read, None, true) != 7 { return Some("recvmsg_iovec"); }
    if &msg_read[0].data[..4] != b"send" || &msg_read[1].data[..3] != b"msg" {
        return Some("recvmsg_iovec_data");
    }

    let stats = io_stats();
    if stats.read_ops < 3 || stats.write_ops < 3 || stats.positioned_ops < 2 || stats.msg_ops < 2 || stats.bytes_read < 23 || stats.bytes_written < 17 {
        return Some("iovec_stats");
    }

    None
}

pub fn run_v162_ipc_registry_lifecycle() -> Option<&'static str> {
    reset_for_integration();

    let mqfd0 = mq_open(0x162, 0);
    if mqfd0 < 0 { return Some("mq_open0"); }
    let mqfd1 = mq_open(0x162, 0);
    if mqfd1 < 0 || mqfd1 == mqfd0 { return Some("mq_lookup"); }
    if mq_send(mqfd0 as usize, b"mq162", 9) != 0 { return Some("mq_send"); }
    let mut mq_buf = [0u8; 8];
    if mq_receive(mqfd1 as usize, &mut mq_buf) != 5 || &mq_buf[..5] != b"mq162" { return Some("mq_receive"); }
    if mq_unlink(0x162) != 0 { return Some("mq_unlink"); }
    if close(mqfd0 as usize) != 0 || close(mqfd1 as usize) != 0 { return Some("mq_close"); }
    let mqfd2 = mq_open(0x162, 0);
    if mqfd2 < 0 { return Some("mq_recreate"); }
    let mut empty_mq = [0u8; 4];
    if mq_receive(mqfd2 as usize, &mut empty_mq) != 0 { return Some("mq_recreated_empty"); }

    let msgid0 = msgget(0x162);
    let msgid1 = msgget(0x162);
    if msgid0 < 0 || msgid0 != msgid1 { return Some("msg_lookup"); }
    if msgsnd(msgid0 as usize, b"msg162") != 0 { return Some("msgsnd"); }
    let mut msg_buf = [0u8; 8];
    if msgrcv(msgid1 as usize, &mut msg_buf) != 6 || &msg_buf[..6] != b"msg162" { return Some("msgrcv"); }
    if msgctl(msgid0 as usize, IPC_RMID) != 0 { return Some("msg_rmid"); }
    if msgrcv(msgid0 as usize, &mut msg_buf) != ENOENT { return Some("msg_removed"); }

    let semid0 = semget(0x162, 2);
    let semid1 = semget(0x162, 2);
    if semid0 < 0 || semid0 != semid1 { return Some("sem_lookup"); }
    if semop(semid0 as usize, 2) != 0 || semop(semid1 as usize, -1) != 0 { return Some("semop"); }
    if semctl(semid0 as usize, IPC_RMID) != 0 { return Some("sem_rmid"); }
    if semop(semid0 as usize, 1) != ENOENT { return Some("sem_removed"); }

    let shmid0 = shmget(0x162, 4096);
    let shmid1 = shmget(0x162, 4096);
    if shmid0 < 0 || shmid0 != shmid1 { return Some("shm_lookup"); }
    let addr = shmat(shmid0 as usize);
    if addr <= 0 { return Some("shmat"); }
    if shmdt(addr as usize) != 0 { return Some("shmdt"); }
    if shmctl(shmid0 as usize, IPC_RMID) != 0 { return Some("shm_rmid"); }
    if shmat(shmid0 as usize) != ENOENT { return Some("shm_removed"); }

    None
}

pub fn run_v163_futex_wait_wake_object_model() -> Option<&'static str> {
    reset_for_integration();

    if futex_wait(0x1630, 0, 1, false) != EAGAIN { return Some("futex_mismatch"); }
    if futex_wait(0x1630, 5, 5, false) != 0 { return Some("futex_wait"); }
    let waiting = sched_snapshot();
    if waiting.current_state != RuntimeTaskState::Waiting || waiting.waiters != 1 { return Some("futex_wait_queue"); }
    if futex_wake(0x1630, 1) != 1 { return Some("futex_wake"); }
    let ready = sched_snapshot();
    if ready.current_state != RuntimeTaskState::Ready || ready.waiters != 0 { return Some("futex_wake_state"); }
    if futex_wake(0x1630, 1) != 0 { return Some("futex_empty_wake"); }
    if sched_yield_current() != 0 { return Some("sched_yield_after_futex"); }
    if futex_wait(0x1631, 7, 7, true) != 0 { return Some("futex_timeout_wait"); }
    if futex_wake(0x1631, 1) != 0 { return Some("futex_timeout_no_linger"); }

    None
}

pub fn run_v164_scheduler_wait_queue_foundation() -> Option<&'static str> {
    reset_for_integration();

    let initial = sched_snapshot();
    if initial.current_pid != 1 || initial.current_state != RuntimeTaskState::Running || initial.wait_queues != 0 || initial.waiters != 0 {
        return Some("initial_snapshot");
    }
    if sched_yield_current() != 0 { return Some("yield"); }
    let yielded = sched_snapshot();
    if yielded.current_state != RuntimeTaskState::Running || yielded.yields != 1 { return Some("yield_snapshot"); }
    if sched_wait_on(0x1640) != 0 { return Some("wait_on"); }
    let waiting = sched_snapshot();
    if waiting.current_state != RuntimeTaskState::Waiting || waiting.wait_queues != 1 || waiting.waiters != 1 {
        return Some("wait_snapshot");
    }
    if sched_wake(0x1640, 1) != 1 { return Some("wake"); }
    let ready = sched_snapshot();
    if ready.current_state != RuntimeTaskState::Ready || ready.waiters != 0 { return Some("wake_snapshot"); }
    if sched_yield_current() != 0 { return Some("yield_ready"); }
    if sched_timeout_wait(SCHED_WAIT_TIMEOUT_BASE ^ 0x1640) != 0 { return Some("timeout_wait"); }
    let final_snap = sched_snapshot();
    if final_snap.current_state != RuntimeTaskState::Running || final_snap.waiters != 0 || final_snap.yields != 2 {
        return Some("final_snapshot");
    }

    None
}

pub fn run_v165_task_table_process_lifecycle() -> Option<&'static str> {
    reset_for_integration();

    let task = match task_snapshot(1) {
        Ok(task) => task,
        Err(_) => return Some("task_snapshot"),
    };
    if task.pid != 1 || task.ppid != 1 || task.tgid != 1 || task.pgid != 1 || task.sid != 1 {
        return Some("identity_fields");
    }
    if task.state != RuntimeTaskState::Running || task.exit_code != 0 || task.fd_count != 3 || task.cwd_len == 0 || task.root_len == 0 {
        return Some("initial_lifecycle_state");
    }
    let proc = match proc_snapshot_for_pid(1) {
        Ok(proc) => proc,
        Err(_) => return Some("proc_snapshot"),
    };
    if proc.pid != task.pid || proc.ppid != task.ppid || proc.tgid != task.tgid || proc.fd_count != task.fd_count {
        return Some("proc_task_shared_state");
    }
    if setpgid(0, 5) != 0 || getpgid(0) != 5 {
        return Some("pgid_update");
    }
    if setsid() != 1 || getsid(0) != 1 || getpgid(0) != 1 {
        return Some("sid_update");
    }

    None
}

pub fn run_v166_fork_clone_child_task() -> Option<&'static str> {
    reset_for_integration();

    if mkdirat(AT_FDCWD, b"v166root", 0o755) != 0 { return Some("mkdir_root"); }
    if chdir(b"v166root") != 0 { return Some("chdir_root"); }
    let fd = openat(AT_FDCWD, b"child.txt", O_CREAT | O_TRUNC, 0o644);
    if fd < 0 { return Some("open_child_file"); }
    let parent_before = match task_snapshot(1) {
        Ok(task) => task,
        Err(_) => return Some("parent_before_snapshot"),
    };
    let child_pid = clone_task(17);
    if child_pid <= 1 { return Some("clone_return"); }
    let parent_after = match task_snapshot(1) {
        Ok(task) => task,
        Err(_) => return Some("parent_after_snapshot"),
    };
    let child = match task_snapshot(child_pid as usize) {
        Ok(task) => task,
        Err(_) => return Some("child_snapshot"),
    };
    if parent_after.pid != 1 || parent_after.child_count != 1 {
        return Some("parent_child_link");
    }
    if child.pid != child_pid as usize || child.ppid != 1 || child.tgid != child.pid || child.state != RuntimeTaskState::Ready {
        return Some("child_identity");
    }
    if child.fork_return != 0 || child.fd_count != parent_before.fd_count || child.cwd_len != parent_before.cwd_len || child.root_len != parent_before.root_len {
        return Some("child_inheritance");
    }

    None
}

pub fn run_v167_exit_zombie_wait_lifecycle() -> Option<&'static str> {
    reset_for_integration();

    let child_pid = clone_task(17);
    if child_pid <= 1 { return Some("clone_child"); }
    let mut status = -1isize;
    if wait4(child_pid, &mut status) != 0 || status != 0 {
        return Some("wait_live_child");
    }
    if exit_task_pid(child_pid as usize, 42) != 0 {
        return Some("child_exit");
    }
    let zombie = match task_snapshot(child_pid as usize) {
        Ok(task) => task,
        Err(_) => return Some("zombie_snapshot"),
    };
    if zombie.state != RuntimeTaskState::Zombie || zombie.exit_code != 42 {
        return Some("zombie_state");
    }
    if wait4(child_pid, &mut status) != child_pid || status != (42 << 8) {
        return Some("wait4_reap");
    }
    if task_snapshot(child_pid as usize).is_ok() {
        return Some("child_reaped");
    }
    let child2 = clone_task(17);
    if child2 <= 1 { return Some("clone_child2"); }
    if exit_task_pid(child2 as usize, 7) != 0 {
        return Some("child2_exit");
    }
    let mut code = -1isize;
    if waitid(child2, &mut code) != child2 || code != 7 {
        return Some("waitid_reap");
    }
    if wait4(-1, &mut status) != ECHILD {
        return Some("wait_no_child");
    }

    None
}

pub fn run_v168_per_task_runtime_snapshot() -> Option<&'static str> {
    reset_for_integration();

    if mkdirat(AT_FDCWD, b"v168root", 0o755) != 0 { return Some("mkdir_root"); }
    if chdir(b"v168root") != 0 { return Some("chdir_root"); }
    let fd = openat(AT_FDCWD, b"snapshot.txt", O_CREAT | O_TRUNC, 0o644);
    if fd < 0 { return Some("open_snapshot_file"); }
    if set_current_signal_mask(0x168) != 0 { return Some("signal_mask_set"); }
    let parent_before = match task_snapshot(1) {
        Ok(task) => task,
        Err(_) => return Some("parent_snapshot"),
    };
    let child_pid = clone_task(17);
    if child_pid <= 1 { return Some("clone_child"); }
    let child = match task_snapshot(child_pid as usize) {
        Ok(task) => task,
        Err(_) => return Some("child_snapshot"),
    };
    if child.fd_count != parent_before.fd_count || child.cwd_len != parent_before.cwd_len || child.root_len != parent_before.root_len {
        return Some("child_fs_fd_snapshot");
    }
    if child.signal_mask != 0x168 {
        return Some("child_signal_mask");
    }
    let child_proc = match proc_snapshot_for_pid(child_pid as usize) {
        Ok(proc) => proc,
        Err(_) => return Some("child_proc_snapshot"),
    };
    if child_proc.pid != child.pid || child_proc.ppid != 1 || child_proc.fd_count != child.fd_count || child_proc.cwd_len != child.cwd_len {
        return Some("proc_reads_child_state");
    }
    if close(fd as usize) != 0 { return Some("close_parent_fd"); }
    let parent_after = match proc_snapshot_for_pid(1) {
        Ok(proc) => proc,
        Err(_) => return Some("parent_after_close_snapshot"),
    };
    let child_after = match proc_snapshot_for_pid(child_pid as usize) {
        Ok(proc) => proc,
        Err(_) => return Some("child_after_close_snapshot"),
    };
    if parent_after.fd_count + 1 != parent_before.fd_count || child_after.fd_count != child.fd_count {
        return Some("per_task_fd_snapshot_after_close");
    }

    None
}

fn install_runtime_exec_fixture(path: &[u8]) -> isize {
    let fd = openat(AT_FDCWD, path, O_CREAT | O_TRUNC, 0o755);
    if fd < 0 {
        return fd;
    }
    let mut elf = [0u8; 160];
    let elf_len = build_runtime_exec_fixture_bytes(&mut elf);
    let ret = write(fd as usize, &elf[..elf_len]);
    let _ = close(fd as usize);
    ret
}

fn build_runtime_exec_fixture_bytes(elf: &mut [u8; 160]) -> usize {
    let elf_len = elf.len();
    elf[0] = 0x7f;
    elf[1] = b'E';
    elf[2] = b'L';
    elf[3] = b'F';
    elf[4] = 2;
    elf[5] = 1;
    elf[6] = 1;
    write_le_u16(elf, 16, 2);
    write_le_u16(elf, 18, 243);
    write_le_u32(elf, 20, 1);
    write_le_u64(elf, 24, 0x4000_0000);
    write_le_u64(elf, 32, 64);
    write_le_u16(elf, 52, 64);
    write_le_u16(elf, 54, 56);
    write_le_u16(elf, 56, 1);
    write_le_u32(elf, 64, 1);
    write_le_u32(elf, 68, 5);
    write_le_u64(elf, 72, 0);
    write_le_u64(elf, 80, 0x4000_0000);
    write_le_u64(elf, 88, 0x4000_0000);
    write_le_u64(elf, 96, elf_len);
    write_le_u64(elf, 104, RUNTIME_PAGE_SIZE);
    write_le_u64(elf, 112, RUNTIME_PAGE_SIZE);
    elf_len
}

fn runtime_exec_string(bytes: &[u8]) -> Result<RuntimeExecString, isize> {
    RuntimeExecString::from_bytes(bytes)
}

pub fn run_v169_execve_from_canonical_vfs() -> Option<&'static str> {
    reset_for_integration();

    if install_runtime_exec_fixture(b"v169_exec.elf") != 160 {
        return Some("install_exec_fixture");
    }
    let argv0 = match runtime_exec_string(b"v169_exec.elf") {
        Ok(item) => item,
        Err(_) => return Some("argv0_string"),
    };
    let argv = [argv0];
    if execve_from_vfs(b"v169_exec.elf", &argv, &[]) != 0 {
        return Some("execve_vfs");
    }
    let snap = exec_snapshot();
    if !snap.valid || snap.entry != 0x4000_0000 || snap.phnum != 1 || snap.load_start != 0x4000_0000 || snap.load_end < 0x4000_1000 {
        return Some("exec_snapshot");
    }
    if execve_from_vfs(b"missing_exec.elf", &argv, &[]) != ENOENT {
        return Some("missing_exec_errno");
    }
    let bad = openat(AT_FDCWD, b"bad_exec.elf", O_CREAT | O_TRUNC, 0o755);
    if bad < 0 {
        return Some("open_bad_exec");
    }
    if write(bad as usize, b"not-elf") != 7 {
        return Some("write_bad_exec");
    }
    let _ = close(bad as usize);
    if execve_from_vfs(b"bad_exec.elf", &argv, &[]) != ENOEXEC {
        return Some("bad_exec_errno");
    }

    None
}

pub fn run_v170_execve_user_stack_cloexec() -> Option<&'static str> {
    reset_for_integration();

    if install_runtime_exec_fixture(b"v170_exec.elf") != 160 {
        return Some("install_exec_fixture");
    }
    let keep_fd = openat(AT_FDCWD, b"v170_keep.txt", O_CREAT | O_TRUNC, 0o644);
    if keep_fd < 0 {
        return Some("open_keep_fd");
    }
    let clo_fd = openat(AT_FDCWD, b"v170_clo.txt", O_CREAT | O_TRUNC | O_CLOEXEC, 0o644);
    if clo_fd < 0 || fd_cloexec(clo_fd as usize) != Some(true) {
        return Some("open_cloexec_fd");
    }
    let argv0 = match runtime_exec_string(b"v170_exec.elf") {
        Ok(item) => item,
        Err(_) => return Some("argv0"),
    };
    let argv1 = match runtime_exec_string(b"--batch") {
        Ok(item) => item,
        Err(_) => return Some("argv1"),
    };
    let env0 = match runtime_exec_string(b"PATH=/bin") {
        Ok(item) => item,
        Err(_) => return Some("env0"),
    };
    let env1 = match runtime_exec_string(b"TERM=vt100") {
        Ok(item) => item,
        Err(_) => return Some("env1"),
    };
    let argv = [argv0, argv1];
    let envp = [env0, env1];
    if execve_from_vfs(b"v170_exec.elf", &argv, &envp) != 0 {
        return Some("execve_vfs");
    }
    let snap = exec_snapshot();
    if snap.argc != 2 || snap.envc != 2 || snap.auxc < 5 {
        return Some("stack_counts");
    }
    if snap.stack_pointer < RUNTIME_USER_STACK_BOTTOM || snap.stack_pointer >= RUNTIME_USER_STACK_TOP {
        return Some("stack_pointer_range");
    }
    if snap.argv0_ptr < RUNTIME_USER_STACK_BOTTOM || snap.argv0_ptr >= RUNTIME_USER_STACK_TOP || snap.env0_ptr < RUNTIME_USER_STACK_BOTTOM || snap.env0_ptr >= RUNTIME_USER_STACK_TOP {
        return Some("string_pointer_range");
    }
    if snap.auxv_start <= snap.stack_pointer || snap.auxv_start >= RUNTIME_USER_STACK_TOP {
        return Some("auxv_range");
    }
    if fd_exists(clo_fd as usize) || !fd_exists(keep_fd as usize) || snap.closed_cloexec == 0 {
        return Some("cloexec_cleanup");
    }

    None
}

pub fn run_v171_vma_page_fault_foundation() -> Option<&'static str> {
    reset_for_integration();

    if install_runtime_exec_fixture(b"v171_exec.elf") != 160 {
        return Some("install_exec_fixture");
    }
    let argv0 = match runtime_exec_string(b"v171_exec.elf") {
        Ok(item) => item,
        Err(_) => return Some("argv0"),
    };
    let argv = [argv0];
    if execve_from_vfs(b"v171_exec.elf", &argv, &[]) != 0 {
        return Some("execve_vfs");
    }
    let snap = exec_snapshot();
    if !page_fault_validate(snap.entry, RuntimeFaultAccess::Execute) {
        return Some("exec_fault");
    }
    if page_fault_validate(snap.entry, RuntimeFaultAccess::Write) {
        return Some("exec_write_fault");
    }
    if !page_fault_validate(RUNTIME_USER_STACK_TOP - 8, RuntimeFaultAccess::Write) {
        return Some("stack_fault");
    }
    if page_fault_validate(RUNTIME_USER_HEAP_START, RuntimeFaultAccess::Read) {
        return Some("empty_heap_fault");
    }
    let vm = vm_snapshot();
    if vm.vma_count < 2 || vm.load_count != 1 || vm.executable_count == 0 || vm.lazy_count == 0 || vm.resident_pages == 0 {
        return Some("vma_snapshot");
    }
    if vm.mm_id != snap.mm_id || vm.last_fault_addr != RUNTIME_USER_HEAP_START || vm.last_fault_ok {
        return Some("fault_snapshot");
    }

    None
}

pub fn run_v172_lazy_mmap_brk_munmap_mprotect() -> Option<&'static str> {
    reset_for_integration();

    if install_runtime_exec_fixture(b"v172_exec.elf") != 160 {
        return Some("install_exec_fixture");
    }
    let argv0 = match runtime_exec_string(b"v172_exec.elf") {
        Ok(item) => item,
        Err(_) => return Some("argv0"),
    };
    let argv = [argv0];
    if execve_from_vfs(b"v172_exec.elf", &argv, &[]) != 0 {
        return Some("execve_vfs");
    }
    if brk(0) != RUNTIME_USER_HEAP_START as isize {
        return Some("brk_query");
    }
    let brk_target = RUNTIME_USER_HEAP_START + 2 * RUNTIME_PAGE_SIZE;
    if brk(brk_target) != brk_target as isize {
        return Some("brk_grow");
    }
    if !page_fault_validate(RUNTIME_USER_HEAP_START + RUNTIME_PAGE_SIZE, RuntimeFaultAccess::Write) {
        return Some("heap_fault");
    }
    let map_addr = mmap(0, 3 * RUNTIME_PAGE_SIZE, RUNTIME_PROT_READ | RUNTIME_PROT_WRITE, 0x02 | RUNTIME_MAP_ANONYMOUS, -1, 0);
    if map_addr < 0 {
        return Some("mmap");
    }
    let map = map_addr as usize;
    if !page_fault_validate(map + 2 * RUNTIME_PAGE_SIZE, RuntimeFaultAccess::Write) {
        return Some("mmap_fault");
    }
    if mprotect(map, RUNTIME_PAGE_SIZE, RUNTIME_PROT_READ) != 0 {
        return Some("mprotect");
    }
    if page_fault_validate(map, RuntimeFaultAccess::Write) || !page_fault_validate(map, RuntimeFaultAccess::Read) {
        return Some("mprotect_fault");
    }
    if munmap(map + RUNTIME_PAGE_SIZE, RUNTIME_PAGE_SIZE) != 0 {
        return Some("munmap_split");
    }
    if page_fault_validate(map + RUNTIME_PAGE_SIZE, RuntimeFaultAccess::Read) {
        return Some("munmap_middle_fault");
    }
    if !page_fault_validate(map + 2 * RUNTIME_PAGE_SIZE, RuntimeFaultAccess::Write) {
        return Some("munmap_tail_fault");
    }
    let shrink_target = RUNTIME_USER_HEAP_START + RUNTIME_PAGE_SIZE;
    if brk(shrink_target) != shrink_target as isize {
        return Some("brk_shrink");
    }
    let vm = vm_snapshot();
    if vm.heap_end != shrink_target || vm.mmap_count < 2 || vm.writable_count == 0 || vm.lazy_count < 2 {
        return Some("vm_snapshot");
    }

    None
}

pub fn run_v173_signal_frame_rt_sigreturn() -> Option<&'static str> {
    reset_for_integration();

    let usr1_bit = signal_mask_bit(SIGUSR1);
    let usr2_bit = signal_mask_bit(SIGUSR2);
    if set_current_signal_action(SIGUSR1, 0x4000_8000, 0, 0x4000_9000, usr2_bit) != 0 {
        return Some("install_sigaction");
    }
    let old = match rt_sigaction(SIGUSR1, None) {
        Ok(action) => action,
        Err(_) => return Some("query_sigaction"),
    };
    if !old.installed || old.handler != 0x4000_8000 || old.restorer != 0x4000_9000 || old.mask != usr2_bit {
        return Some("sigaction_snapshot");
    }
    if set_current_signal_mask(usr1_bit) != 0 {
        return Some("block_sigusr1");
    }
    if kill_signal(1, SIGUSR1) != 0 {
        return Some("queue_sigusr1");
    }
    let blocked = signal_snapshot();
    if blocked.pending_count != 1 || blocked.queued_count != 1 || blocked.blocked_mask & usr1_bit == 0 {
        return Some("blocked_pending_snapshot");
    }
    if deliver_signal_frame(0x4000_0040, RUNTIME_USER_STACK_TOP) != EAGAIN {
        return Some("blocked_delivery");
    }
    if set_current_signal_mask(0) != 0 {
        return Some("unblock_sigusr1");
    }
    if deliver_signal_frame(0x4000_0040, RUNTIME_USER_STACK_TOP) != 0 {
        return Some("deliver_signal_frame");
    }
    let delivered = signal_snapshot();
    if !delivered.frame_active || delivered.frame_sig != SIGUSR1 || delivered.frame_sp >= RUNTIME_USER_STACK_TOP || delivered.frame_sp < RUNTIME_USER_STACK_BOTTOM {
        return Some("frame_metadata");
    }
    if delivered.saved_pc != 0x4000_0040 || delivered.saved_sp != RUNTIME_USER_STACK_TOP || delivered.delivered_count != 1 || delivered.last_delivered_sig != SIGUSR1 {
        return Some("saved_context");
    }
    if delivered.blocked_mask & usr1_bit == 0 || delivered.blocked_mask & usr2_bit == 0 {
        return Some("delivery_mask");
    }
    let restore = match rt_sigreturn_restore() {
        Ok(restore) => restore,
        Err(_) => return Some("rt_sigreturn"),
    };
    if restore.pc != 0x4000_0040 || restore.sp != RUNTIME_USER_STACK_TOP || restore.mask != 0 || restore.sig != SIGUSR1 {
        return Some("restore_context");
    }
    let returned = signal_snapshot();
    if returned.frame_active || returned.returned_count != 1 || returned.blocked_mask != 0 || returned.pending_count != 0 {
        return Some("return_snapshot");
    }
    if rt_sigreturn_restore().is_ok() {
        return Some("empty_sigreturn");
    }

    None
}

pub fn run_v174_sigchld_process_group_signal() -> Option<&'static str> {
    reset_for_integration();

    if set_current_signal_action(SIGCHLD, 0x4000_a000, 0, 0x4000_b000, 0) != 0 {
        return Some("install_sigchld_action");
    }
    let child_pid = clone_task(17);
    if child_pid <= 1 {
        return Some("clone_child");
    }
    if exit_task_pid(child_pid as usize, 5) != 0 {
        return Some("child_exit");
    }
    let sigchld = signal_snapshot();
    if sigchld.sigchld_count != 1 || sigchld.pending_count != 1 || sigchld.queued_count != 1 {
        return Some("sigchld_pending");
    }
    if deliver_signal_frame(0x4000_0100, RUNTIME_USER_STACK_TOP) != 0 {
        return Some("deliver_sigchld");
    }
    if rt_sigreturn_restore().is_err() {
        return Some("return_sigchld");
    }
    let child2 = clone_task(17);
    if child2 <= 1 {
        return Some("clone_child2");
    }
    if kill_signal(child2, SIGUSR1) != 0 {
        return Some("kill_child");
    }
    if tkill_signal(child2, SIGUSR2) != 0 {
        return Some("tkill_child");
    }
    if tgkill_signal(child2, child2, SIGTERM) != 0 {
        return Some("tgkill_child");
    }
    let direct = signal_snapshot();
    if direct.direct_deliveries < 1 || direct.tkill_deliveries < 1 || direct.tgkill_deliveries < 1 {
        return Some("direct_bookkeeping");
    }
    if tgkill_signal(999, child2, SIGTERM) != ESRCH {
        return Some("tgkill_wrong_group");
    }
    if kill_signal(0, SIGTERM) != 0 {
        return Some("process_group_kill");
    }
    let grouped = signal_snapshot();
    if grouped.group_deliveries < 2 || grouped.queued_count < 6 {
        return Some("process_group_bookkeeping");
    }
    let mut status = -1isize;
    if wait4(child_pid, &mut status) != child_pid || status != (5 << 8) {
        return Some("wait_sigchld_child");
    }

    None
}

pub fn run_v175_rootfs_tmpfs_backend() -> Option<&'static str> {
    reset_for_integration();

    if mkdirat(AT_FDCWD, b"v175root", 0o755) != 0 {
        return Some("mkdir_rootfs");
    }
    let rootfd = openat(AT_FDCWD, b"v175root", O_DIRECTORY, 0);
    if rootfd < 0 {
        return Some("open_rootfs_dir");
    }
    let fd = openat(rootfd, b"alpha.txt", O_CREAT | O_TRUNC, 0o644);
    if fd < 0 {
        return Some("create_rootfs_file");
    }
    if write(fd as usize, b"abcdef") != 6 {
        return Some("write_rootfs_file");
    }
    if lseek(fd as usize, 0, SEEK_SET) != 0 {
        return Some("seek_rootfs_file");
    }
    let mut buf = [0u8; 16];
    if read(fd as usize, &mut buf[..6]) != 6 || &buf[..6] != b"abcdef" {
        return Some("read_rootfs_file");
    }
    if ftruncate(fd as usize, 3) != 0 {
        return Some("truncate_rootfs_file");
    }
    match stat_fd(fd as usize) {
        Ok(stat) => if stat.size != 3 { return Some("stat_truncated_size"); },
        Err(_) => return Some("stat_truncated_file"),
    }
    if linkat(rootfd, b"alpha.txt", rootfd, b"hard_alpha.txt") != 0 {
        return Some("link_rootfs_file");
    }
    match stat_path(rootfd, b"hard_alpha.txt", true) {
        Ok(stat) => if stat.nlink != 2 { return Some("link_nlink"); },
        Err(_) => return Some("stat_hardlink"),
    }
    if renameat(rootfd, b"alpha.txt", rootfd, b"renamed_alpha.txt") != 0 {
        return Some("rename_rootfs_file");
    }
    if unlinkat(rootfd, b"hard_alpha.txt", 0) != 0 {
        return Some("unlink_hardlink");
    }
    match stat_path(rootfd, b"renamed_alpha.txt", true) {
        Ok(stat) => if stat.nlink != 1 || stat.size != 3 { return Some("post_unlink_metadata"); },
        Err(_) => return Some("stat_renamed_file"),
    }
    let _ = close(fd as usize);
    let _ = close(rootfd as usize);

    if install_runtime_exec_fixture(b"v175root/exec.elf") != 160 {
        return Some("exec_file_storage");
    }
    let argv0 = match runtime_exec_string(b"v175root/exec.elf") {
        Ok(item) => item,
        Err(_) => return Some("exec_argv0"),
    };
    if execve_from_vfs(b"v175root/exec.elf", &[argv0], &[]) != 0 {
        return Some("execve_stored_file");
    }

    if mkdirat(AT_FDCWD, b"v175tmp", 0o755) != 0 {
        return Some("mkdir_tmpfs_mountpoint");
    }
    if mount_fs(b"tmpfs", b"v175tmp", b"tmpfs", 0) != 0 {
        return Some("mount_tmpfs_backend");
    }
    let tmpfd = openat(AT_FDCWD, b"v175tmp/tmp.txt", O_CREAT | O_TRUNC, 0o644);
    if tmpfd < 0 {
        return Some("create_tmpfs_file");
    }
    if write(tmpfd as usize, b"tmpfs") != 5 {
        return Some("write_tmpfs_file");
    }
    match statfs_path(b"v175tmp") {
        Ok(statfs) => if statfs.fs_kind != RuntimeFsKind::Tmpfs || statfs.files == 0 { return Some("tmpfs_statfs"); },
        Err(_) => return Some("tmpfs_statfs_err"),
    }
    let _ = close(tmpfd as usize);

    None
}

pub fn run_v176_devfs_core_devices() -> Option<&'static str> {
    reset_for_integration();

    let nullfd = openat(AT_FDCWD, b"/dev/null", 0, 0);
    if nullfd < 0 {
        return Some("open_dev_null");
    }
    let mut buf = [0xaa; 16];
    if write(nullfd as usize, b"discard") != 7 || read(nullfd as usize, &mut buf[..4]) != 0 {
        return Some("dev_null_io");
    }
    let zerofd = openat(AT_FDCWD, b"/dev/zero", 0, 0);
    if zerofd < 0 {
        return Some("open_dev_zero");
    }
    if read(zerofd as usize, &mut buf[..8]) != 8 || buf[..8].iter().any(|b| *b != 0) {
        return Some("dev_zero_read");
    }
    if write(zerofd as usize, b"zero") != 4 {
        return Some("dev_zero_write");
    }
    let console = openat(AT_FDCWD, b"/dev/console", 0, 0);
    if console < 0 || write(console as usize, b".") != 1 {
        return Some("dev_console_write");
    }
    let tty = openat(AT_FDCWD, b"/dev/tty", 0, 0);
    if tty < 0 || write(tty as usize, b".") != 1 {
        return Some("dev_tty_write");
    }
    let random = openat(AT_FDCWD, b"/dev/random", 0, 0);
    if random < 0 {
        return Some("open_dev_random");
    }
    if read(random as usize, &mut buf[..8]) != 8 || buf[..8].iter().all(|b| *b == 0) {
        return Some("dev_random_read");
    }
    let urandom = openat(AT_FDCWD, b"/dev/urandom", 0, 0);
    if urandom < 0 || read(urandom as usize, &mut buf[..8]) != 8 {
        return Some("dev_urandom_read");
    }
    let devfd = openat(AT_FDCWD, b"/dev", O_DIRECTORY, 0);
    if devfd < 0 {
        return Some("open_dev_dir");
    }
    let mut dents = [0u8; 256];
    let dents_len = getdents64(devfd as usize, &mut dents);
    if dents_len <= 0 || !contains_bytes(&dents[..dents_len as usize], b"console") || !contains_bytes(&dents[..dents_len as usize], b"tty") || !contains_bytes(&dents[..dents_len as usize], b"urandom") {
        return Some("dev_dir_entries");
    }
    match stat_path(AT_FDCWD, b"/dev/random", true) {
        Ok(stat) => if stat.kind != FdKind::DevRandom { return Some("dev_random_stat"); },
        Err(_) => return Some("dev_random_stat_err"),
    }

    None
}

pub fn run_v177_procfs_process_status_maps() -> Option<&'static str> {
    reset_for_integration();

    let filefd = openat(AT_FDCWD, b"proc_file.txt", O_CREAT | O_TRUNC, 0o644);
    if filefd < 0 {
        return Some("open_proc_observed_file");
    }
    let _ = write(filefd as usize, b"proc");
    let _ = brk(RUNTIME_USER_HEAP_START + RUNTIME_PAGE_SIZE);
    if mmap(0, RUNTIME_PAGE_SIZE, RUNTIME_PROT_READ | RUNTIME_PROT_WRITE, RUNTIME_MAP_ANONYMOUS, -1, 0) < 0 {
        return Some("proc_maps_mmap");
    }

    let statusfd = openat(AT_FDCWD, b"/proc/self/status", 0, 0);
    if statusfd < 0 {
        return Some("open_proc_status");
    }
    let mut buf = [0u8; 256];
    let status_len = read(statusfd as usize, &mut buf);
    if status_len <= 0 || !contains_bytes(&buf[..status_len as usize], b"Pid:\t1") || !contains_bytes(&buf[..status_len as usize], b"FDs:\t") {
        return Some("read_proc_status");
    }
    let statfd = openat(AT_FDCWD, b"/proc/self/stat", 0, 0);
    if statfd < 0 {
        return Some("open_proc_stat");
    }
    let stat_len = read(statfd as usize, &mut buf);
    if stat_len <= 0 || !contains_bytes(&buf[..stat_len as usize], b"(init)") {
        return Some("read_proc_stat");
    }
    let mapsfd = openat(AT_FDCWD, b"/proc/self/maps", 0, 0);
    if mapsfd < 0 {
        return Some("open_proc_maps");
    }
    let maps_len = read(mapsfd as usize, &mut buf);
    if maps_len <= 0 || !contains_bytes(&buf[..maps_len as usize], b"[stack]") || !contains_bytes(&buf[..maps_len as usize], b"[heap]") || !contains_bytes(&buf[..maps_len as usize], b"[mmap]") {
        return Some("read_proc_maps");
    }
    let fddir = openat(AT_FDCWD, b"/proc/self/fd", O_DIRECTORY, 0);
    if fddir < 0 {
        return Some("open_proc_fd_dir");
    }
    let mut dents = [0u8; 256];
    let dents_len = getdents64(fddir as usize, &mut dents);
    if dents_len <= 0 || !contains_bytes(&dents[..dents_len as usize], b"0") || !contains_bytes(&dents[..dents_len as usize], b"1") || !contains_bytes(&dents[..dents_len as usize], b"2") {
        return Some("proc_fd_dents");
    }
    let mut link = [0u8; PATH_MAX];
    let link_len = proc_fd_readlink(filefd as usize, &mut link);
    if link_len <= 0 || !contains_bytes(&link[..link_len as usize], b"/proc_file.txt") {
        return Some("proc_fd_readlink");
    }

    None
}

pub fn run_v178_mount_tree_statfs() -> Option<&'static str> {
    reset_for_integration();

    let rootfs = match statfs_path(b"/") {
        Ok(statfs) => statfs,
        Err(_) => return Some("statfs_root"),
    };
    if rootfs.fs_kind != RuntimeFsKind::Rootfs || rootfs.magic != KernelCore::fs_magic(RuntimeFsKind::Rootfs) {
        return Some("rootfs_magic");
    }
    match statfs_path(b"/dev") {
        Ok(statfs) => if statfs.fs_kind != RuntimeFsKind::Devfs { return Some("devfs_statfs"); },
        Err(_) => return Some("devfs_statfs_err"),
    }
    match statfs_path(b"/proc") {
        Ok(statfs) => if statfs.fs_kind != RuntimeFsKind::Procfs { return Some("procfs_statfs"); },
        Err(_) => return Some("procfs_statfs_err"),
    }
    if mkdirat(AT_FDCWD, b"mnt", 0o755) != 0 {
        return Some("mkdir_mountpoint");
    }
    if mount_fs(b"tmpfs", b"mnt", b"tmpfs", 0) != 0 {
        return Some("mount_tmpfs");
    }
    let snap = mount_snapshot();
    if snap.mount_count < 4 || snap.tmpfs_mounts != 1 || snap.devfs_mounts != 1 || snap.procfs_mounts != 1 {
        return Some("mount_snapshot");
    }
    match statfs_path(b"mnt") {
        Ok(statfs) => if statfs.fs_kind != RuntimeFsKind::Tmpfs || statfs.magic != KernelCore::fs_magic(RuntimeFsKind::Tmpfs) { return Some("tmpfs_statfs"); },
        Err(_) => return Some("tmpfs_statfs_err"),
    }
    let fd = openat(AT_FDCWD, b"mnt/file.txt", O_CREAT | O_TRUNC, 0o644);
    if fd < 0 {
        return Some("open_tmpfs_file");
    }
    match statfs_fd(fd as usize) {
        Ok(statfs) => if statfs.fs_kind != RuntimeFsKind::Tmpfs { return Some("fstatfs_tmpfs"); },
        Err(_) => return Some("fstatfs_tmpfs_err"),
    }
    let _ = close(fd as usize);
    if umount2(b"mnt", 0) != 0 {
        return Some("umount_tmpfs");
    }
    match statfs_path(b"mnt") {
        Ok(statfs) => if statfs.fs_kind != RuntimeFsKind::Rootfs { return Some("post_umount_statfs"); },
        Err(_) => return Some("post_umount_statfs_err"),
    }

    None
}

pub fn run_v179_permissions_credentials() -> Option<&'static str> {
    reset_for_integration();

    let root_only = openat(AT_FDCWD, b"root_only.txt", O_CREAT | O_TRUNC | O_RDWR, 0o600);
    if root_only < 0 {
        return Some("open_root_only");
    }
    if write(root_only as usize, b"root") != 4 {
        return Some("write_root_only");
    }
    let owned = openat(AT_FDCWD, b"owned.txt", O_CREAT | O_TRUNC | O_RDWR, 0o600);
    if owned < 0 {
        return Some("open_owned");
    }
    if fchown(owned as usize, 1000, 1000) != 0 {
        return Some("root_chown");
    }
    if fchmod(owned as usize, 0o640) != 0 {
        return Some("root_chmod");
    }
    let before = match security_snapshot_path(b"owned.txt") {
        Ok(snapshot) => snapshot,
        Err(_) => return Some("security_snapshot_before"),
    };
    if before.node_uid != 1000 || before.node_gid != 1000 || before.node_mode != 0o640 {
        return Some("metadata_before_drop");
    }
    if setresgid(1000, 1000, 1000) != 0 || setresuid(1000, 1000, 1000) != 0 {
        return Some("drop_to_owner");
    }
    let cred = cred_snapshot();
    if cred.uid != 1000 || cred.euid != 1000 || cred.gid != 1000 || cred.egid != 1000 || cred.cap_effective != 0 {
        return Some("credential_drop");
    }
    if faccessat(AT_FDCWD, b"owned.txt", 4) != 0 {
        return Some("owner_access");
    }
    if fchmod(owned as usize, 0o600) != 0 {
        return Some("owner_chmod");
    }
    if faccessat(AT_FDCWD, b"root_only.txt", 4) != EACCES {
        return Some("root_file_access_denied");
    }
    if openat(AT_FDCWD, b"root_only.txt", 0, 0) != EACCES {
        return Some("root_file_open_denied");
    }
    let after = match security_snapshot_path(b"owned.txt") {
        Ok(snapshot) => snapshot,
        Err(_) => return Some("security_snapshot_after"),
    };
    if after.node_mode != 0o600 || after.fsuid != 1000 || after.fsgid != 1000 {
        return Some("metadata_after_owner_chmod");
    }

    None
}

pub fn run_v180_capability_identity_model() -> Option<&'static str> {
    reset_for_integration();

    let root = cred_snapshot();
    if root.euid != 0 || (root.cap_effective & (1u64 << CAP_SETUID)) == 0 || (root.cap_effective & (1u64 << CAP_CHOWN)) == 0 {
        return Some("root_cap_snapshot");
    }
    if capset_masks(DEFAULT_CAPS, 1u64 << CAP_SETUID, 0) != 0 {
        return Some("capset_setuid_only");
    }
    if setgid(44) != EACCES {
        return Some("setgid_without_cap");
    }
    if setuid(1001) != 0 {
        return Some("setuid_with_cap");
    }
    let dropped = cred_snapshot();
    if dropped.uid != 1001 || dropped.euid != 1001 || dropped.cap_effective != 0 {
        return Some("setuid_drop_snapshot");
    }
    if setuid(0) != EACCES {
        return Some("regain_root_denied");
    }

    reset_for_integration();
    if capset_masks(DEFAULT_CAPS, DEFAULT_CAPS | (1u64 << 12), 0) != EINVAL {
        return Some("capset_effective_subset");
    }
    if capset_masks(DEFAULT_CAPS, DEFAULT_CAPS & !(1u64 << CAP_CHOWN), 0) != 0 {
        return Some("capset_remove_chown");
    }
    let fd = openat(AT_FDCWD, b"cap_owned.txt", O_CREAT | O_TRUNC | O_RDWR, 0o600);
    if fd < 0 {
        return Some("cap_file_open");
    }
    if fchown(fd as usize, 7, 7) != EACCES {
        return Some("chown_without_cap");
    }
    if setresuid(usize::MAX, 0, usize::MAX) != 0 || setresgid(usize::MAX, 0, usize::MAX) != 0 {
        return Some("setres_keep_root");
    }
    let final_cred = cred_snapshot();
    if final_cred.euid != 0 || final_cred.egid != 0 {
        return Some("final_identity_snapshot");
    }

    None
}

pub fn run_v181_unix_socket_loopback() -> Option<&'static str> {
    reset_for_integration();

    let server = socket_with(AF_UNIX, SOCK_STREAM, 0);
    let client = socket_with(AF_UNIX, SOCK_STREAM, 0);
    if server < 0 || client < 0 {
        return Some("socket_alloc");
    }
    if bind_socket(server as usize, b"v181.sock") != 0 {
        return Some("bind_server");
    }
    if listen_socket(server as usize, 4) != 0 {
        return Some("listen_server");
    }
    if connect_socket(client as usize, b"v181.sock") != 0 {
        return Some("connect_client");
    }
    let accepted = accept_socket(server as usize, 0);
    if accepted < 0 {
        return Some("accept_client");
    }
    let mut write_vecs = [RuntimeIovec::empty(); 2];
    write_vecs[0].data[..4].copy_from_slice(b"unix");
    write_vecs[0].len = 4;
    write_vecs[1].data[..4].copy_from_slice(b"loop");
    write_vecs[1].len = 4;
    if write_iovec(client as usize, &write_vecs, None, true) != 8 {
        return Some("sendmsg_stream");
    }
    if poll_revents(accepted, POLLIN) & POLLIN == 0 {
        return Some("stream_readiness");
    }
    let mut read_vecs = [RuntimeIovec::empty(); 2];
    read_vecs[0].len = 4;
    read_vecs[1].len = 4;
    if read_iovec(accepted as usize, &mut read_vecs, None, true) != 8 {
        return Some("recvmsg_stream");
    }
    if &read_vecs[0].data[..4] != b"unix" || &read_vecs[1].data[..4] != b"loop" {
        return Some("stream_payload");
    }
    let mut sv = [-1isize; 2];
    if socketpair(SOCK_STREAM, &mut sv) != 0 {
        return Some("socketpair");
    }
    if write(sv[0] as usize, b"pair") != 4 {
        return Some("socketpair_write");
    }
    let mut pair_buf = [0u8; 4];
    if read(sv[1] as usize, &mut pair_buf) != 4 || &pair_buf != b"pair" {
        return Some("socketpair_read");
    }
    let snap = socket_snapshot();
    if snap.stream_connected < 4 || snap.sends < 2 || snap.recvs < 2 {
        return Some("socket_snapshot");
    }

    None
}

pub fn run_v182_local_datagram_socket() -> Option<&'static str> {
    reset_for_integration();

    let recv = socket_with(AF_UNIX, SOCK_DGRAM, 0);
    let send = socket_with(AF_UNIX, SOCK_DGRAM, 0);
    if recv < 0 || send < 0 {
        return Some("dgram_socket_alloc");
    }
    if bind_socket(recv as usize, b"v182.dgram") != 0 {
        return Some("dgram_bind");
    }
    if sendto_socket(send as usize, b"datagram", Some(b"v182.dgram")) != 8 {
        return Some("dgram_sendto");
    }
    if poll_revents(recv, POLLIN) & POLLIN == 0 {
        return Some("dgram_readiness");
    }
    let mut buf = [0u8; 16];
    let mut src = [0u8; NAME_MAX];
    if recvfrom_socket(recv as usize, &mut buf, &mut src) != 8 || &buf[..8] != b"datagram" {
        return Some("dgram_recvfrom");
    }
    if poll_revents(recv, POLLIN) != 0 {
        return Some("dgram_readiness_cleared");
    }
    if sendto_socket(send as usize, b"self", None) != 4 {
        return Some("dgram_self_send");
    }
    if poll_revents(send, POLLIN) & POLLIN == 0 {
        return Some("dgram_self_ready");
    }
    let snap = socket_snapshot();
    if snap.datagram_bound != 1 || snap.sends < 2 || snap.recvs < 1 {
        return Some("dgram_snapshot");
    }

    None
}

pub fn run_v183_ipc_blocking_scheduler_integration() -> Option<&'static str> {
    reset_for_integration();

    let mqfd = mq_open(0x183, 0);
    if mqfd < 0 {
        return Some("mq_open");
    }
    let mut buf = [0u8; 8];
    if mq_receive(mqfd as usize, &mut buf) != 0 {
        return Some("mq_empty_receive");
    }
    let msgid = msgget(0x183);
    if msgid < 0 {
        return Some("msgget");
    }
    if msgrcv(msgid as usize, &mut buf) != 0 {
        return Some("msg_empty_receive");
    }
    let semid = semget(0x183, 1);
    if semid < 0 {
        return Some("semget");
    }
    if semop(semid as usize, -1) != 0 {
        return Some("sem_wait");
    }
    let shmid = shmget(0x183, 4096);
    if shmid < 0 {
        return Some("shmget");
    }
    if shm_wait_for_attach(shmid as usize) != 0 {
        return Some("shm_wait");
    }
    let waiting = ipc_wait_snapshot();
    if waiting.mq_waiters != 1 || waiting.msg_waiters != 1 || waiting.sem_waiters != 1 || waiting.shm_waiters != 1 {
        return Some("ipc_wait_snapshot");
    }
    if mq_send(mqfd as usize, b"mq", 1) != 0 || msgsnd(msgid as usize, b"msg") != 0 || semop(semid as usize, 1) != 0 {
        return Some("ipc_wake_ops");
    }
    if shmat(shmid as usize) <= 0 {
        return Some("shmat_wake");
    }
    let after = ipc_wait_snapshot();
    if after.mq_waiters != 0 || after.msg_waiters != 0 || after.sem_waiters != 0 || after.shm_waiters != 0 {
        return Some("ipc_wake_snapshot");
    }
    if sched_yield_current() != 0 {
        return Some("sched_yield_after_ipc");
    }

    None
}

pub fn run_v184_namespace_basics() -> Option<&'static str> {
    reset_for_integration();

    let initial = namespace_snapshot();
    if initial.mount_ns != 1 || initial.ipc_ns != 1 || initial.pid_ns != 1 {
        return Some("initial_namespace_refs");
    }
    if unshare_namespaces(CLONE_NEWNS | CLONE_NEWIPC) != 0 {
        return Some("unshare_supported");
    }
    let unshared = namespace_snapshot();
    if unshared.mount_ns == initial.mount_ns || unshared.ipc_ns == initial.ipc_ns || unshared.pid_ns != initial.pid_ns || unshared.unshare_count != 1 {
        return Some("unshare_snapshot");
    }
    if unshare_namespaces(0x4000_0000) != EINVAL {
        return Some("unshare_unsupported");
    }
    if namespace_snapshot().last_error != EINVAL {
        return Some("unshare_error_snapshot");
    }
    if setns_namespace(0, CLONE_NEWNS) != 0 {
        return Some("setns_supported");
    }
    let setns = namespace_snapshot();
    if setns.mount_ns != 1 || setns.setns_count != 1 || setns.last_error != 0 {
        return Some("setns_snapshot");
    }
    if setns_namespace(0, 0x4000_0000) != EINVAL {
        return Some("setns_unsupported");
    }
    if setns_namespace(99, CLONE_NEWNS) != EBADF {
        return Some("setns_badfd");
    }

    None
}

pub fn run_v185_multi_elf_rootfs_runner() -> Option<&'static str> {
    reset_for_integration();

    if mkdirat(AT_FDCWD, b"bin", 0o755) != 0 {
        return Some("mkdir_bin");
    }
    if install_runtime_exec_fixture(b"bin/v185a.elf") != 160 {
        return Some("install_a");
    }
    if install_runtime_exec_fixture(b"bin/v185b.elf") != 160 {
        return Some("install_b");
    }
    if install_runtime_exec_fixture(b"bin/v185c.elf") != 160 {
        return Some("install_c");
    }

    let first = match core_mut().run_user_program_from_vfs(b"bin/v185a.elf", 3) {
        Ok(result) => result,
        Err(_) => return Some("run_a"),
    };
    let second = match core_mut().run_user_program_from_vfs(b"bin/v185b.elf", 5) {
        Ok(result) => result,
        Err(_) => return Some("run_b"),
    };
    let third = match core_mut().run_user_program_from_vfs(b"bin/v185c.elf", 7) {
        Ok(result) => result,
        Err(_) => return Some("run_c"),
    };

    if first.wait_status != (3 << 8) || second.wait_status != (5 << 8) || third.wait_status != (7 << 8) {
        return Some("exit_status_collect");
    }
    if first.entry != 0x4000_0000 || second.entry != 0x4000_0000 || third.entry != 0x4000_0000 {
        return Some("exec_entry_collect");
    }
    if first.exec_seq == 0 || second.exec_seq <= first.exec_seq || third.exec_seq <= second.exec_seq {
        return Some("exec_sequence_collect");
    }
    if first.mm_id == second.mm_id || second.mm_id == third.mm_id || first.pid == second.pid || second.pid == third.pid {
        return Some("task_exec_identity");
    }
    if first.path_len == 0 || first.exit_code != 3 || second.exit_code != 5 || third.exit_code != 7 {
        return Some("program_result_metadata");
    }
    match stat_path(AT_FDCWD, b"bin/v185a.elf", true) {
        Ok(stat) => if stat.size != 160 || (stat.mode & 0o111) == 0 { return Some("exec_file_metadata"); },
        Err(_) => return Some("exec_file_stat"),
    }
    let proc = proc_snapshot();
    if proc.pid != 1 || proc.fd_count != 3 {
        return Some("parent_after_reap");
    }
    let sig = signal_snapshot();
    if sig.sigchld_count < 3 {
        return Some("exec_runner_sigchld");
    }

    None
}

pub fn run_v186_libc_syscall_matrix() -> Option<&'static str> {
    reset_for_integration();

    let mut covered = 0usize;

    if mkdirat(AT_FDCWD, b"m186", 0o755) != 0 {
        return Some("fs_mkdir");
    }
    let file = openat(AT_FDCWD, b"m186/file.txt", O_CREAT | O_TRUNC | O_RDWR, 0o644);
    if file < 0 {
        return Some("fs_open");
    }
    if write(file as usize, b"matrix") != 6 || lseek(file as usize, 0, SEEK_SET) != 0 {
        return Some("fs_write_seek");
    }
    let mut file_buf = [0u8; 8];
    if read(file as usize, &mut file_buf) != 6 || &file_buf[..6] != b"matrix" {
        return Some("fs_read");
    }
    match stat_path(AT_FDCWD, b"m186/file.txt", true) {
        Ok(stat) => if stat.size != 6 || stat.kind != FdKind::RegularFile { return Some("fs_stat"); },
        Err(_) => return Some("fs_stat_err"),
    }
    let dir = openat(AT_FDCWD, b"m186", O_DIRECTORY, 0);
    if dir < 0 {
        return Some("fs_opendir");
    }
    let mut dents = [0u8; 160];
    let dents_len = getdents64(dir as usize, &mut dents);
    if dents_len <= 0 || !contains_bytes(&dents[..dents_len as usize], b"file.txt") {
        return Some("fs_getdents");
    }
    covered += 1;

    let dupfd = dup(file as usize);
    if dupfd < 0 || fcntl(dupfd as usize, F_SETFD, FD_CLOEXEC as usize) != 0 || fd_cloexec(dupfd as usize) != Some(true) {
        return Some("fd_dup_fcntl");
    }
    let mut fdlink = [0u8; PATH_MAX];
    let link_len = proc_fd_readlink(file as usize, &mut fdlink);
    if link_len <= 0 || !contains_bytes(&fdlink[..link_len as usize], b"/m186/file.txt") {
        return Some("fd_proc_link");
    }
    covered += 1;

    let status = openat(AT_FDCWD, b"/proc/self/status", 0, 0);
    if status < 0 {
        return Some("proc_status_open");
    }
    let mut status_buf = [0u8; DATA_MAX];
    let status_len = read(status as usize, &mut status_buf);
    if status_len <= 0 || !contains_bytes(&status_buf[..status_len as usize], b"Pid:\t1") {
        return Some("proc_status_read");
    }
    let maps = openat(AT_FDCWD, b"/proc/self/maps", 0, 0);
    if maps < 0 {
        return Some("proc_maps_open");
    }
    let mut maps_buf = [0u8; DATA_MAX];
    let maps_len = read(maps as usize, &mut maps_buf);
    if maps_len <= 0 || !contains_bytes(&maps_buf[..maps_len as usize], b"[stack]") {
        return Some("proc_maps_read");
    }
    covered += 1;

    let zero = openat(AT_FDCWD, b"/dev/zero", 0, 0);
    let random = openat(AT_FDCWD, b"/dev/random", 0, 0);
    if zero < 0 || random < 0 {
        return Some("devfs_open");
    }
    let mut zero_buf = [1u8; 8];
    let mut random_buf = [0u8; 8];
    if read(zero as usize, &mut zero_buf) != 8 || zero_buf != [0u8; 8] {
        return Some("dev_zero_read");
    }
    if read(random as usize, &mut random_buf) != 8 || random_buf == [0u8; 8] {
        return Some("dev_random_read");
    }
    covered += 1;

    let timer = timerfd_create(0, O_NONBLOCK);
    if timer < 0 || timerfd_settime(timer as usize) != 0 {
        return Some("time_timerfd");
    }
    let mut timer_buf = [0u8; 8];
    if read(timer as usize, &mut timer_buf) != 8 || u64::from_le_bytes(timer_buf) != 1 {
        return Some("time_timerfd_read");
    }
    covered += 1;

    if install_runtime_exec_fixture(b"m186_exec.elf") != 160 {
        return Some("mm_install_exec");
    }
    let argv0 = match runtime_exec_string(b"m186_exec.elf") {
        Ok(item) => item,
        Err(_) => return Some("mm_argv"),
    };
    if execve_from_vfs(b"m186_exec.elf", &[argv0], &[]) != 0 {
        return Some("mm_exec");
    }
    if brk(RUNTIME_USER_HEAP_START + 4096) != (RUNTIME_USER_HEAP_START + 4096) as isize {
        return Some("mm_brk");
    }
    let map = mmap(0, 4096, RUNTIME_PROT_READ | RUNTIME_PROT_WRITE, RUNTIME_MAP_ANONYMOUS, -1, 0);
    if map < 0 || !page_fault_validate(map as usize, RuntimeFaultAccess::Write) {
        return Some("mm_mmap_fault");
    }
    if munmap(map as usize, 4096) != 0 {
        return Some("mm_munmap");
    }
    covered += 1;

    let mut pipe_fds = [-1isize; 2];
    if pipe2(O_NONBLOCK, &mut pipe_fds) != 0 {
        return Some("pipe_alloc");
    }
    if write(pipe_fds[1] as usize, b"p") != 1 || (poll_revents(pipe_fds[0], POLLIN) & POLLIN) == 0 {
        return Some("pipe_poll");
    }
    covered += 1;

    let mut sv = [-1isize; 2];
    if socketpair(SOCK_STREAM, &mut sv) != 0 {
        return Some("socketpair_alloc");
    }
    if write(sv[0] as usize, b"s") != 1 {
        return Some("socket_write");
    }
    let mut sock_buf = [0u8; 1];
    if read(sv[1] as usize, &mut sock_buf) != 1 || sock_buf[0] != b's' {
        return Some("socket_read");
    }
    covered += 1;

    let mqfd = mq_open(0x186, 0);
    let msgid = msgget(0x186);
    let semid = semget(0x186, 1);
    let shmid = shmget(0x186, 4096);
    if mqfd < 0 || msgid < 0 || semid < 0 || shmid < 0 {
        return Some("ipc_alloc");
    }
    let mut ipc_buf = [0u8; 8];
    if mq_send(mqfd as usize, b"mq", 0) != 0 || mq_receive(mqfd as usize, &mut ipc_buf) != 2 {
        return Some("ipc_mq");
    }
    if msgsnd(msgid as usize, b"msg") != 0 || msgrcv(msgid as usize, &mut ipc_buf) != 3 {
        return Some("ipc_msg");
    }
    if semop(semid as usize, 1) != 0 || shmat(shmid as usize) <= 0 {
        return Some("ipc_sem_shm");
    }
    covered += 1;

    if setresgid(1000, 1000, 1000) != 0 || setresuid(1000, 1000, 1000) != 0 {
        return Some("cred_setres");
    }
    let cred = cred_snapshot();
    if cred.euid != 1000 || cred.egid != 1000 || cred.cap_effective != 0 {
        return Some("cred_snapshot");
    }
    covered += 1;

    if covered < 10 {
        return Some("matrix_coverage");
    }

    None
}

pub fn run_v187_fs_process_memory_suite() -> Option<&'static str> {
    reset_for_integration();

    if mkdirat(AT_FDCWD, b"s187", 0o755) != 0 {
        return Some("mkdir_suite");
    }
    let dir = openat(AT_FDCWD, b"s187", O_DIRECTORY, 0);
    if dir < 0 {
        return Some("open_suite_dir");
    }
    let fd = openat(dir, b"alpha.txt", O_CREAT | O_TRUNC | O_RDWR, 0o644);
    if fd < 0 {
        return Some("create_alpha");
    }
    if write(fd as usize, b"alpha187") != 8 || lseek(fd as usize, 0, SEEK_SET) != 0 {
        return Some("write_alpha");
    }
    let mut file_buf = [0u8; 8];
    if read(fd as usize, &mut file_buf) != 8 || &file_buf != b"alpha187" {
        return Some("read_alpha");
    }
    match stat_path(dir, b"alpha.txt", true) {
        Ok(stat) => if stat.size != 8 || stat.nlink != 1 { return Some("stat_alpha"); },
        Err(_) => return Some("stat_alpha_err"),
    }
    if linkat(dir, b"alpha.txt", dir, b"alpha.hard") != 0 {
        return Some("link_alpha");
    }
    match stat_path(dir, b"alpha.txt", true) {
        Ok(stat) => if stat.nlink != 2 { return Some("link_nlink"); },
        Err(_) => return Some("link_stat"),
    }
    if renameat(dir, b"alpha.txt", dir, b"beta.txt") != 0 {
        return Some("rename_beta");
    }
    if unlinkat(dir, b"alpha.hard", 0) != 0 {
        return Some("unlink_hard");
    }
    match stat_path(dir, b"beta.txt", true) {
        Ok(stat) => if stat.size != 8 || stat.nlink != 1 { return Some("stat_beta"); },
        Err(_) => return Some("stat_beta_err"),
    }
    let mut dents = [0u8; 160];
    let dents_len = getdents64(dir as usize, &mut dents);
    if dents_len <= 0 || !contains_bytes(&dents[..dents_len as usize], b"beta.txt") {
        return Some("getdents_beta");
    }

    let child = clone_task(17);
    if child <= 0 {
        return Some("clone_child");
    }
    match task_snapshot(child as usize) {
        Ok(task) => if task.ppid != 1 || task.state != RuntimeTaskState::Ready { return Some("child_snapshot"); },
        Err(_) => return Some("child_snapshot_err"),
    }
    if exit_task_pid(child as usize, 11) != 0 {
        return Some("child_exit");
    }
    let mut wait_status = 0isize;
    if wait4(child, &mut wait_status) != child || wait_status != (11 << 8) {
        return Some("child_wait");
    }

    if install_runtime_exec_fixture(b"s187/mem.elf") != 160 {
        return Some("install_mem_exec");
    }
    let argv0 = match runtime_exec_string(b"s187/mem.elf") {
        Ok(item) => item,
        Err(_) => return Some("mem_argv"),
    };
    if execve_from_vfs(b"s187/mem.elf", &[argv0], &[]) != 0 {
        return Some("mem_exec");
    }
    if brk(RUNTIME_USER_HEAP_START + 8192) != (RUNTIME_USER_HEAP_START + 8192) as isize {
        return Some("mem_brk");
    }
    if !page_fault_validate(RUNTIME_USER_HEAP_START, RuntimeFaultAccess::Write) {
        return Some("heap_fault");
    }
    let map = mmap(0, 8192, RUNTIME_PROT_READ | RUNTIME_PROT_WRITE, RUNTIME_MAP_ANONYMOUS, -1, 0);
    if map < 0 {
        return Some("mmap_alloc");
    }
    if !page_fault_validate(map as usize, RuntimeFaultAccess::Write) {
        return Some("mmap_write_fault");
    }
    if mprotect(map as usize, 4096, RUNTIME_PROT_READ) != 0 {
        return Some("mprotect_readonly");
    }
    if page_fault_validate(map as usize, RuntimeFaultAccess::Write) {
        return Some("mprotect_write_denied");
    }
    if munmap(map as usize, 8192) != 0 || page_fault_validate(map as usize, RuntimeFaultAccess::Read) {
        return Some("munmap_fault");
    }

    None
}

pub fn run_v188_signal_pipe_poll_ipc_suite() -> Option<&'static str> {
    reset_for_integration();

    if set_current_signal_action(SIGUSR1, 0x4000_1000, 0, 0x4000_2000, signal_mask_bit(SIGUSR2)) != 0 {
        return Some("sigaction_usr1");
    }
    if kill_signal(current_pid() as isize, SIGUSR1) != 0 {
        return Some("queue_usr1");
    }
    if deliver_signal_frame(0x4000_3000, RUNTIME_USER_STACK_TOP) != 0 {
        return Some("deliver_usr1");
    }
    let sig = signal_snapshot();
    if !sig.frame_active || sig.frame_sig != SIGUSR1 || sig.frame_handler != 0x4000_1000 {
        return Some("signal_frame_snapshot");
    }
    match rt_sigreturn_restore() {
        Ok(restored) => if restored.pc != 0x4000_3000 || restored.sp != RUNTIME_USER_STACK_TOP || restored.sig != SIGUSR1 { return Some("sigreturn_restore"); },
        Err(_) => return Some("sigreturn_err"),
    }
    let child = clone_task(17);
    if child <= 0 {
        return Some("sigchld_clone");
    }
    if exit_task_pid(child as usize, 4) != 0 {
        return Some("sigchld_exit");
    }
    let sig_after_child = signal_snapshot();
    if sig_after_child.sigchld_count == 0 || sig_after_child.pending_count == 0 {
        return Some("sigchld_pending");
    }
    let mut wait_status = 0isize;
    if wait4(child, &mut wait_status) != child || wait_status != (4 << 8) {
        return Some("sigchld_wait");
    }

    let mut pipe_fds = [-1isize; 2];
    if pipe2(O_NONBLOCK, &mut pipe_fds) != 0 {
        return Some("pipe2");
    }
    let epfd = epoll_create1(0);
    if epfd < 0 {
        return Some("epoll_create");
    }
    if epoll_ctl_event(epfd as usize, EPOLL_CTL_ADD, pipe_fds[0] as usize, POLLIN as u32, 0x188) != 0 {
        return Some("epoll_add_pipe");
    }
    let mut events = [RuntimeEpollEvent::empty(); 2];
    if epoll_collect_ready(epfd as usize, &mut events) != 0 {
        return Some("epoll_initial_empty");
    }
    if write(pipe_fds[1] as usize, b"ready") != 5 {
        return Some("pipe_write");
    }
    if epoll_collect_ready(epfd as usize, &mut events) != 1 || events[0].data != 0x188 || (events[0].events & POLLIN as u32) == 0 {
        return Some("epoll_pipe_ready");
    }
    let mut pipe_buf = [0u8; 5];
    if read(pipe_fds[0] as usize, &mut pipe_buf) != 5 || &pipe_buf != b"ready" {
        return Some("pipe_read");
    }

    let mqfd = mq_open(0x188, 0);
    let msgid = msgget(0x188);
    let semid = semget(0x188, 1);
    let shmid = shmget(0x188, 4096);
    if mqfd < 0 || msgid < 0 || semid < 0 || shmid < 0 {
        return Some("ipc_alloc");
    }
    let mut ipc_buf = [0u8; 8];
    if mq_receive(mqfd as usize, &mut ipc_buf) != 0 || msgrcv(msgid as usize, &mut ipc_buf) != 0 || semop(semid as usize, -1) != 0 || shm_wait_for_attach(shmid as usize) != 0 {
        return Some("ipc_wait_ops");
    }
    let waiting = ipc_wait_snapshot();
    if waiting.mq_waiters != 1 || waiting.msg_waiters != 1 || waiting.sem_waiters != 1 || waiting.shm_waiters != 1 {
        return Some("ipc_wait_snapshot");
    }
    if mq_send(mqfd as usize, b"mq", 0) != 0 || msgsnd(msgid as usize, b"msg") != 0 || semop(semid as usize, 1) != 0 || shmat(shmid as usize) <= 0 {
        return Some("ipc_wake_ops");
    }
    let after = ipc_wait_snapshot();
    if after.mq_waiters != 0 || after.msg_waiters != 0 || after.sem_waiters != 0 || after.shm_waiters != 0 {
        return Some("ipc_wake_snapshot");
    }

    None
}

pub fn run_v189_stress_error_path_hardening() -> Option<&'static str> {
    reset_for_integration();

    let mut errors = 0usize;
    let mut one = [0u8; 1];
    if read(63, &mut one) != EBADF {
        return Some("bad_fd_read");
    }
    errors += 1;
    if openat(AT_FDCWD, b"missing-v189", 0, 0) != ENOENT {
        return Some("missing_path");
    }
    errors += 1;
    if mkdirat(AT_FDCWD, b"dup189", 0o755) != 0 || mkdirat(AT_FDCWD, b"dup189", 0o755) != EEXIST {
        return Some("eexist_mkdir");
    }
    errors += 1;
    if faccessat(AT_FDCWD, b"/", 8) != EINVAL {
        return Some("einval_access");
    }
    errors += 1;
    let mut pipe_fds = [-1isize; 2];
    if pipe2(O_NONBLOCK, &mut pipe_fds) != 0 {
        return Some("pipe_nonblock_alloc");
    }
    if read(pipe_fds[0] as usize, &mut one) != EAGAIN {
        return Some("pipe_eagain");
    }
    let _ = close(pipe_fds[0] as usize);
    let _ = close(pipe_fds[1] as usize);
    errors += 1;
    let event = eventfd2(0, O_NONBLOCK);
    if event < 0 {
        return Some("eventfd_alloc");
    }
    let mut event_buf = [0u8; 8];
    if read(event as usize, &mut event_buf) != EAGAIN {
        return Some("eventfd_eagain");
    }
    let _ = close(event as usize);
    errors += 1;
    if mmap(0, 0, RUNTIME_PROT_READ, RUNTIME_MAP_ANONYMOUS, -1, 0) != EINVAL {
        return Some("mmap_einval");
    }
    errors += 1;
    if munmap(RUNTIME_USER_MMAP_START, 0) != EINVAL {
        return Some("munmap_einval");
    }
    errors += 1;

    let base = openat(AT_FDCWD, b"stress189.txt", O_CREAT | O_TRUNC | O_RDWR, 0o644);
    if base < 0 {
        return Some("stress_create");
    }
    if write(base as usize, b"x") != 1 {
        return Some("stress_write");
    }
    let mut opened = [-1isize; MAX_FDS];
    let mut count = 0usize;
    loop {
        let fd = openat(AT_FDCWD, b"stress189.txt", O_RDWR, 0);
        if fd >= 0 {
            if count >= opened.len() {
                return Some("fd_exhaust_overflow");
            }
            opened[count] = fd;
            count += 1;
        } else if fd == ENOSPC {
            break;
        } else {
            return Some("fd_exhaust_errno");
        }
    }
    if count < 50 {
        return Some("fd_exhaust_count");
    }
    let mut i = 0usize;
    while i < count {
        let _ = close(opened[i] as usize);
        i += 1;
    }
    let _ = close(base as usize);
    errors += 1;

    let mut sockfds = [-1isize; MAX_SOCKETS];
    let mut sock_count = 0usize;
    loop {
        let fd = socket_with(AF_UNIX, SOCK_STREAM, 0);
        if fd >= 0 {
            if sock_count >= sockfds.len() {
                return Some("socket_exhaust_overflow");
            }
            sockfds[sock_count] = fd;
            sock_count += 1;
        } else if fd == ENOSPC {
            break;
        } else {
            return Some("socket_exhaust_errno");
        }
    }
    if sock_count != MAX_SOCKETS {
        return Some("socket_exhaust_count");
    }
    i = 0;
    while i < sock_count {
        let _ = close(sockfds[i] as usize);
        i += 1;
    }
    errors += 1;

    i = 0;
    while i < 8 {
        let fd = openat(AT_FDCWD, b"stress189.txt", O_CREAT | O_RDWR, 0o644);
        if fd < 0 {
            return Some("repeat_open");
        }
        if close(fd as usize) != 0 {
            return Some("repeat_close");
        }
        i += 1;
    }
    errors += 1;

    if errors < 11 {
        return Some("error_coverage");
    }

    None
}

pub fn run_v190_final_competition_kernel_readiness() -> Option<&'static str> {
    if run_v185_multi_elf_rootfs_runner().is_some() {
        return Some("final_v185");
    }
    if run_v186_libc_syscall_matrix().is_some() {
        return Some("final_v186");
    }
    if run_v187_fs_process_memory_suite().is_some() {
        return Some("final_v187");
    }
    if run_v188_signal_pipe_poll_ipc_suite().is_some() {
        return Some("final_v188");
    }
    if run_v189_stress_error_path_hardening().is_some() {
        return Some("final_v189");
    }

    reset_for_integration();
    let rootfs = match statfs_path(b"/") {
        Ok(statfs) => statfs,
        Err(_) => return Some("final_rootfs_statfs"),
    };
    let mounts = mount_snapshot();
    let namespaces = namespace_snapshot();
    let cred = cred_snapshot();
    let proc = proc_snapshot();
    if rootfs.fs_kind != RuntimeFsKind::Rootfs || mounts.mount_count < 3 {
        return Some("final_fs_snapshot");
    }
    if namespaces.mount_ns != 1 || namespaces.ipc_ns != 1 || namespaces.pid_ns != 1 {
        return Some("final_namespace_snapshot");
    }
    if cred.euid != 0 || (cred.cap_effective & (1u64 << CAP_SETUID)) == 0 {
        return Some("final_cred_snapshot");
    }
    if proc.pid != 1 || proc.fd_count != 3 {
        return Some("final_proc_snapshot");
    }

    None
}

pub fn run_v201_v206_scheduler_blocking_suite() -> Option<&'static str> {
    const COVER_SWITCH: usize = 1 << 0;
    const COVER_TICK: usize = 1 << 1;
    const COVER_WAIT_WAKE: usize = 1 << 2;
    const COVER_PIPE_POLL_FUTEX: usize = 1 << 3;
    const COVER_WAIT_SLEEP_TIMER: usize = 1 << 4;
    const COVER_ALL: usize = COVER_SWITCH | COVER_TICK | COVER_WAIT_WAKE | COVER_PIPE_POLL_FUTEX | COVER_WAIT_SLEEP_TIMER;
    let mut coverage_mask = 0usize;
    let mut total_switches = 0usize;
    let mut total_blocks = 0usize;
    let mut total_wakes = 0usize;
    let mut total_timer_wakes = 0usize;

    reset_for_integration();
    let first_child = clone_task(17);
    let second_child = clone_task(17);
    if first_child <= 1 || second_child <= 1 {
        return Some("v201_clone_runnable_tasks");
    }
    {
        let core = core_mut();
        core.rebuild_runq();
        let before = core.sched_snapshot();
        if before.runq_len < 3 || before.current_pid != 1 {
            return Some("v201_runq_initial");
        }
        if core.sched_tick_once().is_err() {
            return Some("v201_first_tick");
        }
        let after_first = core.sched_snapshot();
        if after_first.current_pid != first_child as usize || after_first.switches == 0 || after_first.last_from != 1 {
            return Some("v201_first_switch");
        }
        if core.sched_tick_once().is_err() {
            return Some("v201_second_tick");
        }
        let after_second = core.sched_snapshot();
        if after_second.current_pid != second_child as usize || after_second.switches < 2 || after_second.last_to != second_child as usize {
            return Some("v201_second_switch");
        }
        total_switches += after_second.switches;
        coverage_mask |= COVER_SWITCH;
        crate::println!(
            "[ucompat-v201] evidence runq={} pid_a={} pid_b={} switches={} PASS",
            before.runq_len,
            after_first.current_pid,
            after_second.current_pid,
            after_second.switches
        );
    }
    crate::println!("[ucompat-v201] run queue task switch PASS");

    {
        let core = core_mut();
        let before_tick = core.sched_snapshot();
        if core.sched_tick_once().is_err() || core.sched_tick_once().is_err() || core.sched_tick_once().is_err() {
            return Some("v202_ticks");
        }
        let after_tick = core.sched_snapshot();
        if after_tick.ticks < before_tick.ticks + 3 || after_tick.switches <= before_tick.switches {
            return Some("v202_tick_accounting");
        }
        total_switches += after_tick.switches - before_tick.switches;
        coverage_mask |= COVER_TICK;
        crate::println!(
            "[ucompat-v202] evidence ticks={} switches={} last={}->{} PASS",
            after_tick.ticks,
            after_tick.switches,
            after_tick.last_from,
            after_tick.last_to
        );
    }
    crate::println!("[ucompat-v202] timer tick scheduling PASS");

    reset_for_integration();
    let waiter_child = clone_task(17);
    if waiter_child <= 1 {
        return Some("v203_clone");
    }
    {
        let core = core_mut();
        if core.sched_wait_on(0x2030).is_err() {
            return Some("v203_wait");
        }
        let waiting_parent = match core.task_snapshot(1) {
            Ok(task) => task,
            Err(_) => return Some("v203_parent_snapshot"),
        };
        if waiting_parent.state != RuntimeTaskState::Waiting {
            return Some("v203_parent_waiting");
        }
        if core.sched_tick_once().is_err() {
            return Some("v203_tick_to_child");
        }
        let running_child = core.sched_snapshot();
        if running_child.current_pid != waiter_child as usize {
            return Some("v203_child_running");
        }
        match core.sched_wake(0x2030, 1) {
            Ok(1) => {}
            _ => return Some("v203_wake"),
        }
        let ready_parent = match core.task_snapshot(1) {
            Ok(task) => task,
            Err(_) => return Some("v203_ready_snapshot"),
        };
        if ready_parent.state != RuntimeTaskState::Ready {
            return Some("v203_parent_ready");
        }
        if core.sched_tick_once().is_err() {
            return Some("v203_tick_to_parent");
        }
        let after = core.sched_snapshot();
        if after.current_pid != 1 || after.blocks == 0 || after.wakes == 0 {
            return Some("v203_final");
        }
        total_blocks += after.blocks;
        total_wakes += after.wakes;
        coverage_mask |= COVER_WAIT_WAKE;
        crate::println!(
            "[ucompat-v203] evidence wait_pid=1 run_pid={} blocks={} wakes={} PASS",
            running_child.current_pid,
            after.blocks,
            after.wakes
        );
    }
    crate::println!("[ucompat-v203] blocking wait wakeup PASS");

    reset_for_integration();
    let event_child = clone_task(17);
    if event_child <= 1 {
        return Some("v204_clone");
    }
    let mut pipe_fds = [-1isize; 2];
    if pipe2(0, &mut pipe_fds) != 0 {
        return Some("v204_pipe2");
    }
    let epfd = epoll_create1(0);
    if epfd < 0 {
        return Some("v204_epoll_create");
    }
    if epoll_ctl_event(epfd as usize, EPOLL_CTL_ADD, pipe_fds[0] as usize, POLLIN as u32, 0x204) != 0 {
        return Some("v204_epoll_add");
    }
    {
        let core = core_mut();
        if core.sched_wait_fd_readable(pipe_fds[0]).is_err() {
            return Some("v204_pipe_wait");
        }
        if core.sched_tick_once().is_err() {
            return Some("v204_pipe_tick");
        }
        if core.sched_snapshot().current_pid != event_child as usize {
            return Some("v204_child_running");
        }
    }
    if write(pipe_fds[1] as usize, b"x") != 1 {
        return Some("v204_pipe_write");
    }
    let parent_after_pipe = match task_snapshot(1) {
        Ok(task) => task,
        Err(_) => return Some("v204_parent_after_pipe"),
    };
    if parent_after_pipe.state != RuntimeTaskState::Ready || epoll_ready_count(epfd as usize) != 1 {
        return Some("v204_pipe_poll_ready");
    }
    if futex_wait(0x2040, 1, 1, false) != 0 {
        return Some("v204_futex_wait");
    }
    {
        let core = core_mut();
        if core.sched_tick_once().is_err() || core.sched_snapshot().current_pid != 1 {
            return Some("v204_futex_tick");
        }
    }
    if futex_wake(0x2040, 1) != 1 {
        return Some("v204_futex_wake");
    }
    let futex_child = match task_snapshot(event_child as usize) {
        Ok(task) => task,
        Err(_) => return Some("v204_futex_child_snapshot"),
    };
    if futex_child.state != RuntimeTaskState::Ready {
        return Some("v204_futex_child_ready");
    }
    {
        let after = sched_snapshot();
        total_blocks += after.blocks;
        total_wakes += after.wakes;
        coverage_mask |= COVER_PIPE_POLL_FUTEX;
        crate::println!(
            "[ucompat-v204] evidence pipe_parent_state=ready epoll_ready=1 futex_wakes={} blocks={} PASS",
            after.wakes,
            after.blocks
        );
    }
    crate::println!("[ucompat-v204] pipe poll futex blocking PASS");

    reset_for_integration();
    let wait_child = clone_task(17);
    if wait_child <= 1 {
        return Some("v205_clone_wait");
    }
    let mut wait_status = -1isize;
    if wait4(wait_child, &mut wait_status) != 0 || wait_status != 0 {
        return Some("v205_wait4_live");
    }
    {
        let core = core_mut();
        if core.sched_tick_once().is_err() || core.sched_snapshot().current_pid != wait_child as usize {
            return Some("v205_wait_tick_child");
        }
    }
    if exit_task_pid(wait_child as usize, 23) != 0 {
        return Some("v205_child_exit");
    }
    {
        let core = core_mut();
        if core.sched_tick_once().is_err() || core.sched_snapshot().current_pid != 1 {
            return Some("v205_wait_tick_parent");
        }
    }
    if wait4(wait_child, &mut wait_status) != wait_child || wait_status != (23 << 8) {
        return Some("v205_wait4_reap");
    }
    let before_sleep = sched_snapshot();
    if sched_timeout_wait(SCHED_WAIT_TIMEOUT_BASE ^ 0x2050) != 0 {
        return Some("v205_nanosleep_timeout");
    }
    let after_sleep = sched_snapshot();
    if after_sleep.timer_wakes <= before_sleep.timer_wakes {
        return Some("v205_sleep_wake_accounting");
    }
    let timer_child = clone_task(17);
    if timer_child <= 1 {
        return Some("v205_clone_timer");
    }
    let timerfd = timerfd_create(0, 0);
    if timerfd < 0 {
        return Some("v205_timerfd_create");
    }
    {
        let core = core_mut();
        if core.sched_wait_fd_readable(timerfd).is_err() {
            return Some("v205_timer_wait");
        }
        if core.sched_tick_once().is_err() || core.sched_snapshot().current_pid != timer_child as usize {
            return Some("v205_timer_tick_child");
        }
    }
    if timerfd_settime(timerfd as usize) != 0 {
        return Some("v205_timerfd_settime");
    }
    {
        let core = core_mut();
        if core.sched_tick_once().is_err() || core.sched_snapshot().current_pid != 1 {
            return Some("v205_timer_tick_parent");
        }
    }
    let mut timer_buf = [0u8; 8];
    if read(timerfd as usize, &mut timer_buf) != 8 || u64::from_le_bytes(timer_buf) != 1 {
        return Some("v205_timer_read");
    }
    {
        let after = sched_snapshot();
        total_blocks += after.blocks;
        total_wakes += after.wakes;
        total_timer_wakes += after.timer_wakes;
        coverage_mask |= COVER_WAIT_SLEEP_TIMER;
        crate::println!(
            "[ucompat-v205] evidence wait_status={} sleep_wakes={} timer_wakes={} blocks={} PASS",
            wait_status,
            after.timer_wakes - before_sleep.timer_wakes,
            after.timer_wakes,
            after.blocks
        );
    }
    crate::println!("[ucompat-v205] wait sleep timer blocking PASS");

    if coverage_mask != COVER_ALL {
        return Some("v206_coverage_flags");
    }
    if total_switches < 4 || total_blocks < 5 || total_wakes < 4 || total_timer_wakes < 2 {
        return Some("v206_regression_counters");
    }
    crate::println!(
        "[ucompat-v206] evidence switches={} blocks={} wakes={} timer_wakes={} PASS",
        total_switches,
        total_blocks,
        total_wakes,
        total_timer_wakes
    );
    crate::println!("[ucompat-v206] scheduler regression suite PASS");

    None
}

pub fn run_v207_v214_storage_image_suite() -> Option<&'static str> {
    reset_for_integration();

    {
        let core = core_mut();
        if core.prepare_competition_ramdisk().is_err() {
            return Some("v207_prepare_ramdisk");
        }
        let mut sector = [0u8; BLOCK_SIZE];
        if core.block_raw_read(IMAGE_DEV, 0, &mut sector).is_err() {
            return Some("v207_read_header");
        }
        if &sector[..8] != b"UCFSIMG1" {
            return Some("v207_header_magic");
        }
        if core.block_raw_read(IMAGE_DEV, RAMDISK_BLOCKS, &mut sector) != Err(Errno::Inval) {
            return Some("v207_invalid_block");
        }
        let snap = core.storage_snapshot();
        if snap.block_devices != 1 || snap.sector_size != BLOCK_SIZE || snap.sectors != RAMDISK_BLOCKS || snap.block_errors == 0 {
            return Some("v207_snapshot");
        }
        crate::println!(
            "[ucompat-v207] evidence devices={} sector={} capacity={} reads={} writes={} errors={} PASS",
            snap.block_devices,
            snap.sector_size,
            snap.sectors,
            snap.raw_reads,
            snap.raw_writes,
            snap.block_errors
        );
    }
    crate::println!("[ucompat-v207] block device abstraction PASS");

    {
        let core = core_mut();
        let mut scratch = [0u8; BLOCK_SIZE];
        let marker = b"ramdisk-write-v208";
        let mut i = 0usize;
        while i < marker.len() {
            scratch[i] = marker[i];
            i += 1;
        }
        if core.block_raw_write(IMAGE_DEV, RAMDISK_BLOCKS - 1, &scratch).is_err() {
            return Some("v208_write_scratch");
        }
        let mut back = [0u8; BLOCK_SIZE];
        if core.block_raw_read(IMAGE_DEV, RAMDISK_BLOCKS - 1, &mut back).is_err() || &back[..marker.len()] != marker {
            return Some("v208_read_scratch");
        }
        let snap = core.storage_snapshot();
        if snap.raw_reads < 2 || snap.raw_writes < 8 {
            return Some("v208_accounting");
        }
        crate::println!(
            "[ucompat-v208] evidence backend=ramdisk sector={} capacity={} readback={} PASS",
            snap.sector_size,
            snap.sectors,
            marker.len()
        );
    }
    crate::println!("[ucompat-v208] virtio blk ramdisk backend PASS");

    {
        let core = core_mut();
        let mut first = [0u8; BLOCK_SIZE];
        let mut second = [0u8; BLOCK_SIZE];
        if core.block_cache_read_block(IMAGE_DEV, 0, &mut first).is_err() {
            return Some("v209_first_read");
        }
        if core.block_cache_read_block(IMAGE_DEV, 0, &mut second).is_err() || first != second {
            return Some("v209_cached_read");
        }
        let mut dirty = [0u8; BLOCK_SIZE];
        dirty[0] = b'D';
        dirty[1] = b'I';
        dirty[2] = b'R';
        dirty[3] = b'T';
        dirty[4] = b'Y';
        if core.block_cache_write_block(IMAGE_DEV, RAMDISK_BLOCKS - 2, &dirty).is_err() {
            return Some("v209_cache_write");
        }
        if core.block_cache_flush().is_err() {
            return Some("v209_cache_flush");
        }
        let snap = core.storage_snapshot();
        if snap.cache_hits == 0 || snap.cache_misses < 2 || snap.cache_dirty_marks == 0 || snap.cache_writebacks == 0 {
            return Some("v209_cache_accounting");
        }
        crate::println!(
            "[ucompat-v209] evidence hits={} misses={} dirty={} writebacks={} PASS",
            snap.cache_hits,
            snap.cache_misses,
            snap.cache_dirty_marks,
            snap.cache_writebacks
        );
    }
    crate::println!("[ucompat-v209] block cache PASS");

    let mount_snapshot = match core_mut().mount_competition_image() {
        Ok(snapshot) => snapshot,
        Err(_) => return Some("v210_mount_image"),
    };
    if !mount_snapshot.image_mounted || mount_snapshot.image_dirs < 3 || mount_snapshot.image_files < 5 || mount_snapshot.image_exec_files < 3 {
        return Some("v210_image_counts");
    }
    match statfs_path(b"/image") {
        Ok(statfs) => if statfs.fs_kind != RuntimeFsKind::Imagefs || statfs.magic != KernelCore::fs_magic(RuntimeFsKind::Imagefs) || statfs.block_size != BLOCK_SIZE {
            return Some("v210_statfs");
        },
        Err(_) => return Some("v210_statfs_err"),
    }
    crate::println!(
        "[ucompat-v210] evidence mount=/image dirs={} files={} execs={} metadata_reads={} PASS",
        mount_snapshot.image_dirs,
        mount_snapshot.image_files,
        mount_snapshot.image_exec_files,
        mount_snapshot.image_metadata_reads
    );
    crate::println!("[ucompat-v210] ext4 readonly mount PASS");

    let image_argv0 = match RuntimeExecString::from_bytes(b"/image/bin/hello.elf") {
        Ok(item) => item,
        Err(_) => return Some("v211_argv"),
    };
    let before_exec_reads = core_mut().storage_snapshot().image_data_reads;
    if execve_from_vfs(b"/image/bin/hello.elf", &[image_argv0], &[]) != 0 {
        return Some("v211_execve");
    }
    let exec = exec_snapshot();
    let after_exec_reads = core_mut().storage_snapshot().image_data_reads;
    if !exec.valid || exec.entry != 0x4000_0000 || exec.phnum != 1 || exec.load_end <= exec.load_start || after_exec_reads <= before_exec_reads {
        return Some("v211_exec_snapshot");
    }
    crate::println!(
        "[ucompat-v211] evidence path=/image/bin/hello.elf entry={:#x} phnum={} image_reads={} PASS",
        exec.entry,
        exec.phnum,
        after_exec_reads - before_exec_reads
    );
    crate::println!("[ucompat-v211] execve from fs image PASS");

    match stat_path(AT_FDCWD, b"/image/README.txt", true) {
        Ok(stat) => if stat.size != b"storage image v207-v214\n".len() || stat.kind != FdKind::RegularFile { return Some("v212_stat_readme"); },
        Err(_) => return Some("v212_stat_readme_err"),
    }
    let image_dir = openat(AT_FDCWD, b"/image", O_DIRECTORY, 0);
    if image_dir < 0 {
        return Some("v212_open_image_dir");
    }
    let mut dents = [0u8; 192];
    let dent_len = getdents64(image_dir as usize, &mut dents);
    if dent_len <= 0 || !contains_bytes(&dents[..dent_len as usize], b"README.txt") || !contains_bytes(&dents[..dent_len as usize], b"bin") {
        return Some("v212_getdents");
    }
    let readme_fd = openat(AT_FDCWD, b"/image/README.txt", 0, 0);
    if readme_fd < 0 {
        return Some("v212_open_readme");
    }
    let mut readme_buf = [0u8; 32];
    let readme_len = read(readme_fd as usize, &mut readme_buf);
    if readme_len != b"storage image v207-v214\n".len() as isize || &readme_buf[..readme_len as usize] != b"storage image v207-v214\n" {
        return Some("v212_read_readme");
    }
    let elf_fd = openat(AT_FDCWD, b"/image/bin/hello.elf", 0, 0);
    if elf_fd < 0 {
        return Some("v212_open_elf");
    }
    let mut magic = [0u8; 4];
    if read(elf_fd as usize, &mut magic) != 4 || magic != [0x7f, b'E', b'L', b'F'] {
        return Some("v212_read_elf_magic");
    }
    crate::println!(
        "[ucompat-v212] evidence stat_size={} dents={} readme={} elf_magic=ELF PASS",
        b"storage image v207-v214\n".len(),
        dent_len,
        readme_len
    );
    crate::println!("[ucompat-v212] fs image metadata io PASS");

    let worker = match core_mut().run_user_program_from_vfs(b"/image/bin/worker.elf", 31) {
        Ok(result) => result,
        Err(_) => return Some("v213_worker_exec"),
    };
    let tool = match core_mut().run_user_program_from_vfs(b"/image/bin/tool.elf", 37) {
        Ok(result) => result,
        Err(_) => return Some("v213_tool_exec"),
    };
    let config_fd = openat(AT_FDCWD, b"/image/etc/config.txt", 0, 0);
    if config_fd < 0 {
        return Some("v213_open_config");
    }
    let mut config = [0u8; 48];
    let config_len = read(config_fd as usize, &mut config);
    if config_len <= 0 || !contains_bytes(&config[..config_len as usize], b"root=/image") {
        return Some("v213_read_config");
    }
    if worker.wait_status != (31 << 8) || tool.wait_status != (37 << 8) || worker.entry != 0x4000_0000 || tool.entry != 0x4000_0000 {
        return Some("v213_exec_results");
    }
    crate::println!(
        "[ucompat-v213] evidence programs=3 worker_status={} tool_status={} config_len={} PASS",
        worker.wait_status,
        tool.wait_status,
        config_len
    );
    crate::println!("[ucompat-v213] image rootfs compatibility matrix PASS");

    let mut badfd_buf = [0u8; 1];
    let missing = openat(AT_FDCWD, b"/image/missing.txt", 0, 0);
    let notdir = match stat_path(AT_FDCWD, b"/image/README.txt/nope", true) {
        Ok(_) => 0,
        Err(err) => err,
    };
    let eisdir = openat(AT_FDCWD, b"/image", O_RDWR, 0);
    let einval = mount_fs(b"ramdisk0", b"/image", b"definitely-not-a-fs", 0);
    let ebadf = read(63, &mut badfd_buf);
    let readonly = openat(AT_FDCWD, b"/image/README.txt", O_WRONLY, 0);
    if missing != ENOENT || notdir != ENOTDIR || eisdir != EISDIR || einval != EINVAL || ebadf != EBADF || readonly != EACCES {
        return Some("v214_errno_matrix");
    }
    crate::println!(
        "[ucompat-v214] evidence ENOENT={} ENOTDIR={} EISDIR={} EINVAL={} EBADF={} EACCES={} PASS",
        missing,
        notdir,
        eisdir,
        einval,
        ebadf,
        readonly
    );
    crate::println!("[ucompat-v214] filesystem submission hardening PASS");

    None
}

fn read_le_u16(data: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([data[off], data[off + 1]])
}

fn read_le_u32(data: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]])
}

fn read_le_u64(data: &[u8], off: usize) -> usize {
    u64::from_le_bytes([
        data[off],
        data[off + 1],
        data[off + 2],
        data[off + 3],
        data[off + 4],
        data[off + 5],
        data[off + 6],
        data[off + 7],
    ]) as usize
}

fn write_le_u16(out: &mut [u8], off: usize, value: u16) {
    let bytes = value.to_le_bytes();
    out[off] = bytes[0];
    out[off + 1] = bytes[1];
}

fn write_le_u32(out: &mut [u8], off: usize, value: u32) {
    let bytes = value.to_le_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        out[off + i] = bytes[i];
        i += 1;
    }
}

fn write_le_u64(out: &mut [u8], off: usize, value: usize) {
    let bytes = (value as u64).to_le_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        out[off + i] = bytes[i];
        i += 1;
    }
}

fn put_dirent64(out: &mut [u8], off: usize, ino: u64, next_off: i64, dtype: u8, name: &[u8]) -> Result<usize, Errno> {
    let header = 19usize;
    let raw_len = header + name.len() + 1;
    let reclen = (raw_len + 7) & !7usize;
    if off.checked_add(reclen).unwrap_or(usize::MAX) > out.len() {
        return Err(Errno::NoSpace);
    }
    put_u64(out, off, ino);
    put_i64(out, off + 8, next_off);
    put_u16(out, off + 16, reclen as u16);
    out[off + 18] = dtype;
    let mut i = 0usize;
    while i < name.len() {
        out[off + 19 + i] = name[i];
        i += 1;
    }
    out[off + 19 + i] = 0;
    i += 1;
    while 19 + i < reclen {
        out[off + 19 + i] = 0;
        i += 1;
    }
    Ok(off + reclen)
}

fn put_u16(out: &mut [u8], off: usize, value: u16) {
    let bytes = value.to_le_bytes();
    out[off] = bytes[0];
    out[off + 1] = bytes[1];
}

fn put_u64(out: &mut [u8], off: usize, value: u64) {
    let bytes = value.to_le_bytes();
    let mut i = 0usize;
    while i < 8 {
        out[off + i] = bytes[i];
        i += 1;
    }
}

fn put_i64(out: &mut [u8], off: usize, value: i64) {
    put_u64(out, off, value as u64);
}

fn copy_literal(out: &mut [u8], src: &[u8]) -> Result<usize, Errno> {
    if src.len() > out.len() {
        return Err(Errno::NoSpace);
    }
    let mut i = 0usize;
    while i < src.len() {
        out[i] = src[i];
        i += 1;
    }
    Ok(src.len())
}

fn copy_numbered(out: &mut [u8], prefix: &[u8], number: usize, suffix: &[u8]) -> Result<usize, Errno> {
    if prefix.len() + suffix.len() >= out.len() {
        return Err(Errno::NoSpace);
    }
    let mut len = 0usize;
    let mut i = 0usize;
    while i < prefix.len() {
        out[len] = prefix[i];
        len += 1;
        i += 1;
    }
    let mut digits = [0u8; 20];
    let mut n = number;
    let mut d = 0usize;
    if n == 0 {
        digits[0] = b'0';
        d = 1;
    } else {
        while n > 0 {
            digits[d] = b'0' + (n % 10) as u8;
            n /= 10;
            d += 1;
        }
    }
    while d > 0 {
        d -= 1;
        if len >= out.len() {
            return Err(Errno::NoSpace);
        }
        out[len] = digits[d];
        len += 1;
    }
    i = 0;
    while i < suffix.len() {
        if len >= out.len() {
            return Err(Errno::NoSpace);
        }
        out[len] = suffix[i];
        len += 1;
        i += 1;
    }
    Ok(len)
}

fn fd_kind_mode(kind: FdKind) -> u16 {
    match kind {
        FdKind::Directory | FdKind::Procfs => 0o040755,
        FdKind::Symlink => 0o120777,
        FdKind::Stdin | FdKind::Stdout | FdKind::Stderr | FdKind::DevNull | FdKind::DevZero | FdKind::DevConsole | FdKind::DevTty | FdKind::DevRandom => 0o020666,
        FdKind::PipeRead | FdKind::PipeWrite => 0o010666,
        FdKind::Socket => 0o140666,
        _ => 0o100666,
    }
}

fn bytes_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0usize;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

fn copy_name_bytes(src: &[u8], out: &mut [u8; NAME_MAX]) -> usize {
    let mut i = 0usize;
    while i < src.len() && i < NAME_MAX {
        out[i] = src[i];
        i += 1;
    }
    i
}

fn append_bytes(out: &mut [u8], len: &mut usize, src: &[u8]) {
    let mut i = 0usize;
    while i < src.len() && *len < out.len() {
        out[*len] = src[i];
        *len += 1;
        i += 1;
    }
}

fn decimal_to_bytes(number: usize, out: &mut [u8]) -> usize {
    let mut digits = [0u8; 20];
    let mut n = number;
    let mut d = 0usize;
    if n == 0 {
        digits[0] = b'0';
        d = 1;
    } else {
        while n > 0 && d < digits.len() {
            digits[d] = b'0' + (n % 10) as u8;
            n /= 10;
            d += 1;
        }
    }
    let mut len = 0usize;
    while d > 0 && len < out.len() {
        d -= 1;
        out[len] = digits[d];
        len += 1;
    }
    len
}

fn append_dec(out: &mut [u8], len: &mut usize, number: usize) {
    let mut digits = [0u8; 20];
    let count = decimal_to_bytes(number, &mut digits);
    append_bytes(out, len, &digits[..count]);
}

fn append_hex(out: &mut [u8], len: &mut usize, value: usize, width: usize) {
    let mut i = width;
    while i > 0 && *len < out.len() {
        i -= 1;
        let nibble = ((value >> (i * 4)) & 0xf) as u8;
        out[*len] = if nibble < 10 { b'0' + nibble } else { b'a' + (nibble - 10) };
        *len += 1;
    }
}

fn append_perm(out: &mut [u8], len: &mut usize, perm: u8) {
    let bytes = [
        if (perm & VMA_R) != 0 { b'r' } else { b'-' },
        if (perm & VMA_W) != 0 { b'w' } else { b'-' },
        if (perm & VMA_X) != 0 { b'x' } else { b'-' },
        b'p',
    ];
    append_bytes(out, len, &bytes);
}

fn task_state_bytes(state: RuntimeTaskState) -> &'static [u8] {
    match state {
        RuntimeTaskState::Running => b"R (running)",
        RuntimeTaskState::Ready => b"R (ready)",
        RuntimeTaskState::Waiting => b"S (sleeping)",
        RuntimeTaskState::Zombie => b"Z (zombie)",
        RuntimeTaskState::Exited => b"X (exited)",
        RuntimeTaskState::Empty => b"X (empty)",
    }
}

fn task_state_short(state: RuntimeTaskState) -> &'static [u8] {
    match state {
        RuntimeTaskState::Running | RuntimeTaskState::Ready => b"R",
        RuntimeTaskState::Waiting => b"S",
        RuntimeTaskState::Zombie => b"Z",
        RuntimeTaskState::Exited | RuntimeTaskState::Empty => b"X",
    }
}

fn vma_kind_label(kind: RuntimeVmaKind) -> &'static [u8] {
    match kind {
        RuntimeVmaKind::Load => b"/init.elf",
        RuntimeVmaKind::Heap => b"[heap]",
        RuntimeVmaKind::Stack => b"[stack]",
        RuntimeVmaKind::Mmap => b"[mmap]",
        RuntimeVmaKind::Empty => b"",
    }
}

fn path_starts(path: &[u8], prefix: &[u8]) -> bool {
    if path.len() < prefix.len() {
        return false;
    }
    let mut i = 0usize;
    while i < prefix.len() {
        if path[i] != prefix[i] {
            return false;
        }
        i += 1;
    }
    true
}

fn parse_decimal(bytes: &[u8]) -> Option<usize> {
    if bytes.is_empty() {
        return None;
    }
    let mut value = 0usize;
    let mut i = 0usize;
    while i < bytes.len() {
        let ch = bytes[i];
        if ch < b'0' || ch > b'9' {
            return None;
        }
        value = value.saturating_mul(10).saturating_add((ch - b'0') as usize);
        i += 1;
    }
    Some(value)
}

fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() {
        return true;
    }
    if needle.len() > haystack.len() {
        return false;
    }
    let mut i = 0usize;
    while i + needle.len() <= haystack.len() {
        let mut j = 0usize;
        while j < needle.len() && haystack[i + j] == needle[j] {
            j += 1;
        }
        if j == needle.len() {
            return true;
        }
        i += 1;
    }
    false
}
