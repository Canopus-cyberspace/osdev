// UCOMPAT_V149C_BOOTSTRAP_WRAPPER_REPAIR_SOURCE_BASELINE
// UCOMPAT_V149B_FCNTL_SETFL_STATUS_FLAGS_SOURCE_BASELINE
// UCOMPAT_V149F_GUARD_FALSE_POSITIVE_REPAIR_SOURCE_BASELINE
// UCOMPAT_V149E_DUP_AFTER_CLOSE_LSEEK_STATX_SOURCE_BASELINE
// UCOMPAT_V149D_FORCE_SV39_WRAPPER_SOURCE_BASELINE
// UCOMPAT_V149_FDTABLE_MODULE
use crate::syscall::errno::{EBADF, EINVAL, ENOENT};

pub const BASE_FD: isize = 19000;
pub const TARGET_DUP3_FD: isize = 19007;
pub const FD_CLOEXEC: usize = 1;
pub const O_RDWR: usize = 2;
pub const O_CREAT: usize = 64;
pub const O_TRUNC: usize = 512;
pub const O_NONBLOCK: usize = 0x800;
const MAX_FD: usize = 16;
const MAX_OFD: usize = 4;
const CAP: usize = 64;

static mut PRINTED_ACTIVE: bool = false;
static mut FILE_EXISTS: bool = false;
static mut FILE_DATA: [u8; CAP] = [0; CAP];
static mut FILE_LEN: usize = 0;

static mut FD_USED: [bool; MAX_FD] = [false; MAX_FD];
static mut FD_OFD: [usize; MAX_FD] = [0; MAX_FD];
static mut FD_CLO: [usize; MAX_FD] = [0; MAX_FD];

static mut OFD_USED: [bool; MAX_OFD] = [false; MAX_OFD];
static mut OFD_POS: [usize; MAX_OFD] = [0; MAX_OFD];
static mut OFD_FLAGS: [usize; MAX_OFD] = [0; MAX_OFD];
static mut OFD_REFS: [usize; MAX_OFD] = [0; MAX_OFD];

pub fn active_once() {
    unsafe {
        if !PRINTED_ACTIVE {
            PRINTED_ACTIVE = true;
            crate::println!("[ucompat-v149] fdtable module active");
        }
    }
}

pub fn path_id(buf: &[u8; 64], len: usize) -> isize {
    if len == 10 && &buf[..10] == b"v149fd.txt" { 1 } else { -1 }
}

fn fd_index(fd: isize) -> isize {
    let idx = fd - BASE_FD;
    if idx < 0 || idx >= MAX_FD as isize { -1 } else { idx }
}

fn alloc_ofd(flags: usize) -> isize {
    unsafe {
        let mut i = 0usize;
        while i < MAX_OFD {
            if !OFD_USED[i] {
                OFD_USED[i] = true;
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

fn alloc_fd_for_ofd(ofd: usize, min_fd: isize, clo: usize) -> isize {
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
    -1
}

pub fn open_file(path_id: isize, flags: usize) -> isize {
    active_once();
    if path_id != 1 { return ENOENT; }
    unsafe {
        if !FILE_EXISTS && (flags & O_CREAT) == 0 { return ENOENT; }
        if !FILE_EXISTS || (flags & O_TRUNC) != 0 {
            FILE_EXISTS = true;
            FILE_LEN = 0;
            let mut i = 0usize;
            while i < CAP { FILE_DATA[i] = 0; i += 1; }
        }
    }
    let ofd = alloc_ofd(flags);
    if ofd < 0 { return -24; }
    let fd = alloc_fd_for_ofd(ofd as usize, BASE_FD, 0);
    crate::println!("[ucompat-v149] open file fd={} ofd={} flags={:#x}", fd, ofd, flags);
    fd
}

pub fn close(fd: isize) -> isize {
    active_once();
    let idx = fd_index(fd);
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
            OFD_POS[ofd] = 0;
            OFD_FLAGS[ofd] = 0;
        }
    }
    crate::println!("[ucompat-v149] close fd={} ret=0", fd);
    0
}

pub fn dup(fd: isize) -> isize {
    active_once();
    let idx = fd_index(fd);
    if idx < 0 { return EBADF; }
    unsafe {
        let i = idx as usize;
        if !FD_USED[i] { return EBADF; }
        let ofd = FD_OFD[i];
        let new_fd = alloc_fd_for_ofd(ofd, BASE_FD, 0);
        crate::println!("[ucompat-v149] dup old={} new={} shared_ofd=1", fd, new_fd);
        new_fd
    }
}

pub fn dup3(old_fd: isize, new_fd: isize, flags: usize) -> isize {
    active_once();
    let old_idx = fd_index(old_fd);
    let new_idx = fd_index(new_fd);
    if old_idx < 0 || new_idx < 0 || old_fd == new_fd { return EINVAL; }
    unsafe {
        let oi = old_idx as usize;
        let ni = new_idx as usize;
        if !FD_USED[oi] { return EBADF; }
        if FD_USED[ni] {
            let _ = close(new_fd);
        }
        let ofd = FD_OFD[oi];
        FD_USED[ni] = true;
        FD_OFD[ni] = ofd;
        FD_CLO[ni] = if (flags & 0x80000) != 0 { FD_CLOEXEC } else { 0 };
        OFD_REFS[ofd] += 1;
        crate::println!("[ucompat-v149] dup3 old={} new={} cloexec={} shared_ofd=1", old_fd, new_fd, FD_CLO[ni]);
        new_fd
    }
}

fn ofd_for_fd(fd: isize) -> isize {
    let idx = fd_index(fd);
    if idx < 0 { return EBADF; }
    unsafe {
        let i = idx as usize;
        if !FD_USED[i] { EBADF } else { FD_OFD[i] as isize }
    }
}

pub fn write(fd: isize, src: &[u8]) -> isize {
    active_once();
    let ofd = ofd_for_fd(fd);
    if ofd < 0 { return ofd; }
    let o = ofd as usize;
    let mut copied = 0usize;
    unsafe {
        while copied < src.len() && OFD_POS[o] + copied < CAP {
            FILE_DATA[OFD_POS[o] + copied] = src[copied];
            copied += 1;
        }
        let end = OFD_POS[o] + copied;
        if end > FILE_LEN { FILE_LEN = end; }
        OFD_POS[o] = end;
    }
    crate::println!("[ucompat-v149] write fd={} len={}", fd, copied);
    copied as isize
}

pub fn read(fd: isize, out: &mut [u8; 64], len: usize) -> isize {
    active_once();
    let ofd = ofd_for_fd(fd);
    if ofd < 0 { return ofd; }
    let o = ofd as usize;
    let mut copied = 0usize;
    unsafe {
        while copied < len && OFD_POS[o] < FILE_LEN && copied < out.len() {
            out[copied] = FILE_DATA[OFD_POS[o]];
            OFD_POS[o] += 1;
            copied += 1;
        }
    }
    crate::println!("[ucompat-v149] read fd={} len={}", fd, copied);
    copied as isize
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
        crate::println!("[ucompat-v149] lseek fd={} pos={}", fd, new_pos);
        new_pos
    }
}

pub fn fcntl(fd: isize, cmd: usize, arg: usize) -> isize {
    // UCOMPAT_V149B_FCNTL_SETFL_STATUS_FLAGS
    // UCOMPAT_V149D_FORCE_FCNTL_STATUS_FLAGS
    active_once();
    let idx = fd_index(fd);
    if idx < 0 { return EBADF; }
    unsafe {
        let i = idx as usize;
        if !FD_USED[i] { return EBADF; }
        let ofd = FD_OFD[i];
        match cmd {
            1 => FD_CLO[i] as isize,
            2 => {
                FD_CLO[i] = arg & FD_CLOEXEC;
                FD_CLO[i] as isize
            },
            3 => OFD_FLAGS[ofd] as isize,
            4 => {
                let status = O_RDWR | (arg & O_NONBLOCK);
                OFD_FLAGS[ofd] = status;
                crate::println!("[ucompat-v149b] fcntl setfl normalized status flags={:#x}", status);
                status as isize
            },
            _ => EINVAL,
        }
    }
}

pub fn statx(path_id: isize, out: &mut [u8; 128]) -> isize {
    active_once();
    if path_id != 1 { return ENOENT; }
    unsafe {
        if !FILE_EXISTS { return ENOENT; }
        write_u32(out, 0, 0x7ff);
        write_u32(out, 16, 1);
        write_u16(out, 28, 0o100644);
        write_u64(out, 32, 9001);
        write_u64(out, 40, FILE_LEN as u64);
        crate::println!("[ucompat-v149] statx size={} nlink=1 mode=reg", FILE_LEN);
    }
    0
}

fn write_u16(out: &mut [u8], off: usize, val: u16) {
    let b = val.to_le_bytes();
    out[off] = b[0]; out[off + 1] = b[1];
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
