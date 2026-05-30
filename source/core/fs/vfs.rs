//! Minimal read-only VFS surface for file-backed exec.

use crate::core::block::{BlockCache, BlockIoError, BlockProvider};

use super::ext4::{Ext4Error, Ext4File, Ext4Volume};
use super::fd::{FdError, OpenFileDescription, OpenOptions};

pub const VFS_NAME_MAX: usize = 255;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VfsPath<'a> {
    bytes: &'a [u8],
}

impl<'a> VfsPath<'a> {
    pub const fn new(bytes: &'a [u8]) -> Result<Self, VfsError> {
        if bytes.is_empty() || bytes[0] != b'/' {
            Err(VfsError::InvalidPath)
        } else {
            Ok(Self { bytes })
        }
    }

    pub const fn bytes(self) -> &'a [u8] {
        self.bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FileIdentity {
    inode: u32,
    byte_len: u64,
    executable: bool,
    mode: u16,
    links: u16,
    blocks: u64,
    kind: VfsNodeKind,
}

impl FileIdentity {
    pub const fn new(
        inode: u32,
        byte_len: u64,
        executable: bool,
        mode: u16,
        links: u16,
        blocks: u64,
        kind: VfsNodeKind,
    ) -> Self {
        Self {
            inode,
            byte_len,
            executable,
            mode,
            links,
            blocks,
            kind,
        }
    }

    pub const fn inode(self) -> u32 {
        self.inode
    }

    pub const fn byte_len(self) -> u64 {
        self.byte_len
    }

    pub const fn executable(self) -> bool {
        self.executable
    }

    pub const fn mode(self) -> u16 {
        self.mode
    }

    pub const fn links(self) -> u16 {
        self.links
    }

    pub const fn blocks(self) -> u64 {
        self.blocks
    }

    pub const fn kind(self) -> VfsNodeKind {
        self.kind
    }

    pub const fn is_directory(self) -> bool {
        matches!(self.kind, VfsNodeKind::Directory)
    }

    pub const fn is_regular(self) -> bool {
        matches!(self.kind, VfsNodeKind::RegularFile)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VfsNodeKind {
    Directory,
    RegularFile,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VfsDirEntryKind {
    Unknown,
    RegularFile,
    Directory,
    CharacterDevice,
    BlockDevice,
    Fifo,
    Socket,
    Symlink,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VfsDirEntry {
    inode: u32,
    next_offset: u64,
    name: [u8; VFS_NAME_MAX],
    name_len: usize,
    kind: VfsDirEntryKind,
}

impl VfsDirEntry {
    pub fn new(
        inode: u32,
        next_offset: u64,
        name: &[u8],
        kind: VfsDirEntryKind,
    ) -> Result<Self, VfsError> {
        if name.is_empty() || name.len() > VFS_NAME_MAX {
            return Err(VfsError::MetadataCorrupt);
        }
        let mut stored = [0u8; VFS_NAME_MAX];
        stored[..name.len()].copy_from_slice(name);
        Ok(Self {
            inode,
            next_offset,
            name: stored,
            name_len: name.len(),
            kind,
        })
    }

    pub const fn inode(self) -> u32 {
        self.inode
    }

    pub const fn next_offset(self) -> u64 {
        self.next_offset
    }

    pub fn name(&self) -> &[u8] {
        &self.name[..self.name_len]
    }

    pub const fn kind(self) -> VfsDirEntryKind {
        self.kind
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VfsStat {
    inode: u32,
    byte_len: u64,
    mode: u16,
    links: u16,
    block_size: u32,
    blocks: u64,
    kind: VfsNodeKind,
}

impl VfsStat {
    pub const fn new(identity: FileIdentity, block_size: u32) -> Self {
        Self {
            inode: identity.inode(),
            byte_len: identity.byte_len(),
            mode: identity.mode(),
            links: identity.links(),
            block_size,
            blocks: identity.blocks(),
            kind: identity.kind(),
        }
    }

    pub const fn inode(self) -> u32 {
        self.inode
    }

    pub const fn byte_len(self) -> u64 {
        self.byte_len
    }

    pub const fn mode(self) -> u16 {
        self.mode
    }

    pub const fn links(self) -> u16 {
        self.links
    }

    pub const fn block_size(self) -> u32 {
        self.block_size
    }

    pub const fn blocks(self) -> u64 {
        self.blocks
    }

    pub const fn kind(self) -> VfsNodeKind {
        self.kind
    }
}

pub trait SyscallVfs {
    fn stat_path(&self, path: VfsPath<'_>) -> Result<VfsStat, VfsError>;
    fn open_path(&self, path: VfsPath<'_>, options: OpenOptions) -> Result<usize, VfsRuntimeError>;
    fn close_fd(&self, fd: usize) -> Result<(), FdError>;
    fn fstat_fd(&self, fd: usize) -> Result<VfsStat, VfsRuntimeError>;
    fn read_fd(&self, fd: usize, out: &mut [u8]) -> Result<usize, VfsRuntimeError>;
    fn lseek_fd(&self, fd: usize, offset: i64, whence: usize) -> Result<u64, VfsRuntimeError>;
    fn dir_entry_at_fd(&self, fd: usize) -> Result<Option<VfsDirEntry>, VfsRuntimeError>;
    fn set_fd_offset(&self, fd: usize, offset: u64) -> Result<(), FdError>;
    fn set_fd_close_on_exec(&self, fd: usize, close_on_exec: bool) -> Result<(), FdError>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NoSyscallVfs;

impl SyscallVfs for NoSyscallVfs {
    fn stat_path(&self, _path: VfsPath<'_>) -> Result<VfsStat, VfsError> {
        Err(VfsError::RootfsSourceMissing)
    }

    fn open_path(
        &self,
        _path: VfsPath<'_>,
        _options: OpenOptions,
    ) -> Result<usize, VfsRuntimeError> {
        Err(VfsRuntimeError::Vfs(VfsError::RootfsSourceMissing))
    }

    fn close_fd(&self, _fd: usize) -> Result<(), FdError> {
        Err(FdError::BadFileDescriptor)
    }

    fn fstat_fd(&self, _fd: usize) -> Result<VfsStat, VfsRuntimeError> {
        Err(VfsRuntimeError::Fd(FdError::BadFileDescriptor))
    }

    fn read_fd(&self, _fd: usize, _out: &mut [u8]) -> Result<usize, VfsRuntimeError> {
        Err(VfsRuntimeError::Fd(FdError::BadFileDescriptor))
    }

    fn lseek_fd(&self, _fd: usize, _offset: i64, _whence: usize) -> Result<u64, VfsRuntimeError> {
        Err(VfsRuntimeError::Fd(FdError::BadFileDescriptor))
    }

    fn dir_entry_at_fd(&self, _fd: usize) -> Result<Option<VfsDirEntry>, VfsRuntimeError> {
        Err(VfsRuntimeError::Fd(FdError::BadFileDescriptor))
    }

    fn set_fd_offset(&self, _fd: usize, _offset: u64) -> Result<(), FdError> {
        Err(FdError::BadFileDescriptor)
    }

    fn set_fd_close_on_exec(&self, _fd: usize, _close_on_exec: bool) -> Result<(), FdError> {
        Err(FdError::BadFileDescriptor)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VfsRuntimeError {
    Fd(FdError),
    Vfs(VfsError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VfsReadResult {
    bytes: usize,
}

impl VfsReadResult {
    pub const fn bytes(self) -> usize {
        self.bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VfsError {
    Block(BlockIoError),
    DirectoryExpected,
    EmptyFile,
    FileTooLarge,
    InvalidPath,
    MetadataCorrupt,
    NotExecutable,
    NotRegularFile,
    PathNotFound,
    RootfsSourceMissing,
    UnsupportedPath,
    UnsupportedRootfs,
}

#[derive(Clone, Copy)]
pub struct MountedRootfs {
    provider: BlockProvider,
    cache: BlockCache,
    volume: Ext4Volume,
}

impl MountedRootfs {
    pub fn mount_ext4(provider: BlockProvider) -> Result<Self, VfsError> {
        let mut cache = BlockCache::new();
        let volume = Ext4Volume::mount(&mut cache, provider).map_err(map_ext4_error)?;
        Ok(Self {
            provider,
            cache,
            volume,
        })
    }

    pub fn open(&mut self, path: VfsPath<'_>) -> Result<OpenFileDescription, VfsError> {
        self.open_with_options(
            path,
            OpenOptions::from_linux_flags(0).map_err(|_| VfsError::UnsupportedPath)?,
        )
    }

    pub fn open_with_options(
        &mut self,
        path: VfsPath<'_>,
        options: OpenOptions,
    ) -> Result<OpenFileDescription, VfsError> {
        validate_path(path.bytes())?;
        let file = self
            .volume
            .lookup_path(&mut self.cache, self.provider, path.bytes())
            .map_err(map_ext4_error)?;
        let identity = file.identity();
        if options.directory_required() && !identity.is_directory() {
            return Err(VfsError::DirectoryExpected);
        }

        Ok(OpenFileDescription::new(identity, options.close_on_exec()))
    }

    pub fn read_all(
        &mut self,
        file: &OpenFileDescription,
        out: &mut [u8],
    ) -> Result<VfsReadResult, VfsError> {
        let byte_len = file.identity().byte_len();
        if byte_len == 0 {
            return Err(VfsError::EmptyFile);
        }
        if byte_len > out.len() as u64 {
            return Err(VfsError::FileTooLarge);
        }

        self.read_at(file, 0, out)
    }

    pub fn read_at(
        &mut self,
        file: &OpenFileDescription,
        offset: u64,
        out: &mut [u8],
    ) -> Result<VfsReadResult, VfsError> {
        let identity = file.identity();
        if !identity.is_regular() {
            return Err(VfsError::NotRegularFile);
        }
        let ext4_file = Ext4File::new(
            identity.inode(),
            identity.byte_len(),
            identity.executable(),
            true,
            identity.mode(),
            identity.links(),
            identity.blocks(),
        );
        let bytes = self
            .volume
            .read_file(&mut self.cache, self.provider, ext4_file, offset, out)
            .map_err(map_ext4_error)?;
        Ok(VfsReadResult { bytes })
    }

    pub fn stat(&mut self, path: VfsPath<'_>) -> Result<VfsStat, VfsError> {
        validate_path(path.bytes())?;
        let file = self
            .volume
            .lookup_path(&mut self.cache, self.provider, path.bytes())
            .map_err(map_ext4_error)?;
        Ok(VfsStat::new(
            file.identity(),
            self.volume.block_size() as u32,
        ))
    }

    pub fn stat_open(&self, file: OpenFileDescription) -> VfsStat {
        VfsStat::new(file.identity(), self.volume.block_size() as u32)
    }

    pub fn dir_entry_at(
        &mut self,
        file: OpenFileDescription,
        offset: u64,
    ) -> Result<Option<VfsDirEntry>, VfsError> {
        if !file.is_directory() {
            return Err(VfsError::DirectoryExpected);
        }
        self.volume
            .read_dir_entry_at(&mut self.cache, self.provider, file.identity(), offset)
            .map_err(map_ext4_error)
    }

    pub const fn cache(&self) -> &BlockCache {
        &self.cache
    }
}

fn validate_path(path: &[u8]) -> Result<(), VfsError> {
    if path.is_empty() || path[0] != b'/' {
        return Err(VfsError::InvalidPath);
    }

    let mut index = 0usize;
    while index < path.len() {
        match path[index] {
            0 => return Err(VfsError::InvalidPath),
            b'.' => {
                let previous_is_separator = index == 0 || path[index - 1] == b'/';
                let next = index + 1;
                let single_dot = next == path.len() || path[next] == b'/';
                let double_dot = next < path.len()
                    && path[next] == b'.'
                    && (next + 1 == path.len() || path[next + 1] == b'/');
                if previous_is_separator && (single_dot || double_dot) {
                    return Err(VfsError::UnsupportedPath);
                }
            }
            _ => {}
        }
        index += 1;
    }

    Ok(())
}

fn map_ext4_error(error: Ext4Error) -> VfsError {
    match error {
        Ext4Error::Block(block) => match block {
            BlockIoError::ProviderMissing => VfsError::RootfsSourceMissing,
            other => VfsError::Block(other),
        },
        Ext4Error::DirectoryExpected => VfsError::DirectoryExpected,
        Ext4Error::FileTooLarge => VfsError::FileTooLarge,
        Ext4Error::MetadataCorrupt => VfsError::MetadataCorrupt,
        Ext4Error::NotRegularFile => VfsError::NotRegularFile,
        Ext4Error::PathNotFound => VfsError::PathNotFound,
        Ext4Error::UnsupportedPath => VfsError::UnsupportedPath,
        Ext4Error::UnsupportedRootfs => VfsError::UnsupportedRootfs,
    }
}
