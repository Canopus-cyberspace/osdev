#![allow(dead_code)]

pub const MAX_FD: usize = 16;

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

pub fn self_test() {
    crate::println!("[fd-v53c] self-test begin");

    let mut table = FdTable::with_stdio();

    let stdio_ok =
        table.get(0).map(|fd| fd.kind == FileKind::Stdin && fd.readable && !fd.writable).unwrap_or(false)
        && table.get(1).map(|fd| fd.kind == FileKind::Stdout && !fd.readable && fd.writable).unwrap_or(false)
        && table.get(2).map(|fd| fd.kind == FileKind::Stderr && fd.writable).unwrap_or(false);

    crate::println!("[fd-v53c] stdio ok = {}", stdio_ok as usize);

    if let Some(null_fd) = table.alloc(FileKind::DevNull, true, true) {
        crate::println!("[fd-v53c] allocated /dev/null fd = {}", null_fd);
        let closed = table.close(null_fd);
        crate::println!("[fd-v53c] closed /dev/null = {}", closed as usize);
    } else {
        crate::println!("[fd-v53c] /dev/null allocation skipped");
    }

    crate::println!("[fd-v53c] self-test passed");
}
