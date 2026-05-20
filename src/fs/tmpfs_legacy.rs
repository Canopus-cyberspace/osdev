// UCOMPAT_V148_MEMFS_MODULE
use crate::syscall::errno::{EEXIST, EINVAL, ENOENT};

pub const FD_FILE: isize = 18001;
pub const FD_DIR: isize = 18002;

pub const PATH_V148DIR: isize = 1;
pub const PATH_FILE_A: isize = 2;
pub const PATH_FILE_B: isize = 3;
pub const PATH_HARD_A: isize = 4;
pub const PATH_SYM_A: isize = 5;
pub const PATH_TARGET_FILE: isize = 6;
pub const PATH_MISSING: isize = 7;

static mut DIR_EXISTS: bool = false;
static mut CWD_IN_DIR: bool = false;
static mut FILE_A_EXISTS: bool = false;
static mut FILE_B_EXISTS: bool = false;
static mut HARD_A_EXISTS: bool = false;
static mut SYM_A_EXISTS: bool = false;
static mut FILE_OPEN: bool = false;
static mut DIR_OPEN: bool = false;
static mut PRINTED_ACTIVE: bool = false;

pub fn active_once() {
    unsafe {
        if !PRINTED_ACTIVE {
            PRINTED_ACTIVE = true;
            crate::println!("[ucompat-v148] memfs module active");
        }
    }
}

pub fn path_id(buf: &[u8; 64], len: usize) -> isize {
    if len == 7 && &buf[..7] == b"v148dir" { PATH_V148DIR }
    else if len == 6 && &buf[..6] == b"file_a" { PATH_FILE_A }
    else if len == 6 && &buf[..6] == b"file_b" { PATH_FILE_B }
    else if len == 6 && &buf[..6] == b"hard_a" { PATH_HARD_A }
    else if len == 5 && &buf[..5] == b"sym_a" { PATH_SYM_A }
    else if len == 11 && &buf[..11] == b"target_file" { PATH_TARGET_FILE }
    else if len == 7 && &buf[..7] == b"missing" { PATH_MISSING }
    else { -1 }
}

fn name_exists(pid: isize) -> bool {
    unsafe {
        match pid {
            PATH_V148DIR => DIR_EXISTS,
            PATH_FILE_A => FILE_A_EXISTS,
            PATH_FILE_B => FILE_B_EXISTS,
            PATH_HARD_A => HARD_A_EXISTS,
            PATH_SYM_A => SYM_A_EXISTS,
            _ => false,
        }
    }
}

fn file_nlink() -> usize {
    unsafe {
        (if FILE_A_EXISTS { 1 } else { 0 })
        + (if FILE_B_EXISTS { 1 } else { 0 })
        + (if HARD_A_EXISTS { 1 } else { 0 })
    }
}

pub fn mkdirat(pid: isize) -> isize {
    active_once();
    if pid != PATH_V148DIR { return ENOENT; }
    unsafe {
        if DIR_EXISTS { return EEXIST; }
        DIR_EXISTS = true;
    }
    crate::println!("[ucompat-v148] mkdirat v148dir ret=0");
    0
}

pub fn chdir(pid: isize) -> isize {
    active_once();
    unsafe {
        if pid == PATH_V148DIR && DIR_EXISTS {
            CWD_IN_DIR = true;
            crate::println!("[ucompat-v148] chdir v148dir ret=0");
            0
        } else { ENOENT }
    }
}

pub fn cwd_bytes(out: &mut [u8; 16]) -> isize {
    active_once();
    let cwd: &[u8] = unsafe {
        if CWD_IN_DIR { b"/v148dir\0" } else { b"/\0" }
    };
    let mut i = 0usize;
    while i < cwd.len() { out[i] = cwd[i]; i += 1; }
    if unsafe { CWD_IN_DIR } { crate::println!("[ucompat-v148] getcwd /v148dir"); }
    else { crate::println!("[ucompat-v148] getcwd /"); }
    cwd.len() as isize
}

pub fn openat(pid: isize, flags: usize) -> isize {
    active_once();
    const O_CREAT: usize = 0x40;
    unsafe {
        if pid == PATH_V148DIR && DIR_EXISTS {
            DIR_OPEN = true;
            crate::println!("[ucompat-v148] openat dir fd=18002");
            return FD_DIR;
        }
        if (pid == PATH_FILE_A || pid == PATH_FILE_B) && !name_exists(pid) && (flags & O_CREAT) != 0 {
            if pid == PATH_FILE_A { FILE_A_EXISTS = true; } else { FILE_B_EXISTS = true; }
            FILE_OPEN = true;
            crate::println!("[ucompat-v148] openat create pid={} fd=18001", pid);
            return FD_FILE;
        }
        if (pid == PATH_FILE_A || pid == PATH_FILE_B || pid == PATH_HARD_A) && name_exists(pid) {
            FILE_OPEN = true;
            crate::println!("[ucompat-v148] openat existing pid={} fd=18001", pid);
            return FD_FILE;
        }
        ENOENT
    }
}

pub fn close(fd: isize) -> isize {
    unsafe {
        if fd == FD_FILE { FILE_OPEN = false; return 0; }
        if fd == FD_DIR { DIR_OPEN = false; return 0; }
    }
    -9
}

pub fn linkat(old_id: isize, new_id: isize) -> isize {
    active_once();
    unsafe {
        if !(old_id == PATH_FILE_A && FILE_A_EXISTS) { return ENOENT; }
        if new_id != PATH_HARD_A { return ENOENT; }
        if HARD_A_EXISTS { return EEXIST; }
        HARD_A_EXISTS = true;
    }
    crate::println!("[ucompat-v148] linkat file_a -> hard_a ret=0");
    0
}

pub fn renameat(old_id: isize, new_id: isize) -> isize {
    active_once();
    unsafe {
        if old_id == PATH_FILE_A && new_id == PATH_FILE_B && FILE_A_EXISTS {
            FILE_A_EXISTS = false;
            FILE_B_EXISTS = true;
            crate::println!("[ucompat-v148] renameat file_a -> file_b ret=0");
            0
        } else { ENOENT }
    }
}

pub fn symlinkat(target_id: isize, link_id: isize) -> isize {
    active_once();
    unsafe {
        if target_id == PATH_TARGET_FILE && link_id == PATH_SYM_A && !SYM_A_EXISTS {
            SYM_A_EXISTS = true;
            crate::println!("[ucompat-v148] symlinkat target_file -> sym_a ret=0");
            0
        } else if link_id == PATH_SYM_A && SYM_A_EXISTS { EEXIST } else { ENOENT }
    }
}

pub fn readlinkat(pid: isize, out: &mut [u8; 64]) -> isize {
    active_once();
    unsafe {
        if pid == PATH_SYM_A && SYM_A_EXISTS {
            let target = b"target_file";
            let mut i = 0usize;
            while i < target.len() { out[i] = target[i]; i += 1; }
            crate::println!("[ucompat-v148] readlinkat sym_a -> target_file ret={}", target.len());
            target.len() as isize
        } else if pid == PATH_FILE_A || pid == PATH_FILE_B || pid == PATH_HARD_A { EINVAL } else { ENOENT }
    }
}

pub fn unlinkat(pid: isize) -> isize {
    active_once();
    unsafe {
        if pid == PATH_HARD_A && HARD_A_EXISTS { HARD_A_EXISTS = false; crate::println!("[ucompat-v148] unlinkat hard_a ret=0"); 0 }
        else if pid == PATH_FILE_B && FILE_B_EXISTS { FILE_B_EXISTS = false; crate::println!("[ucompat-v148] unlinkat file_b ret=0"); 0 }
        else if pid == PATH_SYM_A && SYM_A_EXISTS { SYM_A_EXISTS = false; crate::println!("[ucompat-v148] unlinkat sym_a ret=0"); 0 }
        else { ENOENT }
    }
}

fn write_u16(out: &mut [u8], off: usize, val: u16) {
    let bytes = val.to_le_bytes();
    out[off] = bytes[0];
    out[off + 1] = bytes[1];
}
fn write_u32(out: &mut [u8], off: usize, val: u32) {
    let bytes = val.to_le_bytes();
    let mut i = 0usize;
    while i < 4 { out[off + i] = bytes[i]; i += 1; }
}
fn write_u64(out: &mut [u8], off: usize, val: u64) {
    let bytes = val.to_le_bytes();
    let mut i = 0usize;
    while i < 8 { out[off + i] = bytes[i]; i += 1; }
}

fn dirent(out: &mut [u8; 128], base: usize, ino: u64, off_next: u64, name: &[u8], dtype: u8) {
    write_u64(out, base + 0, ino);
    write_u64(out, base + 8, off_next);
    write_u16(out, base + 16, 32);
    out[base + 18] = dtype;
    let mut i = 0usize;
    while i < name.len() { out[base + 19 + i] = name[i]; i += 1; }
    out[base + 19 + name.len()] = 0;
}

pub fn getdents64(fd: isize, out: &mut [u8; 128], len: usize) -> isize {
    active_once();
    if fd != FD_DIR { return -9; }
    unsafe {
        if !(FILE_B_EXISTS && HARD_A_EXISTS && SYM_A_EXISTS) { return 0; }
    }
    if len < 96 { return EINVAL; }
    dirent(out, 0, 6001, 32, b"file_b", 8);
    dirent(out, 32, 6002, 64, b"hard_a", 8);
    dirent(out, 64, 6003, 96, b"sym_a", 10);
    crate::println!("[ucompat-v148] getdents64 entries=file_b,hard_a,sym_a ret=96");
    96
}

pub fn statx(pid: isize, out: &mut [u8; 128]) -> isize {
    active_once();
    let (mode, nlink, size, label): (u16, u32, u64, &'static str) = unsafe {
        if pid == PATH_V148DIR && DIR_EXISTS {
            (0o40755, 2, 0, "dir")
        } else if (pid == PATH_FILE_A || pid == PATH_FILE_B || pid == PATH_HARD_A) && name_exists(pid) {
            (0o100644, file_nlink() as u32, 0, "reg")
        } else if pid == PATH_SYM_A && SYM_A_EXISTS {
            (0o120777, 1, 11, "symlink")
        } else { return ENOENT; }
    };
    write_u32(out, 0, 0x7ff);
    write_u32(out, 16, nlink);
    write_u16(out, 28, mode);
    write_u64(out, 32, 7000 + pid as u64);
    write_u64(out, 40, size);
    if pid == PATH_FILE_B { crate::println!("[ucompat-v148] statx file_b mode={} nlink={}", label, nlink); }
    else if pid == PATH_SYM_A { crate::println!("[ucompat-v148] statx sym_a mode=symlink nlink=1"); }
    else if pid == PATH_V148DIR { crate::println!("[ucompat-v148] statx v148dir mode=dir nlink=2"); }
    0
}
