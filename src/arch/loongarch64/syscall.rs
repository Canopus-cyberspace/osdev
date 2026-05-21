use crate::console::{write_bytes, write_usize_dec, write_usize_hex};
use crate::early_console_write;
use crate::fd_table::{
    self, LaFd, PathBuf, AT_FDCWD, FD_CLOSED, FD_CONSOLE, FD_DIRECTORY, FD_IO_BUFFER_SIZE,
    FD_PIPE_READ, FD_PIPE_WRITE, FD_REGULAR, FD_STDIN, MAX_FDS, O_CREAT, O_DIRECTORY,
    PATH_BUF_SIZE, S_IFDIR, S_IFREG,
};
use crate::process;
use crate::real_elf;
use crate::sdcard_ext4;
use crate::trap::{self, LoongArchTrapFrame};
use crate::user;
use crate::user_mem;

#[path = "../../syscall/numbers.rs"]
mod syscall_numbers;

const ENOENT: isize = -2;
const EBADF: isize = -9;
const E2BIG: isize = -7;
const EFAULT: isize = -14;
const EINVAL: isize = -22;
const ENOTTY: isize = -25;
const ENOSYS: isize = -38;
const ENAMETOOLONG: isize = -36;
const DT_DIR: u8 = 4;
const DT_REG: u8 = 8;

static mut TIME_TICK: usize = 1;

struct LoongArchSyscallAbi<'a> {
    frame: &'a mut LoongArchTrapFrame,
}

impl<'a> LoongArchSyscallAbi<'a> {
    fn new(frame: &'a mut LoongArchTrapFrame) -> Self {
        Self { frame }
    }

    fn syscall_id(&self) -> usize {
        self.frame.regs[11]
    }

    fn syscall_args(&self) -> [usize; 6] {
        [
            self.frame.regs[4],
            self.frame.regs[5],
            self.frame.regs[6],
            self.frame.regs[7],
            self.frame.regs[8],
            self.frame.regs[9],
        ]
    }

    fn set_return_value(&mut self, value: isize) {
        self.frame.regs[4] = value as usize;
    }

    fn advance_user_pc(&mut self) {
        self.frame.era = self.frame.era.wrapping_add(4);
    }
}

pub(crate) fn is_quiet_real_write(frame: &LoongArchTrapFrame) -> bool {
    frame.regs[11] == syscall_numbers::SYS_WRITE && real_elf::has_loaded_user_elf()
}

pub(crate) fn handle_syscall(frame: &mut LoongArchTrapFrame) {
    let mut abi = LoongArchSyscallAbi::new(frame);
    let id = abi.syscall_id();
    let args = abi.syscall_args();
    let from_user = (abi.frame.prmd & 0x3) == 3;
    let quiet_group = user::is_basic_group_active();
    let quiet_real_write = id == syscall_numbers::SYS_WRITE && real_elf::has_loaded_user_elf();

    if !quiet_group && !quiet_real_write {
        early_console_write("[loongarch64-syscall] id=");
        write_usize_dec(id);
        early_console_write(" a0=");
        write_usize_hex(args[0]);
        early_console_write(" a1=");
        write_usize_hex(args[1]);
        early_console_write(" a2=");
        write_usize_dec(args[2]);
        early_console_write("\n");
    }

    match id {
        syscall_numbers::SYS_WRITE => {
            let ret = syscall_write(args[0], args[1], args[2], quiet_real_write);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_READ => {
            let ret = syscall_read(args[0], args[1], args[2]);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_OPENAT => {
            let ret = syscall_openat(args[0], args[1], args[2], args[3]);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_PIPE2 => {
            let ret = syscall_pipe2(args[0], args[1]);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_CLONE => match process::sys_clone(abi.frame, args[0], args[1]) {
            Ok(child_pid) => abi.set_return_value(child_pid as isize),
            Err(ret) => abi.set_return_value(ret),
        },
        syscall_numbers::SYS_WAIT4 => {
            let ret = process::sys_wait4(args[0], args[1], args[2]);
            abi.set_return_value(ret);
        },
        syscall_numbers::SYS_EXECVE => {
            let ret = syscall_execve(args[0], args[1], args[2], abi.frame);
            if ret == 0 {
                return;
            }
            abi.set_return_value(ret);
        },
        syscall_numbers::SYS_CLOSE => {
            let ret = syscall_close(args[0]);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_DUP => {
            let ret = syscall_dup(args[0]);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_DUP3 => {
            let ret = syscall_dup3(args[0], args[1], args[2]);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_FSTAT => {
            let ret = syscall_fstat(args[0], args[1]);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_STATX => {
            let ret = syscall_statx(args[0], args[1], args[2], args[3], args[4]);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_NEWFSTATAT => {
            let ret = syscall_newfstatat(args[0], args[1], args[2], args[3]);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_FACCESSAT | syscall_numbers::SYS_FACCESSAT2 => {
            let ret = syscall_faccessat(args[0], args[1], args[2], args[3]);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_READLINKAT => {
            let ret = syscall_readlinkat(args[0], args[1], args[2], args[3]);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_GETDENTS64 => {
            let ret = syscall_getdents64(args[0], args[1], args[2]);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_GETCWD => {
            let ret = syscall_getcwd(args[0], args[1]);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_UNAME => {
            let ret = syscall_uname(args[0]);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_TIMES => {
            let ret = syscall_times(args[0]);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_CHDIR => {
            let ret = syscall_chdir(args[0]);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_MKDIRAT => {
            let ret = syscall_mkdirat(args[0], args[1], args[2]);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_UNLINKAT => {
            let ret = syscall_unlinkat(args[0], args[1], args[2]);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_MOUNT => {
            let ret = syscall_mount(args[0], args[1], args[2], args[3], args[4]);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_UMOUNT2 => {
            let ret = syscall_umount2(args[0], args[1]);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_BRK => {
            let ret = real_elf::sys_brk(args[0]);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_MMAP => {
            let ret = syscall_mmap(args[0], args[1], args[2], args[3], args[4], args[5]);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_MUNMAP => {
            let ret = real_elf::sys_munmap(args[0], args[1]);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_SET_TID_ADDRESS => {
            abi.set_return_value(1);
        }
        syscall_numbers::SYS_SET_ROBUST_LIST => {
            abi.set_return_value(0);
        }
        syscall_numbers::SYS_GETRLIMIT | syscall_numbers::SYS_PRLIMIT64 => {
            let out_ptr = if id == syscall_numbers::SYS_GETRLIMIT {
                args[1]
            } else {
                args[3]
            };
            let ret = syscall_getrlimit(out_ptr);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_CLOCK_GETTIME => {
            let ret = syscall_clock_gettime(args[1]);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_GETTIMEOFDAY => {
            let ret = syscall_gettimeofday(args[0]);
            abi.set_return_value(ret);
        }
        syscall_numbers::SYS_IOCTL => {
            abi.set_return_value(ENOTTY);
        }
        syscall_numbers::SYS_GETPID => {
            abi.set_return_value(process::current_pid() as isize);
        }
        syscall_numbers::SYS_GETTID => {
            abi.set_return_value(process::current_pid() as isize);
        }
        syscall_numbers::SYS_GETPPID => {
            abi.set_return_value(process::current_ppid() as isize);
        }
        syscall_numbers::SYS_GETUID
        | syscall_numbers::SYS_GETEUID
        | syscall_numbers::SYS_GETGID
        | syscall_numbers::SYS_GETEGID => {
            abi.set_return_value(0);
        }
        syscall_numbers::SYS_SCHED_YIELD
        | syscall_numbers::SYS_NANOSLEEP
        | syscall_numbers::SYS_CLOCK_NANOSLEEP => {
            abi.set_return_value(0);
        }
        syscall_numbers::SYS_EXIT | syscall_numbers::SYS_EXIT_GROUP => {
            if process::exit_current_and_maybe_restore_parent(abi.frame, args[0]) {
                return;
            }
            user::record_user_exit(args[0]);
            if !quiet_group {
                early_console_write("[loongarch64-syscall] exit code=");
                write_usize_dec(args[0]);
                early_console_write("\n");
            }
            abi.set_return_value(0);
            if from_user {
                abi.frame.era = trap::user_exit_return_addr();
                abi.frame.prmd &= !0x3;
                return;
            }
        }
        _ => {
            user::record_missing_syscall(id);
            if !quiet_group {
                early_console_write("[loongarch64-syscall] unsupported\n");
            }
            abi.set_return_value(ENOSYS);
        }
    }

    abi.advance_user_pc();
}

fn syscall_write(fd: usize, buf: usize, len: usize, quiet: bool) -> isize {
    if !user_mem::user_range_valid(buf, len) {
        if !user::is_basic_group_active() {
            early_console_write("[loongarch64-usercopy] invalid write buffer\n");
        }
        return EFAULT;
    }
    let entry = match fd_table::fd_entry(fd) {
        Some(entry) => entry,
        None => return EBADF,
    };

    if entry.kind == FD_CONSOLE {
        if quiet {
            user::record_write_syscall();
        } else {
            early_console_write("[loongarch64-user] write fd=");
            write_usize_dec(fd);
            early_console_write(" len=");
            write_usize_dec(len);
            early_console_write("\n");
        }

        let mut done = 0usize;
        while done < len {
            let take = core::cmp::min(len - done, FD_IO_BUFFER_SIZE);
            let tmp = unsafe { &mut fd_table::fd_io_buffer_mut()[..take] };
            if user_mem::copy_from_user(buf + done, tmp).is_err() {
                return EFAULT;
            }
            write_bytes(tmp);
            done += take;
        }
        return len as isize;
    }

    if entry.kind == FD_REGULAR {
        let path = match entry.path.as_str() {
            Ok(path) => path,
            Err(_) => return EINVAL,
        };
        if !fd_table::is_virtual_file_path(path) {
            return EBADF;
        }
        if len > FD_IO_BUFFER_SIZE {
            return EINVAL;
        }
        let tmp = unsafe { &mut fd_table::fd_io_buffer_mut()[..len] };
        if user_mem::copy_from_user(buf, tmp).is_err() {
            return EFAULT;
        }
        let written = match fd_table::write_virtual_file(path, entry.offset, tmp) {
            Ok(written) => written,
            Err(_) => return EINVAL,
        };
        let new_offset = entry.offset.saturating_add(written);
        let _ = fd_table::update_fd_offset(fd, new_offset);
        if let Some(size) = fd_table::virtual_file_size(path) {
            let _ = fd_table::update_fd_size(fd, size);
        }
        return written as isize;
    }

    if entry.kind == FD_PIPE_WRITE {
        if len > FD_IO_BUFFER_SIZE {
            return EINVAL;
        }
        let tmp = unsafe { &mut fd_table::fd_io_buffer_mut()[..len] };
        if user_mem::copy_from_user(buf, tmp).is_err() {
            return EFAULT;
        }
        return match fd_table::write_pipe(fd, tmp) {
            Ok(written) => written as isize,
            Err(_) => EBADF,
        };
    }

    EBADF
}

fn syscall_read(fd: usize, buf: usize, len: usize) -> isize {
    if len == 0 {
        return 0;
    }
    if !user_mem::real_user_range_valid(buf, len) {
        return EFAULT;
    }
    let entry = match fd_table::fd_entry(fd) {
        Some(entry) => entry,
        None => return EBADF,
    };
    if entry.kind == FD_STDIN {
        return 0;
    }
    if entry.kind == FD_PIPE_READ {
        let tmp = unsafe { &mut fd_table::fd_io_buffer_mut()[..core::cmp::min(len, FD_IO_BUFFER_SIZE)] };
        let read = match fd_table::read_pipe(fd, tmp) {
            Ok(read) => read,
            Err(_) => return EBADF,
        };
        if user_mem::copy_to_user(buf, &tmp[..read]).is_err() {
            return EFAULT;
        }
        return read as isize;
    }
    if entry.kind != FD_REGULAR && entry.kind != FD_DIRECTORY && entry.kind != FD_CONSOLE {
        return EBADF;
    }
    if entry.kind == FD_CONSOLE {
        return EBADF;
    }
    if entry.kind == FD_DIRECTORY {
        return EINVAL;
    }
    let path = match entry.path.as_str() {
        Ok(path) => path,
        Err(_) => return EINVAL,
    };
    if fd_table::is_virtual_file_path(path) {
        let tmp = unsafe { &mut fd_table::fd_io_buffer_mut()[..core::cmp::min(len, FD_IO_BUFFER_SIZE)] };
        let read = match fd_table::read_virtual_file(path, entry.offset, tmp) {
            Ok(read) => read,
            Err(_) => return ENOENT,
        };
        if user_mem::copy_to_user(buf, &tmp[..read]).is_err() {
            return EFAULT;
        }
        let _ = fd_table::update_fd_offset(fd, entry.offset.saturating_add(read));
        return read as isize;
    }
    let file = match unsafe { sdcard_ext4::load_path(path, fd_table::fd_io_buffer_mut()) } {
        Ok(file) => file,
        Err(_) => return ENOENT,
    };
    let offset = entry.offset;
    if offset >= file.size {
        return 0;
    }
    let take = core::cmp::min(
        len,
        core::cmp::min(file.size - offset, FD_IO_BUFFER_SIZE - offset),
    );
    let src = unsafe { &fd_table::fd_io_buffer_mut()[offset..offset + take] };
    if user_mem::copy_to_user(buf, src).is_err() {
        return EFAULT;
    }
    let _ = fd_table::update_fd_offset(fd, entry.offset.saturating_add(take));
    take as isize
}

fn syscall_openat(dirfd_raw: usize, path_ptr: usize, flags: usize, mode: usize) -> isize {
    let mut raw = [0u8; PATH_BUF_SIZE];
    let len = match user_mem::read_user_cstr(path_ptr, &mut raw) {
        Ok(len) => len,
        Err(_) => return EFAULT,
    };
    let mut path = PathBuf::empty();
    if fd_table::normalize_user_path(dirfd_raw, &raw[..len], &mut path).is_err() {
        return ENAMETOOLONG;
    }
    let fd = match fd_table::alloc_fd() {
        Some(fd) => fd,
        None => return -24,
    };
    let path_str = match path.as_str() {
        Ok(path_str) => path_str,
        Err(_) => return EINVAL,
    };

    if fd_table::is_virtual_dir_path(path_str) {
        let _ = fd_table::set_fd(
            fd,
            LaFd {
                kind: FD_DIRECTORY,
                inode: 0,
                mode: S_IFDIR | 0o755,
                size: 0,
                offset: 0,
                path,
            },
        );
        return fd as isize;
    }
    if fd_table::is_virtual_file_path(path_str) {
        let size = fd_table::virtual_file_size(path_str).unwrap_or(0);
        let _ = fd_table::set_fd(
            fd,
            LaFd {
                kind: FD_REGULAR,
                inode: 0,
                mode: S_IFREG | 0o644,
                size,
                offset: 0,
                path,
            },
        );
        return fd as isize;
    }

    match sdcard_ext4::stat_path(path_str) {
        Ok(stat) => {
            let kind = if (stat.mode & S_IFDIR) == S_IFDIR || (flags & O_DIRECTORY) != 0 {
                FD_DIRECTORY
            } else {
                FD_REGULAR
            };
            let _ = fd_table::set_fd(
                fd,
                LaFd {
                    kind,
                    inode: stat.inode,
                    mode: stat.mode,
                    size: stat.size,
                    offset: 0,
                    path,
                },
            );
            fd as isize
        }
        Err(_) => {
            if (flags & O_CREAT) != 0 {
                fd_table::create_virtual_path(path_str, mode);
                let _ = fd_table::set_fd(
                    fd,
                    LaFd {
                        kind: if fd_table::is_virtual_dir_path(path_str) {
                            FD_DIRECTORY
                        } else {
                            FD_REGULAR
                        },
                        inode: 0,
                        mode: if fd_table::is_virtual_dir_path(path_str) {
                            S_IFDIR | 0o755
                        } else {
                            S_IFREG | 0o644
                        },
                        size: fd_table::virtual_file_size(path_str).unwrap_or(0),
                        offset: 0,
                        path,
                    },
                );
                fd as isize
            } else {
                ENOENT
            }
        }
    }
}

fn syscall_close(fd: usize) -> isize {
    if fd_table::close_fd(fd) {
        0
    } else {
        EBADF
    }
}

fn syscall_dup(fd: usize) -> isize {
    match fd_table::dup_fd(fd) {
        Some(new_fd) => new_fd as isize,
        None => EBADF,
    }
}

fn syscall_dup3(oldfd: usize, newfd: usize, flags: usize) -> isize {
    if flags != 0 || oldfd >= MAX_FDS || newfd >= MAX_FDS {
        return EINVAL;
    }
    if oldfd == newfd {
        return EINVAL;
    }
    if fd_table::dup3_fd(oldfd, newfd) {
        newfd as isize
    } else {
        EBADF
    }
}

fn syscall_pipe2(pipefd_ptr: usize, flags: usize) -> isize {
    if flags != 0 {
        return EINVAL;
    }
    if !user_mem::real_user_range_valid(pipefd_ptr, 8) {
        return EFAULT;
    }
    let (read_fd, write_fd) = match fd_table::create_pipe_pair() {
        Some(pair) => pair,
        None => return -24,
    };
    let mut fds = [0u8; 8];
    write_le32(&mut fds, 0, read_fd as u32);
    write_le32(&mut fds, 4, write_fd as u32);
    if user_mem::copy_to_user(pipefd_ptr, &fds).is_err() {
        let _ = fd_table::close_fd(read_fd);
        let _ = fd_table::close_fd(write_fd);
        return EFAULT;
    }
    0
}

fn syscall_execve(
    path_ptr: usize,
    argv_ptr: usize,
    envp_ptr: usize,
    frame: &mut LoongArchTrapFrame,
) -> isize {
    let mut raw = [0u8; PATH_BUF_SIZE];
    let len = match user_mem::read_user_cstr(path_ptr, &mut raw) {
        Ok(len) => len,
        Err(_) => return EFAULT,
    };
    let mut path = PathBuf::empty();
    if fd_table::normalize_user_path(AT_FDCWD as usize, &raw[..len], &mut path).is_err() {
        return ENAMETOOLONG;
    }
    let path_str = match path.as_str() {
        Ok(path_str) => path_str,
        Err(_) => return EINVAL,
    };
    let mut argv = [real_elf::ExecString::empty(); real_elf::EXEC_ARG_MAX];
    let argc = match read_exec_strings(argv_ptr, &mut argv) {
        Ok(argc) => argc,
        Err(ret) => return ret,
    };
    let mut envp = [real_elf::ExecString::empty(); real_elf::EXEC_ENV_MAX];
    let envc = match read_exec_strings(envp_ptr, &mut envp) {
        Ok(envc) => envc,
        Err(ret) => return ret,
    };
    process::exec_current(frame, path_str, &argv[..argc], &envp[..envc])
}

fn read_exec_strings(
    vec_ptr: usize,
    out: &mut [real_elf::ExecString],
) -> Result<usize, isize> {
    if vec_ptr == 0 {
        return Ok(0);
    }
    let mut i = 0usize;
    while i < out.len() {
        let ptr = user_mem::read_user_usize(vec_ptr + i * core::mem::size_of::<usize>())
            .map_err(|_| EFAULT)?;
        if ptr == 0 {
            return Ok(i);
        }
        let mut raw = [0u8; real_elf::EXEC_STRING_MAX];
        let len = user_mem::read_user_cstr(ptr, &mut raw).map_err(|_| EFAULT)?;
        out[i].set_from_slice(&raw[..len]).map_err(|_| E2BIG)?;
        i += 1;
    }
    let next = user_mem::read_user_usize(vec_ptr + out.len() * core::mem::size_of::<usize>())
        .map_err(|_| EFAULT)?;
    if next == 0 {
        Ok(out.len())
    } else {
        Err(E2BIG)
    }
}

fn syscall_getcwd(buf: usize, size: usize) -> isize {
    if size == 0 {
        return EINVAL;
    }
    let cwd_buf = fd_table::current_cwd();
    let cwd = &cwd_buf.bytes[..cwd_buf.len];
    if cwd.len() + 1 > size {
        return -34;
    }
    if user_mem::copy_to_user(buf, cwd).is_err()
        || user_mem::copy_to_user(buf + cwd.len(), &[0]).is_err()
    {
        return EFAULT;
    }
    buf as isize
}

fn syscall_uname(ptr: usize) -> isize {
    if ptr == 0 {
        return EFAULT;
    }
    if !user_mem::real_user_range_valid(ptr, 390) {
        return EFAULT;
    }
    if write_uts_field(ptr, 0, b"Linux").is_err()
        || write_uts_field(ptr, 65, b"oscomp-la").is_err()
        || write_uts_field(ptr, 130, b"6.0.0").is_err()
        || write_uts_field(ptr, 195, b"#1").is_err()
        || write_uts_field(ptr, 260, b"loongarch64").is_err()
        || write_uts_field(ptr, 325, b"localdomain").is_err()
    {
        return EFAULT;
    }
    0
}

fn syscall_gettimeofday(ptr: usize) -> isize {
    if ptr == 0 {
        return 0;
    }
    let tick = next_time_tick();
    match user_mem::write_user_usize_pair(ptr, tick / 1000, (tick % 1000) * 1000) {
        Ok(()) => 0,
        Err(_) => EFAULT,
    }
}

fn syscall_clock_gettime(ptr: usize) -> isize {
    if ptr == 0 {
        return EFAULT;
    }
    let tick = next_time_tick();
    match user_mem::write_user_usize_pair(ptr, tick / 1000, (tick % 1000) * 1_000_000) {
        Ok(()) => 0,
        Err(_) => EFAULT,
    }
}

fn syscall_times(ptr: usize) -> isize {
    if ptr != 0 {
        let values = [1usize, 0, 0, 0];
        let bytes = unsafe {
            core::slice::from_raw_parts(
                values.as_ptr() as *const u8,
                core::mem::size_of_val(&values),
            )
        };
        if user_mem::copy_to_user(ptr, bytes).is_err() {
            return EFAULT;
        }
    }
    next_time_tick() as isize
}

fn syscall_fstat(fd: usize, stat_ptr: usize) -> isize {
    let entry = match fd_table::fd_entry(fd) {
        Some(entry) => entry,
        None => return EBADF,
    };
    if entry.kind == FD_CLOSED {
        return EBADF;
    }
    let mode = if entry.kind == FD_DIRECTORY {
        S_IFDIR | 0o755
    } else if entry.kind == FD_CONSOLE || entry.kind == FD_STDIN {
        0o020000 | 0o666
    } else {
        entry.mode
    };
    let size = if entry.kind == FD_REGULAR {
        match entry.path.as_str() {
            Ok(path) => fd_table::virtual_file_size(path).unwrap_or(entry.size),
            Err(_) => entry.size,
        }
    } else {
        0
    };
    write_stat(stat_ptr, entry.inode as usize, mode as usize, size)
}

fn syscall_statx(
    dirfd_raw: usize,
    path_ptr: usize,
    flags: usize,
    mask: usize,
    statx_ptr: usize,
) -> isize {
    let _ = flags;
    let _ = mask;
    let mut inode = 1usize;
    let mut mode = (S_IFREG | 0o644) as usize;
    let mut size = 0usize;

    if path_ptr != 0 {
        let mut raw = [0u8; PATH_BUF_SIZE];
        let len = match user_mem::read_user_cstr(path_ptr, &mut raw) {
            Ok(len) => len,
            Err(_) => return EFAULT,
        };
        if len == 0 && dirfd_raw < MAX_FDS {
            let entry = match fd_table::fd_entry(dirfd_raw) {
                Some(entry) if entry.kind != FD_CLOSED => entry,
                _ => return EBADF,
            };
            inode = entry.inode as usize;
            mode = if entry.kind == FD_DIRECTORY {
                (S_IFDIR | 0o755) as usize
            } else {
                entry.mode as usize
            };
            size = entry.size;
        } else {
            let mut path = PathBuf::empty();
            if fd_table::normalize_user_path(dirfd_raw, &raw[..len], &mut path).is_err() {
                return ENAMETOOLONG;
            }
            let path_str = match path.as_str() {
                Ok(path_str) => path_str,
                Err(_) => return EINVAL,
            };
            if fd_table::is_virtual_dir_path(path_str) {
                mode = (S_IFDIR | 0o755) as usize;
            } else if fd_table::is_virtual_file_path(path_str) {
                mode = (S_IFREG | 0o644) as usize;
            } else {
                match sdcard_ext4::stat_path(path_str) {
                    Ok(stat) => {
                        inode = stat.inode as usize;
                        mode = stat.mode as usize;
                        size = stat.size;
                    }
                    Err(_) => return ENOENT,
                }
            }
        }
    }

    write_statx(statx_ptr, inode, mode, size)
}

fn syscall_newfstatat(dirfd_raw: usize, path_ptr: usize, stat_ptr: usize, flags: usize) -> isize {
    let _ = flags;
    if path_ptr == 0 {
        return EBADF;
    }
    let mut raw = [0u8; PATH_BUF_SIZE];
    let len = match user_mem::read_user_cstr(path_ptr, &mut raw) {
        Ok(len) => len,
        Err(_) => return EFAULT,
    };
    if len == 0 && dirfd_raw < MAX_FDS {
        let entry = match fd_table::fd_entry(dirfd_raw) {
            Some(entry) if entry.kind != FD_CLOSED => entry,
            _ => return EBADF,
        };
        return write_stat(stat_ptr, entry.inode as usize, entry.mode as usize, entry.size);
    }
    let mut path = PathBuf::empty();
    if fd_table::normalize_user_path(dirfd_raw, &raw[..len], &mut path).is_err() {
        return ENAMETOOLONG;
    }
    let path_str = match path.as_str() {
        Ok(path_str) => path_str,
        Err(_) => return EINVAL,
    };
    match stat_path_like(path_str) {
        Some((inode, mode, size)) => write_stat(stat_ptr, inode, mode, size),
        None => ENOENT,
    }
}

fn syscall_faccessat(dirfd_raw: usize, path_ptr: usize, mode: usize, flags: usize) -> isize {
    let _ = (mode, flags);
    let mut raw = [0u8; PATH_BUF_SIZE];
    let len = match user_mem::read_user_cstr(path_ptr, &mut raw) {
        Ok(len) => len,
        Err(_) => return EFAULT,
    };
    let mut path = PathBuf::empty();
    if fd_table::normalize_user_path(dirfd_raw, &raw[..len], &mut path).is_err() {
        return ENAMETOOLONG;
    }
    let path_str = match path.as_str() {
        Ok(path_str) => path_str,
        Err(_) => return EINVAL,
    };
    if path_exists_like(path_str) {
        0
    } else {
        ENOENT
    }
}

fn syscall_readlinkat(dirfd_raw: usize, path_ptr: usize, buf: usize, bufsiz: usize) -> isize {
    if bufsiz == 0 {
        return EINVAL;
    }
    let mut raw = [0u8; PATH_BUF_SIZE];
    let len = match user_mem::read_user_cstr(path_ptr, &mut raw) {
        Ok(len) => len,
        Err(_) => return EFAULT,
    };
    let mut path = PathBuf::empty();
    if fd_table::normalize_user_path(dirfd_raw, &raw[..len], &mut path).is_err() {
        return ENAMETOOLONG;
    }
    let path_str = match path.as_str() {
        Ok(path_str) => path_str,
        Err(_) => return EINVAL,
    };
    let target = if path_str == "/proc/self/exe" {
        b"/musl/busybox".as_slice()
    } else {
        return EINVAL;
    };
    let take = core::cmp::min(bufsiz, target.len());
    if user_mem::copy_to_user(buf, &target[..take]).is_err() {
        return EFAULT;
    }
    take as isize
}

fn syscall_getrlimit(out_ptr: usize) -> isize {
    if out_ptr == 0 {
        return 0;
    }
    let mut rlim = [0u8; 16];
    write_le64(&mut rlim, 0, 8 * 1024 * 1024);
    write_le64(&mut rlim, 8, 8 * 1024 * 1024);
    if user_mem::copy_to_user(out_ptr, &rlim).is_err() {
        return EFAULT;
    }
    0
}

fn path_exists_like(path: &str) -> bool {
    stat_path_like(path).is_some()
}

fn stat_path_like(path: &str) -> Option<(usize, usize, usize)> {
    if fd_table::is_virtual_dir_path(path) {
        return Some((0, (S_IFDIR | 0o755) as usize, 0));
    }
    if fd_table::is_virtual_file_path(path) {
        return Some((
            0,
            (S_IFREG | 0o644) as usize,
            fd_table::virtual_file_size(path).unwrap_or(0),
        ));
    }
    match sdcard_ext4::stat_path(path) {
        Ok(stat) => Some((stat.inode as usize, stat.mode as usize, stat.size)),
        Err(_) => None,
    }
}

fn syscall_getdents64(fd: usize, buf: usize, len: usize) -> isize {
    let entry = match fd_table::fd_entry(fd) {
        Some(entry) => entry,
        None => return EBADF,
    };
    if entry.kind != FD_DIRECTORY {
        return EINVAL;
    }
    let mut written = 0usize;
    if write_dirent64(buf, len, &mut written, 2, DT_DIR, b".").is_err()
        || write_dirent64(buf, len, &mut written, 20, DT_DIR, b"basic").is_err()
        || write_dirent64(buf, len, &mut written, 49, DT_REG, b"text.txt").is_err()
    {
        return EINVAL;
    }
    written as isize
}

fn syscall_chdir(path_ptr: usize) -> isize {
    let mut raw = [0u8; PATH_BUF_SIZE];
    let len = match user_mem::read_user_cstr(path_ptr, &mut raw) {
        Ok(len) => len,
        Err(_) => return EFAULT,
    };
    let mut path = PathBuf::empty();
    if fd_table::normalize_user_path(AT_FDCWD as usize, &raw[..len], &mut path).is_err() {
        return ENAMETOOLONG;
    }
    let path_str = match path.as_str() {
        Ok(path_str) => path_str,
        Err(_) => return EINVAL,
    };
    if !fd_table::is_virtual_dir_path(path_str) && sdcard_ext4::stat_path(path_str).is_err() {
        return ENOENT;
    }
    fd_table::set_cwd(&path);
    0
}

fn syscall_mkdirat(dirfd_raw: usize, path_ptr: usize, mode: usize) -> isize {
    let mut raw = [0u8; PATH_BUF_SIZE];
    let len = match user_mem::read_user_cstr(path_ptr, &mut raw) {
        Ok(len) => len,
        Err(_) => return EFAULT,
    };
    let mut path = PathBuf::empty();
    if fd_table::normalize_user_path(dirfd_raw, &raw[..len], &mut path).is_err() {
        return ENAMETOOLONG;
    }
    let path_str = match path.as_str() {
        Ok(path_str) => path_str,
        Err(_) => return EINVAL,
    };
    fd_table::create_virtual_dir_path(path_str, mode);
    0
}

fn syscall_unlinkat(dirfd_raw: usize, path_ptr: usize, flags: usize) -> isize {
    let _ = flags;
    let mut raw = [0u8; PATH_BUF_SIZE];
    let len = match user_mem::read_user_cstr(path_ptr, &mut raw) {
        Ok(len) => len,
        Err(_) => return EFAULT,
    };
    let mut path = PathBuf::empty();
    if fd_table::normalize_user_path(dirfd_raw, &raw[..len], &mut path).is_err() {
        return ENAMETOOLONG;
    }
    let path_str = match path.as_str() {
        Ok(path_str) => path_str,
        Err(_) => return EINVAL,
    };
    if fd_table::remove_virtual_path(path_str) {
        0
    } else {
        ENOENT
    }
}

fn syscall_mount(
    source_ptr: usize,
    target_ptr: usize,
    fstype_ptr: usize,
    flags: usize,
    data_ptr: usize,
) -> isize {
    let _ = (flags, data_ptr);
    if read_optional_user_string(source_ptr).is_err()
        || read_optional_user_string(fstype_ptr).is_err()
    {
        return EFAULT;
    }
    let mut raw = [0u8; PATH_BUF_SIZE];
    let len = match user_mem::read_user_cstr(target_ptr, &mut raw) {
        Ok(len) => len,
        Err(_) => return EFAULT,
    };
    let mut path = PathBuf::empty();
    if fd_table::normalize_user_path(AT_FDCWD as usize, &raw[..len], &mut path).is_err() {
        return ENAMETOOLONG;
    }
    let path_str = match path.as_str() {
        Ok(path_str) => path_str,
        Err(_) => return EINVAL,
    };
    fd_table::create_virtual_dir_path(path_str, S_IFDIR as usize);
    0
}

fn syscall_umount2(target_ptr: usize, flags: usize) -> isize {
    let _ = flags;
    let mut raw = [0u8; PATH_BUF_SIZE];
    match user_mem::read_user_cstr(target_ptr, &mut raw) {
        Ok(_) => 0,
        Err(_) => EFAULT,
    }
}

fn read_optional_user_string(ptr: usize) -> Result<(), ()> {
    if ptr == 0 {
        return Ok(());
    }
    let mut raw = [0u8; PATH_BUF_SIZE];
    user_mem::read_user_cstr(ptr, &mut raw).map(|_| ()).map_err(|_| ())
}

fn syscall_mmap(
    addr: usize,
    len: usize,
    prot: usize,
    flags: usize,
    fd_raw: usize,
    offset: usize,
) -> isize {
    let fd_signed = fd_raw as isize;
    if fd_signed < 0 {
        return real_elf::sys_mmap(addr, len, prot, flags, None);
    }
    let fd = fd_raw;
    let entry = match fd_table::fd_entry(fd) {
        Some(entry) if entry.kind == FD_REGULAR => entry,
        _ => return EBADF,
    };
    let path = match entry.path.as_str() {
        Ok(path) => path,
        Err(_) => return EINVAL,
    };
    let tmp = unsafe { fd_table::fd_io_buffer_mut() };
    let bytes = if fd_table::is_virtual_file_path(path) {
        let read = match fd_table::read_virtual_file(path, offset, tmp) {
            Ok(read) => read,
            Err(_) => return ENOENT,
        };
        &tmp[..read]
    } else {
        let file = match sdcard_ext4::load_path(path, tmp) {
            Ok(file) => file,
            Err(_) => return ENOENT,
        };
        if offset >= file.size {
            &tmp[..0]
        } else {
            let take = core::cmp::min(file.size - offset, FD_IO_BUFFER_SIZE - offset);
            &tmp[offset..offset + take]
        }
    };
    real_elf::sys_mmap(addr, len, prot, flags, Some(bytes))
}

fn write_uts_field(base: usize, offset: usize, value: &[u8]) -> Result<(), &'static str> {
    let mut field = [0u8; 65];
    let mut i = 0usize;
    while i < value.len() && i < 64 {
        field[i] = value[i];
        i += 1;
    }
    user_mem::copy_to_user(base + offset, &field)
}

fn write_stat(ptr: usize, inode: usize, mode: usize, size: usize) -> isize {
    let mut stat = [0u8; 128];
    write_le64(&mut stat, 0, 1);
    write_le64(&mut stat, 8, inode);
    write_le32(&mut stat, 16, mode as u32);
    write_le32(&mut stat, 20, 1);
    write_le64(&mut stat, 48, size);
    write_le32(&mut stat, 56, 4096);
    write_le64(&mut stat, 64, 1);
    if user_mem::copy_to_user(ptr, &stat).is_err() {
        return EFAULT;
    }
    0
}

fn write_statx(ptr: usize, inode: usize, mode: usize, size: usize) -> isize {
    let mut statx = [0u8; 256];
    write_le32(&mut statx, 0, 0x17ff);
    write_le32(&mut statx, 4, 4096);
    write_le32(&mut statx, 16, 1);
    write_le32(&mut statx, 28, mode as u32);
    write_le64(&mut statx, 32, inode);
    write_le64(&mut statx, 40, size);
    write_le64(&mut statx, 48, 1);
    if user_mem::copy_to_user(ptr, &statx).is_err() {
        return EFAULT;
    }
    0
}

fn write_dirent64(
    base: usize,
    total_len: usize,
    written: &mut usize,
    inode: usize,
    dtype: u8,
    name: &[u8],
) -> Result<(), &'static str> {
    let reclen = (19 + name.len() + 1 + 7) & !7;
    if *written + reclen > total_len {
        return Err("dirent_space");
    }
    let ptr = base + *written;
    if !user_mem::real_user_range_valid(ptr, reclen) {
        return Err("dirent_range");
    }
    let mut rec = [0u8; 64];
    if reclen > rec.len() {
        return Err("dirent_large");
    }
    write_le64(&mut rec, 0, inode);
    write_le64(&mut rec, 8, *written + reclen);
    write_le16(&mut rec, 16, reclen as u16);
    rec[18] = dtype;
    let mut i = 0usize;
    while i < name.len() {
        rec[19 + i] = name[i];
        i += 1;
    }
    rec[19 + name.len()] = 0;
    user_mem::copy_to_user(ptr, &rec[..reclen])?;
    *written += reclen;
    Ok(())
}

fn write_le16(dst: &mut [u8], off: usize, value: u16) {
    let bytes = value.to_le_bytes();
    dst[off] = bytes[0];
    dst[off + 1] = bytes[1];
}

fn write_le32(dst: &mut [u8], off: usize, value: u32) {
    let bytes = value.to_le_bytes();
    let mut i = 0usize;
    while i < 4 {
        dst[off + i] = bytes[i];
        i += 1;
    }
}

fn write_le64(dst: &mut [u8], off: usize, value: usize) {
    let bytes = (value as u64).to_le_bytes();
    let mut i = 0usize;
    while i < 8 {
        dst[off + i] = bytes[i];
        i += 1;
    }
}

fn next_time_tick() -> usize {
    unsafe {
        TIME_TICK = TIME_TICK.saturating_add(37);
        TIME_TICK
    }
}
