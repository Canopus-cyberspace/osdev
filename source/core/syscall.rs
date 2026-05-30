use core::sync::atomic::{AtomicUsize, Ordering};

use crate::core::fs::{
    encode_linux_stat, FdError, NoSyscallVfs, OpenOptions, OpenOptionsError, StatEncodingError,
    SyscallVfs, VfsDirEntry, VfsDirEntryKind, VfsError, VfsPath, VfsRuntimeError, VfsStat,
    LINUX_STAT_SIZE,
};
use crate::core::mm::{
    copy_from_user, copy_to_user, map_zeroed_user_pages, NoUserMemory, UserMapError,
    UserMemoryMapper, UserMemoryReader, UserMemoryWriter, PAGE_SIZE,
};
use crate::core::task::{
    single_heap_base, single_mmap_cursor, single_pid, single_program_break, single_record_exit,
    single_set_mmap_cursor, single_set_program_break, single_set_tid_address, ExitCode, ExitState,
    Process,
};
use crate::official::user_output::{write_user_fd, UserOutputError};

pub const SYS_DUP3: usize = 24;
pub const SYS_FCNTL: usize = 25;
pub const SYS_IOCTL: usize = 29;
pub const SYS_OPENAT: usize = 56;
pub const SYS_CLOSE: usize = 57;
pub const SYS_GETDENTS64: usize = 61;
pub const SYS_LSEEK: usize = 62;
pub const SYS_READ: usize = 63;
pub const SYS_WRITE: usize = 64;
pub const SYS_WRITEV: usize = 66;
pub const SYS_NEWFSTATAT: usize = 79;
pub const SYS_FSTAT: usize = 80;
pub const SYS_EXIT: usize = 93;
pub const SYS_EXIT_GROUP: usize = 94;
pub const SYS_SET_TID_ADDRESS: usize = 96;
pub const SYS_CLOCK_GETTIME: usize = 113;
pub const SYS_GETTIMEOFDAY: usize = 169;
pub const SYS_GETPID: usize = 172;
pub const SYS_GETUID: usize = 174;
pub const SYS_BRK: usize = 214;
pub const SYS_MMAP: usize = 222;

const TRACE_CAPACITY: usize = 32;
const SYSCALL_PATH_MAX: usize = 256;
const LINUX_DIRENT64_HEADER_SIZE: usize = 19;
const LINUX_DIRENT64_MAX_SIZE: usize = 280;
const LINUX_DIRENT64_ALIGN: usize = 8;
const WRITE_CHUNK: usize = 256;
const READ_CHUNK: usize = 256;
const IOVEC_SIZE: usize = 16;
const IOV_MAX: usize = 16;
const STDOUT_FD: usize = 1;
const STDERR_FD: usize = 2;
const AT_FDCWD: isize = -100;
const AT_SYMLINK_NOFOLLOW: usize = 0x100;
const TIOCGWINSZ: usize = 0x5413;
const F_SETFD: usize = 2;
const FD_CLOEXEC: usize = 1;
const USER_MMAP_LIMIT: usize = (1usize << 37) - PAGE_SIZE;
const PROT_READ: usize = 0x1;
const PROT_WRITE: usize = 0x2;
const PROT_EXEC: usize = 0x4;
const MAP_PRIVATE: usize = 0x02;
const MAP_FIXED: usize = 0x10;
const MAP_ANONYMOUS: usize = 0x20;
const TRACE_STATUS_HANDLED: usize = 1;
const TRACE_STATUS_ENOSYS: usize = 2;
const TRACE_STATUS_EXIT: usize = 3;
const TRACE_STATUS_ERRNO: usize = 4;

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
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SyscallError {
    BadFileDescriptor,
    Fault,
    Invalid,
    Io,
    IsDirectory,
    NameTooLong,
    NoDevice,
    NoEntry,
    NoMemory,
    NotDirectory,
    NotTerminal,
    NotSupported,
    ReadOnlyFilesystem,
    TooManyOpenFiles,
    Unsupported,
}

impl SyscallError {
    pub const fn errno(self) -> isize {
        match self {
            Self::BadFileDescriptor => -9,
            Self::Fault => -14,
            Self::Invalid => -22,
            Self::Io => -5,
            Self::IsDirectory => -21,
            Self::NameTooLong => -36,
            Self::NoDevice => -19,
            Self::NoEntry => -2,
            Self::NoMemory => -12,
            Self::NotDirectory => -20,
            Self::NotTerminal => -25,
            Self::NotSupported => -95,
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
        SYS_GETUID => SyscallOutcome::Return(0),
        SYS_SET_TID_ADDRESS => {
            process.set_tid_address(frame.arg(0));
            SyscallOutcome::Return(process.pid().value() as isize)
        }
        SYS_DUP3 => SyscallOutcome::Return(sys_dup3(frame)),
        SYS_FCNTL => SyscallOutcome::Return(sys_fcntl(frame, vfs)),
        SYS_IOCTL => SyscallOutcome::Return(sys_ioctl(frame)),
        SYS_EXIT | SYS_EXIT_GROUP => {
            record_exit_code(frame.arg(0));
            let state = process.exit(ExitCode::new(frame.arg(0) as i32));
            SyscallOutcome::Exit(state)
        }
        SYS_WRITE => SyscallOutcome::Return(sys_write(frame, memory)),
        SYS_WRITEV => SyscallOutcome::Return(sys_writev(frame, memory)),
        SYS_READ => SyscallOutcome::Return(sys_read(frame, memory, vfs)),
        SYS_OPENAT => SyscallOutcome::Return(sys_openat(frame, memory, vfs)),
        SYS_GETDENTS64 => SyscallOutcome::Return(sys_getdents64(frame, memory, vfs)),
        SYS_CLOSE => SyscallOutcome::Return(sys_close(frame, vfs)),
        SYS_FSTAT => SyscallOutcome::Return(sys_fstat(frame, memory, vfs)),
        SYS_LSEEK => SyscallOutcome::Return(sys_lseek(frame, vfs)),
        SYS_BRK => SyscallOutcome::Return(sys_brk(frame, memory)),
        SYS_MMAP => SyscallOutcome::Return(sys_mmap(frame, memory)),
        SYS_CLOCK_GETTIME => SyscallOutcome::Return(sys_clock_gettime(frame, memory)),
        SYS_GETTIMEOFDAY => SyscallOutcome::Return(sys_gettimeofday(frame, memory)),
        SYS_NEWFSTATAT => SyscallOutcome::Return(sys_newfstatat(frame, memory, vfs)),
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
        SYS_GETUID => SyscallOutcome::Return(0),
        SYS_SET_TID_ADDRESS => {
            single_set_tid_address(frame.arg(0));
            SyscallOutcome::Return(single_pid().value() as isize)
        }
        SYS_DUP3 => SyscallOutcome::Return(sys_dup3(frame)),
        SYS_FCNTL => SyscallOutcome::Return(sys_fcntl(frame, vfs)),
        SYS_IOCTL => SyscallOutcome::Return(sys_ioctl(frame)),
        SYS_EXIT | SYS_EXIT_GROUP => {
            record_exit_code(frame.arg(0));
            let state = ExitState::new(single_pid(), ExitCode::new(frame.arg(0) as i32));
            single_record_exit(state);
            SyscallOutcome::Exit(state)
        }
        SYS_WRITE => SyscallOutcome::Return(sys_write(frame, memory)),
        SYS_WRITEV => SyscallOutcome::Return(sys_writev(frame, memory)),
        SYS_READ => SyscallOutcome::Return(sys_read(frame, memory, vfs)),
        SYS_OPENAT => SyscallOutcome::Return(sys_openat(frame, memory, vfs)),
        SYS_GETDENTS64 => SyscallOutcome::Return(sys_getdents64(frame, memory, vfs)),
        SYS_CLOSE => SyscallOutcome::Return(sys_close(frame, vfs)),
        SYS_FSTAT => SyscallOutcome::Return(sys_fstat(frame, memory, vfs)),
        SYS_LSEEK => SyscallOutcome::Return(sys_lseek(frame, vfs)),
        SYS_BRK => SyscallOutcome::Return(sys_brk(frame, memory)),
        SYS_MMAP => SyscallOutcome::Return(sys_mmap(frame, memory)),
        SYS_CLOCK_GETTIME => SyscallOutcome::Return(sys_clock_gettime(frame, memory)),
        SYS_GETTIMEOFDAY => SyscallOutcome::Return(sys_gettimeofday(frame, memory)),
        SYS_NEWFSTATAT => SyscallOutcome::Return(sys_newfstatat(frame, memory, vfs)),
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

    write_zero_pair(memory, timespec)
}

fn sys_gettimeofday<W: UserMemoryWriter>(frame: SyscallFrame, memory: &W) -> isize {
    let timeval = frame.arg(0);
    let timezone = frame.arg(1);
    if timeval != 0 {
        let written = write_zero_pair(memory, timeval);
        if written != 0 {
            return written;
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

fn sys_newfstatat<R: UserMemoryReader + UserMemoryWriter, V: SyscallVfs>(
    frame: SyscallFrame,
    memory: &R,
    vfs: &V,
) -> isize {
    let dirfd = frame.arg(0) as isize;
    let path_ptr = frame.arg(1);
    let statbuf = frame.arg(2);
    let flags = frame.arg(3);

    if flags & !AT_SYMLINK_NOFOLLOW != 0 {
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
        return SyscallError::NoEntry.errno();
    }

    let path_slice = &path_bytes[..path_len];
    record_newfstatat_path(path_slice);
    if path_slice[0] != b'/' {
        if dirfd != AT_FDCWD {
            return SyscallError::BadFileDescriptor.errno();
        }
        return SyscallError::NotSupported.errno();
    }

    let path = match VfsPath::new(path_slice) {
        Ok(path) => path,
        Err(error) => return map_vfs_error(error).errno(),
    };
    let stat = match vfs.stat_path(path) {
        Ok(stat) => stat,
        Err(error) => return map_vfs_error(error).errno(),
    };
    record_newfstatat_stat(stat);

    write_stat_to_user(memory, statbuf, stat)
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

    let path_slice = &path_bytes[..path_len];
    if path_slice[0] != b'/' {
        let dirfd = frame.arg(0) as isize;
        if dirfd != AT_FDCWD {
            return SyscallError::BadFileDescriptor.errno();
        }
        return SyscallError::NotSupported.errno();
    }
    let path = match VfsPath::new(path_slice) {
        Ok(path) => path,
        Err(error) => return map_vfs_error(error).errno(),
    };

    match vfs.open_path(path, options) {
        Ok(fd) => fd as isize,
        Err(error) => map_vfs_runtime_error(error).errno(),
    }
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

    let mut total = 0usize;
    let mut buffer = [0u8; READ_CHUNK];
    while total < len {
        let chunk_len = min_usize(READ_CHUNK, len - total);
        let bytes = match vfs.read_fd(fd, &mut buffer[..chunk_len]) {
            Ok(bytes) => bytes,
            Err(error) => return map_vfs_runtime_error(error).errno(),
        };
        if bytes == 0 {
            break;
        }
        let target = match user_ptr.checked_add(total) {
            Some(address) => address,
            None => return SyscallError::Fault.errno(),
        };
        if copy_to_user(memory, target, &buffer[..bytes]).is_err() {
            return SyscallError::Fault.errno();
        }
        match vfs.lseek_fd(fd, bytes as i64, 1) {
            Ok(_) => {}
            Err(error) => return map_vfs_runtime_error(error).errno(),
        }
        total += bytes;
        if bytes < chunk_len {
            break;
        }
    }

    total as isize
}

fn write_zero_pair<W: UserMemoryWriter>(memory: &W, address: usize) -> isize {
    let zeros = [0u8; 16];
    if copy_to_user(memory, address, &zeros).is_err() {
        SyscallError::Fault.errno()
    } else {
        0
    }
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

fn sys_mmap<M: UserMemoryMapper>(frame: SyscallFrame, memory: &M) -> isize {
    let requested = frame.arg(0);
    let byte_len = frame.arg(1);
    let prot = frame.arg(2);
    let flags = frame.arg(3);
    let fd = frame.arg(4);
    let offset = frame.arg(5);

    if requested != 0 || byte_len == 0 || offset != 0 {
        return SyscallError::Invalid.errno();
    }
    if flags & MAP_FIXED != 0 {
        return SyscallError::Invalid.errno();
    }
    if flags & MAP_ANONYMOUS == 0 || flags & MAP_PRIVATE == 0 {
        return SyscallError::Unsupported.errno();
    }
    if fd != usize::MAX {
        return SyscallError::BadFileDescriptor.errno();
    }
    if prot & !(PROT_READ | PROT_WRITE | PROT_EXEC) != 0 || prot & PROT_READ == 0 {
        return SyscallError::Invalid.errno();
    }
    if prot & PROT_EXEC != 0 {
        return SyscallError::Unsupported.errno();
    }

    let mapped_len = match align_up(byte_len, PAGE_SIZE) {
        Some(value) => value,
        None => return SyscallError::NoMemory.errno(),
    };
    let start = match align_up(single_mmap_cursor(), PAGE_SIZE) {
        Some(value) => value,
        None => return SyscallError::NoMemory.errno(),
    };
    let end = match start.checked_add(mapped_len) {
        Some(value) => value,
        None => return SyscallError::NoMemory.errno(),
    };
    if end > USER_MMAP_LIMIT {
        return SyscallError::NoMemory.errno();
    }

    match map_zeroed_user_pages(memory, start, mapped_len) {
        Ok(()) => {
            single_set_mmap_cursor(end);
            start as isize
        }
        Err(error) => map_user_map_error(error),
    }
}

fn sys_write<R: UserMemoryReader>(frame: SyscallFrame, memory: &R) -> isize {
    let fd = frame.arg(0);
    let user_ptr = frame.arg(1);
    let len = frame.arg(2);
    match write_user_buffer(fd, user_ptr, len, memory) {
        Ok(written) => written as isize,
        Err(error) => error.errno(),
    }
}

fn sys_writev<R: UserMemoryReader>(frame: SyscallFrame, memory: &R) -> isize {
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
        let written = match write_user_buffer(fd, base, len, memory) {
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

fn write_user_buffer<R: UserMemoryReader>(
    fd: usize,
    user_ptr: usize,
    len: usize,
    memory: &R,
) -> Result<usize, SyscallError> {
    if !is_stdio_fd(fd) {
        return Err(SyscallError::BadFileDescriptor);
    }
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

        let sink_written = match write_user_fd(fd, &buffer[..chunk_len]) {
            Ok(count) => count,
            Err(UserOutputError::UnsupportedFd) => return Err(SyscallError::BadFileDescriptor),
            Err(UserOutputError::SinkMissing) => return Err(SyscallError::Io),
        };
        written += sink_written;
        if sink_written < chunk_len {
            break;
        }
    }

    Ok(written)
}

fn sys_dup3(frame: SyscallFrame) -> isize {
    let old_fd = frame.arg(0);
    let new_fd = frame.arg(1);
    let flags = frame.arg(2);
    if old_fd == new_fd || flags != 0 {
        return SyscallError::Invalid.errno();
    }
    if is_stdio_fd(old_fd) && is_stdio_fd(new_fd) {
        new_fd as isize
    } else {
        SyscallError::BadFileDescriptor.errno()
    }
}

fn sys_ioctl(frame: SyscallFrame) -> isize {
    let fd = frame.arg(0);
    let request = frame.arg(1);
    if !is_stdio_fd(fd) {
        return SyscallError::BadFileDescriptor.errno();
    }
    if request == TIOCGWINSZ {
        SyscallError::NotTerminal.errno()
    } else {
        SyscallError::Invalid.errno()
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
        VfsError::PathNotFound => SyscallError::NoEntry,
        VfsError::InvalidPath => SyscallError::Invalid,
        VfsError::UnsupportedPath | VfsError::UnsupportedRootfs => SyscallError::NotSupported,
        VfsError::DirectoryExpected => SyscallError::NotDirectory,
        VfsError::Block(_) | VfsError::MetadataCorrupt | VfsError::RootfsSourceMissing => {
            SyscallError::Io
        }
        VfsError::EmptyFile | VfsError::FileTooLarge | VfsError::NotExecutable => {
            SyscallError::Invalid
        }
        VfsError::NotRegularFile => SyscallError::IsDirectory,
    }
}
