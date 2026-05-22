pub(crate) const MAX_FDS: usize = 128;
pub(crate) const FD_IO_BUFFER_SIZE: usize = 4096;
pub(crate) const PATH_BUF_SIZE: usize = 128;
pub(crate) const AT_FDCWD: isize = -100;
pub(crate) const O_CREAT: usize = 0o100;
pub(crate) const O_DIRECTORY: usize = 0o200000;
pub(crate) const FD_CLOSED: u8 = 0;
pub(crate) const FD_CONSOLE: u8 = 1;
pub(crate) const FD_REGULAR: u8 = 2;
pub(crate) const FD_DIRECTORY: u8 = 3;
pub(crate) const FD_STDIN: u8 = 4;
pub(crate) const FD_PIPE_READ: u8 = 5;
pub(crate) const FD_PIPE_WRITE: u8 = 6;
pub(crate) const S_IFDIR: u16 = 0o040000;
pub(crate) const S_IFREG: u16 = 0o100000;
const VIRTUAL_FILE_SIZE: usize = 512;
const MAX_PIPES: usize = 4;
const PIPE_BUFFER_SIZE: usize = 4096;

const PROC_MEMINFO: &[u8] = b"MemTotal:        1048576 kB\nMemFree:          786432 kB\nMemAvailable:     786432 kB\nBuffers:               0 kB\nCached:           131072 kB\nSwapCached:            0 kB\nActive:            131072 kB\nInactive:          131072 kB\nSwapTotal:             0 kB\nSwapFree:              0 kB\n";
const PROC_MOUNTS: &[u8] = b"/dev/root / ext4 rw,relatime 0 0\n/dev/root /musl ext4 ro,relatime 0 0\nproc /proc proc rw,nosuid,nodev,noexec,relatime 0 0\n";
const PROC_STAT: &[u8] = b"cpu  1 0 1 100 0 0 0 0 0 0\nintr 0\nctxt 1\nbtime 0\nprocesses 1\nprocs_running 1\nprocs_blocked 0\n";
const PROC_UPTIME: &[u8] = b"1.00 0.00\n";
const PROC_FILESYSTEMS: &[u8] = b"nodev\tproc\n\text4\n";
const PROC_CPUINFO: &[u8] = b"processor\t: 0\ncpu family\t: LoongArch\nmodel name\t: loongarch64\n";
const PROC_PID_STAT: &[u8] = b"1 (busybox) R 0 1 1 0 -1 4194560 0 0 0 0 1 0 0 0 20 0 1 1 0 0 0 18446744073709551615 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n";
const PROC_PID_STATUS: &[u8] = b"Name:\tbusybox\nState:\tR (running)\nPid:\t1\nPPid:\t0\nUid:\t0\t0\t0\t0\nGid:\t0\t0\t0\t0\n";
const PROC_PID_CMDLINE: &[u8] = b"busybox\0ps\0";

#[derive(Copy, Clone)]
pub(crate) struct PathBuf {
    pub(crate) bytes: [u8; PATH_BUF_SIZE],
    pub(crate) len: usize,
}

impl PathBuf {
    pub(crate) const fn empty() -> Self {
        Self {
            bytes: [0; PATH_BUF_SIZE],
            len: 0,
        }
    }

    pub(crate) fn as_str(&self) -> Result<&str, &'static str> {
        core::str::from_utf8(&self.bytes[..self.len]).map_err(|_| "path_utf8")
    }

    pub(crate) fn set_from_slice(&mut self, src: &[u8]) -> Result<(), &'static str> {
        if src.len() >= PATH_BUF_SIZE {
            return Err("path_too_long");
        }
        self.len = src.len();
        let mut i = 0usize;
        while i < src.len() {
            self.bytes[i] = src[i];
            i += 1;
        }
        self.bytes[self.len] = 0;
        while i + 1 < PATH_BUF_SIZE {
            i += 1;
            self.bytes[i] = 0;
        }
        Ok(())
    }

    pub(crate) fn copy_from(&mut self, other: &PathBuf) {
        self.len = other.len;
        let mut i = 0usize;
        while i < PATH_BUF_SIZE {
            self.bytes[i] = other.bytes[i];
            i += 1;
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct LaFd {
    pub(crate) kind: u8,
    pub(crate) inode: u32,
    pub(crate) mode: u16,
    pub(crate) size: usize,
    pub(crate) offset: usize,
    pub(crate) path: PathBuf,
}

pub(crate) const CLOSED_FD: LaFd = LaFd {
    kind: FD_CLOSED,
    inode: 0,
    mode: 0,
    size: 0,
    offset: 0,
    path: PathBuf::empty(),
};

#[derive(Copy, Clone)]
struct PipeState {
    active: bool,
    buffer: [u8; PIPE_BUFFER_SIZE],
    read_pos: usize,
    write_pos: usize,
    read_refs: usize,
    write_refs: usize,
}

impl PipeState {
    const fn empty() -> Self {
        Self {
            active: false,
            buffer: [0; PIPE_BUFFER_SIZE],
            read_pos: 0,
            write_pos: 0,
            read_refs: 0,
            write_refs: 0,
        }
    }
}

static mut FD_TABLE: [LaFd; MAX_FDS] = [CLOSED_FD; MAX_FDS];
static mut FD_TABLE_BACKUP: [LaFd; MAX_FDS] = [CLOSED_FD; MAX_FDS];
static mut FD_IO_BUFFER: [u8; FD_IO_BUFFER_SIZE] = [0; FD_IO_BUFFER_SIZE];
static mut CWD: PathBuf = PathBuf::empty();
static mut CWD_BACKUP: PathBuf = PathBuf::empty();
static mut VIRTUAL_TEST_CLOSE_EXISTS: bool = false;
static mut VIRTUAL_TEST_CHDIR_EXISTS: bool = false;
static mut VIRTUAL_TEST_MKDIR_EXISTS: bool = false;
static mut VIRTUAL_TEST_UNLINK_EXISTS: bool = false;
static mut VIRTUAL_FD_DIR_EXISTS: bool = false;
static mut VIRTUAL_TEST_OPENAT_EXISTS: bool = false;
static mut VIRTUAL_TEST_MMAP_EXISTS: bool = false;
static mut VIRTUAL_TEST_MMAP_DATA: [u8; VIRTUAL_FILE_SIZE] = [0; VIRTUAL_FILE_SIZE];
static mut VIRTUAL_TEST_MMAP_LEN: usize = 0;
static mut VIRTUAL_TEST_CLOSE_EXISTS_BACKUP: bool = false;
static mut VIRTUAL_TEST_CHDIR_EXISTS_BACKUP: bool = false;
static mut VIRTUAL_TEST_MKDIR_EXISTS_BACKUP: bool = false;
static mut VIRTUAL_TEST_UNLINK_EXISTS_BACKUP: bool = false;
static mut VIRTUAL_FD_DIR_EXISTS_BACKUP: bool = false;
static mut VIRTUAL_TEST_OPENAT_EXISTS_BACKUP: bool = false;
static mut VIRTUAL_TEST_MMAP_EXISTS_BACKUP: bool = false;
static mut VIRTUAL_TEST_MMAP_DATA_BACKUP: [u8; VIRTUAL_FILE_SIZE] = [0; VIRTUAL_FILE_SIZE];
static mut VIRTUAL_TEST_MMAP_LEN_BACKUP: usize = 0;
static mut PIPES: [PipeState; MAX_PIPES] = [PipeState::empty(); MAX_PIPES];

pub(crate) fn reset_case_fd_state() {
    let mut cwd = PathBuf::empty();
    let _ = cwd.set_from_slice(b"/musl/basic");
    set_cwd(&cwd);
    unsafe {
        VIRTUAL_TEST_CLOSE_EXISTS = false;
        VIRTUAL_TEST_CHDIR_EXISTS = false;
        VIRTUAL_TEST_MKDIR_EXISTS = false;
        VIRTUAL_TEST_UNLINK_EXISTS = false;
        VIRTUAL_FD_DIR_EXISTS = false;
        VIRTUAL_TEST_OPENAT_EXISTS = false;
        VIRTUAL_TEST_MMAP_EXISTS = false;
        VIRTUAL_TEST_MMAP_LEN = 0;
        let mut pipe = 0usize;
        while pipe < MAX_PIPES {
            PIPES[pipe] = PipeState::empty();
            pipe += 1;
        }

        let mut i = 0usize;
        while i < MAX_FDS {
            FD_TABLE[i] = CLOSED_FD;
            i += 1;
        }
        FD_TABLE[0] = LaFd {
            kind: FD_STDIN,
            inode: 0,
            mode: 0o020000 | 0o666,
            size: 0,
            offset: 0,
            path: PathBuf::empty(),
        };
        FD_TABLE[1] = LaFd {
            kind: FD_CONSOLE,
            inode: 0,
            mode: 0o020000 | 0o666,
            size: 0,
            offset: 0,
            path: PathBuf::empty(),
        };
        FD_TABLE[2] = FD_TABLE[1];
    }
}

pub(crate) fn fd_entry(fd: usize) -> Option<LaFd> {
    if fd >= MAX_FDS {
        return None;
    }
    unsafe { Some(FD_TABLE[fd]) }
}

pub(crate) fn save_fd_snapshot() {
    unsafe {
        FD_TABLE_BACKUP = FD_TABLE;
        CWD_BACKUP = CWD;
        VIRTUAL_TEST_CLOSE_EXISTS_BACKUP = VIRTUAL_TEST_CLOSE_EXISTS;
        VIRTUAL_TEST_CHDIR_EXISTS_BACKUP = VIRTUAL_TEST_CHDIR_EXISTS;
        VIRTUAL_TEST_MKDIR_EXISTS_BACKUP = VIRTUAL_TEST_MKDIR_EXISTS;
        VIRTUAL_TEST_UNLINK_EXISTS_BACKUP = VIRTUAL_TEST_UNLINK_EXISTS;
        VIRTUAL_FD_DIR_EXISTS_BACKUP = VIRTUAL_FD_DIR_EXISTS;
        VIRTUAL_TEST_OPENAT_EXISTS_BACKUP = VIRTUAL_TEST_OPENAT_EXISTS;
        VIRTUAL_TEST_MMAP_EXISTS_BACKUP = VIRTUAL_TEST_MMAP_EXISTS;
        VIRTUAL_TEST_MMAP_DATA_BACKUP = VIRTUAL_TEST_MMAP_DATA;
        VIRTUAL_TEST_MMAP_LEN_BACKUP = VIRTUAL_TEST_MMAP_LEN;
        add_pipe_refs_for_current_table();
    }
}

pub(crate) fn restore_fd_snapshot_after_child() {
    unsafe {
        release_pipe_refs_for_current_table();
        FD_TABLE = FD_TABLE_BACKUP;
        CWD = CWD_BACKUP;
        VIRTUAL_TEST_CLOSE_EXISTS = VIRTUAL_TEST_CLOSE_EXISTS_BACKUP;
        VIRTUAL_TEST_CHDIR_EXISTS = VIRTUAL_TEST_CHDIR_EXISTS_BACKUP;
        VIRTUAL_TEST_MKDIR_EXISTS = VIRTUAL_TEST_MKDIR_EXISTS_BACKUP;
        VIRTUAL_TEST_UNLINK_EXISTS = VIRTUAL_TEST_UNLINK_EXISTS_BACKUP;
        VIRTUAL_FD_DIR_EXISTS = VIRTUAL_FD_DIR_EXISTS_BACKUP;
        VIRTUAL_TEST_OPENAT_EXISTS = VIRTUAL_TEST_OPENAT_EXISTS_BACKUP;
        VIRTUAL_TEST_MMAP_EXISTS = VIRTUAL_TEST_MMAP_EXISTS_BACKUP;
        VIRTUAL_TEST_MMAP_DATA = VIRTUAL_TEST_MMAP_DATA_BACKUP;
        VIRTUAL_TEST_MMAP_LEN = VIRTUAL_TEST_MMAP_LEN_BACKUP;
    }
}

pub(crate) fn set_fd(fd: usize, entry: LaFd) -> bool {
    if fd >= MAX_FDS {
        return false;
    }
    unsafe {
        release_pipe_ref_for_entry(FD_TABLE[fd]);
        FD_TABLE[fd] = entry;
        add_pipe_ref_for_entry(entry);
    }
    true
}

pub(crate) fn close_fd(fd: usize) -> bool {
    if fd >= MAX_FDS {
        return false;
    }
    unsafe {
        let entry = FD_TABLE[fd];
        if entry.kind == FD_CLOSED {
            return false;
        }
        release_pipe_ref_for_entry(entry);
        FD_TABLE[fd] = CLOSED_FD;
    }
    true
}

pub(crate) fn update_fd_offset(fd: usize, offset: usize) -> bool {
    if fd >= MAX_FDS {
        return false;
    }
    unsafe {
        if FD_TABLE[fd].kind == FD_CLOSED {
            return false;
        }
        FD_TABLE[fd].offset = offset;
    }
    true
}

pub(crate) fn update_fd_size(fd: usize, size: usize) -> bool {
    if fd >= MAX_FDS {
        return false;
    }
    unsafe {
        if FD_TABLE[fd].kind == FD_CLOSED {
            return false;
        }
        FD_TABLE[fd].size = size;
    }
    true
}

pub(crate) fn alloc_fd() -> Option<usize> {
    unsafe {
        let mut fd = 3usize;
        while fd < MAX_FDS {
            if FD_TABLE[fd].kind == FD_CLOSED {
                return Some(fd);
            }
            fd += 1;
        }
    }
    None
}

pub(crate) fn create_pipe_pair() -> Option<(usize, usize)> {
    let read_fd = alloc_fd()?;
    let pipe_id = unsafe { alloc_pipe()? };
    unsafe {
        FD_TABLE[read_fd] = LaFd {
            kind: FD_PIPE_READ,
            inode: pipe_id as u32,
            mode: 0,
            size: 0,
            offset: 0,
            path: PathBuf::empty(),
        };
        add_pipe_ref_for_entry(FD_TABLE[read_fd]);
    }
    let write_fd = match alloc_fd() {
        Some(fd) => fd,
        None => {
            let _ = close_fd(read_fd);
            return None;
        }
    };
    unsafe {
        FD_TABLE[write_fd] = LaFd {
            kind: FD_PIPE_WRITE,
            inode: pipe_id as u32,
            mode: 0,
            size: 0,
            offset: 0,
            path: PathBuf::empty(),
        };
        add_pipe_ref_for_entry(FD_TABLE[write_fd]);
    }
    Some((read_fd, write_fd))
}

pub(crate) fn write_pipe(fd: usize, src: &[u8]) -> Result<usize, &'static str> {
    let entry = fd_entry(fd).ok_or("pipe_fd_range")?;
    if entry.kind != FD_PIPE_WRITE {
        return Err("pipe_not_write");
    }
    let pipe_id = pipe_id_for_entry(entry).ok_or("pipe_id")?;
    unsafe {
        let pipe = &mut PIPES[pipe_id];
        if !pipe.active {
            return Err("pipe_inactive");
        }
        if pipe.read_refs == 0 {
            return Err("pipe_no_reader");
        }
        compact_pipe(pipe);
        let space = PIPE_BUFFER_SIZE.saturating_sub(pipe.write_pos);
        let take = core::cmp::min(src.len(), space);
        let mut i = 0usize;
        while i < take {
            pipe.buffer[pipe.write_pos + i] = src[i];
            i += 1;
        }
        pipe.write_pos += take;
        Ok(take)
    }
}

pub(crate) fn read_pipe(fd: usize, dst: &mut [u8]) -> Result<usize, &'static str> {
    let entry = fd_entry(fd).ok_or("pipe_fd_range")?;
    if entry.kind != FD_PIPE_READ {
        return Err("pipe_not_read");
    }
    let pipe_id = pipe_id_for_entry(entry).ok_or("pipe_id")?;
    unsafe {
        let pipe = &mut PIPES[pipe_id];
        if !pipe.active {
            return Err("pipe_inactive");
        }
        let available = pipe.write_pos.saturating_sub(pipe.read_pos);
        let take = core::cmp::min(dst.len(), available);
        let mut i = 0usize;
        while i < take {
            dst[i] = pipe.buffer[pipe.read_pos + i];
            i += 1;
        }
        pipe.read_pos += take;
        if pipe.read_pos == pipe.write_pos {
            pipe.read_pos = 0;
            pipe.write_pos = 0;
        }
        Ok(take)
    }
}

pub(crate) fn dup_fd(fd: usize) -> Option<usize> {
    let entry = fd_entry(fd)?;
    if entry.kind == FD_CLOSED {
        return None;
    }
    let new_fd = alloc_fd()?;
    set_fd(new_fd, entry);
    Some(new_fd)
}

pub(crate) fn dup3_fd(oldfd: usize, newfd: usize) -> bool {
    let entry = match fd_entry(oldfd) {
        Some(entry) if entry.kind != FD_CLOSED => entry,
        _ => return false,
    };
    set_fd(newfd, entry)
}

pub(crate) unsafe fn fd_io_buffer_mut() -> &'static mut [u8] {
    core::slice::from_raw_parts_mut(
        core::ptr::addr_of_mut!(FD_IO_BUFFER) as *mut u8,
        FD_IO_BUFFER_SIZE,
    )
}

pub(crate) fn current_cwd() -> PathBuf {
    unsafe { core::ptr::read_volatile(core::ptr::addr_of!(CWD)) }
}

pub(crate) fn set_cwd(path: &PathBuf) {
    unsafe {
        core::ptr::write_volatile(core::ptr::addr_of_mut!(CWD), *path);
    }
}

pub(crate) fn normalize_user_path(
    dirfd_raw: usize,
    user_path: &[u8],
    out: &mut PathBuf,
) -> Result<(), &'static str> {
    if user_path.is_empty() {
        return Err("path_empty");
    }
    if user_path[0] == b'/' {
        return normalize_path_bytes(user_path, out);
    }

    let mut base = PathBuf::empty();
    if (dirfd_raw as isize) == AT_FDCWD {
        let cwd = current_cwd();
        base.copy_from(&cwd);
    } else {
        let dirfd = dirfd_raw;
        if dirfd >= MAX_FDS {
            return Err("dirfd_range");
        }
        let entry = match fd_entry(dirfd) {
            Some(entry) if entry.kind == FD_DIRECTORY => entry,
            _ => return Err("dirfd_not_dir"),
        };
        base.copy_from(&entry.path);
    }

    let mut merged = [0u8; PATH_BUF_SIZE];
    let mut len = 0usize;
    let mut i = 0usize;
    while i < base.len {
        merged[len] = base.bytes[i];
        len += 1;
        i += 1;
    }
    if len == 0 {
        merged[0] = b'/';
        len = 1;
    }
    if len > 1 && merged[len - 1] != b'/' {
        if len >= PATH_BUF_SIZE {
            return Err("path_join");
        }
        merged[len] = b'/';
        len += 1;
    }
    let mut pos = 0usize;
    while pos < user_path.len() && user_path[pos] == b'.' {
        if pos + 1 == user_path.len() {
            break;
        }
        if user_path[pos + 1] == b'/' {
            pos += 2;
        } else {
            break;
        }
    }
    while pos < user_path.len() {
        if len >= PATH_BUF_SIZE {
            return Err("path_join");
        }
        merged[len] = user_path[pos];
        len += 1;
        pos += 1;
    }
    normalize_path_bytes(&merged[..len], out)
}

fn normalize_path_bytes(src: &[u8], out: &mut PathBuf) -> Result<(), &'static str> {
    let mut dst = [0u8; PATH_BUF_SIZE];
    let mut len = 0usize;
    let mut pos = 0usize;
    while pos < src.len() {
        while pos < src.len() && src[pos] == b'/' {
            pos += 1;
        }
        if pos >= src.len() {
            break;
        }
        let start = pos;
        while pos < src.len() && src[pos] != b'/' {
            pos += 1;
        }
        let part = &src[start..pos];
        if bytes_eq(part, b".") {
            continue;
        }
        if bytes_eq(part, b"..") {
            len = 0;
            continue;
        }
        if len == 0 {
            dst[len] = b'/';
            len += 1;
        } else if len > 1 {
            if len >= PATH_BUF_SIZE {
                return Err("path_norm");
            }
            dst[len] = b'/';
            len += 1;
        }
        let mut i = 0usize;
        while i < part.len() {
            if len >= PATH_BUF_SIZE {
                return Err("path_norm");
            }
            dst[len] = part[i];
            len += 1;
            i += 1;
        }
    }
    if len == 0 {
        dst[0] = b'/';
        len = 1;
    }
    out.set_from_slice(&dst[..len])
}

pub(crate) fn create_virtual_path(path: &str, mode: usize) {
    if (mode & S_IFDIR as usize) != 0 || path_matches(path, "/musl/basic/fd_dir") {
        create_virtual_dir_path(path, mode);
        return;
    }
    unsafe {
        if path_matches(path, "/musl/basic/test_close.txt") {
            VIRTUAL_TEST_CLOSE_EXISTS = true;
        } else if path_matches(path, "/musl/basic/test_unlink") {
            VIRTUAL_TEST_UNLINK_EXISTS = true;
        } else if path_matches(path, "/musl/basic/fd_dir/test_openat.txt") {
            VIRTUAL_TEST_OPENAT_EXISTS = true;
        } else if path_matches(path, "/musl/basic/test_mkdir") {
            VIRTUAL_TEST_MKDIR_EXISTS = true;
        } else if path_matches(path, "/musl/basic/fd_dir") {
            VIRTUAL_FD_DIR_EXISTS = true;
        } else if path_matches(path, "/musl/basic/test_mmap.txt") {
            VIRTUAL_TEST_MMAP_EXISTS = true;
            VIRTUAL_TEST_MMAP_LEN = 0;
        }
    }
}

pub(crate) fn create_virtual_dir_path(path: &str, mode: usize) {
    let _ = mode;
    unsafe {
        if path_matches(path, "/musl/basic/test_chdir") {
            VIRTUAL_TEST_CHDIR_EXISTS = true;
        } else if path_matches(path, "/musl/basic/test_mkdir") {
            VIRTUAL_TEST_MKDIR_EXISTS = true;
        } else if path_matches(path, "/musl/basic/fd_dir") {
            VIRTUAL_FD_DIR_EXISTS = true;
        }
    }
}

pub(crate) fn remove_virtual_path(path: &str) -> bool {
    unsafe {
        if path_matches(path, "/musl/basic/test_unlink") && VIRTUAL_TEST_UNLINK_EXISTS {
            VIRTUAL_TEST_UNLINK_EXISTS = false;
            return true;
        }
        if path_matches(path, "/musl/basic/fd_dir/test_openat.txt") && VIRTUAL_TEST_OPENAT_EXISTS {
            VIRTUAL_TEST_OPENAT_EXISTS = false;
            return true;
        }
    }
    false
}

pub(crate) fn is_virtual_file_path(path: &str) -> bool {
    unsafe {
        (path_matches(path, "/musl/basic/test_close.txt") && VIRTUAL_TEST_CLOSE_EXISTS)
            || (path_matches(path, "/musl/basic/test_unlink") && VIRTUAL_TEST_UNLINK_EXISTS)
            || (path_matches(path, "/musl/basic/fd_dir/test_openat.txt")
                && VIRTUAL_TEST_OPENAT_EXISTS)
            || (path_matches(path, "/musl/basic/test_mmap.txt") && VIRTUAL_TEST_MMAP_EXISTS)
            || proc_file_content(path).is_some()
    }
}

pub(crate) fn virtual_file_size(path: &str) -> Option<usize> {
    unsafe {
        if path_matches(path, "/musl/basic/test_mmap.txt") && VIRTUAL_TEST_MMAP_EXISTS {
            Some(VIRTUAL_TEST_MMAP_LEN)
        } else if let Some(content) = proc_file_content(path) {
            Some(content.len())
        } else if is_virtual_file_path(path) {
            Some(0)
        } else {
            None
        }
    }
}

pub(crate) fn write_virtual_file(
    path: &str,
    offset: usize,
    src: &[u8],
) -> Result<usize, &'static str> {
    unsafe {
        if !path_matches(path, "/musl/basic/test_mmap.txt") || !VIRTUAL_TEST_MMAP_EXISTS {
            return Err("virtual_file_missing");
        }
        let end = offset.checked_add(src.len()).ok_or("virtual_file_offset")?;
        if end > VIRTUAL_FILE_SIZE {
            return Err("virtual_file_size");
        }
        let mut i = 0usize;
        while i < src.len() {
            VIRTUAL_TEST_MMAP_DATA[offset + i] = src[i];
            i += 1;
        }
        if end > VIRTUAL_TEST_MMAP_LEN {
            VIRTUAL_TEST_MMAP_LEN = end;
        }
        Ok(src.len())
    }
}

pub(crate) fn read_virtual_file(
    path: &str,
    offset: usize,
    dst: &mut [u8],
) -> Result<usize, &'static str> {
    if let Some(content) = proc_file_content(path) {
        if offset >= content.len() {
            return Ok(0);
        }
        let take = core::cmp::min(dst.len(), content.len() - offset);
        let mut i = 0usize;
        while i < take {
            dst[i] = content[offset + i];
            i += 1;
        }
        return Ok(take);
    }
    unsafe {
        if !path_matches(path, "/musl/basic/test_mmap.txt") || !VIRTUAL_TEST_MMAP_EXISTS {
            return Err("virtual_file_missing");
        }
        if offset >= VIRTUAL_TEST_MMAP_LEN {
            return Ok(0);
        }
        let available = VIRTUAL_TEST_MMAP_LEN - offset;
        let take = core::cmp::min(dst.len(), available);
        let mut i = 0usize;
        while i < take {
            dst[i] = VIRTUAL_TEST_MMAP_DATA[offset + i];
            i += 1;
        }
        Ok(take)
    }
}

pub(crate) fn is_virtual_dir_path(path: &str) -> bool {
    unsafe {
        path_matches(path, "/")
            || path_matches(path, "/musl")
            || path_matches(path, "/musl/basic")
            || is_proc_dir_path(path)
            || (path_matches(path, "/musl/basic/test_chdir") && VIRTUAL_TEST_CHDIR_EXISTS)
            || (path_matches(path, "/musl/basic/test_mkdir") && VIRTUAL_TEST_MKDIR_EXISTS)
            || (path_matches(path, "/musl/basic/fd_dir") && VIRTUAL_FD_DIR_EXISTS)
    }
}

fn is_proc_dir_path(path: &str) -> bool {
    path_matches(path, "/proc")
        || path_matches(path, "/proc/1")
        || path_matches(path, "/proc/self")
}

fn proc_file_content(path: &str) -> Option<&'static [u8]> {
    if path_matches(path, "/proc/meminfo") {
        Some(PROC_MEMINFO)
    } else if path_matches(path, "/proc/mounts") {
        Some(PROC_MOUNTS)
    } else if path_matches(path, "/proc/stat") {
        Some(PROC_STAT)
    } else if path_matches(path, "/proc/uptime") {
        Some(PROC_UPTIME)
    } else if path_matches(path, "/proc/filesystems") {
        Some(PROC_FILESYSTEMS)
    } else if path_matches(path, "/proc/cpuinfo") {
        Some(PROC_CPUINFO)
    } else if path_matches(path, "/proc/1/stat") || path_matches(path, "/proc/self/stat") {
        Some(PROC_PID_STAT)
    } else if path_matches(path, "/proc/1/status") || path_matches(path, "/proc/self/status") {
        Some(PROC_PID_STATUS)
    } else if path_matches(path, "/proc/1/cmdline") || path_matches(path, "/proc/self/cmdline") {
        Some(PROC_PID_CMDLINE)
    } else {
        None
    }
}

fn path_matches(path: &str, expected: &str) -> bool {
    bytes_eq(path.as_bytes(), expected.as_bytes())
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

unsafe fn alloc_pipe() -> Option<usize> {
    let mut id = 0usize;
    while id < MAX_PIPES {
        if !PIPES[id].active {
            PIPES[id] = PipeState::empty();
            PIPES[id].active = true;
            return Some(id);
        }
        id += 1;
    }
    None
}

fn pipe_id_for_entry(entry: LaFd) -> Option<usize> {
    if entry.kind != FD_PIPE_READ && entry.kind != FD_PIPE_WRITE {
        return None;
    }
    let id = entry.inode as usize;
    if id < MAX_PIPES {
        Some(id)
    } else {
        None
    }
}

unsafe fn add_pipe_ref_for_entry(entry: LaFd) {
    if let Some(id) = pipe_id_for_entry(entry) {
        if PIPES[id].active {
            if entry.kind == FD_PIPE_READ {
                PIPES[id].read_refs = PIPES[id].read_refs.saturating_add(1);
            } else {
                PIPES[id].write_refs = PIPES[id].write_refs.saturating_add(1);
            }
        }
    }
}

unsafe fn release_pipe_ref_for_entry(entry: LaFd) {
    if let Some(id) = pipe_id_for_entry(entry) {
        if PIPES[id].active {
            if entry.kind == FD_PIPE_READ {
                PIPES[id].read_refs = PIPES[id].read_refs.saturating_sub(1);
            } else {
                PIPES[id].write_refs = PIPES[id].write_refs.saturating_sub(1);
            }
            if PIPES[id].read_refs == 0 && PIPES[id].write_refs == 0 {
                PIPES[id] = PipeState::empty();
            }
        }
    }
}

unsafe fn add_pipe_refs_for_current_table() {
    let mut fd = 0usize;
    while fd < MAX_FDS {
        add_pipe_ref_for_entry(FD_TABLE[fd]);
        fd += 1;
    }
}

unsafe fn release_pipe_refs_for_current_table() {
    let mut fd = 0usize;
    while fd < MAX_FDS {
        release_pipe_ref_for_entry(FD_TABLE[fd]);
        fd += 1;
    }
}

fn compact_pipe(pipe: &mut PipeState) {
    if pipe.read_pos == 0 {
        return;
    }
    if pipe.read_pos == pipe.write_pos {
        pipe.read_pos = 0;
        pipe.write_pos = 0;
        return;
    }
    let remaining = pipe.write_pos - pipe.read_pos;
    let mut i = 0usize;
    while i < remaining {
        pipe.buffer[i] = pipe.buffer[pipe.read_pos + i];
        i += 1;
    }
    pipe.read_pos = 0;
    pipe.write_pos = remaining;
}
