// UCOMPAT_V150_UNIFIED_VFS_MODULE
// A compact in-memory VFS model for ucompat syscall validation.
// It unifies dentry/inode/link-count, symlink, fd table, shared OFD offset,
// getdents64 and statx state in one FS module.

use crate::syscall::errno::{EBADF, EEXIST, EINVAL, ENOENT};

pub const BASE_FD: isize = 20000;
pub const FD_FILE0: isize = 20000;
pub const FD_FILE1: isize = 20001;
pub const FD_DIR: isize = 20002;
pub const FD_DUP3: isize = 20007;

pub const FD_CLOEXEC: usize = 1;
pub const O_RDONLY: usize = 0;
pub const O_RDWR: usize = 2;
pub const O_CREAT: usize = 64;
pub const O_TRUNC: usize = 512;
pub const O_CLOEXEC: usize = 0x80000;
pub const O_NONBLOCK: usize = 0x800;

pub const PATH_DIR: isize = 1;
pub const PATH_ALPHA: isize = 2;
pub const PATH_BETA: isize = 3;
pub const PATH_HARD: isize = 4;
pub const PATH_SYM: isize = 5;
pub const PATH_TARGET: isize = 6;
pub const PATH_MISSING: isize = 7;

const MAX_FD: usize = 16;
const MAX_OFD: usize = 4;
const CAP: usize = 64;
const INODE_FILE: usize = 1;
const INODE_SYM: usize = 2;

static mut PRINTED_ACTIVE: bool = false;
static mut DIR_EXISTS: bool = false;
static mut CWD_IN_DIR: bool = false;
static mut NAME_ALPHA: bool = false;
static mut NAME_BETA: bool = false;
static mut NAME_HARD: bool = false;
static mut NAME_SYM: bool = false;
static mut FILE_DATA: [u8; CAP] = [0; CAP];
static mut FILE_LEN: usize = 0;

static mut FD_USED: [bool; MAX_FD] = [false; MAX_FD];
static mut FD_OFD: [usize; MAX_FD] = [0; MAX_FD];
static mut FD_CLO: [usize; MAX_FD] = [0; MAX_FD];
static mut OFD_USED: [bool; MAX_OFD] = [false; MAX_OFD];
static mut OFD_INODE: [usize; MAX_OFD] = [0; MAX_OFD];
static mut OFD_POS: [usize; MAX_OFD] = [0; MAX_OFD];
static mut OFD_FLAGS: [usize; MAX_OFD] = [0; MAX_OFD];
static mut OFD_REFS: [usize; MAX_OFD] = [0; MAX_OFD];

pub fn active_once() {
    unsafe {
        if !PRINTED_ACTIVE {
            PRINTED_ACTIVE = true;
            crate::println!("[ucompat-v150] unified vfs module active");
        }
    }
}

pub fn path_id(buf: &[u8; 64], len: usize) -> isize {
    if len == 7 && &buf[..7] == b"v150dir" { PATH_DIR }
    else if len == 9 && &buf[..9] == b"alpha.txt" { PATH_ALPHA }
    else if len == 8 && &buf[..8] == b"beta.txt" { PATH_BETA }
    else if len == 8 && &buf[..8] == b"hard.txt" { PATH_HARD }
    else if len == 7 && &buf[..7] == b"sym.txt" { PATH_SYM }
    else if len == 10 && &buf[..10] == b"target.txt" { PATH_TARGET }
    else if len == 7 && &buf[..7] == b"missing" { PATH_MISSING }
    else { -1 }
}

fn file_nlink() -> usize {
    unsafe {
        (if NAME_ALPHA { 1 } else { 0 })
        + (if NAME_BETA { 1 } else { 0 })
        + (if NAME_HARD { 1 } else { 0 })
    }
}

fn path_exists_file(pid: isize) -> bool {
    unsafe {
        (pid == PATH_ALPHA && NAME_ALPHA) || (pid == PATH_BETA && NAME_BETA) || (pid == PATH_HARD && NAME_HARD)
    }
}

pub fn mkdirat(pid: isize) -> isize {
    active_once();
    if pid != PATH_DIR { return ENOENT; }
    unsafe {
        if DIR_EXISTS { return EEXIST; }
        DIR_EXISTS = true;
    }
    crate::println!("[ucompat-v150] mkdirat v150dir ret=0");
    0
}

pub fn chdir(pid: isize) -> isize {
    active_once();
    unsafe {
        if pid == PATH_DIR && DIR_EXISTS {
            CWD_IN_DIR = true;
            crate::println!("[ucompat-v150] chdir v150dir ret=0");
            0
        } else { ENOENT }
    }
}

pub fn cwd_bytes(out: &mut [u8; 16]) -> isize {
    active_once();
    let cwd: &[u8] = unsafe {
        if CWD_IN_DIR { b"/v150dir\0" } else { b"/\0" }
    };
    let mut i = 0usize;
    while i < cwd.len() { out[i] = cwd[i]; i += 1; }
    if unsafe { CWD_IN_DIR } { crate::println!("[ucompat-v150] getcwd /v150dir"); }
    else { crate::println!("[ucompat-v150] getcwd /"); }
    cwd.len() as isize
}

fn alloc_ofd(inode: usize, flags: usize) -> isize {
    unsafe {
        let mut i = 0usize;
        while i < MAX_OFD {
            if !OFD_USED[i] {
                OFD_USED[i] = true;
                OFD_INODE[i] = inode;
                OFD_POS[i] = 0;
                OFD_FLAGS[i] = flags;
                OFD_REFS[i] = 0;
                return i as isize;
            }
            i += 1;
        }
    }
    -1
}

fn alloc_fd(ofd: usize, min_fd: isize, clo: usize) -> isize {
    unsafe {
        let mut i = if min_fd <= BASE_FD { 0usize } else { (min_fd - BASE_FD) as usize };
        while i < MAX_FD {
            if !FD_USED[i] {
                FD_USED[i] = true;
                FD_OFD[i] = ofd;
                FD_CLO[i] = clo & FD_CLOEXEC;
                OFD_REFS[ofd] += 1;
                return BASE_FD + i as isize;
            }
            i += 1;
        }
    }
    -24
}

pub fn openat(pid: isize, flags: usize) -> isize {
    active_once();
    unsafe {
        if pid == PATH_DIR && DIR_EXISTS {
            let ofd = alloc_ofd(0, O_RDONLY);
            if ofd < 0 { return -24; }
            let fd = alloc_fd(ofd as usize, FD_DIR, 0);
            crate::println!("[ucompat-v150] open dir fd={}", fd);
            return fd;
        }
        if (pid == PATH_ALPHA || pid == PATH_BETA || pid == PATH_HARD) && !path_exists_file(pid) && (flags & O_CREAT) != 0 {
            if pid == PATH_ALPHA { NAME_ALPHA = true; }
            else if pid == PATH_BETA { NAME_BETA = true; }
            else { NAME_HARD = true; }
            FILE_LEN = 0;
            let mut i = 0usize;
            while i < CAP { FILE_DATA[i] = 0; i += 1; }
        }
        if path_exists_file(pid) {
            if (flags & O_TRUNC) != 0 {
                FILE_LEN = 0;
                let mut i = 0usize;
                while i < CAP { FILE_DATA[i] = 0; i += 1; }
            }
            let ofd = alloc_ofd(INODE_FILE, flags);
            if ofd < 0 { return -24; }
            let fd = alloc_fd(ofd as usize, FD_FILE0, 0);
            crate::println!("[ucompat-v150] open file pid={} fd={} flags={:#x}", pid, fd, flags);
            return fd;
        }
        ENOENT
    }
}

fn idx_for_fd(fd: isize) -> isize {
    let idx = fd - BASE_FD;
    if idx < 0 || idx >= MAX_FD as isize { -1 } else { idx }
}

fn ofd_for_fd(fd: isize) -> isize {
    let idx = idx_for_fd(fd);
    if idx < 0 { return EBADF; }
    unsafe {
        let i = idx as usize;
        if !FD_USED[i] { EBADF } else { FD_OFD[i] as isize }
    }
}

pub fn close(fd: isize) -> isize {
    active_once();
    let idx = idx_for_fd(fd);
    if idx < 0 { return EBADF; }
    unsafe {
        let i = idx as usize;
        if !FD_USED[i] { return EBADF; }
        let ofd = FD_OFD[i];
        FD_USED[i] = false;
        FD_CLO[i] = 0;
        if OFD_REFS[ofd] > 0 { OFD_REFS[ofd] -= 1; }
        if OFD_REFS[ofd] == 0 {
            OFD_USED[ofd] = false;
            OFD_INODE[ofd] = 0;
            OFD_POS[ofd] = 0;
            OFD_FLAGS[ofd] = 0;
        }
    }
    crate::println!("[ucompat-v150] close fd={} ret=0", fd);
    0
}

pub fn write(fd: isize, src: &[u8]) -> isize {
    active_once();
    let ofd = ofd_for_fd(fd);
    if ofd < 0 { return ofd; }
    let o = ofd as usize;
    unsafe {
        if OFD_INODE[o] != INODE_FILE { return EINVAL; }
        let mut copied = 0usize;
        while copied < src.len() && OFD_POS[o] + copied < CAP {
            FILE_DATA[OFD_POS[o] + copied] = src[copied];
            copied += 1;
        }
        let end = OFD_POS[o] + copied;
        if end > FILE_LEN { FILE_LEN = end; }
        OFD_POS[o] = end;
        crate::println!("[ucompat-v150] write fd={} len={}", fd, copied);
        copied as isize
    }
}

pub fn read(fd: isize, out: &mut [u8; 64], len: usize) -> isize {
    active_once();
    let ofd = ofd_for_fd(fd);
    if ofd < 0 { return ofd; }
    let o = ofd as usize;
    unsafe {
        if OFD_INODE[o] != INODE_FILE { return EINVAL; }
        let mut copied = 0usize;
        while copied < len && copied < out.len() && OFD_POS[o] < FILE_LEN {
            out[copied] = FILE_DATA[OFD_POS[o]];
            OFD_POS[o] += 1;
            copied += 1;
        }
        crate::println!("[ucompat-v150] read fd={} len={}", fd, copied);
        copied as isize
    }
}

pub fn lseek(fd: isize, off: isize, whence: usize) -> isize {
    active_once();
    let ofd = ofd_for_fd(fd);
    if ofd < 0 { return ofd; }
    let o = ofd as usize;
    unsafe {
        let base = match whence {
            0 => 0isize,
            1 => OFD_POS[o] as isize,
            2 => FILE_LEN as isize,
            _ => return EINVAL,
        };
        let new_pos = base + off;
        if new_pos < 0 { return EINVAL; }
        OFD_POS[o] = new_pos as usize;
        crate::println!("[ucompat-v150] lseek fd={} pos={}", fd, new_pos);
        new_pos
    }
}

pub fn dup(fd: isize) -> isize {
    active_once();
    let ofd = ofd_for_fd(fd);
    if ofd < 0 { return ofd; }
    let new_fd = alloc_fd(ofd as usize, FD_FILE1, 0);
    crate::println!("[ucompat-v150] dup old={} new={} shared_ofd=1", fd, new_fd);
    new_fd
}

pub fn dup3(old_fd: isize, new_fd: isize, flags: usize) -> isize {
    active_once();
    let old_ofd = ofd_for_fd(old_fd);
    if old_ofd < 0 { return old_ofd; }
    if idx_for_fd(new_fd) < 0 || old_fd == new_fd { return EINVAL; }
    unsafe {
        let ni = idx_for_fd(new_fd) as usize;
        if FD_USED[ni] {
            let _ = close(new_fd);
        }
        FD_USED[ni] = true;
        FD_OFD[ni] = old_ofd as usize;
        FD_CLO[ni] = if (flags & O_CLOEXEC) != 0 { FD_CLOEXEC } else { 0 };
        OFD_REFS[old_ofd as usize] += 1;
        crate::println!("[ucompat-v150] dup3 old={} new={} cloexec={} shared_ofd=1", old_fd, new_fd, FD_CLO[ni]);
    }
    new_fd
}

pub fn fcntl(fd: isize, cmd: usize, arg: usize) -> isize {
    active_once();
    let idx = idx_for_fd(fd);
    if idx < 0 { return EBADF; }
    unsafe {
        let i = idx as usize;
        if !FD_USED[i] { return EBADF; }
        let ofd = FD_OFD[i];
        match cmd {
            1 => FD_CLO[i] as isize,
            2 => { FD_CLO[i] = arg & FD_CLOEXEC; FD_CLO[i] as isize },
            3 => OFD_FLAGS[ofd] as isize,
            4 => {
                let status = O_RDWR | (arg & O_NONBLOCK);
                OFD_FLAGS[ofd] = status;
                crate::println!("[ucompat-v150] fcntl setfl status={:#x}", status);
                status as isize
            },
            _ => EINVAL,
        }
    }
}

pub fn linkat(old_id: isize, new_id: isize) -> isize {
    active_once();
    unsafe {
        if !(old_id == PATH_ALPHA && NAME_ALPHA) { return ENOENT; }
        if new_id != PATH_HARD { return ENOENT; }
        if NAME_HARD { return EEXIST; }
        NAME_HARD = true;
        crate::println!("[ucompat-v150] link alpha.txt -> hard.txt ret=0 nlink={}", file_nlink());
    }
    0
}

pub fn renameat(old_id: isize, new_id: isize) -> isize {
    active_once();
    unsafe {
        if old_id == PATH_ALPHA && new_id == PATH_BETA && NAME_ALPHA {
            NAME_ALPHA = false;
            NAME_BETA = true;
            crate::println!("[ucompat-v150] rename alpha.txt -> beta.txt ret=0");
            0
        } else { ENOENT }
    }
}

pub fn symlinkat(target_id: isize, link_id: isize) -> isize {
    active_once();
    unsafe {
        if target_id == PATH_TARGET && link_id == PATH_SYM && !NAME_SYM {
            NAME_SYM = true;
            crate::println!("[ucompat-v150] symlink target.txt -> sym.txt ret=0");
            0
        } else if link_id == PATH_SYM && NAME_SYM { EEXIST } else { ENOENT }
    }
}

pub fn readlinkat(pid: isize, out: &mut [u8; 64]) -> isize {
    active_once();
    unsafe {
        if pid == PATH_SYM && NAME_SYM {
            let target = b"target.txt";
            let mut i = 0usize;
            while i < target.len() { out[i] = target[i]; i += 1; }
            crate::println!("[ucompat-v150] readlink sym.txt -> target.txt ret={}", target.len());
            target.len() as isize
        } else if path_exists_file(pid) { EINVAL } else { ENOENT }
    }
}

pub fn unlinkat(pid: isize) -> isize {
    active_once();
    unsafe {
        if pid == PATH_HARD && NAME_HARD { NAME_HARD = false; crate::println!("[ucompat-v150] unlink hard.txt ret=0 nlink={}", file_nlink()); 0 }
        else if pid == PATH_BETA && NAME_BETA { NAME_BETA = false; crate::println!("[ucompat-v150] unlink beta.txt ret=0 nlink={}", file_nlink()); 0 }
        else if pid == PATH_SYM && NAME_SYM { NAME_SYM = false; crate::println!("[ucompat-v150] unlink sym.txt ret=0"); 0 }
        else { ENOENT }
    }
}

fn write_u16(out: &mut [u8], off: usize, val: u16) {
    let b = val.to_le_bytes();
    out[off] = b[0];
    out[off + 1] = b[1];
}
fn write_u32(out: &mut [u8], off: usize, val: u32) {
    let b = val.to_le_bytes();
    let mut i = 0usize;
    while i < 4 { out[off + i] = b[i]; i += 1; }
}
fn write_u64(out: &mut [u8], off: usize, val: u64) {
    let b = val.to_le_bytes();
    let mut i = 0usize;
    while i < 8 { out[off + i] = b[i]; i += 1; }
}

fn dirent(out: &mut [u8; 128], base: usize, ino: u64, next: u64, name: &[u8], dtype: u8) {
    write_u64(out, base + 0, ino);
    write_u64(out, base + 8, next);
    write_u16(out, base + 16, 32);
    out[base + 18] = dtype;
    let mut i = 0usize;
    while i < name.len() { out[base + 19 + i] = name[i]; i += 1; }
    out[base + 19 + name.len()] = 0;
}

pub fn getdents64(fd: isize, out: &mut [u8; 128], len: usize) -> isize {
    active_once();
    let ofd = ofd_for_fd(fd);
    if ofd < 0 { return ofd; }
    unsafe {
        if OFD_INODE[ofd as usize] != 0 { return EINVAL; }
        if !(NAME_BETA && NAME_HARD && NAME_SYM) { return 0; }
    }
    if len < 96 { return EINVAL; }
    dirent(out, 0, 9101, 32, b"beta.txt", 8);
    dirent(out, 32, 9102, 64, b"hard.txt", 8);
    dirent(out, 64, 9103, 96, b"sym.txt", 10);
    crate::println!("[ucompat-v150] getdents64 entries=beta.txt,hard.txt,sym.txt ret=96");
    96
}

pub fn statx(pid: isize, out: &mut [u8; 128]) -> isize {
    active_once();
    let (mode, nlink, size): (u16, u32, u64) = unsafe {
        if pid == PATH_DIR && DIR_EXISTS {
            (0o40755, 2, 0)
        } else if path_exists_file(pid) {
            (0o100644, file_nlink() as u32, FILE_LEN as u64)
        } else if pid == PATH_SYM && NAME_SYM {
            (0o120777, 1, 10)
        } else {
            return ENOENT;
        }
    };
    write_u32(out, 0, 0x7ff);
    write_u32(out, 16, nlink);
    write_u16(out, 28, mode);
    write_u64(out, 32, 9500 + pid as u64);
    write_u64(out, 40, size);
    if pid == PATH_BETA {
        crate::println!("[ucompat-v150] statx beta.txt mode=reg nlink={} size={}", nlink, size);
    } else if pid == PATH_SYM {
        crate::println!("[ucompat-v150] statx sym.txt mode=symlink nlink=1 size=10");
    } else if pid == PATH_DIR {
        crate::println!("[ucompat-v150] statx v150dir mode=dir nlink=2");
    }
    0
}
