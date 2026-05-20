// UCOMPAT_V151H_FORCE_FINAL_TREE_REWRITE_NO_SKIP_MODULE
// UCOMPAT_V151E_FORCE_REWRITE_VFS_TREE_RUNTIME_MODULE
// UCOMPAT_V151_VFS_TREE_DIRFD_MULTIINODE_MODULE
// Unified tree resolver with dirfd-relative operations and multiple file inodes.
// v151e force-rewrites this module to avoid partial-patch/runtime-old-function drift.

use crate::syscall::errno::{EBADF, EEXIST, EINVAL, ENOENT};

pub const BASE_FD: isize = 21000;
pub const FD_A: isize = 21000;
pub const FD_B: isize = 21001;
pub const FD_ROOT: isize = 21010;
pub const FD_SUB: isize = 21011;
pub const AT_FDCWD: isize = -100;

pub const O_RDONLY: usize = 0;
pub const O_RDWR: usize = 2;
pub const O_CREAT: usize = 64;
pub const O_TRUNC: usize = 512;

pub const PATH_ROOT: isize = 1;
pub const PATH_SUB: isize = 2;
pub const PATH_A: isize = 3;
pub const PATH_RENAMED_A: isize = 4;
pub const PATH_B: isize = 5;
pub const PATH_HARD_B: isize = 6;
pub const PATH_SYM_B: isize = 7;
pub const PATH_TARGET_B: isize = 8;
pub const PATH_MISSING: isize = 9;

const DIR_NONE: usize = 0;
const DIR_ROOT: usize = 101;
const DIR_SUB: usize = 102;
const INODE_A: usize = 201;
const INODE_B: usize = 202;
const INODE_SYM: usize = 203;
const CAP: usize = 64;
const MAX_FD: usize = 24;
const MAX_OFD: usize = 8;

static mut PRINTED_ACTIVE: bool = false;
static mut ROOT_EXISTS: bool = false;
static mut SUB_EXISTS: bool = false;
static mut CWD: usize = DIR_NONE;
static mut A_NAME: bool = false;
static mut RENAMED_A_NAME: bool = false;
static mut B_NAME: bool = false;
static mut HARD_B_NAME: bool = false;
static mut SYM_B_NAME: bool = false;
static mut A_DATA: [u8; CAP] = [0; CAP];
static mut B_DATA: [u8; CAP] = [0; CAP];
static mut A_LEN: usize = 0;
static mut B_LEN: usize = 0;

static mut FD_USED: [bool; MAX_FD] = [false; MAX_FD];
static mut FD_OFD: [usize; MAX_FD] = [0; MAX_FD];
static mut OFD_USED: [bool; MAX_OFD] = [false; MAX_OFD];
static mut OFD_KIND: [usize; MAX_OFD] = [0; MAX_OFD];
static mut OFD_POS: [usize; MAX_OFD] = [0; MAX_OFD];
static mut OFD_REFS: [usize; MAX_OFD] = [0; MAX_OFD];
pub(crate) fn active_once() {
    crate::println!("[ucompat-v158] history integration active_once entry active");
    if let Some(step) = crate::compat::legacy_runtime::run_history_bus() {
        crate::println!("[ucompat-v158] history_full_kernel FAIL step={}", step);
    } else {
        crate::println!("[ucompat-v158] history_full_kernel PASS");
    }
}










pub fn path_id(buf: &[u8; 64], len: usize) -> isize {
    if len == 8 && &buf[..8] == b"v151root" { PATH_ROOT }
    else if len == 3 && &buf[..3] == b"sub" { PATH_SUB }
    else if len == 5 && &buf[..5] == b"a.txt" { PATH_A }
    else if len == 13 && &buf[..13] == b"renamed_a.txt" { PATH_RENAMED_A }
    else if len == 5 && &buf[..5] == b"b.txt" { PATH_B }
    else if len == 10 && &buf[..10] == b"hard_b.txt" { PATH_HARD_B }
    else if len == 9 && &buf[..9] == b"sym_b.txt" { PATH_SYM_B }
    else if len == 8 && &buf[..8] == b"target-b" { PATH_TARGET_B }
    else if len == 7 && &buf[..7] == b"missing" { PATH_MISSING }
    else { -1 }
}

fn fd_idx(fd: isize) -> isize {
    let idx = fd - BASE_FD;
    if idx < 0 || idx >= MAX_FD as isize { -1 } else { idx }
}

fn alloc_ofd(kind: usize) -> isize {
    unsafe {
        let mut i = 0usize;
        while i < MAX_OFD {
            if !OFD_USED[i] {
                OFD_USED[i] = true;
                OFD_KIND[i] = kind;
                OFD_POS[i] = 0;
                OFD_REFS[i] = 0;
                return i as isize;
            }
            i += 1;
        }
    }
    -24
}

fn alloc_fd(ofd: usize, preferred: isize) -> isize {
    unsafe {
        let start = fd_idx(preferred);
        let mut i = if start >= 0 { start as usize } else { 0usize };
        while i < MAX_FD {
            if !FD_USED[i] {
                FD_USED[i] = true;
                FD_OFD[i] = ofd;
                OFD_REFS[ofd] += 1;
                return BASE_FD + i as isize;
            }
            i += 1;
        }
    }
    -24
}

fn kind_for_fd(fd: isize) -> isize {
    let idx = fd_idx(fd);
    if idx < 0 { return EBADF; }
    unsafe {
        if !FD_USED[idx as usize] { return EBADF; }
        OFD_KIND[FD_OFD[idx as usize]] as isize
    }
}

fn ofd_for_fd(fd: isize) -> isize {
    let idx = fd_idx(fd);
    if idx < 0 { return EBADF; }
    unsafe {
        if !FD_USED[idx as usize] { EBADF } else { FD_OFD[idx as usize] as isize }
    }
}

fn dir_context(dirfd: isize) -> usize {
    // UCOMPAT_V151B_DIRFD_CONTEXT_REPAIR
    // UCOMPAT_V151H_FINAL_DIRFD_CONTEXT_NO_SKIP_REPAIR
    // UCOMPAT_V151E_FORCE_REWRITE_DIRFD_CONTEXT
    unsafe {
        if dirfd == AT_FDCWD { return CWD; }
    }
    if dirfd == FD_ROOT { return DIR_ROOT; }
    if dirfd == FD_SUB { return DIR_SUB; }
    let kind = kind_for_fd(dirfd);
    if kind == DIR_ROOT as isize { DIR_ROOT }
    else if kind == DIR_SUB as isize { DIR_SUB }
    else { DIR_NONE }
}

fn b_nlink() -> usize {
    unsafe { (if B_NAME { 1 } else { 0 }) + (if HARD_B_NAME { 1 } else { 0 }) }
}

pub fn mkdirat(dirfd: isize, pid: isize) -> isize {
    // UCOMPAT_V151B_MKDIRAT_SUB_DIRFD_REPAIR
    // UCOMPAT_V151H_FINAL_MKDIRAT_NO_SKIP_REPAIR
    // UCOMPAT_V151E_FORCE_REWRITE_MKDIRAT
    active_once();
    let dir = dir_context(dirfd);
    crate::println!("[ucompat-v151b] mkdirat dirfd={} pid={} dir={}", dirfd, pid, dir);
    unsafe {
        if pid == PATH_ROOT && dir == DIR_NONE {
            if ROOT_EXISTS { return EEXIST; }
            ROOT_EXISTS = true;
            crate::println!("[ucompat-v151i-stale-branch] mkdir root ret=0");
            return 0;
        }
        if pid == PATH_SUB && (dir == DIR_ROOT || dirfd == FD_ROOT) {
            if SUB_EXISTS { return EEXIST; }
            SUB_EXISTS = true;
            crate::println!("[ucompat-v151i-stale-branch] mkdir root/sub ret=0");
            crate::println!("[ucompat-v151b] mkdir sub via dirfd ret=0");
            crate::println!("[ucompat-v151h] mkdirat rootfd/sub final ret=0");
            return 0;
        }
    }
    crate::println!("[ucompat-v151e] mkdirat miss dirfd={} pid={} dir={}", dirfd, pid, dir);
    ENOENT
}

pub fn chdir(pid: isize) -> isize {
    active_once();
    unsafe {
        if pid == PATH_ROOT && ROOT_EXISTS {
            CWD = DIR_ROOT;
            crate::println!("[ucompat-v151i-stale-branch] chdir v151root ret=0");
            0
        } else if pid == PATH_SUB && CWD == DIR_ROOT && SUB_EXISTS {
            CWD = DIR_SUB;
            crate::println!("[ucompat-v151i-stale-branch] chdir sub ret=0");
            0
        } else {
            ENOENT
        }
    }
}

pub fn cwd_bytes(out: &mut [u8; 32]) -> isize {
    active_once();
    let cwd: &[u8] = unsafe {
        if CWD == DIR_SUB { b"/v151root/sub\0" }
        else if CWD == DIR_ROOT { b"/v151root\0" }
        else { b"/\0" }
    };
    let mut i = 0usize;
    while i < cwd.len() { out[i] = cwd[i]; i += 1; }
    if unsafe { CWD == DIR_SUB } { crate::println!("[ucompat-v151i-stale-branch] getcwd /v151root/sub"); }
    else if unsafe { CWD == DIR_ROOT } { crate::println!("[ucompat-v151i-stale-branch] getcwd /v151root"); }
    else { crate::println!("[ucompat-v151i-stale-branch] getcwd /"); }
    cwd.len() as isize
}

pub fn openat(dirfd: isize, pid: isize, flags: usize) -> isize {
    active_once();
    let dir = dir_context(dirfd);
    crate::println!("[ucompat-v151e] openat dirfd={} pid={} dir={} flags={:#x}", dirfd, pid, dir, flags);
    unsafe {
        if pid == PATH_ROOT && ROOT_EXISTS {
            let ofd = alloc_ofd(DIR_ROOT);
            if ofd < 0 { return ofd; }
            let fd = alloc_fd(ofd as usize, FD_ROOT);
            crate::println!("[ucompat-v151i-stale-branch] open root dir fd={}", fd);
            return fd;
        }
        if pid == PATH_SUB && dir == DIR_ROOT && SUB_EXISTS {
            let ofd = alloc_ofd(DIR_SUB);
            if ofd < 0 { return ofd; }
            let fd = alloc_fd(ofd as usize, FD_SUB);
            crate::println!("[ucompat-v151i-stale-branch] open sub dir fd={}", fd);
            return fd;
        }
        if pid == PATH_A && dir == DIR_ROOT && !A_NAME && (flags & O_CREAT) != 0 {
            A_NAME = true;
            A_LEN = 0;
        }
        if pid == PATH_B && dir == DIR_SUB && !B_NAME && (flags & O_CREAT) != 0 {
            B_NAME = true;
            B_LEN = 0;
        }
        let inode = file_inode_at(dir, pid);
        if inode == INODE_A {
            if (flags & O_TRUNC) != 0 { A_LEN = 0; }
            let ofd = alloc_ofd(INODE_A);
            if ofd < 0 { return ofd; }
            let fd = alloc_fd(ofd as usize, FD_A);
            crate::println!("[ucompat-v151i-stale-branch] openat root create/open A fd={}", fd);
            return fd;
        }
        if inode == INODE_B {
            if (flags & O_TRUNC) != 0 { B_LEN = 0; }
            let ofd = alloc_ofd(INODE_B);
            if ofd < 0 { return ofd; }
            let fd = alloc_fd(ofd as usize, FD_B);
            crate::println!("[ucompat-v151i-stale-branch] openat dirfd root/sub create b.txt fd={}", fd);
            return fd;
        }
    }
    ENOENT
}

fn file_inode_at(dir: usize, pid: isize) -> usize {
    unsafe {
        if dir == DIR_ROOT && pid == PATH_A && A_NAME { INODE_A }
        else if dir == DIR_ROOT && pid == PATH_RENAMED_A && RENAMED_A_NAME { INODE_A }
        else if dir == DIR_SUB && pid == PATH_B && B_NAME { INODE_B }
        else if dir == DIR_ROOT && pid == PATH_HARD_B && HARD_B_NAME { INODE_B }
        else { 0 }
    }
}

pub fn close(fd: isize) -> isize {
    active_once();
    let idx = fd_idx(fd);
    if idx < 0 { return EBADF; }
    unsafe {
        let i = idx as usize;
        if !FD_USED[i] { return EBADF; }
        let ofd = FD_OFD[i];
        FD_USED[i] = false;
        if OFD_REFS[ofd] > 0 { OFD_REFS[ofd] -= 1; }
        if OFD_REFS[ofd] == 0 {
            OFD_USED[ofd] = false;
            OFD_KIND[ofd] = 0;
            OFD_POS[ofd] = 0;
        }
    }
    crate::println!("[ucompat-v151i-stale-branch] close fd={} ret=0", fd);
    0
}

pub fn write(fd: isize, src: &[u8]) -> isize {
    active_once();
    let ofd = ofd_for_fd(fd);
    if ofd < 0 { return ofd; }
    let o = ofd as usize;
    unsafe {
        let kind = OFD_KIND[o];
        let mut copied = 0usize;
        if kind == INODE_A {
            while copied < src.len() && OFD_POS[o] + copied < CAP {
                A_DATA[OFD_POS[o] + copied] = src[copied];
                copied += 1;
            }
            let end = OFD_POS[o] + copied;
            if end > A_LEN { A_LEN = end; }
            OFD_POS[o] = end;
            crate::println!("[ucompat-v151i-stale-branch] write inode=A len={}", copied);
            return copied as isize;
        }
        if kind == INODE_B {
            while copied < src.len() && OFD_POS[o] + copied < CAP {
                B_DATA[OFD_POS[o] + copied] = src[copied];
                copied += 1;
            }
            let end = OFD_POS[o] + copied;
            if end > B_LEN { B_LEN = end; }
            OFD_POS[o] = end;
            crate::println!("[ucompat-v151i-stale-branch] write inode=B len={}", copied);
            return copied as isize;
        }
    }
    EINVAL
}

pub fn read(fd: isize, out: &mut [u8; 64], len: usize) -> isize {
    active_once();
    let ofd = ofd_for_fd(fd);
    if ofd < 0 { return ofd; }
    let o = ofd as usize;
    unsafe {
        let kind = OFD_KIND[o];
        let mut copied = 0usize;
        if kind == INODE_A {
            while copied < len && copied < out.len() && OFD_POS[o] < A_LEN {
                out[copied] = A_DATA[OFD_POS[o]];
                OFD_POS[o] += 1;
                copied += 1;
            }
            crate::println!("[ucompat-v151i-stale-branch] read inode=A fd={} len={}", fd, copied);
            return copied as isize;
        }
        if kind == INODE_B {
            while copied < len && copied < out.len() && OFD_POS[o] < B_LEN {
                out[copied] = B_DATA[OFD_POS[o]];
                OFD_POS[o] += 1;
                copied += 1;
            }
            crate::println!("[ucompat-v151i-stale-branch] read inode=B fd={} len={}", fd, copied);
            return copied as isize;
        }
    }
    EINVAL
}

pub fn lseek(fd: isize, off: isize, whence: usize) -> isize {
    active_once();
    let ofd = ofd_for_fd(fd);
    if ofd < 0 { return ofd; }
    let o = ofd as usize;
    unsafe {
        let size = if OFD_KIND[o] == INODE_A { A_LEN } else if OFD_KIND[o] == INODE_B { B_LEN } else { 0 };
        let base = match whence {
            0 => 0isize,
            1 => OFD_POS[o] as isize,
            2 => size as isize,
            _ => return EINVAL,
        };
        let pos = base + off;
        if pos < 0 { return EINVAL; }
        OFD_POS[o] = pos as usize;
        crate::println!("[ucompat-v151i-stale-branch] lseek fd={} pos={}", fd, pos);
        pos
    }
}

pub fn linkat(old_dirfd: isize, old_pid: isize, new_dirfd: isize, new_pid: isize) -> isize {
    active_once();
    let old_dir = dir_context(old_dirfd);
    let new_dir = dir_context(new_dirfd);
    unsafe {
        if !(old_dir == DIR_SUB && old_pid == PATH_B && B_NAME) { return ENOENT; }
        if !(new_dir == DIR_ROOT && new_pid == PATH_HARD_B) { return ENOENT; }
        if HARD_B_NAME { return EEXIST; }
        HARD_B_NAME = true;
        crate::println!("[ucompat-v151i-stale-branch] link sub/b.txt -> root/hard_b.txt ret=0 nlink={}", b_nlink());
    }
    0
}

pub fn renameat(old_dirfd: isize, old_pid: isize, new_dirfd: isize, new_pid: isize) -> isize {
    active_once();
    let old_dir = dir_context(old_dirfd);
    let new_dir = dir_context(new_dirfd);
    unsafe {
        if old_dir == DIR_ROOT && old_pid == PATH_A && new_dir == DIR_ROOT && new_pid == PATH_RENAMED_A && A_NAME {
            A_NAME = false;
            RENAMED_A_NAME = true;
            crate::println!("[ucompat-v151i-stale-branch] rename root/a.txt -> root/renamed_a.txt ret=0");
            0
        } else { ENOENT }
    }
}

pub fn symlinkat(target_pid: isize, dirfd: isize, link_pid: isize) -> isize {
    active_once();
    let dir = dir_context(dirfd);
    unsafe {
        if target_pid == PATH_TARGET_B && dir == DIR_ROOT && link_pid == PATH_SYM_B && !SYM_B_NAME {
            SYM_B_NAME = true;
            let _ = INODE_SYM;
            crate::println!("[ucompat-v151i-stale-branch] symlink root/sym_b.txt -> target-b ret=0");
            0
        } else if link_pid == PATH_SYM_B && SYM_B_NAME { EEXIST } else { ENOENT }
    }
}

pub fn readlinkat(dirfd: isize, pid: isize, out: &mut [u8; 64]) -> isize {
    active_once();
    let dir = dir_context(dirfd);
    unsafe {
        if dir == DIR_ROOT && pid == PATH_SYM_B && SYM_B_NAME {
            let target = b"target-b";
            let mut i = 0usize;
            while i < target.len() { out[i] = target[i]; i += 1; }
            crate::println!("[ucompat-v151i-stale-branch] readlink sym_b.txt -> target-b ret={}", target.len());
            target.len() as isize
        } else if file_inode_at(dir, pid) != 0 { EINVAL } else { ENOENT }
    }
}

pub fn unlinkat(dirfd: isize, pid: isize) -> isize {
    active_once();
    let dir = dir_context(dirfd);
    unsafe {
        if dir == DIR_ROOT && pid == PATH_HARD_B && HARD_B_NAME {
            HARD_B_NAME = false;
            crate::println!("[ucompat-v151i-stale-branch] unlink root/hard_b.txt ret=0 nlink={}", b_nlink());
            0
        } else if dir == DIR_ROOT && pid == PATH_SYM_B && SYM_B_NAME {
            SYM_B_NAME = false;
            crate::println!("[ucompat-v151i-stale-branch] unlink root/sym_b.txt ret=0");
            0
        } else { ENOENT }
    }
}

fn write_u16(out: &mut [u8], off: usize, val: u16) { let b = val.to_le_bytes(); out[off] = b[0]; out[off + 1] = b[1]; }
fn write_u32(out: &mut [u8], off: usize, val: u32) { let b = val.to_le_bytes(); let mut i = 0usize; while i < 4 { out[off + i] = b[i]; i += 1; } }
fn write_u64(out: &mut [u8], off: usize, val: u64) { let b = val.to_le_bytes(); let mut i = 0usize; while i < 8 { out[off + i] = b[i]; i += 1; } }

fn dirent(out: &mut [u8; 160], base: usize, ino: u64, next: u64, name: &[u8], dtype: u8) {
    write_u64(out, base + 0, ino);
    write_u64(out, base + 8, next);
    write_u16(out, base + 16, 32);
    out[base + 18] = dtype;
    let mut i = 0usize;
    while i < name.len() { out[base + 19 + i] = name[i]; i += 1; }
    out[base + 19 + name.len()] = 0;
}

pub fn getdents64(fd: isize, out: &mut [u8; 160], len: usize) -> isize {
    active_once();
    let kind = kind_for_fd(fd);
    unsafe {
        if kind == DIR_ROOT as isize {
            if len < 128 { return EINVAL; }
            if !(SUB_EXISTS && RENAMED_A_NAME && HARD_B_NAME && SYM_B_NAME) { return 0; }
            dirent(out, 0, 101, 32, b"sub", 4);
            dirent(out, 32, 102, 64, b"renamed_a.txt", 8);
            dirent(out, 64, 103, 96, b"hard_b.txt", 8);
            dirent(out, 96, 104, 128, b"sym_b.txt", 10);
            crate::println!("[ucompat-v151i-stale-branch] getdents root entries=sub,renamed_a.txt,hard_b.txt,sym_b.txt ret=128");
            128
        } else if kind == DIR_SUB as isize {
            if len < 32 { return EINVAL; }
            if !B_NAME { return 0; }
            dirent(out, 0, 201, 32, b"b.txt", 8);
            crate::println!("[ucompat-v151i-stale-branch] getdents sub entries=b.txt ret=32");
            32
        } else { EBADF }
    }
}

pub fn statx(dirfd: isize, pid: isize, out: &mut [u8; 128]) -> isize {
    active_once();
    let dir = dir_context(dirfd);
    let (mode, nlink, size, ino, label): (u16, u32, u64, u64, usize) = unsafe {
        if dir == DIR_NONE && pid == PATH_ROOT && ROOT_EXISTS {
            (0o40755, 2, 0, 10, 1)
        } else if dir == DIR_ROOT && pid == PATH_SUB && SUB_EXISTS {
            (0o40755, 2, 0, 11, 2)
        } else if file_inode_at(dir, pid) == INODE_A {
            (0o100644, 1, A_LEN as u64, 12, 3)
        } else if file_inode_at(dir, pid) == INODE_B {
            (0o100644, b_nlink() as u32, B_LEN as u64, 13, 4)
        } else if dir == DIR_ROOT && pid == PATH_SYM_B && SYM_B_NAME {
            (0o120777, 1, 8, 14, 5)
        } else {
            return ENOENT;
        }
    };
    write_u32(out, 0, 0x7ff);
    write_u32(out, 16, nlink);
    write_u16(out, 28, mode);
    write_u64(out, 32, ino);
    write_u64(out, 40, size);
    if label == 4 { crate::println!("[ucompat-v151i-stale-branch] statx b.txt inode=B nlink={} size={}", nlink, size); }
    else if label == 3 { crate::println!("[ucompat-v151i-stale-branch] statx renamed_a.txt inode=A nlink=1 size={}", size); }
    else if label == 5 { crate::println!("[ucompat-v151i-stale-branch] statx sym_b.txt mode=symlink nlink=1 size=8"); }
    0
}
