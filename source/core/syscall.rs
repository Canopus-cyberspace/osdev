use core::sync::atomic::{AtomicUsize, Ordering};

use crate::core::fs::{
    encode_linux_stat, FdError, NoSyscallVfs, OpenOptions, OpenOptionsError, StatEncodingError,
    SyscallVfs, VfsDirEntry, VfsDirEntryKind, VfsError, VfsIoctl, VfsRuntimeError, VfsStat,
    LINUX_STAT_SIZE, VFS_NAME_MAX,
};
use crate::core::mm::{
    copy_from_user, copy_to_user, map_zeroed_user_pages, MappingFlags, NoUserMemory, UserMapError,
    UserMemoryMapper, UserMemoryReader, UserMemoryWriter, PAGE_SIZE,
};
use crate::core::task::{
    process_resource_limit, single_heap_base, single_mmap_cursor, single_parent_pid, single_pid,
    single_program_break, single_record_exit, single_set_mmap_cursor, single_set_program_break,
    single_set_robust_list, single_set_tid_address, single_signal_process, single_wait_for_child,
    ExitCode, ExitState, Pid, Process, ResourceLimitKind,
};
use crate::core::time::next_time_value;
use crate::official::user_output::{write_user_fd, UserOutputError};

pub const SYS_GETCWD: usize = 17;
pub const SYS_DUP: usize = 23;
pub const SYS_DUP3: usize = 24;
pub const SYS_FCNTL: usize = 25;
pub const SYS_IOCTL: usize = 29;
pub const SYS_MKDIRAT: usize = 34;
pub const SYS_UNLINKAT: usize = 35;
pub const SYS_RENAMEAT: usize = 38;
pub const SYS_UMOUNT2: usize = 39;
pub const SYS_MOUNT: usize = 40;
pub const SYS_STATFS: usize = 43;
pub const SYS_FSTATFS: usize = 44;
pub const SYS_FACCESSAT: usize = 48;
pub const SYS_CHDIR: usize = 49;
pub const SYS_OPENAT: usize = 56;
pub const SYS_CLOSE: usize = 57;
pub const SYS_PIPE2: usize = 59;
pub const SYS_GETDENTS64: usize = 61;
pub const SYS_LSEEK: usize = 62;
pub const SYS_READ: usize = 63;
pub const SYS_WRITE: usize = 64;
pub const SYS_READV: usize = 65;
pub const SYS_WRITEV: usize = 66;
pub const SYS_PREAD64: usize = 67;
pub const SYS_PPOLL: usize = 73;
pub const SYS_NEWFSTATAT: usize = 79;
pub const SYS_FSTAT: usize = 80;
pub const SYS_UTIMENSAT: usize = 88;
pub const SYS_EXIT: usize = 93;
pub const SYS_EXIT_GROUP: usize = 94;
pub const SYS_SET_TID_ADDRESS: usize = 96;
pub const SYS_SET_ROBUST_LIST: usize = 99;
pub const SYS_NANOSLEEP: usize = 101;
pub const SYS_CLOCK_GETTIME: usize = 113;
pub const SYS_SYSLOG: usize = 116;
pub const SYS_SCHED_YIELD: usize = 124;
pub const SYS_KILL: usize = 129;
pub const SYS_SIGALTSTACK: usize = 134;
pub const SYS_RT_SIGACTION: usize = 135;
pub const SYS_TIMES: usize = 153;
pub const SYS_UNAME: usize = 160;
pub const SYS_GETTIMEOFDAY: usize = 169;
pub const SYS_GETPID: usize = 172;
pub const SYS_GETPPID: usize = 173;
pub const SYS_GETUID: usize = 174;
pub const SYS_GETEUID: usize = 175;
pub const SYS_GETGID: usize = 176;
pub const SYS_GETEGID: usize = 177;
pub const SYS_BRK: usize = 214;
pub const SYS_MUNMAP: usize = 215;
pub const SYS_CLONE: usize = 220;
pub const SYS_EXECVE: usize = 221;
pub const SYS_MMAP: usize = 222;
pub const SYS_MPROTECT: usize = 226;
pub const SYS_WAIT4: usize = 260;
pub const SYS_PRLIMIT64: usize = 261;
pub const SYS_RENAMEAT2: usize = 276;

const TRACE_CAPACITY: usize = 32;
const SYSCALL_PATH_MAX: usize = 256;
const LINUX_DIRENT64_HEADER_SIZE: usize = 19;
const LINUX_DIRENT64_MAX_SIZE: usize = 280;
const LINUX_DIRENT64_ALIGN: usize = 8;
const LINUX_STATFS_SIZE: usize = 120;
const WRITE_CHUNK: usize = 256;
const READ_CHUNK: usize = 256;
const EXEC_ARG_COUNT: usize = 8;
const EXEC_ARG_MAX: usize = 128;
const IOVEC_SIZE: usize = 16;
const IOV_MAX: usize = 16;
const POLLFD_SIZE: usize = 8;
const POLLFD_MAX: usize = 16;
const STDOUT_FD: usize = 1;
const STDERR_FD: usize = 2;
const AT_SYMLINK_NOFOLLOW: usize = 0x100;
const AT_EMPTY_PATH: usize = 0x1000;
const AT_REMOVEDIR: usize = 0x200;
const ROBUST_LIST_HEAD_SIZE: usize = 24;
const TIOCGWINSZ: usize = 0x5413;
const F_DUPFD: usize = 0;
const F_GETFD: usize = 1;
const F_SETFD: usize = 2;
const F_GETFL: usize = 3;
const F_DUPFD_CLOEXEC: usize = 0x406;
const FD_CLOEXEC: usize = 1;
const POLLIN: usize = 0x001;
const POLLOUT: usize = 0x004;
const POLLNVAL: usize = 0x020;
const POLLRDNORM: usize = 0x040;
const POLLWRNORM: usize = 0x100;
const POLL_READ_MASK: usize = POLLIN | POLLRDNORM;
const POLL_WRITE_MASK: usize = POLLOUT | POLLWRNORM;
const SYSLOG_ACTION_READ: usize = 2;
const SYSLOG_ACTION_READ_ALL: usize = 3;
const SYSLOG_ACTION_READ_CLEAR: usize = 4;
const SYSLOG_ACTION_CLEAR: usize = 5;
const SYSLOG_ACTION_SIZE_UNREAD: usize = 9;
const SYSLOG_ACTION_SIZE_BUFFER: usize = 10;
const USER_MMAP_LIMIT: usize = (1usize << 37) - PAGE_SIZE;
const PROT_READ: usize = 0x1;
const PROT_WRITE: usize = 0x2;
const PROT_EXEC: usize = 0x4;
const MAP_PRIVATE: usize = 0x02;
const MAP_SHARED: usize = 0x01;
const MAP_FIXED: usize = 0x10;
const MAP_ANONYMOUS: usize = 0x20;
const MAP_DENYWRITE: usize = 0x800;
const MAP_STACK: usize = 0x20000;
const SIGCHLD: usize = 17;
const CLONE_SIGNAL_MASK: usize = 0xff;
const CLONE_CHILD_CLEARTID: usize = 0x0020_0000;
const CLONE_CHILD_SETTID: usize = 0x0100_0000;
const SUPPORTED_FORK_CLONE_FLAGS: usize =
    CLONE_SIGNAL_MASK | CLONE_CHILD_CLEARTID | CLONE_CHILD_SETTID;
const RLIMIT_STACK: usize = 3;
const LINUX_RLIMIT64_SIZE: usize = 16;
const TRACE_STATUS_HANDLED: usize = 1;
const TRACE_STATUS_ENOSYS: usize = 2;
const TRACE_STATUS_EXIT: usize = 3;
const TRACE_STATUS_ERRNO: usize = 4;
const SYSCALL_TRACE_ENABLED: bool = false;

macro_rules! syscall_trace_slots {
    () => {
        [
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
        ]
    };
}

#[no_mangle]
static SYSCALL_TRACE_COUNT: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
static SYSCALL_TRACE_BUF: [AtomicUsize; TRACE_CAPACITY] = syscall_trace_slots!();

#[no_mangle]
static SYSCALL_TRACE_ARG0: [AtomicUsize; TRACE_CAPACITY] = syscall_trace_slots!();

#[no_mangle]
static SYSCALL_TRACE_ARG1: [AtomicUsize; TRACE_CAPACITY] = syscall_trace_slots!();

#[no_mangle]
static SYSCALL_TRACE_ARG2: [AtomicUsize; TRACE_CAPACITY] = syscall_trace_slots!();

#[no_mangle]
static SYSCALL_TRACE_ARG3: [AtomicUsize; TRACE_CAPACITY] = syscall_trace_slots!();

#[no_mangle]
static SYSCALL_TRACE_ARG4: [AtomicUsize; TRACE_CAPACITY] = syscall_trace_slots!();

#[no_mangle]
static SYSCALL_TRACE_ARG5: [AtomicUsize; TRACE_CAPACITY] = syscall_trace_slots!();

#[no_mangle]
static SYSCALL_TRACE_RET: [AtomicUsize; TRACE_CAPACITY] = syscall_trace_slots!();

#[no_mangle]
static SYSCALL_TRACE_STATUS: [AtomicUsize; TRACE_CAPACITY] = syscall_trace_slots!();

#[no_mangle]
static LAST_EXIT_CODE: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
static NEWFSTATAT_PATH_LEN: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
static NEWFSTATAT_PATH_BYTES: [AtomicUsize; 32] = syscall_trace_slots!();

#[no_mangle]
static NEWFSTATAT_STAT_INODE: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
static NEWFSTATAT_STAT_MODE: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
static NEWFSTATAT_STAT_SIZE: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
static NEWFSTATAT_STAT_BLOCKS: AtomicUsize = AtomicUsize::new(0);

fn record_syscall(frame: SyscallFrame) -> usize {
    if !SYSCALL_TRACE_ENABLED {
        return usize::MAX;
    }

    let index = SYSCALL_TRACE_COUNT.fetch_add(1, Ordering::Relaxed);
    if index < TRACE_CAPACITY {
        SYSCALL_TRACE_BUF[index].store(frame.number(), Ordering::Relaxed);
        SYSCALL_TRACE_ARG0[index].store(frame.arg(0), Ordering::Relaxed);
        SYSCALL_TRACE_ARG1[index].store(frame.arg(1), Ordering::Relaxed);
        SYSCALL_TRACE_ARG2[index].store(frame.arg(2), Ordering::Relaxed);
        SYSCALL_TRACE_ARG3[index].store(frame.arg(3), Ordering::Relaxed);
        SYSCALL_TRACE_ARG4[index].store(frame.arg(4), Ordering::Relaxed);
        SYSCALL_TRACE_ARG5[index].store(frame.arg(5), Ordering::Relaxed);
    }

    index
}

fn record_syscall_outcome(index: usize, outcome: SyscallOutcome) {
    if !SYSCALL_TRACE_ENABLED {
        return;
    }

    if index >= TRACE_CAPACITY {
        return;
    }

    match outcome {
        SyscallOutcome::Return(value) => {
            SYSCALL_TRACE_RET[index].store(value as usize, Ordering::Relaxed);
            let status = if value == SyscallError::Unsupported.errno() {
                TRACE_STATUS_ENOSYS
            } else if value < 0 {
                TRACE_STATUS_ERRNO
            } else {
                TRACE_STATUS_HANDLED
            };
            SYSCALL_TRACE_STATUS[index].store(status, Ordering::Relaxed);
        }
        SyscallOutcome::Exit(exit) => {
            SYSCALL_TRACE_RET[index].store(exit.code().value() as usize, Ordering::Relaxed);
            SYSCALL_TRACE_STATUS[index].store(TRACE_STATUS_EXIT, Ordering::Relaxed);
        }
        SyscallOutcome::Fork(_) => {
            SYSCALL_TRACE_STATUS[index].store(TRACE_STATUS_HANDLED, Ordering::Relaxed);
        }
        SyscallOutcome::Exec(_) => {
            SYSCALL_TRACE_STATUS[index].store(TRACE_STATUS_HANDLED, Ordering::Relaxed);
        }
    }
}

fn record_exit_code(raw_code: usize) {
    LAST_EXIT_CODE.store(raw_code, Ordering::Relaxed);
}

fn record_newfstatat_path(bytes: &[u8]) {
    NEWFSTATAT_PATH_LEN.store(bytes.len(), Ordering::Relaxed);
    let mut word = 0usize;
    while word < NEWFSTATAT_PATH_BYTES.len() {
        NEWFSTATAT_PATH_BYTES[word].store(0, Ordering::Relaxed);
        word += 1;
    }

    let mut index = 0usize;
    while index < bytes.len() && index < NEWFSTATAT_PATH_BYTES.len() * core::mem::size_of::<usize>()
    {
        store_packed_trace_byte(&NEWFSTATAT_PATH_BYTES, index, bytes[index]);
        index += 1;
    }
}

fn record_newfstatat_stat(stat: VfsStat) {
    NEWFSTATAT_STAT_INODE.store(stat.inode() as usize, Ordering::Relaxed);
    NEWFSTATAT_STAT_MODE.store(stat.mode() as usize, Ordering::Relaxed);
    NEWFSTATAT_STAT_SIZE.store(stat.byte_len() as usize, Ordering::Relaxed);
    NEWFSTATAT_STAT_BLOCKS.store(stat.blocks() as usize, Ordering::Relaxed);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SyscallFrame {
    number: usize,
    args: [usize; 6],
}

impl SyscallFrame {
    pub const fn new(number: usize, args: [usize; 6]) -> Self {
        Self { number, args }
    }

    pub const fn number(self) -> usize {
        self.number
    }

    pub const fn arg(self, index: usize) -> usize {
        self.args[index]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SyscallOutcome {
    Return(isize),
    Exit(ExitState),
    Fork(ForkRequest),
    Exec(ExecRequest),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForkRequest {
    child_stack: usize,
    child_tid: usize,
    set_child_tid: bool,
    clear_child_tid: bool,
}

impl ForkRequest {
    pub const fn new(
        child_stack: usize,
        child_tid: usize,
        set_child_tid: bool,
        clear_child_tid: bool,
    ) -> Self {
        Self {
            child_stack,
            child_tid,
            set_child_tid,
            clear_child_tid,
        }
    }

    pub const fn child_stack(self) -> usize {
        self.child_stack
    }

    pub const fn child_tid(self) -> usize {
        self.child_tid
    }

    pub const fn set_child_tid(self) -> bool {
        self.set_child_tid
    }

    pub const fn clear_child_tid(self) -> bool {
        self.clear_child_tid
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExecRequest {
    path: [u8; SYSCALL_PATH_MAX],
    path_len: usize,
    args: [[u8; EXEC_ARG_MAX]; EXEC_ARG_COUNT],
    arg_lens: [usize; EXEC_ARG_COUNT],
    arg_count: usize,
}

impl ExecRequest {
    pub fn path(&self) -> &[u8] {
        &self.path[..self.path_len]
    }

    pub const fn arg_count(self) -> usize {
        self.arg_count
    }

    pub fn arg(&self, index: usize) -> Option<&[u8]> {
        if index < self.arg_count {
            Some(&self.args[index][..self.arg_lens[index]])
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SyscallError {
    BadFileDescriptor,
    Child,
    ExecFormat,
    Exists,
    Fault,
    Invalid,
    Io,
    IsDirectory,
    NameTooLong,
    NoDevice,
    NoEntry,
    NoMemory,
    NoSpace,
    NoProcess,
    NotDirectory,
    NotTerminal,
    NotSupported,
    PermissionDenied,
    ReadOnlyFilesystem,
    TooManyOpenFiles,
    Unsupported,
}

impl SyscallError {
    pub const fn errno(self) -> isize {
        match self {
            Self::BadFileDescriptor => -9,
            Self::Child => -10,
            Self::ExecFormat => -8,
            Self::Exists => -17,
            Self::Fault => -14,
            Self::Invalid => -22,
            Self::Io => -5,
            Self::IsDirectory => -21,
            Self::NameTooLong => -36,
            Self::NoDevice => -19,
            Self::NoEntry => -2,
            Self::NoMemory => -12,
            Self::NoSpace => -28,
            Self::NoProcess => -3,
            Self::NotDirectory => -20,
            Self::NotTerminal => -25,
            Self::NotSupported => -95,
            Self::PermissionDenied => -13,
            Self::ReadOnlyFilesystem => -30,
            Self::TooManyOpenFiles => -24,
            Self::Unsupported => -38,
        }
    }
}

pub fn dispatch(frame: SyscallFrame, process: &mut Process) -> SyscallOutcome {
    let memory = NoUserMemory;
    dispatch_with_memory(frame, process, &memory)
}

pub fn dispatch_with_memory<R: UserMemoryReader + UserMemoryWriter + UserMemoryMapper>(
    frame: SyscallFrame,
    process: &mut Process,
    memory: &R,
) -> SyscallOutcome {
    let vfs = NoSyscallVfs;
    dispatch_with_runtime(frame, process, memory, &vfs)
}

pub fn dispatch_with_runtime<
    R: UserMemoryReader + UserMemoryWriter + UserMemoryMapper,
    V: SyscallVfs,
>(
    frame: SyscallFrame,
    process: &mut Process,
    memory: &R,
    vfs: &V,
) -> SyscallOutcome {
    let trace = record_syscall(frame);
    let outcome = match frame.number() {
        SYS_GETPID => SyscallOutcome::Return(process.pid().value() as isize),
        SYS_GETPPID => SyscallOutcome::Return(sys_getppid(process)),
        SYS_GETUID | SYS_GETEUID | SYS_GETGID | SYS_GETEGID => SyscallOutcome::Return(0),
        SYS_SET_TID_ADDRESS => {
            process.set_tid_address(frame.arg(0));
            SyscallOutcome::Return(process.pid().value() as isize)
        }
        SYS_SET_ROBUST_LIST => SyscallOutcome::Return(sys_set_robust_list_process(frame, process)),
        SYS_DUP => SyscallOutcome::Return(sys_dup(frame, vfs)),
        SYS_DUP3 => SyscallOutcome::Return(sys_dup3(frame, vfs)),
        SYS_FCNTL => SyscallOutcome::Return(sys_fcntl(frame, vfs)),
        SYS_IOCTL => SyscallOutcome::Return(sys_ioctl(frame, memory, vfs)),
        SYS_EXIT | SYS_EXIT_GROUP => {
            record_exit_code(frame.arg(0));
            let state = process.exit(ExitCode::new(frame.arg(0) as i32));
            SyscallOutcome::Exit(state)
        }
        SYS_WRITE => SyscallOutcome::Return(sys_write(frame, memory, vfs)),
        SYS_READV => SyscallOutcome::Return(sys_readv(frame, memory, vfs)),
        SYS_WRITEV => SyscallOutcome::Return(sys_writev(frame, memory, vfs)),
        SYS_PREAD64 => SyscallOutcome::Return(sys_pread64(frame, memory, vfs)),
        SYS_READ => SyscallOutcome::Return(sys_read(frame, memory, vfs)),
        SYS_OPENAT => SyscallOutcome::Return(sys_openat(frame, memory, vfs)),
        SYS_CHDIR => SyscallOutcome::Return(sys_chdir(frame, memory, vfs)),
        SYS_MKDIRAT => SyscallOutcome::Return(sys_mkdirat(frame, memory, vfs)),
        SYS_UNLINKAT => SyscallOutcome::Return(sys_unlinkat(frame, memory, vfs)),
        SYS_RENAMEAT => SyscallOutcome::Return(sys_renameat(frame, memory, vfs, 0)),
        SYS_UMOUNT2 => SyscallOutcome::Return(sys_umount2(frame, memory, vfs)),
        SYS_MOUNT => SyscallOutcome::Return(sys_mount(frame, memory, vfs)),
        SYS_STATFS => SyscallOutcome::Return(sys_statfs(frame, memory, vfs)),
        SYS_FSTATFS => SyscallOutcome::Return(sys_fstatfs(frame, memory, vfs)),
        SYS_FACCESSAT => SyscallOutcome::Return(sys_faccessat(frame, memory, vfs)),
        SYS_PIPE2 => SyscallOutcome::Return(sys_pipe2(frame, memory, vfs)),
        SYS_GETDENTS64 => SyscallOutcome::Return(sys_getdents64(frame, memory, vfs)),
        SYS_CLOSE => SyscallOutcome::Return(sys_close(frame, vfs)),
        SYS_FSTAT => SyscallOutcome::Return(sys_fstat(frame, memory, vfs)),
        SYS_UTIMENSAT => SyscallOutcome::Return(sys_utimensat(frame, memory, vfs)),
        SYS_LSEEK => SyscallOutcome::Return(sys_lseek(frame, vfs)),
        SYS_PPOLL => SyscallOutcome::Return(sys_ppoll(frame, memory, vfs)),
        SYS_BRK => SyscallOutcome::Return(sys_brk(frame, memory)),
        SYS_MMAP => SyscallOutcome::Return(sys_mmap(frame, memory, vfs)),
        SYS_MUNMAP => SyscallOutcome::Return(sys_munmap(frame, memory)),
        SYS_MPROTECT => SyscallOutcome::Return(sys_mprotect(frame, memory)),
        SYS_CLONE => sys_clone(frame),
        SYS_EXECVE => sys_execve(frame, memory),
        SYS_WAIT4 => SyscallOutcome::Return(sys_wait4(frame, memory)),
        SYS_PRLIMIT64 => SyscallOutcome::Return(sys_prlimit64(frame, memory, process.pid())),
        SYS_CLOCK_GETTIME => SyscallOutcome::Return(sys_clock_gettime(frame, memory)),
        SYS_SYSLOG => SyscallOutcome::Return(sys_syslog(frame)),
        SYS_KILL => SyscallOutcome::Return(sys_kill(frame)),
        SYS_GETTIMEOFDAY => SyscallOutcome::Return(sys_gettimeofday(frame, memory)),
        SYS_TIMES => SyscallOutcome::Return(sys_times(frame, memory)),
        SYS_NANOSLEEP => SyscallOutcome::Return(0),
        SYS_SCHED_YIELD => SyscallOutcome::Return(0),
        SYS_UNAME => SyscallOutcome::Return(sys_uname(frame, memory)),
        SYS_GETCWD => SyscallOutcome::Return(sys_getcwd(frame, memory, vfs)),
        SYS_SIGALTSTACK => SyscallOutcome::Return(0),
        SYS_RT_SIGACTION => SyscallOutcome::Return(0),
        SYS_NEWFSTATAT => SyscallOutcome::Return(sys_newfstatat(frame, memory, vfs)),
        SYS_RENAMEAT2 => SyscallOutcome::Return(sys_renameat(frame, memory, vfs, frame.arg(4))),
        _ => SyscallOutcome::Return(SyscallError::Unsupported.errno()),
    };
    record_syscall_outcome(trace, outcome);
    outcome
}

pub fn dispatch_single(frame: SyscallFrame) -> SyscallOutcome {
    let memory = NoUserMemory;
    dispatch_single_with_memory(frame, &memory)
}

pub fn dispatch_single_with_memory<R: UserMemoryReader + UserMemoryWriter + UserMemoryMapper>(
    frame: SyscallFrame,
    memory: &R,
) -> SyscallOutcome {
    let vfs = NoSyscallVfs;
    dispatch_single_with_runtime(frame, memory, &vfs)
}

pub fn dispatch_single_with_runtime<
    R: UserMemoryReader + UserMemoryWriter + UserMemoryMapper,
    V: SyscallVfs,
>(
    frame: SyscallFrame,
    memory: &R,
    vfs: &V,
) -> SyscallOutcome {
    let trace = record_syscall(frame);
    let outcome = match frame.number() {
        SYS_GETPID => SyscallOutcome::Return(single_pid().value() as isize),
        SYS_GETPPID => SyscallOutcome::Return(sys_single_getppid()),
        SYS_GETUID | SYS_GETEUID | SYS_GETGID | SYS_GETEGID => SyscallOutcome::Return(0),
        SYS_SET_TID_ADDRESS => {
            single_set_tid_address(frame.arg(0));
            SyscallOutcome::Return(single_pid().value() as isize)
        }
        SYS_SET_ROBUST_LIST => SyscallOutcome::Return(sys_set_robust_list_single(frame)),
        SYS_DUP => SyscallOutcome::Return(sys_dup(frame, vfs)),
        SYS_DUP3 => SyscallOutcome::Return(sys_dup3(frame, vfs)),
        SYS_FCNTL => SyscallOutcome::Return(sys_fcntl(frame, vfs)),
        SYS_IOCTL => SyscallOutcome::Return(sys_ioctl(frame, memory, vfs)),
        SYS_EXIT | SYS_EXIT_GROUP => {
            record_exit_code(frame.arg(0));
            let state = ExitState::new(single_pid(), ExitCode::new(frame.arg(0) as i32));
            single_record_exit(state);
            SyscallOutcome::Exit(state)
        }
        SYS_WRITE => SyscallOutcome::Return(sys_write(frame, memory, vfs)),
        SYS_READV => SyscallOutcome::Return(sys_readv(frame, memory, vfs)),
        SYS_WRITEV => SyscallOutcome::Return(sys_writev(frame, memory, vfs)),
        SYS_PREAD64 => SyscallOutcome::Return(sys_pread64(frame, memory, vfs)),
        SYS_READ => SyscallOutcome::Return(sys_read(frame, memory, vfs)),
        SYS_OPENAT => SyscallOutcome::Return(sys_openat(frame, memory, vfs)),
        SYS_CHDIR => SyscallOutcome::Return(sys_chdir(frame, memory, vfs)),
        SYS_MKDIRAT => SyscallOutcome::Return(sys_mkdirat(frame, memory, vfs)),
        SYS_UNLINKAT => SyscallOutcome::Return(sys_unlinkat(frame, memory, vfs)),
        SYS_RENAMEAT => SyscallOutcome::Return(sys_renameat(frame, memory, vfs, 0)),
        SYS_UMOUNT2 => SyscallOutcome::Return(sys_umount2(frame, memory, vfs)),
        SYS_MOUNT => SyscallOutcome::Return(sys_mount(frame, memory, vfs)),
        SYS_STATFS => SyscallOutcome::Return(sys_statfs(frame, memory, vfs)),
        SYS_FSTATFS => SyscallOutcome::Return(sys_fstatfs(frame, memory, vfs)),
        SYS_FACCESSAT => SyscallOutcome::Return(sys_faccessat(frame, memory, vfs)),
        SYS_PIPE2 => SyscallOutcome::Return(sys_pipe2(frame, memory, vfs)),
        SYS_GETDENTS64 => SyscallOutcome::Return(sys_getdents64(frame, memory, vfs)),
        SYS_CLOSE => SyscallOutcome::Return(sys_close(frame, vfs)),
        SYS_FSTAT => SyscallOutcome::Return(sys_fstat(frame, memory, vfs)),
        SYS_UTIMENSAT => SyscallOutcome::Return(sys_utimensat(frame, memory, vfs)),
        SYS_LSEEK => SyscallOutcome::Return(sys_lseek(frame, vfs)),
        SYS_PPOLL => SyscallOutcome::Return(sys_ppoll(frame, memory, vfs)),
        SYS_BRK => SyscallOutcome::Return(sys_brk(frame, memory)),
        SYS_MMAP => SyscallOutcome::Return(sys_mmap(frame, memory, vfs)),
        SYS_MUNMAP => SyscallOutcome::Return(sys_munmap(frame, memory)),
        SYS_MPROTECT => SyscallOutcome::Return(sys_mprotect(frame, memory)),
        SYS_CLONE => sys_clone(frame),
        SYS_EXECVE => sys_execve(frame, memory),
        SYS_WAIT4 => SyscallOutcome::Return(sys_wait4(frame, memory)),
        SYS_PRLIMIT64 => SyscallOutcome::Return(sys_prlimit64(frame, memory, single_pid())),
        SYS_CLOCK_GETTIME => SyscallOutcome::Return(sys_clock_gettime(frame, memory)),
        SYS_SYSLOG => SyscallOutcome::Return(sys_syslog(frame)),
        SYS_KILL => SyscallOutcome::Return(sys_kill(frame)),
        SYS_GETTIMEOFDAY => SyscallOutcome::Return(sys_gettimeofday(frame, memory)),
        SYS_TIMES => SyscallOutcome::Return(sys_times(frame, memory)),
        SYS_NANOSLEEP => SyscallOutcome::Return(0),
        SYS_SCHED_YIELD => SyscallOutcome::Return(0),
        SYS_UNAME => SyscallOutcome::Return(sys_uname(frame, memory)),
        SYS_GETCWD => SyscallOutcome::Return(sys_getcwd(frame, memory, vfs)),
        SYS_SIGALTSTACK => SyscallOutcome::Return(0),
        SYS_RT_SIGACTION => SyscallOutcome::Return(0),
        SYS_NEWFSTATAT => SyscallOutcome::Return(sys_newfstatat(frame, memory, vfs)),
        SYS_RENAMEAT2 => SyscallOutcome::Return(sys_renameat(frame, memory, vfs, frame.arg(4))),
        _ => SyscallOutcome::Return(SyscallError::Unsupported.errno()),
    };
    record_syscall_outcome(trace, outcome);
    outcome
}

fn sys_clock_gettime<W: UserMemoryWriter>(frame: SyscallFrame, memory: &W) -> isize {
    let timespec = frame.arg(1);
    if timespec == 0 {
        return SyscallError::Fault.errno();
    }

    let now = next_time_value();
    let mut raw = [0u8; 16];
    raw[..8].copy_from_slice(&now.seconds().to_le_bytes());
    raw[8..].copy_from_slice(&now.subsecond_nanos().to_le_bytes());
    if copy_to_user(memory, timespec, &raw).is_err() {
        SyscallError::Fault.errno()
    } else {
        0
    }
}

fn sys_gettimeofday<W: UserMemoryWriter>(frame: SyscallFrame, memory: &W) -> isize {
    let timeval = frame.arg(0);
    let timezone = frame.arg(1);
    if timeval != 0 {
        let now = next_time_value();
        let mut raw = [0u8; 16];
        raw[..8].copy_from_slice(&now.seconds().to_le_bytes());
        raw[8..].copy_from_slice(&now.subsecond_micros().to_le_bytes());
        if copy_to_user(memory, timeval, &raw).is_err() {
            return SyscallError::Fault.errno();
        }
    }
    if timezone != 0 {
        let zeros = [0u8; 8];
        if copy_to_user(memory, timezone, &zeros).is_err() {
            return SyscallError::Fault.errno();
        }
    }

    0
}

fn sys_times<W: UserMemoryWriter>(frame: SyscallFrame, memory: &W) -> isize {
    let tms = frame.arg(0);
    if tms != 0 {
        let raw = [0u8; 32];
        if copy_to_user(memory, tms, &raw).is_err() {
            return SyscallError::Fault.errno();
        }
    }
    0
}

fn sys_syslog(frame: SyscallFrame) -> isize {
    match frame.arg(0) {
        SYSLOG_ACTION_READ | SYSLOG_ACTION_READ_ALL | SYSLOG_ACTION_READ_CLEAR => 0,
        SYSLOG_ACTION_CLEAR => 0,
        SYSLOG_ACTION_SIZE_UNREAD => 0,
        SYSLOG_ACTION_SIZE_BUFFER => 4096,
        _ => SyscallError::Invalid.errno(),
    }
}

fn sys_kill(frame: SyscallFrame) -> isize {
    let pid = frame.arg(0) as isize;
    let signal = frame.arg(1);
    if pid <= 0 {
        return SyscallError::NotSupported.errno();
    }
    if signal > 64 {
        return SyscallError::Invalid.errno();
    }
    if single_signal_process(Pid::new(pid as usize), signal) {
        0
    } else {
        SyscallError::NoProcess.errno()
    }
}

fn sys_newfstatat<R: UserMemoryReader + UserMemoryWriter, V: SyscallVfs>(
    frame: SyscallFrame,
    memory: &R,
    vfs: &V,
) -> isize {
    let dirfd = frame.arg(0) as isize;
    let path_ptr = frame.arg(1);
    let statbuf = frame.arg(2);
    let flags = frame.arg(3);

    if flags & !(AT_SYMLINK_NOFOLLOW | AT_EMPTY_PATH) != 0 {
        return SyscallError::NotSupported.errno();
    }
    if path_ptr == 0 || statbuf == 0 {
        return SyscallError::Fault.errno();
    }

    let mut path_bytes = [0u8; SYSCALL_PATH_MAX];
    let path_len = match copy_user_c_string(memory, path_ptr, &mut path_bytes) {
        Ok(len) => len,
        Err(error) => return error.errno(),
    };
    if path_len == 0 {
        if flags & AT_EMPTY_PATH == 0 {
            return SyscallError::NoEntry.errno();
        }
        if dirfd < 0 {
            return SyscallError::BadFileDescriptor.errno();
        }
        let stat = match vfs.fstat_fd(dirfd as usize) {
            Ok(stat) => stat,
            Err(error) => return map_vfs_runtime_error(error).errno(),
        };
        record_newfstatat_stat(stat);
        return write_stat_to_user(memory, statbuf, stat);
    }

    let path_slice = &path_bytes[..path_len];
    record_newfstatat_path(path_slice);
    let stat = match vfs.stat_path_at(dirfd, path_slice) {
        Ok(stat) => stat,
        Err(error) => return map_vfs_runtime_error(error).errno(),
    };
    record_newfstatat_stat(stat);

    write_stat_to_user(memory, statbuf, stat)
}

fn sys_faccessat<R: UserMemoryReader, V: SyscallVfs>(
    frame: SyscallFrame,
    memory: &R,
    vfs: &V,
) -> isize {
    let dirfd = frame.arg(0) as isize;
    let path_ptr = frame.arg(1);
    let mode = frame.arg(2);

    if path_ptr == 0 {
        return SyscallError::Fault.errno();
    }
    if mode & !0x7 != 0 {
        return SyscallError::Invalid.errno();
    }

    let mut path_bytes = [0u8; SYSCALL_PATH_MAX];
    let path_len = match copy_user_c_string(memory, path_ptr, &mut path_bytes) {
        Ok(len) => len,
        Err(error) => return error.errno(),
    };
    if path_len == 0 {
        return SyscallError::NoEntry.errno();
    }

    match vfs.access_path_at(dirfd, &path_bytes[..path_len], mode as u8) {
        Ok(()) => 0,
        Err(error) => map_vfs_runtime_error(error).errno(),
    }
}

fn sys_openat<R: UserMemoryReader, V: SyscallVfs>(
    frame: SyscallFrame,
    memory: &R,
    vfs: &V,
) -> isize {
    let path_ptr = frame.arg(1);
    let flags = frame.arg(2);
    if path_ptr == 0 {
        return SyscallError::Fault.errno();
    }

    let options = match OpenOptions::from_linux_flags(flags) {
        Ok(options) => options,
        Err(error) => return map_open_options_error(error).errno(),
    };
    let mut path_bytes = [0u8; SYSCALL_PATH_MAX];
    let path_len = match copy_user_c_string(memory, path_ptr, &mut path_bytes) {
        Ok(len) => len,
        Err(error) => return error.errno(),
    };
    if path_len == 0 {
        return SyscallError::NoEntry.errno();
    }

    match vfs.open_path_at(
        frame.arg(0) as isize,
        &path_bytes[..path_len],
        options,
        frame.arg(3) as u16,
    ) {
        Ok(fd) => fd as isize,
        Err(error) => map_vfs_runtime_error(error).errno(),
    }
}

fn sys_chdir<R: UserMemoryReader, V: SyscallVfs>(
    frame: SyscallFrame,
    memory: &R,
    vfs: &V,
) -> isize {
    let path_ptr = frame.arg(0);
    if path_ptr == 0 {
        return SyscallError::Fault.errno();
    }

    let mut path_bytes = [0u8; SYSCALL_PATH_MAX];
    let path_len = match copy_user_c_string(memory, path_ptr, &mut path_bytes) {
        Ok(len) => len,
        Err(error) => return error.errno(),
    };
    if path_len == 0 {
        return SyscallError::NoEntry.errno();
    }

    match vfs.change_dir(&path_bytes[..path_len]) {
        Ok(()) => 0,
        Err(error) => map_vfs_runtime_error(error).errno(),
    }
}

fn sys_mkdirat<R: UserMemoryReader, V: SyscallVfs>(
    frame: SyscallFrame,
    memory: &R,
    vfs: &V,
) -> isize {
    let path_ptr = frame.arg(1);
    if path_ptr == 0 {
        return SyscallError::Fault.errno();
    }

    let mut path_bytes = [0u8; SYSCALL_PATH_MAX];
    let path_len = match copy_user_c_string(memory, path_ptr, &mut path_bytes) {
        Ok(len) => len,
        Err(error) => return error.errno(),
    };
    if path_len == 0 {
        return SyscallError::NoEntry.errno();
    }

    match vfs.mkdir_at(
        frame.arg(0) as isize,
        &path_bytes[..path_len],
        frame.arg(2) as u16,
    ) {
        Ok(()) => 0,
        Err(error) => map_vfs_runtime_error(error).errno(),
    }
}

fn sys_unlinkat<R: UserMemoryReader, V: SyscallVfs>(
    frame: SyscallFrame,
    memory: &R,
    vfs: &V,
) -> isize {
    let path_ptr = frame.arg(1);
    let flags = frame.arg(2);
    if path_ptr == 0 {
        return SyscallError::Fault.errno();
    }
    if flags & !AT_REMOVEDIR != 0 {
        return SyscallError::NotSupported.errno();
    }

    let mut path_bytes = [0u8; SYSCALL_PATH_MAX];
    let path_len = match copy_user_c_string(memory, path_ptr, &mut path_bytes) {
        Ok(len) => len,
        Err(error) => return error.errno(),
    };
    if path_len == 0 {
        return SyscallError::NoEntry.errno();
    }

    match vfs.unlink_at(frame.arg(0) as isize, &path_bytes[..path_len], flags) {
        Ok(()) => 0,
        Err(error) => map_vfs_runtime_error(error).errno(),
    }
}

fn sys_renameat<R: UserMemoryReader, V: SyscallVfs>(
    frame: SyscallFrame,
    memory: &R,
    vfs: &V,
    flags: usize,
) -> isize {
    let old_path_ptr = frame.arg(1);
    let new_path_ptr = frame.arg(3);
    if old_path_ptr == 0 || new_path_ptr == 0 {
        return SyscallError::Fault.errno();
    }

    let mut old_path = [0u8; SYSCALL_PATH_MAX];
    let old_len = match copy_user_c_string(memory, old_path_ptr, &mut old_path) {
        Ok(len) => len,
        Err(error) => return error.errno(),
    };
    let mut new_path = [0u8; SYSCALL_PATH_MAX];
    let new_len = match copy_user_c_string(memory, new_path_ptr, &mut new_path) {
        Ok(len) => len,
        Err(error) => return error.errno(),
    };
    if old_len == 0 || new_len == 0 {
        return SyscallError::NoEntry.errno();
    }

    match vfs.rename_at(
        frame.arg(0) as isize,
        &old_path[..old_len],
        frame.arg(2) as isize,
        &new_path[..new_len],
        flags,
    ) {
        Ok(()) => 0,
        Err(error) => map_vfs_runtime_error(error).errno(),
    }
}

fn sys_mount<R: UserMemoryReader, V: SyscallVfs>(
    frame: SyscallFrame,
    memory: &R,
    vfs: &V,
) -> isize {
    let source_ptr = frame.arg(0);
    let target_ptr = frame.arg(1);
    let filesystem_ptr = frame.arg(2);
    let flags = frame.arg(3);
    let data_ptr = frame.arg(4);
    if source_ptr == 0 || target_ptr == 0 || filesystem_ptr == 0 {
        return SyscallError::Fault.errno();
    }
    if data_ptr != 0 {
        return SyscallError::NotSupported.errno();
    }

    let mut source = [0u8; SYSCALL_PATH_MAX];
    let source_len = match copy_user_c_string(memory, source_ptr, &mut source) {
        Ok(len) => len,
        Err(error) => return error.errno(),
    };
    let mut target = [0u8; SYSCALL_PATH_MAX];
    let target_len = match copy_user_c_string(memory, target_ptr, &mut target) {
        Ok(len) => len,
        Err(error) => return error.errno(),
    };
    let mut filesystem = [0u8; SYSCALL_PATH_MAX];
    let filesystem_len = match copy_user_c_string(memory, filesystem_ptr, &mut filesystem) {
        Ok(len) => len,
        Err(error) => return error.errno(),
    };
    if target_len == 0 {
        return SyscallError::NoEntry.errno();
    }
    if source_len == 0 || filesystem_len == 0 {
        return SyscallError::Invalid.errno();
    }

    match vfs.mount(
        &source[..source_len],
        &target[..target_len],
        &filesystem[..filesystem_len],
        flags,
    ) {
        Ok(()) => 0,
        Err(error) => map_vfs_runtime_error(error).errno(),
    }
}

fn sys_umount2<R: UserMemoryReader, V: SyscallVfs>(
    frame: SyscallFrame,
    memory: &R,
    vfs: &V,
) -> isize {
    let target_ptr = frame.arg(0);
    if target_ptr == 0 {
        return SyscallError::Fault.errno();
    }

    let mut target = [0u8; SYSCALL_PATH_MAX];
    let target_len = match copy_user_c_string(memory, target_ptr, &mut target) {
        Ok(len) => len,
        Err(error) => return error.errno(),
    };
    if target_len == 0 {
        return SyscallError::NoEntry.errno();
    }

    match vfs.unmount(&target[..target_len], frame.arg(1)) {
        Ok(()) => 0,
        Err(error) => map_vfs_runtime_error(error).errno(),
    }
}

fn sys_statfs<R: UserMemoryReader + UserMemoryWriter, V: SyscallVfs>(
    frame: SyscallFrame,
    memory: &R,
    vfs: &V,
) -> isize {
    let path_ptr = frame.arg(0);
    let statfs = frame.arg(1);
    if path_ptr == 0 || statfs == 0 {
        return SyscallError::Fault.errno();
    }
    let mut path = [0u8; SYSCALL_PATH_MAX];
    let path_len = match copy_user_c_string(memory, path_ptr, &mut path) {
        Ok(len) => len,
        Err(error) => return error.errno(),
    };
    if path_len == 0 {
        return SyscallError::NoEntry.errno();
    }
    let stat = match vfs.stat_path_at(-100, &path[..path_len]) {
        Ok(stat) => stat,
        Err(error) => return map_vfs_runtime_error(error).errno(),
    };
    write_statfs_to_user(memory, statfs, stat)
}

fn sys_fstatfs<W: UserMemoryWriter, V: SyscallVfs>(
    frame: SyscallFrame,
    memory: &W,
    vfs: &V,
) -> isize {
    let statfs = frame.arg(1);
    if statfs == 0 {
        return SyscallError::Fault.errno();
    }
    let stat = match vfs.fstat_fd(frame.arg(0)) {
        Ok(stat) => stat,
        Err(error) => return map_vfs_runtime_error(error).errno(),
    };
    write_statfs_to_user(memory, statfs, stat)
}

fn sys_pipe2<W: UserMemoryWriter, V: SyscallVfs>(
    frame: SyscallFrame,
    memory: &W,
    vfs: &V,
) -> isize {
    let pipefd = frame.arg(0);
    let flags = frame.arg(1);
    if pipefd == 0 {
        return SyscallError::Fault.errno();
    }
    if flags != 0 {
        return SyscallError::NotSupported.errno();
    }

    let (read_fd, write_fd) = match vfs.pipe() {
        Ok(pair) => pair,
        Err(error) => return map_vfs_runtime_error(error).errno(),
    };
    if read_fd > u32::MAX as usize || write_fd > u32::MAX as usize {
        return SyscallError::Invalid.errno();
    }

    let mut raw = [0u8; 8];
    raw[..4].copy_from_slice(&(read_fd as u32).to_le_bytes());
    raw[4..].copy_from_slice(&(write_fd as u32).to_le_bytes());
    if copy_to_user(memory, pipefd, &raw).is_err() {
        SyscallError::Fault.errno()
    } else {
        0
    }
}

fn sys_ppoll<R: UserMemoryReader + UserMemoryWriter, V: SyscallVfs>(
    frame: SyscallFrame,
    memory: &R,
    vfs: &V,
) -> isize {
    let fds = frame.arg(0);
    let nfds = frame.arg(1);
    let timeout = frame.arg(2);
    let sigmask = frame.arg(3);
    if nfds == 0 {
        return 0;
    }
    if fds == 0 {
        return SyscallError::Fault.errno();
    }
    if nfds > POLLFD_MAX {
        return SyscallError::Invalid.errno();
    }
    if sigmask != 0 {
        return SyscallError::NotSupported.errno();
    }
    if timeout != 0 {
        let mut raw_timeout = [0u8; 16];
        if copy_from_user(memory, timeout, &mut raw_timeout).is_err() {
            return SyscallError::Fault.errno();
        }
    }

    let mut ready = 0usize;
    let mut index = 0usize;
    while index < nfds {
        let entry = match fds.checked_add(index * POLLFD_SIZE) {
            Some(value) => value,
            None => return SyscallError::Fault.errno(),
        };
        let mut raw = [0u8; POLLFD_SIZE];
        if copy_from_user(memory, entry, &mut raw).is_err() {
            return SyscallError::Fault.errno();
        }
        let fd = i32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]);
        let events = u16::from_le_bytes([raw[4], raw[5]]) as usize;
        let revents = if fd < 0 {
            Ok(0)
        } else {
            poll_fd_revents(vfs, fd as usize, events)
        };
        let revents = match revents {
            Ok(value) => value,
            Err(SyscallError::BadFileDescriptor) => POLLNVAL,
            Err(error) => return error.errno(),
        };
        if revents != 0 {
            ready += 1;
        }
        let raw_revents = (revents as u16).to_le_bytes();
        if copy_to_user(memory, entry + 6, &raw_revents).is_err() {
            return SyscallError::Fault.errno();
        }
        index += 1;
    }

    ready as isize
}

fn poll_fd_revents<V: SyscallVfs>(
    vfs: &V,
    fd: usize,
    events: usize,
) -> Result<usize, SyscallError> {
    let mut revents = 0usize;
    if events & POLL_READ_MASK != 0 {
        match vfs.fd_readable(fd) {
            Ok(true) => revents |= events & POLL_READ_MASK,
            Ok(false) => {}
            Err(error) => return Err(map_vfs_runtime_error(error)),
        }
    }
    if events & POLL_WRITE_MASK != 0 {
        match vfs.fd_writable(fd) {
            Ok(true) => revents |= events & POLL_WRITE_MASK,
            Ok(false) => {}
            Err(error) => return Err(map_vfs_runtime_error(error)),
        }
    }
    if events & !(POLL_READ_MASK | POLL_WRITE_MASK) != 0 && revents == 0 {
        if let Err(error) = vfs.fstat_fd(fd) {
            return Err(map_vfs_runtime_error(error));
        }
    }
    Ok(revents)
}

fn sys_getdents64<R: UserMemoryWriter, V: SyscallVfs>(
    frame: SyscallFrame,
    memory: &R,
    vfs: &V,
) -> isize {
    let fd = frame.arg(0);
    let dirp = frame.arg(1);
    let count = frame.arg(2);
    if count == 0 {
        return 0;
    }
    if dirp == 0 {
        return SyscallError::Fault.errno();
    }
    if count > isize::MAX as usize {
        return SyscallError::Invalid.errno();
    }

    let mut written = 0usize;
    while written < count {
        let entry = match vfs.dir_entry_at_fd(fd) {
            Ok(Some(entry)) => entry,
            Ok(None) => return written as isize,
            Err(error) => return map_vfs_runtime_error(error).errno(),
        };
        let record_len = linux_dirent64_record_len(entry.name().len());
        if record_len > count - written {
            if written == 0 {
                return SyscallError::Invalid.errno();
            }
            return written as isize;
        }

        let mut record = [0u8; LINUX_DIRENT64_MAX_SIZE];
        let encoded = match encode_linux_dirent64(entry, &mut record) {
            Ok(len) => len,
            Err(error) => return error.errno(),
        };
        let target = match dirp.checked_add(written) {
            Some(address) => address,
            None => return SyscallError::Fault.errno(),
        };
        if copy_to_user(memory, target, &record[..encoded]).is_err() {
            return SyscallError::Fault.errno();
        }
        if let Err(error) = vfs.set_fd_offset(fd, entry.next_offset()) {
            return map_fd_error(error).errno();
        }
        written += encoded;
    }

    written as isize
}

fn sys_close<V: SyscallVfs>(frame: SyscallFrame, vfs: &V) -> isize {
    match vfs.close_fd(frame.arg(0)) {
        Ok(()) => 0,
        Err(error) => map_fd_error(error).errno(),
    }
}

fn sys_fcntl<V: SyscallVfs>(frame: SyscallFrame, vfs: &V) -> isize {
    let fd = frame.arg(0);
    let command = frame.arg(1);
    let value = frame.arg(2);
    match command {
        F_DUPFD => match vfs.duplicate_fd_min(fd, value, false) {
            Ok(new_fd) => new_fd as isize,
            Err(error) => map_fd_error(error).errno(),
        },
        F_GETFD => {
            if is_stdio_fd(fd) {
                return 0;
            }
            match vfs.fd_close_on_exec(fd) {
                Ok(true) => FD_CLOEXEC as isize,
                Ok(false) => 0,
                Err(error) => map_fd_error(error).errno(),
            }
        }
        F_SETFD => {
            if value & !FD_CLOEXEC != 0 {
                return SyscallError::Invalid.errno();
            }
            if is_stdio_fd(fd) {
                return 0;
            }
            match vfs.set_fd_close_on_exec(fd, value & FD_CLOEXEC != 0) {
                Ok(()) => 0,
                Err(error) => map_fd_error(error).errno(),
            }
        }
        F_GETFL => {
            if is_stdio_fd(fd) {
                return 0x1;
            }
            match vfs.fd_status_flags(fd) {
                Ok(flags) => flags as isize,
                Err(error) => map_fd_error(error).errno(),
            }
        }
        F_DUPFD_CLOEXEC => match vfs.duplicate_fd_min(fd, value, true) {
            Ok(new_fd) => new_fd as isize,
            Err(error) => map_fd_error(error).errno(),
        },
        _ => SyscallError::Unsupported.errno(),
    }
}

fn sys_fstat<W: UserMemoryWriter, V: SyscallVfs>(
    frame: SyscallFrame,
    memory: &W,
    vfs: &V,
) -> isize {
    let statbuf = frame.arg(1);
    if statbuf == 0 {
        return SyscallError::Fault.errno();
    }
    match vfs.fstat_fd(frame.arg(0)) {
        Ok(stat) => write_stat_to_user(memory, statbuf, stat),
        Err(error) => map_vfs_runtime_error(error).errno(),
    }
}

fn sys_utimensat<R: UserMemoryReader, V: SyscallVfs>(
    frame: SyscallFrame,
    memory: &R,
    vfs: &V,
) -> isize {
    let dirfd = frame.arg(0) as isize;
    let path_ptr = frame.arg(1);
    let times = frame.arg(2);
    let flags = frame.arg(3);
    if flags & !AT_SYMLINK_NOFOLLOW != 0 {
        return SyscallError::NotSupported.errno();
    }
    if times != 0 {
        let mut raw_times = [0u8; 32];
        if copy_from_user(memory, times, &mut raw_times).is_err() {
            return SyscallError::Fault.errno();
        }
    }
    if path_ptr == 0 {
        return 0;
    }

    let mut path = [0u8; SYSCALL_PATH_MAX];
    let path_len = match copy_user_c_string(memory, path_ptr, &mut path) {
        Ok(len) => len,
        Err(error) => return error.errno(),
    };
    if path_len == 0 {
        return SyscallError::NoEntry.errno();
    }
    match vfs.stat_path_at(dirfd, &path[..path_len]) {
        Ok(_) => 0,
        Err(error) => map_vfs_runtime_error(error).errno(),
    }
}

fn sys_lseek<V: SyscallVfs>(frame: SyscallFrame, vfs: &V) -> isize {
    let offset = frame.arg(1) as i64;
    match vfs.lseek_fd(frame.arg(0), offset, frame.arg(2)) {
        Ok(offset) if offset <= isize::MAX as u64 => offset as isize,
        Ok(_) => SyscallError::Invalid.errno(),
        Err(error) => map_vfs_runtime_error(error).errno(),
    }
}

fn sys_read<W: UserMemoryWriter, V: SyscallVfs>(frame: SyscallFrame, memory: &W, vfs: &V) -> isize {
    let fd = frame.arg(0);
    let user_ptr = frame.arg(1);
    let len = frame.arg(2);
    if len == 0 {
        return 0;
    }
    if user_ptr == 0 {
        return SyscallError::Fault.errno();
    }
    if len > isize::MAX as usize {
        return SyscallError::Invalid.errno();
    }

    match read_to_user_buffer(fd, user_ptr, len, memory, vfs) {
        Ok(read) => read as isize,
        Err(error) => error.errno(),
    }
}

fn sys_readv<R: UserMemoryReader + UserMemoryWriter, V: SyscallVfs>(
    frame: SyscallFrame,
    memory: &R,
    vfs: &V,
) -> isize {
    let fd = frame.arg(0);
    let iov = frame.arg(1);
    let iovcnt = frame.arg(2);
    if iovcnt == 0 {
        return 0;
    }
    if iov == 0 {
        return SyscallError::Fault.errno();
    }
    if iovcnt > IOV_MAX {
        return SyscallError::Invalid.errno();
    }

    let mut total = 0usize;
    let mut index = 0usize;
    while index < iovcnt {
        let entry_ptr = match iov.checked_add(index * IOVEC_SIZE) {
            Some(address) => address,
            None => return SyscallError::Fault.errno(),
        };
        let mut raw = [0u8; IOVEC_SIZE];
        if copy_from_user(memory, entry_ptr, &mut raw).is_err() {
            return SyscallError::Fault.errno();
        }
        let base = read_usize_le(&raw, 0);
        let len = read_usize_le(&raw, 8);
        let read = match read_to_user_buffer(fd, base, len, memory, vfs) {
            Ok(read) => read,
            Err(error) => return error.errno(),
        };
        total = match total.checked_add(read) {
            Some(value) if value <= isize::MAX as usize => value,
            _ => return SyscallError::Invalid.errno(),
        };
        if read < len {
            break;
        }
        index += 1;
    }

    total as isize
}

fn read_to_user_buffer<W: UserMemoryWriter, V: SyscallVfs>(
    fd: usize,
    user_ptr: usize,
    len: usize,
    memory: &W,
    vfs: &V,
) -> Result<usize, SyscallError> {
    if len == 0 {
        return Ok(0);
    }
    if user_ptr == 0 {
        return Err(SyscallError::Fault);
    }
    if len > isize::MAX as usize {
        return Err(SyscallError::Invalid);
    }

    let mut total = 0usize;
    let mut buffer = [0u8; READ_CHUNK];
    while total < len {
        let chunk_len = min_usize(READ_CHUNK, len - total);
        let bytes = vfs
            .read_fd(fd, &mut buffer[..chunk_len])
            .map_err(map_vfs_runtime_error)?;
        if bytes == 0 {
            break;
        }
        let target = user_ptr.checked_add(total).ok_or(SyscallError::Fault)?;
        if copy_to_user(memory, target, &buffer[..bytes]).is_err() {
            return Err(SyscallError::Fault);
        }
        vfs.lseek_fd(fd, bytes as i64, 1)
            .map_err(map_vfs_runtime_error)?;
        total += bytes;
        if bytes < chunk_len {
            break;
        }
    }

    Ok(total)
}

fn sys_pread64<W: UserMemoryWriter, V: SyscallVfs>(
    frame: SyscallFrame,
    memory: &W,
    vfs: &V,
) -> isize {
    let fd = frame.arg(0);
    let user_ptr = frame.arg(1);
    let len = frame.arg(2);
    let offset = frame.arg(3) as u64;
    if len == 0 {
        return 0;
    }
    if user_ptr == 0 {
        return SyscallError::Fault.errno();
    }
    if len > isize::MAX as usize {
        return SyscallError::Invalid.errno();
    }

    let mut total = 0usize;
    let mut buffer = [0u8; READ_CHUNK];
    while total < len {
        let chunk_len = min_usize(READ_CHUNK, len - total);
        let read = match vfs.read_fd_at(fd, offset + total as u64, &mut buffer[..chunk_len]) {
            Ok(read) => read,
            Err(error) => return map_vfs_runtime_error(error).errno(),
        };
        if read == 0 {
            break;
        }
        let target = match user_ptr.checked_add(total) {
            Some(address) => address,
            None => return SyscallError::Fault.errno(),
        };
        if copy_to_user(memory, target, &buffer[..read]).is_err() {
            return SyscallError::Fault.errno();
        }
        total += read;
        if read < chunk_len {
            break;
        }
    }

    total as isize
}

fn sys_brk<M: UserMemoryMapper>(frame: SyscallFrame, memory: &M) -> isize {
    let requested = frame.arg(0);
    let current = single_program_break();
    if current == 0 {
        return SyscallError::NoMemory.errno();
    }
    if requested == 0 {
        return current as isize;
    }

    let heap_base = single_heap_base();
    if requested < heap_base || requested >= USER_MMAP_LIMIT {
        return current as isize;
    }

    let old_page_end = match align_up(current, PAGE_SIZE) {
        Some(value) => value,
        None => return current as isize,
    };
    let new_page_end = match align_up(requested, PAGE_SIZE) {
        Some(value) => value,
        None => return current as isize,
    };
    if new_page_end > old_page_end {
        let byte_len = new_page_end - old_page_end;
        if map_zeroed_user_pages(memory, old_page_end, byte_len).is_err() {
            return current as isize;
        }
    }

    single_set_program_break(requested);
    requested as isize
}

fn sys_mmap<M: UserMemoryMapper + UserMemoryWriter, V: SyscallVfs>(
    frame: SyscallFrame,
    memory: &M,
    vfs: &V,
) -> isize {
    let requested = frame.arg(0);
    let byte_len = frame.arg(1);
    let prot = frame.arg(2);
    let flags = frame.arg(3);
    let fd = frame.arg(4);
    let offset = frame.arg(5);

    if byte_len == 0 || offset % PAGE_SIZE != 0 {
        return SyscallError::Invalid.errno();
    }
    let fixed = flags & MAP_FIXED != 0;
    if fixed && requested == 0 {
        return SyscallError::Invalid.errno();
    }
    if requested != 0 && requested % PAGE_SIZE != 0 {
        return SyscallError::Invalid.errno();
    }
    if flags & !(MAP_PRIVATE | MAP_SHARED | MAP_FIXED | MAP_ANONYMOUS | MAP_DENYWRITE | MAP_STACK)
        != 0
    {
        return SyscallError::Unsupported.errno();
    }
    let anonymous = flags & MAP_ANONYMOUS != 0;
    let file_backed = !anonymous && (flags & (MAP_PRIVATE | MAP_SHARED)) != 0;
    if !anonymous && !file_backed {
        return SyscallError::Unsupported.errno();
    }
    if anonymous && (flags & MAP_PRIVATE == 0 || fd != usize::MAX) {
        return SyscallError::BadFileDescriptor.errno();
    }
    if prot & !(PROT_READ | PROT_WRITE | PROT_EXEC) != 0 {
        return SyscallError::Invalid.errno();
    }

    let mapped_len = match align_up(byte_len, PAGE_SIZE) {
        Some(value) => value,
        None => return SyscallError::NoMemory.errno(),
    };
    let start = if fixed {
        requested
    } else {
        match align_up(single_mmap_cursor(), PAGE_SIZE) {
            Some(value) => value,
            None => return SyscallError::NoMemory.errno(),
        }
    };
    let end = match start.checked_add(mapped_len) {
        Some(value) => value,
        None => return SyscallError::NoMemory.errno(),
    };
    if end > USER_MMAP_LIMIT {
        return SyscallError::NoMemory.errno();
    }

    let writable_for_load = prot & PROT_WRITE != 0 || file_backed;
    let mapping_flags = user_mapping_flags(prot, writable_for_load);
    let map_result = if fixed {
        memory.replace_zeroed_user_pages_with_flags(start, mapped_len, mapping_flags)
    } else {
        memory.map_zeroed_user_pages_with_flags(start, mapped_len, mapping_flags)
    };
    if let Err(error) = map_result {
        return map_user_map_error(error);
    }
    if file_backed {
        let copied =
            match copy_file_mapping_to_user(memory, vfs, fd, offset as u64, start, byte_len) {
                Ok(bytes) => bytes,
                Err(error) => return error.errno(),
            };
        if copied == 0 && byte_len != 0 {
            return SyscallError::Invalid.errno();
        }
    }

    if !fixed {
        single_set_mmap_cursor(end);
    }
    start as isize
}

fn sys_munmap<M: UserMemoryMapper>(frame: SyscallFrame, memory: &M) -> isize {
    let start = frame.arg(0);
    let byte_len = frame.arg(1);
    if start % PAGE_SIZE != 0 || byte_len == 0 {
        return SyscallError::Invalid.errno();
    }
    let mapped_len = match align_up(byte_len, PAGE_SIZE) {
        Some(value) => value,
        None => return SyscallError::Invalid.errno(),
    };
    match memory.unmap_user_pages(start, mapped_len) {
        Ok(()) => 0,
        Err(error) => map_user_map_error(error),
    }
}

fn sys_mprotect<M: UserMemoryMapper>(frame: SyscallFrame, memory: &M) -> isize {
    let start = frame.arg(0);
    let byte_len = frame.arg(1);
    let prot = frame.arg(2);
    if start % PAGE_SIZE != 0 || byte_len == 0 || prot & !(PROT_READ | PROT_WRITE | PROT_EXEC) != 0
    {
        return SyscallError::Invalid.errno();
    }
    let mapped_len = match align_up(byte_len, PAGE_SIZE) {
        Some(value) => value,
        None => return SyscallError::Invalid.errno(),
    };
    let flags = user_mapping_flags(prot, prot & PROT_WRITE != 0);
    match memory.protect_user_pages_with_flags(start, mapped_len, flags) {
        Ok(()) => 0,
        Err(error) => map_user_map_error(error),
    }
}

fn user_mapping_flags(prot: usize, writable: bool) -> MappingFlags {
    MappingFlags::user(
        prot & PROT_READ != 0 || writable,
        writable,
        prot & PROT_EXEC != 0,
    )
}

fn copy_file_mapping_to_user<W: UserMemoryWriter, V: SyscallVfs>(
    memory: &W,
    vfs: &V,
    fd: usize,
    file_offset: u64,
    user_start: usize,
    byte_len: usize,
) -> Result<usize, SyscallError> {
    let mut copied = 0usize;
    let mut buffer = [0u8; READ_CHUNK];
    while copied < byte_len {
        let chunk_len = min_usize(READ_CHUNK, byte_len - copied);
        let read = vfs
            .read_fd_at(fd, file_offset + copied as u64, &mut buffer[..chunk_len])
            .map_err(map_vfs_runtime_error)?;
        if read == 0 {
            break;
        }
        let target = user_start.checked_add(copied).ok_or(SyscallError::Fault)?;
        if copy_to_user(memory, target, &buffer[..read]).is_err() {
            return Err(SyscallError::Fault);
        }
        copied += read;
        if read < chunk_len {
            break;
        }
    }
    Ok(copied)
}

fn sys_clone(frame: SyscallFrame) -> SyscallOutcome {
    let flags = frame.arg(0);
    let signal = flags & CLONE_SIGNAL_MASK;
    if flags & !SUPPORTED_FORK_CLONE_FLAGS != 0 || signal != SIGCHLD {
        return SyscallOutcome::Return(SyscallError::NotSupported.errno());
    }
    let set_child_tid = flags & CLONE_CHILD_SETTID != 0;
    let clear_child_tid = flags & CLONE_CHILD_CLEARTID != 0;
    let child_tid = frame.arg(4);
    if (set_child_tid || clear_child_tid) && child_tid == 0 {
        return SyscallOutcome::Return(SyscallError::Fault.errno());
    }

    SyscallOutcome::Fork(ForkRequest::new(
        frame.arg(1),
        child_tid,
        set_child_tid,
        clear_child_tid,
    ))
}

fn sys_wait4<W: UserMemoryWriter>(frame: SyscallFrame, memory: &W) -> isize {
    let target = frame.arg(0) as isize;
    let status_ptr = frame.arg(1);
    let options = frame.arg(2);
    if options != 0 {
        return SyscallError::NotSupported.errno();
    }

    let exit = match single_wait_for_child(target) {
        Some(exit) => exit,
        None => return SyscallError::Child.errno(),
    };
    if status_ptr != 0 {
        let status = (exit.code().value() << 8).to_le_bytes();
        if copy_to_user(memory, status_ptr, &status).is_err() {
            return SyscallError::Fault.errno();
        }
    }

    exit.pid().value() as isize
}

fn sys_prlimit64<W: UserMemoryWriter>(frame: SyscallFrame, memory: &W, current_pid: Pid) -> isize {
    let target_pid = frame.arg(0);
    let resource = frame.arg(1);
    let new_limit = frame.arg(2);
    let old_limit = frame.arg(3);

    let kind = match linux_resource_limit_kind(resource) {
        Some(kind) => kind,
        None => return SyscallError::Invalid.errno(),
    };
    if target_pid != 0 && target_pid != current_pid.value() {
        return SyscallError::NoProcess.errno();
    }
    if new_limit != 0 {
        return SyscallError::NotSupported.errno();
    }
    if old_limit == 0 {
        return 0;
    }

    let limit = process_resource_limit(kind);
    let mut raw = [0u8; LINUX_RLIMIT64_SIZE];
    raw[..8].copy_from_slice(&limit.current().to_le_bytes());
    raw[8..].copy_from_slice(&limit.maximum().to_le_bytes());
    if copy_to_user(memory, old_limit, &raw).is_err() {
        SyscallError::Fault.errno()
    } else {
        0
    }
}

const fn linux_resource_limit_kind(resource: usize) -> Option<ResourceLimitKind> {
    match resource {
        RLIMIT_STACK => Some(ResourceLimitKind::Stack),
        _ => None,
    }
}

fn sys_execve<R: UserMemoryReader>(frame: SyscallFrame, memory: &R) -> SyscallOutcome {
    let path_ptr = frame.arg(0);
    let argv_ptr = frame.arg(1);
    if path_ptr == 0 {
        return SyscallOutcome::Return(SyscallError::Fault.errno());
    }

    let mut request = ExecRequest {
        path: [0; SYSCALL_PATH_MAX],
        path_len: 0,
        args: [[0; EXEC_ARG_MAX]; EXEC_ARG_COUNT],
        arg_lens: [0; EXEC_ARG_COUNT],
        arg_count: 0,
    };
    request.path_len = match copy_user_c_string(memory, path_ptr, &mut request.path) {
        Ok(len) => len,
        Err(error) => return SyscallOutcome::Return(error.errno()),
    };
    if request.path_len == 0 {
        return SyscallOutcome::Return(SyscallError::NoEntry.errno());
    }

    if argv_ptr != 0 {
        let mut index = 0usize;
        while index < EXEC_ARG_COUNT {
            let entry_ptr = match argv_ptr.checked_add(index * core::mem::size_of::<usize>()) {
                Some(address) => address,
                None => return SyscallOutcome::Return(SyscallError::Fault.errno()),
            };
            let arg_ptr = match read_user_usize(memory, entry_ptr) {
                Ok(value) => value,
                Err(error) => return SyscallOutcome::Return(error.errno()),
            };
            if arg_ptr == 0 {
                break;
            }
            request.arg_lens[index] =
                match copy_user_c_string(memory, arg_ptr, &mut request.args[index]) {
                    Ok(len) => len,
                    Err(error) => return SyscallOutcome::Return(error.errno()),
                };
            request.arg_count += 1;
            index += 1;
        }
    }

    if request.arg_count == 0 {
        let copy_len = min_usize(request.path_len, EXEC_ARG_MAX);
        request.args[0][..copy_len].copy_from_slice(&request.path[..copy_len]);
        request.arg_lens[0] = copy_len;
        request.arg_count = 1;
    }

    SyscallOutcome::Exec(request)
}

fn sys_write<R: UserMemoryReader, V: SyscallVfs>(
    frame: SyscallFrame,
    memory: &R,
    vfs: &V,
) -> isize {
    let fd = frame.arg(0);
    let user_ptr = frame.arg(1);
    let len = frame.arg(2);
    match write_user_buffer(fd, user_ptr, len, memory, vfs) {
        Ok(written) => written as isize,
        Err(error) => error.errno(),
    }
}

fn sys_writev<R: UserMemoryReader, V: SyscallVfs>(
    frame: SyscallFrame,
    memory: &R,
    vfs: &V,
) -> isize {
    let fd = frame.arg(0);
    let iov = frame.arg(1);
    let iovcnt = frame.arg(2);
    if iovcnt == 0 {
        return 0;
    }
    if iov == 0 {
        return SyscallError::Fault.errno();
    }
    if iovcnt > IOV_MAX {
        return SyscallError::Invalid.errno();
    }

    let mut total = 0usize;
    let mut index = 0usize;
    while index < iovcnt {
        let entry_ptr = match iov.checked_add(index * IOVEC_SIZE) {
            Some(address) => address,
            None => return SyscallError::Fault.errno(),
        };
        let mut raw = [0u8; IOVEC_SIZE];
        if copy_from_user(memory, entry_ptr, &mut raw).is_err() {
            return SyscallError::Fault.errno();
        }
        let base = read_usize_le(&raw, 0);
        let len = read_usize_le(&raw, 8);
        let written = match write_user_buffer(fd, base, len, memory, vfs) {
            Ok(written) => written,
            Err(error) => return error.errno(),
        };
        total = match total.checked_add(written) {
            Some(value) if value <= isize::MAX as usize => value,
            _ => return SyscallError::Invalid.errno(),
        };
        if written < len {
            break;
        }
        index += 1;
    }

    total as isize
}

fn write_user_buffer<R: UserMemoryReader, V: SyscallVfs>(
    fd: usize,
    user_ptr: usize,
    len: usize,
    memory: &R,
    vfs: &V,
) -> Result<usize, SyscallError> {
    if len == 0 {
        return Ok(0);
    }
    if len > isize::MAX as usize {
        return Err(SyscallError::Invalid);
    }

    let mut written = 0usize;
    let mut buffer = [0u8; WRITE_CHUNK];
    while written < len {
        let chunk_len = min_usize(WRITE_CHUNK, len - written);
        let chunk_ptr = match user_ptr.checked_add(written) {
            Some(address) => address,
            None => return Err(SyscallError::Fault),
        };
        if copy_from_user(memory, chunk_ptr, &mut buffer[..chunk_len]).is_err() {
            return Err(SyscallError::Fault);
        }

        let sink_written = if is_stdio_fd(fd) && !vfs.owns_stdio_fds() {
            match write_user_fd(fd, &buffer[..chunk_len]) {
                Ok(count) => count,
                Err(UserOutputError::UnsupportedFd) => return Err(SyscallError::BadFileDescriptor),
                Err(UserOutputError::SinkMissing) => return Err(SyscallError::Io),
            }
        } else {
            match vfs.write_fd(fd, &buffer[..chunk_len]) {
                Ok(count) => count,
                Err(error) => return Err(map_vfs_runtime_error(error)),
            }
        };
        written += sink_written;
        if sink_written < chunk_len {
            break;
        }
    }

    Ok(written)
}

fn sys_dup<V: SyscallVfs>(frame: SyscallFrame, vfs: &V) -> isize {
    match vfs.duplicate_fd_min(frame.arg(0), 0, false) {
        Ok(fd) => fd as isize,
        Err(error) => map_fd_error(error).errno(),
    }
}

fn sys_dup3<V: SyscallVfs>(frame: SyscallFrame, vfs: &V) -> isize {
    let old_fd = frame.arg(0);
    let new_fd = frame.arg(1);
    let flags = frame.arg(2);
    if flags != 0 {
        return SyscallError::Invalid.errno();
    }
    if old_fd == new_fd {
        return match vfs.fd_close_on_exec(old_fd) {
            Ok(_) => old_fd as isize,
            Err(error) => map_fd_error(error).errno(),
        };
    }
    match vfs.duplicate_fd_to(old_fd, new_fd, false) {
        Ok(fd) => fd as isize,
        Err(error) => map_fd_error(error).errno(),
    }
}

fn sys_ioctl<W: UserMemoryWriter, V: SyscallVfs>(
    frame: SyscallFrame,
    memory: &W,
    vfs: &V,
) -> isize {
    let fd = frame.arg(0);
    let request = frame.arg(1);
    if is_stdio_fd(fd) && request == TIOCGWINSZ {
        SyscallError::NotTerminal.errno()
    } else {
        match vfs.ioctl_fd(fd, request) {
            Ok(VfsIoctl::RtcReadTime) => write_rtc_time(memory, frame.arg(2)),
            Err(error) => map_vfs_runtime_error(error).errno(),
        }
    }
}

fn write_rtc_time<W: UserMemoryWriter>(memory: &W, address: usize) -> isize {
    if address == 0 {
        return SyscallError::Fault.errno();
    }
    let mut raw = [0u8; 36];
    put_i32(&mut raw, 0, 0);
    put_i32(&mut raw, 4, 0);
    put_i32(&mut raw, 8, 0);
    put_i32(&mut raw, 12, 1);
    put_i32(&mut raw, 16, 0);
    put_i32(&mut raw, 20, 70);
    put_i32(&mut raw, 24, 4);
    put_i32(&mut raw, 28, 0);
    put_i32(&mut raw, 32, 0);
    if copy_to_user(memory, address, &raw).is_err() {
        SyscallError::Fault.errno()
    } else {
        0
    }
}

const fn is_stdio_fd(fd: usize) -> bool {
    fd == STDOUT_FD || fd == STDERR_FD
}

const fn min_usize(lhs: usize, rhs: usize) -> usize {
    if lhs < rhs {
        lhs
    } else {
        rhs
    }
}

const fn align_up_usize(value: usize, align: usize) -> usize {
    let remainder = value % align;
    if remainder == 0 {
        value
    } else {
        value + (align - remainder)
    }
}

fn copy_user_c_string<R: UserMemoryReader>(
    memory: &R,
    address: usize,
    out: &mut [u8],
) -> Result<usize, SyscallError> {
    let mut index = 0usize;
    while index < out.len() {
        let user_byte = match address.checked_add(index) {
            Some(value) => value,
            None => return Err(SyscallError::Fault),
        };
        let mut byte = [0u8; 1];
        if copy_from_user(memory, user_byte, &mut byte).is_err() {
            return Err(SyscallError::Fault);
        }
        if byte[0] == 0 {
            return Ok(index);
        }
        out[index] = byte[0];
        index += 1;
    }

    Err(SyscallError::NameTooLong)
}

fn read_usize_le(bytes: &[u8; IOVEC_SIZE], offset: usize) -> usize {
    let mut raw = [0u8; core::mem::size_of::<usize>()];
    raw.copy_from_slice(&bytes[offset..offset + core::mem::size_of::<usize>()]);
    usize::from_le_bytes(raw)
}

fn read_user_usize<R: UserMemoryReader>(memory: &R, address: usize) -> Result<usize, SyscallError> {
    let mut raw = [0u8; core::mem::size_of::<usize>()];
    if copy_from_user(memory, address, &mut raw).is_err() {
        return Err(SyscallError::Fault);
    }
    Ok(usize::from_le_bytes(raw))
}

fn write_stat_to_user<W: UserMemoryWriter>(memory: &W, address: usize, stat: VfsStat) -> isize {
    let mut bytes = [0u8; LINUX_STAT_SIZE];
    if let Err(error) = encode_linux_stat(stat, &mut bytes) {
        return map_stat_encoding_error(error).errno();
    }
    if copy_to_user(memory, address, &bytes).is_err() {
        SyscallError::Fault.errno()
    } else {
        0
    }
}

fn write_statfs_to_user<W: UserMemoryWriter>(memory: &W, address: usize, stat: VfsStat) -> isize {
    let mut bytes = [0u8; LINUX_STATFS_SIZE];
    let blocks = if stat.kind().is_directory() {
        262_144
    } else if stat.blocks() == 0 {
        1
    } else {
        stat.blocks()
    };
    let free = blocks / 2;
    put_u64(&mut bytes, 0, 0xEF53);
    put_u64(&mut bytes, 8, stat.block_size() as u64);
    put_u64(&mut bytes, 16, blocks);
    put_u64(&mut bytes, 24, free);
    put_u64(&mut bytes, 32, free);
    put_u64(&mut bytes, 40, 1024);
    put_u64(&mut bytes, 48, 512);
    put_u64(&mut bytes, 64, VFS_NAME_MAX as u64);
    put_u64(&mut bytes, 72, stat.block_size() as u64);
    if copy_to_user(memory, address, &bytes).is_err() {
        SyscallError::Fault.errno()
    } else {
        0
    }
}

fn encode_linux_dirent64(
    entry: VfsDirEntry,
    out: &mut [u8; LINUX_DIRENT64_MAX_SIZE],
) -> Result<usize, SyscallError> {
    let name = entry.name();
    let record_len = linux_dirent64_record_len(name.len());
    if record_len > out.len()
        || record_len > u16::MAX as usize
        || entry.next_offset() > i64::MAX as u64
    {
        return Err(SyscallError::Invalid);
    }

    out[..record_len].fill(0);
    put_u64(out, 0, entry.inode() as u64);
    put_i64(out, 8, entry.next_offset() as i64);
    put_u16(out, 16, record_len as u16);
    out[18] = linux_dirent_type(entry.kind());
    out[LINUX_DIRENT64_HEADER_SIZE..LINUX_DIRENT64_HEADER_SIZE + name.len()].copy_from_slice(name);
    Ok(record_len)
}

fn linux_dirent64_record_len(name_len: usize) -> usize {
    align_up_usize(
        LINUX_DIRENT64_HEADER_SIZE + name_len + 1,
        LINUX_DIRENT64_ALIGN,
    )
}

fn linux_dirent_type(kind: VfsDirEntryKind) -> u8 {
    match kind {
        VfsDirEntryKind::Unknown => 0,
        VfsDirEntryKind::Fifo => 1,
        VfsDirEntryKind::CharacterDevice => 2,
        VfsDirEntryKind::Directory => 4,
        VfsDirEntryKind::BlockDevice => 6,
        VfsDirEntryKind::RegularFile => 8,
        VfsDirEntryKind::Symlink => 10,
        VfsDirEntryKind::Socket => 12,
    }
}

fn put_u64(out: &mut [u8], offset: usize, value: u64) {
    out[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

fn put_i64(out: &mut [u8], offset: usize, value: i64) {
    out[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

fn put_i32(out: &mut [u8], offset: usize, value: i32) {
    out[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn put_u16(out: &mut [u8], offset: usize, value: u16) {
    out[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

fn store_packed_trace_byte(slots: &[AtomicUsize], offset: usize, byte: u8) {
    let width = core::mem::size_of::<usize>();
    let word_offset = offset / width;
    if word_offset >= slots.len() {
        return;
    }
    let byte_offset = offset % width;
    slots[word_offset].fetch_or((byte as usize) << (byte_offset * 8), Ordering::Relaxed);
}

fn align_up(value: usize, align: usize) -> Option<usize> {
    if value % align == 0 {
        Some(value)
    } else {
        value
            .checked_div(align)?
            .checked_mul(align)?
            .checked_add(align)
    }
}

const fn map_user_map_error(error: UserMapError) -> isize {
    match error {
        UserMapError::FrameExhausted => SyscallError::NoMemory.errno(),
        UserMapError::AddressOverflow
        | UserMapError::AlreadyMapped
        | UserMapError::InvalidRange
        | UserMapError::PermissionDenied => SyscallError::Invalid.errno(),
        UserMapError::NotReady => SyscallError::NoDevice.errno(),
        UserMapError::Unsupported => SyscallError::Unsupported.errno(),
    }
}

const fn map_open_options_error(error: OpenOptionsError) -> SyscallError {
    match error {
        OpenOptionsError::ReadOnlyFilesystem => SyscallError::ReadOnlyFilesystem,
        OpenOptionsError::UnsupportedFlags => SyscallError::NotSupported,
    }
}

const fn map_fd_error(error: FdError) -> SyscallError {
    match error {
        FdError::BadFileDescriptor => SyscallError::BadFileDescriptor,
        FdError::InvalidOffset => SyscallError::Invalid,
        FdError::NotDirectory => SyscallError::NotDirectory,
        FdError::NotRegularFile => SyscallError::IsDirectory,
        FdError::TableFull => SyscallError::TooManyOpenFiles,
    }
}

const fn map_vfs_runtime_error(error: VfsRuntimeError) -> SyscallError {
    match error {
        VfsRuntimeError::Fd(error) => map_fd_error(error),
        VfsRuntimeError::Vfs(error) => map_vfs_error(error),
    }
}

const fn map_stat_encoding_error(error: StatEncodingError) -> SyscallError {
    match error {
        StatEncodingError::ValueOutOfRange => SyscallError::Invalid,
    }
}

const fn map_vfs_error(error: VfsError) -> SyscallError {
    match error {
        VfsError::AlreadyExists => SyscallError::Exists,
        VfsError::PathNotFound => SyscallError::NoEntry,
        VfsError::InvalidPath => SyscallError::Invalid,
        VfsError::UnsupportedPath | VfsError::UnsupportedRootfs => SyscallError::NotSupported,
        VfsError::DirectoryExpected => SyscallError::NotDirectory,
        VfsError::Block(_) | VfsError::MetadataCorrupt | VfsError::RootfsSourceMissing => {
            SyscallError::Io
        }
        VfsError::NoSpace => SyscallError::NoSpace,
        VfsError::EmptyFile | VfsError::FileTooLarge | VfsError::NotExecutable => {
            SyscallError::Invalid
        }
        VfsError::PermissionDenied => SyscallError::PermissionDenied,
        VfsError::NotRegularFile => SyscallError::IsDirectory,
    }
}

fn sys_getppid(process: &Process) -> isize {
    match process.parent_pid() {
        Some(pid) => pid.value() as isize,
        None => 0,
    }
}

fn sys_single_getppid() -> isize {
    match single_parent_pid() {
        Some(pid) => pid.value() as isize,
        None => 0,
    }
}

fn sys_set_robust_list_process(frame: SyscallFrame, process: &mut Process) -> isize {
    let (head, len) = match validate_robust_list(frame) {
        Ok(values) => values,
        Err(error) => return error.errno(),
    };
    process.set_robust_list(head, len);
    0
}

fn sys_set_robust_list_single(frame: SyscallFrame) -> isize {
    let (head, len) = match validate_robust_list(frame) {
        Ok(values) => values,
        Err(error) => return error.errno(),
    };
    single_set_robust_list(head, len);
    0
}

fn validate_robust_list(frame: SyscallFrame) -> Result<(usize, usize), SyscallError> {
    let head = frame.arg(0);
    let len = frame.arg(1);
    if len != ROBUST_LIST_HEAD_SIZE {
        return Err(SyscallError::Invalid);
    }
    Ok((head, len))
}

const UNAME_FIELD_SIZE: usize = 65;
const UNAME_STRUCT_SIZE: usize = 6 * UNAME_FIELD_SIZE;

fn sys_uname<W: UserMemoryWriter>(frame: SyscallFrame, memory: &W) -> isize {
    let buf = frame.arg(0);
    if buf == 0 {
        return SyscallError::Fault.errno();
    }
    let mut utsname = [0u8; UNAME_STRUCT_SIZE];
    write_utsname_field(&mut utsname, 0 * UNAME_FIELD_SIZE, b"Linux");
    write_utsname_field(&mut utsname, 1 * UNAME_FIELD_SIZE, b"localhost");
    write_utsname_field(&mut utsname, 2 * UNAME_FIELD_SIZE, b"6.6.0");
    write_utsname_field(&mut utsname, 3 * UNAME_FIELD_SIZE, b"#1 SMP");
    write_utsname_field(&mut utsname, 4 * UNAME_FIELD_SIZE, b"riscv64");
    write_utsname_field(&mut utsname, 5 * UNAME_FIELD_SIZE, b"(none)");
    if copy_to_user(memory, buf, &utsname).is_err() {
        SyscallError::Fault.errno()
    } else {
        0
    }
}

fn write_utsname_field(buf: &mut [u8], offset: usize, value: &[u8]) {
    let limit = min_usize(value.len(), UNAME_FIELD_SIZE - 1);
    buf[offset..offset + limit].copy_from_slice(&value[..limit]);
}

fn sys_getcwd<W: UserMemoryWriter, V: SyscallVfs>(
    frame: SyscallFrame,
    memory: &W,
    vfs: &V,
) -> isize {
    let buf = frame.arg(0);
    let size = frame.arg(1);
    if buf == 0 {
        return SyscallError::Fault.errno();
    }
    if size == 0 {
        return SyscallError::Invalid.errno();
    }

    let mut path = [0u8; SYSCALL_PATH_MAX];
    let len = match vfs.getcwd(&mut path[..min_usize(size, SYSCALL_PATH_MAX)]) {
        Ok(len) => len,
        Err(error) => return map_vfs_runtime_error(error).errno(),
    };
    if len > size {
        return SyscallError::Invalid.errno();
    }
    if copy_to_user(memory, buf, &path[..len]).is_err() {
        return SyscallError::Fault.errno();
    }
    len as isize
}
