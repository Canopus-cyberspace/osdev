//! Fixed single-task fd table and open-file-description ownership.

use super::vfs::{FileIdentity, VfsNodeKind};

pub const FD_TABLE_CAPACITY: usize = 16;
pub const FIRST_VFS_FD: usize = 3;

const O_ACCMODE: usize = 0x3;
const O_WRONLY: usize = 0x1;
const O_RDWR: usize = 0x2;
const O_CREAT: usize = 0x40;
const O_EXCL: usize = 0x80;
const O_TRUNC: usize = 0x200;
const O_APPEND: usize = 0x400;
const O_LARGEFILE: usize = 0x8000;
const O_DIRECTORY: usize = 0x1_0000;
const O_CLOEXEC: usize = 0x8_0000;
const SUPPORTED_OPEN_FLAGS: usize = O_ACCMODE | O_LARGEFILE | O_DIRECTORY | O_CLOEXEC;
const READ_ONLY_REJECT_FLAGS: usize = O_WRONLY | O_RDWR | O_CREAT | O_EXCL | O_TRUNC | O_APPEND;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpenOptions {
    directory_required: bool,
    close_on_exec: bool,
}

impl OpenOptions {
    pub const fn from_linux_flags(flags: usize) -> Result<Self, OpenOptionsError> {
        if flags & O_ACCMODE != 0 {
            return Err(OpenOptionsError::ReadOnlyFilesystem);
        }
        if flags & READ_ONLY_REJECT_FLAGS != 0 {
            return Err(OpenOptionsError::ReadOnlyFilesystem);
        }
        if flags & !SUPPORTED_OPEN_FLAGS != 0 {
            return Err(OpenOptionsError::UnsupportedFlags);
        }

        Ok(Self {
            directory_required: flags & O_DIRECTORY != 0,
            close_on_exec: flags & O_CLOEXEC != 0,
        })
    }

    pub const fn directory_required(self) -> bool {
        self.directory_required
    }

    pub const fn close_on_exec(self) -> bool {
        self.close_on_exec
    }

    pub fn set_close_on_exec(&mut self, close_on_exec: bool) {
        self.close_on_exec = close_on_exec;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpenOptionsError {
    ReadOnlyFilesystem,
    UnsupportedFlags,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpenFileDescription {
    identity: FileIdentity,
    offset: u64,
    close_on_exec: bool,
}

impl OpenFileDescription {
    pub const fn new(identity: FileIdentity, close_on_exec: bool) -> Self {
        Self {
            identity,
            offset: 0,
            close_on_exec,
        }
    }

    pub const fn identity(self) -> FileIdentity {
        self.identity
    }

    pub const fn offset(self) -> u64 {
        self.offset
    }

    pub const fn close_on_exec(self) -> bool {
        self.close_on_exec
    }

    pub fn set_close_on_exec(&mut self, close_on_exec: bool) {
        self.close_on_exec = close_on_exec;
    }

    pub const fn is_directory(self) -> bool {
        matches!(self.identity.kind(), VfsNodeKind::Directory)
    }

    pub const fn is_regular(self) -> bool {
        matches!(self.identity.kind(), VfsNodeKind::RegularFile)
    }

    pub fn set_offset(&mut self, offset: u64) {
        self.offset = offset;
    }

    pub fn advance_offset(&mut self, bytes: usize) -> Result<u64, FdError> {
        let next = self
            .offset
            .checked_add(bytes as u64)
            .ok_or(FdError::InvalidOffset)?;
        self.offset = next;
        Ok(next)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FdError {
    BadFileDescriptor,
    InvalidOffset,
    NotDirectory,
    NotRegularFile,
    TableFull,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FdTable {
    slots: [Option<OpenFileDescription>; FD_TABLE_CAPACITY],
}

impl FdTable {
    pub const fn new() -> Self {
        Self {
            slots: [None; FD_TABLE_CAPACITY],
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn insert(&mut self, description: OpenFileDescription) -> Result<usize, FdError> {
        let mut fd = FIRST_VFS_FD;
        while fd < self.slots.len() {
            if self.slots[fd].is_none() {
                self.slots[fd] = Some(description);
                return Ok(fd);
            }
            fd += 1;
        }

        Err(FdError::TableFull)
    }

    pub fn close(&mut self, fd: usize) -> Result<(), FdError> {
        if fd >= self.slots.len() || self.slots[fd].is_none() {
            return Err(FdError::BadFileDescriptor);
        }
        self.slots[fd] = None;
        Ok(())
    }

    pub fn get(&self, fd: usize) -> Result<OpenFileDescription, FdError> {
        if fd >= self.slots.len() {
            return Err(FdError::BadFileDescriptor);
        }
        self.slots[fd].ok_or(FdError::BadFileDescriptor)
    }

    pub fn get_mut(&mut self, fd: usize) -> Result<&mut OpenFileDescription, FdError> {
        if fd >= self.slots.len() {
            return Err(FdError::BadFileDescriptor);
        }
        self.slots[fd].as_mut().ok_or(FdError::BadFileDescriptor)
    }

    pub fn set_offset(&mut self, fd: usize, offset: u64) -> Result<(), FdError> {
        let description = self.get_mut(fd)?;
        description.set_offset(offset);
        Ok(())
    }

    pub fn set_close_on_exec(&mut self, fd: usize, close_on_exec: bool) -> Result<(), FdError> {
        let description = self.get_mut(fd)?;
        description.set_close_on_exec(close_on_exec);
        Ok(())
    }
}

pub fn seek_offset(
    current: u64,
    file_size: u64,
    offset: i64,
    whence: usize,
) -> Result<u64, FdError> {
    let base = match whence {
        0 => 0,
        1 => current,
        2 => file_size,
        _ => return Err(FdError::InvalidOffset),
    };
    let next = base as i128 + offset as i128;
    if next < 0 || next > u64::MAX as i128 {
        return Err(FdError::InvalidOffset);
    }
    Ok(next as u64)
}
