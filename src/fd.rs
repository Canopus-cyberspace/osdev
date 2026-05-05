#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, Ordering};

pub const MAX_FD: usize = 16;

pub const EBADF: isize = -9;
pub const ENOENT: isize = -2;
pub const EINVAL: isize = -22;

static DEVNULL_OPEN: AtomicBool = AtomicBool::new(false);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FileKind {
    Empty,
    Stdin,
    Stdout,
    Stderr,
    DevNull,
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
    pub const fn new() -> Self {
        Self { entries: [None; MAX_FD] }
    }

    pub fn with_stdio() -> Self {
        let mut table = Self::new();
        table.entries[0] = Some(FileDescriptor::new(0, FileKind::Stdin, true, false));
        table.entries[1] = Some(FileDescriptor::new(1, FileKind::Stdout, false, true));
        table.entries[2] = Some(FileDescriptor::new(2, FileKind::Stderr, false, true));
        table
    }

    pub const fn get(&self, fd: usize) -> Option<FileDescriptor> {
        if fd >= MAX_FD {
            return None;
        }
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
        if fd >= MAX_FD || self.entries[fd].is_none() {
            return false;
        }
        self.entries[fd] = None;
        true
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RuntimeWriteTarget {
    Console,
    DevNull,
}

pub fn runtime_open_devnull() -> isize {
    DEVNULL_OPEN.store(true, Ordering::SeqCst);
    3
}

pub fn runtime_close_fd(fd: usize) -> isize {
    match fd {
        0 | 1 | 2 => 0,
        3 => {
            if DEVNULL_OPEN.swap(false, Ordering::SeqCst) {
                0
            } else {
                EBADF
            }
        }
        _ => EBADF,
    }
}

pub fn runtime_write_target(fd: usize) -> Result<RuntimeWriteTarget, isize> {
    match fd {
        1 | 2 => Ok(RuntimeWriteTarget::Console),
        3 => {
            if DEVNULL_OPEN.load(Ordering::SeqCst) {
                Ok(RuntimeWriteTarget::DevNull)
            } else {
                Err(EBADF)
            }
        }
        _ => Err(EBADF),
    }
}

pub fn self_test() {
    crate::println!("[fd-v56] self-test begin");

    let mut table = FdTable::with_stdio();

    let stdin = table.get(0).expect("[fd-v56] missing stdin");
    let stdout = table.get(1).expect("[fd-v56] missing stdout");
    let stderr = table.get(2).expect("[fd-v56] missing stderr");

    assert_eq!(stdin.kind, FileKind::Stdin);
    assert!(stdin.readable);
    assert!(!stdin.writable);

    assert_eq!(stdout.kind, FileKind::Stdout);
    assert!(!stdout.readable);
    assert!(stdout.writable);

    assert_eq!(stderr.kind, FileKind::Stderr);
    assert!(stderr.writable);

    assert_eq!(runtime_write_target(1), Ok(RuntimeWriteTarget::Console));
    assert_eq!(runtime_write_target(2), Ok(RuntimeWriteTarget::Console));
    assert_eq!(runtime_write_target(3), Err(EBADF));

    let fd = runtime_open_devnull();
    assert_eq!(fd, 3);
    assert_eq!(runtime_write_target(3), Ok(RuntimeWriteTarget::DevNull));
    assert_eq!(runtime_close_fd(3), 0);
    assert_eq!(runtime_write_target(3), Err(EBADF));

    let null_fd = table.alloc(FileKind::DevNull, true, true)
        .expect("[fd-v56] alloc devnull fd failed");
    crate::println!("[fd-v56] allocated /dev/null fd = {}", null_fd);
    assert!(null_fd >= 3);
    assert!(table.close(null_fd));

    crate::println!("[fd-v56] self-test passed");
}
