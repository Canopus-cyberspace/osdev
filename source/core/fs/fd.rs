//! Fixed single-task fd table and open-file-description ownership.

use super::vfs::{FileIdentity, VfsNodeKind};

pub const FD_TABLE_CAPACITY: usize = 128;
pub const FIRST_VFS_FD: usize = 3;
pub const PIPE_TABLE_CAPACITY: usize = 16;
pub const PIPE_BUFFER_BYTES: usize = 4096;

const O_ACCMODE: usize = 0x3;
const O_WRONLY: usize = 0x1;
const O_RDWR: usize = 0x2;
const O_CREAT: usize = 0x40;
const O_EXCL: usize = 0x80;
const O_NOCTTY: usize = 0x100;
const O_TRUNC: usize = 0x200;
const O_APPEND: usize = 0x400;
const O_NONBLOCK: usize = 0x800;
const O_LARGEFILE: usize = 0x8000;
const O_DIRECTORY: usize = 0x1_0000;
const O_CLOEXEC: usize = 0x8_0000;
const SUPPORTED_OPEN_FLAGS: usize = O_ACCMODE
    | O_CREAT
    | O_EXCL
    | O_NOCTTY
    | O_TRUNC
    | O_APPEND
    | O_NONBLOCK
    | O_LARGEFILE
    | O_DIRECTORY
    | O_CLOEXEC;
const PIPE_INODE_BASE: u32 = 0x9000_0000;
const PIPE_MODE: u16 = 0o010600;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpenOptions {
    readable: bool,
    writable: bool,
    create: bool,
    exclusive: bool,
    truncate: bool,
    append: bool,
    directory_required: bool,
    close_on_exec: bool,
}

impl OpenOptions {
    pub const fn from_linux_flags(flags: usize) -> Result<Self, OpenOptionsError> {
        if flags & !SUPPORTED_OPEN_FLAGS != 0 {
            return Err(OpenOptionsError::UnsupportedFlags);
        }
        let access = flags & O_ACCMODE;

        Ok(Self {
            readable: access != O_WRONLY,
            writable: access == O_WRONLY || access == O_RDWR,
            create: flags & O_CREAT != 0,
            exclusive: flags & O_EXCL != 0,
            truncate: flags & O_TRUNC != 0,
            append: flags & O_APPEND != 0,
            directory_required: flags & O_DIRECTORY != 0,
            close_on_exec: flags & O_CLOEXEC != 0,
        })
    }

    pub const fn readable(self) -> bool {
        self.readable
    }

    pub const fn writable(self) -> bool {
        self.writable
    }

    pub const fn create(self) -> bool {
        self.create
    }

    pub const fn exclusive(self) -> bool {
        self.exclusive
    }

    pub const fn truncate(self) -> bool {
        self.truncate
    }

    pub const fn append(self) -> bool {
        self.append
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
    readable: bool,
    writable: bool,
    append: bool,
    user_output_fd: Option<usize>,
}

impl OpenFileDescription {
    pub const fn new(identity: FileIdentity) -> Self {
        Self {
            identity,
            offset: 0,
            readable: true,
            writable: false,
            append: false,
            user_output_fd: None,
        }
    }

    pub const fn with_options(identity: FileIdentity, options: OpenOptions) -> Self {
        Self {
            identity,
            offset: 0,
            readable: options.readable(),
            writable: options.writable(),
            append: options.append(),
            user_output_fd: None,
        }
    }

    pub const fn user_output(fd: usize) -> Self {
        Self {
            identity: FileIdentity::new(0, 0, false, 0o020666, 1, 0, VfsNodeKind::CharacterDevice),
            offset: 0,
            readable: false,
            writable: true,
            append: false,
            user_output_fd: Some(fd),
        }
    }

    pub const fn user_input() -> Self {
        Self {
            identity: FileIdentity::new(0, 0, false, 0o020444, 1, 0, VfsNodeKind::CharacterDevice),
            offset: 0,
            readable: true,
            writable: false,
            append: false,
            user_output_fd: None,
        }
    }

    pub const fn pipe_reader(pipe_id: usize) -> Self {
        Self {
            identity: FileIdentity::new(
                PIPE_INODE_BASE + pipe_id as u32,
                0,
                false,
                PIPE_MODE,
                1,
                0,
                VfsNodeKind::Fifo,
            ),
            offset: 0,
            readable: true,
            writable: false,
            append: false,
            user_output_fd: None,
        }
    }

    pub const fn pipe_writer(pipe_id: usize) -> Self {
        Self {
            identity: FileIdentity::new(
                PIPE_INODE_BASE + pipe_id as u32,
                0,
                false,
                PIPE_MODE,
                1,
                0,
                VfsNodeKind::Fifo,
            ),
            offset: 0,
            readable: false,
            writable: true,
            append: false,
            user_output_fd: None,
        }
    }

    pub const fn identity(self) -> FileIdentity {
        self.identity
    }

    pub const fn offset(self) -> u64 {
        self.offset
    }

    pub const fn is_directory(self) -> bool {
        matches!(self.identity.kind(), VfsNodeKind::Directory)
    }

    pub const fn is_regular(self) -> bool {
        matches!(self.identity.kind(), VfsNodeKind::RegularFile)
    }

    pub const fn is_pipe(self) -> bool {
        matches!(self.identity.kind(), VfsNodeKind::Fifo)
            && self.identity.inode() >= PIPE_INODE_BASE
            && self.identity.inode() < PIPE_INODE_BASE + PIPE_TABLE_CAPACITY as u32
    }

    pub const fn pipe_id(self) -> Option<usize> {
        if self.is_pipe() {
            Some((self.identity.inode() - PIPE_INODE_BASE) as usize)
        } else {
            None
        }
    }

    pub const fn readable(self) -> bool {
        self.readable
    }

    pub const fn writable(self) -> bool {
        self.writable
    }

    pub const fn append(self) -> bool {
        self.append
    }

    pub const fn status_flags(self) -> usize {
        let access = if self.readable && self.writable {
            O_RDWR
        } else if self.writable {
            O_WRONLY
        } else {
            0
        };
        let append = if self.append { O_APPEND } else { 0 };
        access | append
    }

    pub const fn user_output_fd(self) -> Option<usize> {
        self.user_output_fd
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
pub struct PipeTable {
    pipes: [Pipe; PIPE_TABLE_CAPACITY],
}

impl PipeTable {
    pub const fn new() -> Self {
        Self {
            pipes: [Pipe::empty(); PIPE_TABLE_CAPACITY],
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn create(&mut self) -> Result<(usize, OpenFileDescription, OpenFileDescription), FdError> {
        let pipe_id = self.free_pipe().ok_or(FdError::TableFull)?;
        self.pipes[pipe_id] = Pipe::new();
        Ok((
            pipe_id,
            OpenFileDescription::pipe_reader(pipe_id),
            OpenFileDescription::pipe_writer(pipe_id),
        ))
    }

    pub fn release(&mut self, pipe_id: usize) -> Result<(), FdError> {
        if pipe_id >= self.pipes.len() || !self.pipes[pipe_id].used {
            return Err(FdError::BadFileDescriptor);
        }
        self.pipes[pipe_id] = Pipe::empty();
        Ok(())
    }

    pub fn read(
        &mut self,
        description: OpenFileDescription,
        out: &mut [u8],
    ) -> Result<usize, FdError> {
        if !description.readable() {
            return Err(FdError::BadFileDescriptor);
        }
        let pipe_id = description.pipe_id().ok_or(FdError::BadFileDescriptor)?;
        self.pipes
            .get_mut(pipe_id)
            .ok_or(FdError::BadFileDescriptor)?
            .read(out)
    }

    pub fn readable(&self, description: OpenFileDescription) -> Result<bool, FdError> {
        if !description.readable() {
            return Ok(false);
        }
        let pipe_id = description.pipe_id().ok_or(FdError::BadFileDescriptor)?;
        self.pipes
            .get(pipe_id)
            .ok_or(FdError::BadFileDescriptor)
            .map(Pipe::readable)
    }

    pub fn write(
        &mut self,
        description: OpenFileDescription,
        bytes: &[u8],
    ) -> Result<usize, FdError> {
        if !description.writable() {
            return Err(FdError::BadFileDescriptor);
        }
        let pipe_id = description.pipe_id().ok_or(FdError::BadFileDescriptor)?;
        self.pipes
            .get_mut(pipe_id)
            .ok_or(FdError::BadFileDescriptor)?
            .write(bytes)
    }

    pub fn writable(&self, description: OpenFileDescription) -> Result<bool, FdError> {
        if !description.writable() {
            return Ok(false);
        }
        let pipe_id = description.pipe_id().ok_or(FdError::BadFileDescriptor)?;
        self.pipes
            .get(pipe_id)
            .ok_or(FdError::BadFileDescriptor)
            .map(Pipe::writable)
    }

    fn free_pipe(&self) -> Option<usize> {
        let mut index = 0usize;
        while index < self.pipes.len() {
            if !self.pipes[index].used {
                return Some(index);
            }
            index += 1;
        }
        None
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Pipe {
    used: bool,
    len: usize,
    bytes: [u8; PIPE_BUFFER_BYTES],
}

impl Pipe {
    pub const fn empty() -> Self {
        Self {
            used: false,
            len: 0,
            bytes: [0; PIPE_BUFFER_BYTES],
        }
    }

    pub const fn new() -> Self {
        Self {
            used: true,
            len: 0,
            bytes: [0; PIPE_BUFFER_BYTES],
        }
    }

    pub fn read(&mut self, out: &mut [u8]) -> Result<usize, FdError> {
        if !self.used {
            return Err(FdError::BadFileDescriptor);
        }
        if out.is_empty() || self.len == 0 {
            return Ok(0);
        }

        let count = core::cmp::min(out.len(), self.len);
        out[..count].copy_from_slice(&self.bytes[..count]);
        let mut index = count;
        while index < self.len {
            self.bytes[index - count] = self.bytes[index];
            index += 1;
        }
        self.len -= count;
        Ok(count)
    }

    pub const fn readable(&self) -> bool {
        self.used && self.len > 0
    }

    pub fn write(&mut self, bytes: &[u8]) -> Result<usize, FdError> {
        if !self.used {
            return Err(FdError::BadFileDescriptor);
        }
        if bytes.is_empty() {
            return Ok(0);
        }

        let available = self.bytes.len() - self.len;
        let count = core::cmp::min(bytes.len(), available);
        self.bytes[self.len..self.len + count].copy_from_slice(&bytes[..count]);
        self.len += count;
        Ok(count)
    }

    pub const fn writable(&self) -> bool {
        self.used && self.len < PIPE_BUFFER_BYTES
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
struct FdSlot {
    ofd_index: usize,
    close_on_exec: bool,
}

impl FdSlot {
    pub const fn new(ofd_index: usize, close_on_exec: bool) -> Self {
        Self {
            ofd_index,
            close_on_exec,
        }
    }

    pub const fn ofd_index(self) -> usize {
        self.ofd_index
    }

    pub const fn close_on_exec(self) -> bool {
        self.close_on_exec
    }

    pub fn set_close_on_exec(&mut self, close_on_exec: bool) {
        self.close_on_exec = close_on_exec;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FdTable {
    slots: [Option<FdSlot>; FD_TABLE_CAPACITY],
    descriptions: [Option<OpenFileDescription>; FD_TABLE_CAPACITY],
    ref_counts: [usize; FD_TABLE_CAPACITY],
}

impl FdTable {
    pub const fn new() -> Self {
        Self {
            slots: [None; FD_TABLE_CAPACITY],
            descriptions: [None; FD_TABLE_CAPACITY],
            ref_counts: [0; FD_TABLE_CAPACITY],
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn insert(
        &mut self,
        description: OpenFileDescription,
        close_on_exec: bool,
    ) -> Result<usize, FdError> {
        let ofd_index = self.free_description_slot()?;
        let mut fd = 0usize;
        while fd < self.slots.len() {
            if self.slots[fd].is_none() {
                self.descriptions[ofd_index] = Some(description);
                self.ref_counts[ofd_index] = 1;
                self.slots[fd] = Some(FdSlot::new(ofd_index, close_on_exec));
                return Ok(fd);
            }
            fd += 1;
        }

        self.descriptions[ofd_index] = None;
        self.ref_counts[ofd_index] = 0;
        Err(FdError::TableFull)
    }

    pub fn insert_at(
        &mut self,
        fd: usize,
        description: OpenFileDescription,
        close_on_exec: bool,
    ) -> Result<usize, FdError> {
        if fd >= self.slots.len() {
            return Err(FdError::BadFileDescriptor);
        }
        if self.slots[fd].is_some() {
            self.close(fd)?;
        }
        let ofd_index = self.free_description_slot()?;
        self.descriptions[ofd_index] = Some(description);
        self.ref_counts[ofd_index] = 1;
        self.slots[fd] = Some(FdSlot::new(ofd_index, close_on_exec));
        Ok(fd)
    }

    pub fn close(&mut self, fd: usize) -> Result<(), FdError> {
        let slot = self.slot(fd)?;
        let ofd_index = slot.ofd_index();
        self.slots[fd] = None;
        if self.ref_counts[ofd_index] > 0 {
            self.ref_counts[ofd_index] -= 1;
        }
        if self.ref_counts[ofd_index] == 0 {
            self.descriptions[ofd_index] = None;
        }
        Ok(())
    }

    pub fn get(&self, fd: usize) -> Result<OpenFileDescription, FdError> {
        let ofd_index = self.slot(fd)?.ofd_index();
        self.descriptions[ofd_index].ok_or(FdError::BadFileDescriptor)
    }

    pub fn contains(&self, fd: usize) -> bool {
        self.slot(fd)
            .ok()
            .and_then(|slot| self.descriptions[slot.ofd_index()])
            .is_some()
    }

    pub fn get_mut(&mut self, fd: usize) -> Result<&mut OpenFileDescription, FdError> {
        let ofd_index = self.slot(fd)?.ofd_index();
        self.descriptions[ofd_index]
            .as_mut()
            .ok_or(FdError::BadFileDescriptor)
    }

    pub fn set_offset(&mut self, fd: usize, offset: u64) -> Result<(), FdError> {
        let description = self.get_mut(fd)?;
        description.set_offset(offset);
        Ok(())
    }

    pub fn set_close_on_exec(&mut self, fd: usize, close_on_exec: bool) -> Result<(), FdError> {
        let slot = self.slot_mut(fd)?;
        slot.set_close_on_exec(close_on_exec);
        Ok(())
    }

    pub fn close_on_exec(&self, fd: usize) -> Result<bool, FdError> {
        Ok(self.slot(fd)?.close_on_exec())
    }

    pub fn status_flags(&self, fd: usize) -> Result<usize, FdError> {
        Ok(self.get(fd)?.status_flags())
    }

    pub fn duplicate_min(
        &mut self,
        old_fd: usize,
        min_fd: usize,
        close_on_exec: bool,
    ) -> Result<usize, FdError> {
        if min_fd >= self.slots.len() {
            return Err(FdError::InvalidOffset);
        }
        let old_slot = self.slot(old_fd)?;
        let ofd_index = old_slot.ofd_index();
        if self.descriptions[ofd_index].is_none() {
            return Err(FdError::BadFileDescriptor);
        }

        let mut fd = min_fd;
        while fd < self.slots.len() {
            if self.slots[fd].is_none() {
                self.ref_counts[ofd_index] += 1;
                self.slots[fd] = Some(FdSlot::new(ofd_index, close_on_exec));
                return Ok(fd);
            }
            fd += 1;
        }

        Err(FdError::TableFull)
    }

    pub fn duplicate_to(
        &mut self,
        old_fd: usize,
        new_fd: usize,
        close_on_exec: bool,
    ) -> Result<usize, FdError> {
        if new_fd >= self.slots.len() {
            return Err(FdError::BadFileDescriptor);
        }
        let old_slot = self.slot(old_fd)?;
        let ofd_index = old_slot.ofd_index();
        if self.descriptions[ofd_index].is_none() {
            return Err(FdError::BadFileDescriptor);
        }
        if self.slots[new_fd].is_some() {
            self.close(new_fd)?;
        }
        self.ref_counts[ofd_index] += 1;
        self.slots[new_fd] = Some(FdSlot::new(ofd_index, close_on_exec));
        Ok(new_fd)
    }

    fn free_description_slot(&self) -> Result<usize, FdError> {
        let mut index = 0usize;
        while index < self.descriptions.len() {
            if self.descriptions[index].is_none() {
                return Ok(index);
            }
            index += 1;
        }

        Err(FdError::TableFull)
    }

    fn slot(&self, fd: usize) -> Result<FdSlot, FdError> {
        if fd >= self.slots.len() {
            return Err(FdError::BadFileDescriptor);
        }
        self.slots[fd].ok_or(FdError::BadFileDescriptor)
    }

    fn slot_mut(&mut self, fd: usize) -> Result<&mut FdSlot, FdError> {
        if fd >= self.slots.len() {
            return Err(FdError::BadFileDescriptor);
        }
        self.slots[fd].as_mut().ok_or(FdError::BadFileDescriptor)
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
