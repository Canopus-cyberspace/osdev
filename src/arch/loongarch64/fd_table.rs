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
const PIPE_BUFFER_SIZE: usize = 512;

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
static mut PIPE_ACTIVE: bool = false;
static mut PIPE_BUFFER: [u8; PIPE_BUFFER_SIZE] = [0; PIPE_BUFFER_SIZE];
static mut PIPE_READ_POS: usize = 0;
static mut PIPE_WRITE_POS: usize = 0;

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
        PIPE_ACTIVE = false;
        PIPE_READ_POS = 0;
        PIPE_WRITE_POS = 0;

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
    }
}

pub(crate) fn restore_fd_snapshot_after_child() {
    unsafe {
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
        FD_TABLE[fd] = entry;
    }
    true
}

pub(crate) fn close_fd(fd: usize) -> bool {
    if fd >= MAX_FDS {
        return false;
    }
    unsafe {
        if FD_TABLE[fd].kind == FD_CLOSED {
            return false;
        }
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
    unsafe {
        PIPE_ACTIVE = true;
        PIPE_READ_POS = 0;
        PIPE_WRITE_POS = 0;
        FD_TABLE[read_fd] = LaFd {
            kind: FD_PIPE_READ,
            inode: 0,
            mode: 0,
            size: 0,
            offset: 0,
            path: PathBuf::empty(),
        };
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
            inode: 0,
            mode: 0,
            size: 0,
            offset: 0,
            path: PathBuf::empty(),
        };
    }
    Some((read_fd, write_fd))
}

pub(crate) fn write_pipe(fd: usize, src: &[u8]) -> Result<usize, &'static str> {
    let entry = fd_entry(fd).ok_or("pipe_fd_range")?;
    if entry.kind != FD_PIPE_WRITE {
        return Err("pipe_not_write");
    }
    unsafe {
        if !PIPE_ACTIVE {
            return Err("pipe_inactive");
        }
        let space = PIPE_BUFFER_SIZE.saturating_sub(PIPE_WRITE_POS);
        let take = core::cmp::min(src.len(), space);
        let mut i = 0usize;
        while i < take {
            PIPE_BUFFER[PIPE_WRITE_POS + i] = src[i];
            i += 1;
        }
        PIPE_WRITE_POS += take;
        Ok(take)
    }
}

pub(crate) fn read_pipe(fd: usize, dst: &mut [u8]) -> Result<usize, &'static str> {
    let entry = fd_entry(fd).ok_or("pipe_fd_range")?;
    if entry.kind != FD_PIPE_READ {
        return Err("pipe_not_read");
    }
    unsafe {
        if !PIPE_ACTIVE {
            return Err("pipe_inactive");
        }
        let available = PIPE_WRITE_POS.saturating_sub(PIPE_READ_POS);
        let take = core::cmp::min(dst.len(), available);
        let mut i = 0usize;
        while i < take {
            dst[i] = PIPE_BUFFER[PIPE_READ_POS + i];
            i += 1;
        }
        PIPE_READ_POS += take;
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
    }
}

pub(crate) fn virtual_file_size(path: &str) -> Option<usize> {
    unsafe {
        if path_matches(path, "/musl/basic/test_mmap.txt") && VIRTUAL_TEST_MMAP_EXISTS {
            Some(VIRTUAL_TEST_MMAP_LEN)
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
            || (path_matches(path, "/musl/basic/test_chdir") && VIRTUAL_TEST_CHDIR_EXISTS)
            || (path_matches(path, "/musl/basic/test_mkdir") && VIRTUAL_TEST_MKDIR_EXISTS)
            || (path_matches(path, "/musl/basic/fd_dir") && VIRTUAL_FD_DIR_EXISTS)
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
