#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, Ordering};

pub const MAX_FD: usize = 16;

pub const EBADF: isize = -9;
pub const ENOENT: isize = -2;
pub const EINVAL: isize = -22;
pub const ESPIPE: isize = -29;

static DEVNULL_OPEN: AtomicBool = AtomicBool::new(false);
static DEVZERO_OPEN: AtomicBool = AtomicBool::new(false);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FileKind {
    Empty,
    Stdin,
    Stdout,
    Stderr,
    DevNull,
    DevZero,
}

#[derive(Copy, Clone, Debug)]
pub struct FileDescriptor {
    pub fd: usize,
    pub kind: FileKind,
    pub readable: bool,
    pub writable: bool,
}

impl FileDescriptor {
    pub const fn new(fd: usize, kind: FileKind, readable: bool, writable: bool) -> Self {
        Self { fd, kind, readable, writable }
    }
}

#[derive(Copy, Clone)]
pub struct FdTable {
    entries: [Option<FileDescriptor>; MAX_FD],
}

impl FdTable {
    pub const fn new() -> Self { Self { entries: [None; MAX_FD] } }

    pub fn with_stdio() -> Self {
        let mut table = Self::new();
        table.entries[0] = Some(FileDescriptor::new(0, FileKind::Stdin, true, false));
        table.entries[1] = Some(FileDescriptor::new(1, FileKind::Stdout, false, true));
        table.entries[2] = Some(FileDescriptor::new(2, FileKind::Stderr, false, true));
        table
    }

    pub const fn get(&self, fd: usize) -> Option<FileDescriptor> {
        if fd >= MAX_FD { return None; }
        self.entries[fd]
    }

    pub fn alloc(&mut self, kind: FileKind, readable: bool, writable: bool) -> Option<usize> {
        let mut fd = 3;
        while fd < MAX_FD {
            if self.entries[fd].is_none() {
                self.entries[fd] = Some(FileDescriptor::new(fd, kind, readable, writable));
                return Some(fd);
            }
            fd += 1;
        }
        None
    }

    pub fn close(&mut self, fd: usize) -> bool {
        if fd >= MAX_FD || self.entries[fd].is_none() { return false; }
        self.entries[fd] = None;
        true
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RuntimeWriteTarget {
    Console,
    DevNull,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RuntimeReadTarget {
    Stdin,
    DevZero,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RuntimeFdKind {
    Stdin,
    Stdout,
    Stderr,
    DevNull,
    DevZero,
}

pub fn runtime_open_devnull() -> isize {
    DEVNULL_OPEN.store(true, Ordering::SeqCst);
    3
}

pub fn runtime_open_devzero() -> isize {
    DEVZERO_OPEN.store(true, Ordering::SeqCst);
    4
}

pub fn runtime_close_fd(fd: usize) -> isize {
    match fd {
        0 | 1 | 2 => 0,
        3 => {
            if DEVNULL_OPEN.swap(false, Ordering::SeqCst) { 0 } else { EBADF }
        }
        4 => {
            if DEVZERO_OPEN.swap(false, Ordering::SeqCst) { 0 } else { EBADF }
        }
        _ => EBADF,
    }
}

pub fn runtime_fd_kind(fd: usize) -> Result<RuntimeFdKind, isize> {
    match fd {
        0 => Ok(RuntimeFdKind::Stdin),
        1 => Ok(RuntimeFdKind::Stdout),
        2 => Ok(RuntimeFdKind::Stderr),
        3 => {
            if DEVNULL_OPEN.load(Ordering::SeqCst) { Ok(RuntimeFdKind::DevNull) } else { Err(EBADF) }
        }
        4 => {
            if DEVZERO_OPEN.load(Ordering::SeqCst) { Ok(RuntimeFdKind::DevZero) } else { Err(EBADF) }
        }
        _ => Err(EBADF),
    }
}

pub fn runtime_write_target(fd: usize) -> Result<RuntimeWriteTarget, isize> {
    match fd {
        1 | 2 => Ok(RuntimeWriteTarget::Console),
        3 => {
            if DEVNULL_OPEN.load(Ordering::SeqCst) { Ok(RuntimeWriteTarget::DevNull) } else { Err(EBADF) }
        }
        _ => Err(EBADF),
    }
}

pub fn runtime_read_target(fd: usize) -> Result<RuntimeReadTarget, isize> {
    match fd {
        0 => Ok(RuntimeReadTarget::Stdin),
        4 => {
            if DEVZERO_OPEN.load(Ordering::SeqCst) { Ok(RuntimeReadTarget::DevZero) } else { Err(EBADF) }
        }
        _ => Err(EBADF),
    }
}

pub fn runtime_lseek_result(fd: usize) -> isize {
    match runtime_fd_kind(fd) {
        Ok(RuntimeFdKind::Stdin | RuntimeFdKind::Stdout | RuntimeFdKind::Stderr | RuntimeFdKind::DevNull | RuntimeFdKind::DevZero) => ESPIPE,
        Err(err) => err,
    }
}

pub fn runtime_fstat_result(fd: usize) -> Result<RuntimeFdKind, isize> {
    runtime_fd_kind(fd)
}

pub fn self_test() {
    crate::println!("[fd-v58] self-test begin");

    let mut table = FdTable::with_stdio();

    let stdin = table.get(0).expect("[fd-v58] missing stdin");
    let stdout = table.get(1).expect("[fd-v58] missing stdout");
    let stderr = table.get(2).expect("[fd-v58] missing stderr");

    assert_eq!(stdin.kind, FileKind::Stdin);
    assert!(stdin.readable);
    assert!(!stdin.writable);
    assert_eq!(stdout.kind, FileKind::Stdout);
    assert!(stdout.writable);
    assert_eq!(stderr.kind, FileKind::Stderr);
    assert!(stderr.writable);

    assert_eq!(runtime_write_target(1), Ok(RuntimeWriteTarget::Console));
    assert_eq!(runtime_read_target(0), Ok(RuntimeReadTarget::Stdin));
    assert_eq!(runtime_lseek_result(1), ESPIPE);
    assert_eq!(runtime_fstat_result(1), Ok(RuntimeFdKind::Stdout));

    assert_eq!(runtime_open_devnull(), 3);
    assert_eq!(runtime_fstat_result(3), Ok(RuntimeFdKind::DevNull));
    assert_eq!(runtime_close_fd(3), 0);

    assert_eq!(runtime_open_devzero(), 4);
    assert_eq!(runtime_read_target(4), Ok(RuntimeReadTarget::DevZero));
    assert_eq!(runtime_fstat_result(4), Ok(RuntimeFdKind::DevZero));
    assert_eq!(runtime_lseek_result(4), ESPIPE);
    assert_eq!(runtime_close_fd(4), 0);

    let null_fd = table.alloc(FileKind::DevNull, true, true).expect("[fd-v58] alloc devnull fd failed");
    crate::println!("[fd-v58] allocated /dev/null fd = {}", null_fd);
    assert!(table.close(null_fd));

    crate::println!("[fd-v58] self-test passed");
}
