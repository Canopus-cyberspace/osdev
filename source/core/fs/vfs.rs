//! Minimal VFS surface for file-backed exec and runtime-owned writable overlay state.

use crate::core::block::{BlockCache, BlockIoError, BlockProvider};

use super::ext4::{Ext4Error, Ext4File, Ext4Volume};
use super::fd::{FdError, OpenFileDescription, OpenOptions};

pub const VFS_NAME_MAX: usize = 255;
pub const VFS_PATH_MAX: usize = 256;
pub const VFS_OVERLAY_DIR_CAPACITY: usize = 32;
pub const VFS_OVERLAY_FILE_CAPACITY: usize = 32;
pub const VFS_OVERLAY_FILE_BYTES: usize = 4096;
pub const VFS_MOUNT_CAPACITY: usize = 8;

const VFS_BUILTIN_PROC_DIR_INODE: u32 = 0x7000_0001;
const VFS_BUILTIN_PROC_MOUNTS_INODE: u32 = 0x7000_0002;
const VFS_BUILTIN_PROC_MEMINFO_INODE: u32 = 0x7000_0003;
const VFS_BUILTIN_DEV_DIR_INODE: u32 = 0x7000_0004;
const VFS_BUILTIN_DEV_NULL_INODE: u32 = 0x7000_0005;
const VFS_BUILTIN_DEV_MISC_INODE: u32 = 0x7000_0006;
const VFS_BUILTIN_DEV_RTC_INODE: u32 = 0x7000_0007;
const VFS_BUILTIN_BLOCK_SIZE: u32 = 4096;
const VFS_BUILTIN_DIR_MODE: u16 = 0o040555;
const VFS_BUILTIN_REG_MODE: u16 = 0o100444;
const VFS_BUILTIN_CHAR_MODE: u16 = 0o020666;
const VFS_BUILTIN_TEXT_BYTES: usize = 512;
const VFS_OVERLAY_INODE_BASE: u32 = 0x8000_0000;
const VFS_OVERLAY_FILE_INODE_BASE: u32 = VFS_OVERLAY_INODE_BASE + 0x1000;
const VFS_OVERLAY_BLOCK_SIZE: u32 = 4096;
const VFS_OVERLAY_DIR_TYPE: u16 = 0o040000;
const VFS_OVERLAY_REG_TYPE: u16 = 0o100000;
const VFS_MOUNT_TEXT_MAX: usize = VFS_PATH_MAX;
const PROC_MEMINFO_BYTES: &[u8] =
    b"MemTotal:        1048576 kB\nMemFree:          786432 kB\nMemAvailable:     786432 kB\nBuffers:               0 kB\nCached:           131072 kB\n";

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
pub struct VfsPathBuffer {
    bytes: [u8; VFS_PATH_MAX],
    len: usize,
}

impl VfsPathBuffer {
    pub const fn root() -> Self {
        let mut bytes = [0u8; VFS_PATH_MAX];
        bytes[0] = b'/';
        Self { bytes, len: 1 }
    }

    pub fn from_absolute(bytes: &[u8]) -> Result<Self, VfsError> {
        let root = Self::root();
        resolve_path(root.as_path()?, bytes)
    }

    pub fn parent_of_absolute(bytes: &[u8]) -> Result<Self, VfsError> {
        let mut path = Self::from_absolute(bytes)?;
        path.pop_component();
        Ok(path)
    }

    pub fn as_path(&self) -> Result<VfsPath<'_>, VfsError> {
        VfsPath::new(&self.bytes[..self.len])
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }

    fn push_component(&mut self, component: &[u8]) -> Result<(), VfsError> {
        if component.is_empty() || component.len() > VFS_NAME_MAX {
            return Err(VfsError::InvalidPath);
        }
        let separator = if self.len == 1 { 0 } else { 1 };
        let next_len = self
            .len
            .checked_add(separator)
            .and_then(|len| len.checked_add(component.len()))
            .ok_or(VfsError::InvalidPath)?;
        if next_len > self.bytes.len() {
            return Err(VfsError::InvalidPath);
        }
        if separator != 0 {
            self.bytes[self.len] = b'/';
            self.len += 1;
        }
        self.bytes[self.len..self.len + component.len()].copy_from_slice(component);
        self.len += component.len();
        Ok(())
    }

    fn pop_component(&mut self) {
        if self.len == 1 {
            return;
        }
        while self.len > 1 && self.bytes[self.len - 1] != b'/' {
            self.len -= 1;
        }
        if self.len > 1 {
            self.len -= 1;
        }
    }
}

pub fn resolve_path(cwd: VfsPath<'_>, path: &[u8]) -> Result<VfsPathBuffer, VfsError> {
    if path.is_empty() {
        return Err(VfsError::InvalidPath);
    }

    let mut resolved = if path[0] == b'/' {
        VfsPathBuffer::root()
    } else {
        VfsPathBuffer::from_absolute(cwd.bytes())?
    };
    let mut cursor = if path[0] == b'/' { 1 } else { 0 };
    while cursor < path.len() {
        while cursor < path.len() && path[cursor] == b'/' {
            cursor += 1;
        }
        if cursor >= path.len() {
            break;
        }
        let start = cursor;
        while cursor < path.len() && path[cursor] != b'/' {
            if path[cursor] == 0 {
                return Err(VfsError::InvalidPath);
            }
            cursor += 1;
        }
        let component = &path[start..cursor];
        if component == b"." {
            continue;
        }
        if component == b".." {
            resolved.pop_component();
            continue;
        }
        resolved.push_component(component)?;
    }

    Ok(resolved)
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
    CharacterDevice,
    Directory,
    Fifo,
    RegularFile,
}

impl VfsNodeKind {
    pub const fn is_directory(self) -> bool {
        matches!(self, Self::Directory)
    }
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

    pub const fn allows_access(self, mode: u8) -> bool {
        const X_OK: u8 = 0x1;
        const W_OK: u8 = 0x2;
        const R_OK: u8 = 0x4;

        if mode == 0 {
            return true;
        }

        let bits = self.mode & 0o777;
        if mode & R_OK != 0 && bits & 0o444 == 0 {
            return false;
        }
        if mode & W_OK != 0 && bits & 0o222 == 0 {
            return false;
        }
        if mode & X_OK != 0 && bits & 0o111 == 0 {
            return false;
        }
        true
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VfsIoctl {
    RtcReadTime,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BuiltinNode {
    ProcDir,
    ProcMounts,
    ProcMeminfo,
    DevDir,
    DevNull,
    DevMisc,
    DevRtc,
}

impl BuiltinNode {
    fn from_path(path: &[u8]) -> Option<Self> {
        match path {
            b"/proc" => Some(Self::ProcDir),
            b"/proc/mounts" => Some(Self::ProcMounts),
            b"/proc/meminfo" => Some(Self::ProcMeminfo),
            b"/dev" => Some(Self::DevDir),
            b"/dev/null" => Some(Self::DevNull),
            b"/dev/misc" => Some(Self::DevMisc),
            b"/dev/misc/rtc" => Some(Self::DevRtc),
            _ => None,
        }
    }

    fn from_inode(inode: u32) -> Option<Self> {
        match inode {
            VFS_BUILTIN_PROC_DIR_INODE => Some(Self::ProcDir),
            VFS_BUILTIN_PROC_MOUNTS_INODE => Some(Self::ProcMounts),
            VFS_BUILTIN_PROC_MEMINFO_INODE => Some(Self::ProcMeminfo),
            VFS_BUILTIN_DEV_DIR_INODE => Some(Self::DevDir),
            VFS_BUILTIN_DEV_NULL_INODE => Some(Self::DevNull),
            VFS_BUILTIN_DEV_MISC_INODE => Some(Self::DevMisc),
            VFS_BUILTIN_DEV_RTC_INODE => Some(Self::DevRtc),
            _ => None,
        }
    }

    const fn kind(self) -> VfsNodeKind {
        match self {
            Self::ProcDir | Self::DevDir | Self::DevMisc => VfsNodeKind::Directory,
            Self::DevNull | Self::DevRtc => VfsNodeKind::CharacterDevice,
            Self::ProcMounts | Self::ProcMeminfo => VfsNodeKind::RegularFile,
        }
    }

    const fn mode(self) -> u16 {
        match self {
            Self::ProcDir | Self::DevDir | Self::DevMisc => VFS_BUILTIN_DIR_MODE,
            Self::DevNull | Self::DevRtc => VFS_BUILTIN_CHAR_MODE,
            Self::ProcMounts | Self::ProcMeminfo => VFS_BUILTIN_REG_MODE,
        }
    }

    const fn inode(self) -> u32 {
        match self {
            Self::ProcDir => VFS_BUILTIN_PROC_DIR_INODE,
            Self::ProcMounts => VFS_BUILTIN_PROC_MOUNTS_INODE,
            Self::ProcMeminfo => VFS_BUILTIN_PROC_MEMINFO_INODE,
            Self::DevDir => VFS_BUILTIN_DEV_DIR_INODE,
            Self::DevNull => VFS_BUILTIN_DEV_NULL_INODE,
            Self::DevMisc => VFS_BUILTIN_DEV_MISC_INODE,
            Self::DevRtc => VFS_BUILTIN_DEV_RTC_INODE,
        }
    }

    fn byte_len(self, mounts: &VfsMountTable) -> u64 {
        match self {
            Self::ProcMounts => proc_mounts_len(mounts) as u64,
            Self::ProcMeminfo => PROC_MEMINFO_BYTES.len() as u64,
            Self::ProcDir | Self::DevDir | Self::DevNull | Self::DevMisc | Self::DevRtc => 0,
        }
    }

    fn identity(self, mounts: &VfsMountTable) -> FileIdentity {
        let len = self.byte_len(mounts);
        FileIdentity::new(
            self.inode(),
            len,
            false,
            self.mode(),
            1,
            blocks_for_len(len as usize),
            self.kind(),
        )
    }
}

pub fn builtin_stat(path: VfsPath<'_>, mounts: &VfsMountTable) -> Option<VfsStat> {
    BuiltinNode::from_path(path.bytes())
        .map(|node| VfsStat::new(node.identity(mounts), VFS_BUILTIN_BLOCK_SIZE))
}

pub fn builtin_open(
    path: VfsPath<'_>,
    options: OpenOptions,
    mounts: &VfsMountTable,
) -> Result<Option<OpenFileDescription>, VfsError> {
    let node = match BuiltinNode::from_path(path.bytes()) {
        Some(node) => node,
        None => return Ok(None),
    };
    if options.create() && options.exclusive() {
        return Err(VfsError::AlreadyExists);
    }
    if options.directory_required() && !node.kind().is_directory() {
        return Err(VfsError::DirectoryExpected);
    }
    if !node.kind().is_directory()
        && !matches!(node, BuiltinNode::DevNull | BuiltinNode::DevRtc)
        && options.writable()
    {
        return Err(VfsError::UnsupportedPath);
    }
    Ok(Some(OpenFileDescription::with_options(
        node.identity(mounts),
        options,
    )))
}

pub fn builtin_stat_open(file: OpenFileDescription, mounts: &VfsMountTable) -> Option<VfsStat> {
    BuiltinNode::from_inode(file.identity().inode())
        .map(|node| VfsStat::new(node.identity(mounts), VFS_BUILTIN_BLOCK_SIZE))
}

pub fn builtin_read_at(
    file: OpenFileDescription,
    mounts: &VfsMountTable,
    offset: u64,
    out: &mut [u8],
) -> Option<Result<usize, VfsError>> {
    let node = BuiltinNode::from_inode(file.identity().inode())?;
    Some(match node {
        BuiltinNode::ProcMounts => {
            let mut text = [0u8; VFS_BUILTIN_TEXT_BYTES];
            let len = write_proc_mounts(mounts, &mut text);
            Ok(read_builtin_bytes(&text[..len], offset, out))
        }
        BuiltinNode::ProcMeminfo => Ok(read_builtin_bytes(PROC_MEMINFO_BYTES, offset, out)),
        BuiltinNode::DevNull | BuiltinNode::DevRtc => Ok(0),
        BuiltinNode::ProcDir | BuiltinNode::DevDir | BuiltinNode::DevMisc => {
            Err(VfsError::NotRegularFile)
        }
    })
}

pub fn builtin_write(file: OpenFileDescription, bytes: &[u8]) -> Option<Result<usize, VfsError>> {
    let node = BuiltinNode::from_inode(file.identity().inode())?;
    Some(match node {
        BuiltinNode::DevNull | BuiltinNode::DevRtc => Ok(bytes.len()),
        _ => Err(VfsError::UnsupportedPath),
    })
}

pub fn builtin_ioctl(
    file: OpenFileDescription,
    request: usize,
) -> Option<Result<VfsIoctl, VfsError>> {
    const RTC_RD_TIME: usize = 0x8024_7009;
    let node = BuiltinNode::from_inode(file.identity().inode())?;
    Some(match (node, request & 0xffff_ffff) {
        (BuiltinNode::DevRtc, RTC_RD_TIME) => Ok(VfsIoctl::RtcReadTime),
        _ => Err(VfsError::UnsupportedPath),
    })
}

pub fn builtin_dir_entry_at(
    file: OpenFileDescription,
    offset: u64,
) -> Option<Result<Option<VfsDirEntry>, VfsError>> {
    let node = BuiltinNode::from_inode(file.identity().inode())?;
    Some(match node {
        BuiltinNode::ProcDir => builtin_dir_entry(
            offset,
            &[
                (b"mounts" as &[u8], VfsDirEntryKind::RegularFile),
                (b"meminfo" as &[u8], VfsDirEntryKind::RegularFile),
            ],
        ),
        BuiltinNode::DevDir => builtin_dir_entry(
            offset,
            &[
                (b"null" as &[u8], VfsDirEntryKind::CharacterDevice),
                (b"misc" as &[u8], VfsDirEntryKind::Directory),
            ],
        ),
        BuiltinNode::DevMisc => builtin_dir_entry(
            offset,
            &[(b"rtc" as &[u8], VfsDirEntryKind::CharacterDevice)],
        ),
        _ => Err(VfsError::DirectoryExpected),
    })
}

fn proc_mounts_len(mounts: &VfsMountTable) -> usize {
    let mut text = [0u8; VFS_BUILTIN_TEXT_BYTES];
    write_proc_mounts(mounts, &mut text)
}

fn write_proc_mounts(mounts: &VfsMountTable, out: &mut [u8]) -> usize {
    let mut cursor = 0usize;
    append_mount_line(out, &mut cursor, b"rootfs", b"/", b"rootfs");
    append_mount_line(out, &mut cursor, b"proc", b"/proc", b"proc");
    append_mount_line(out, &mut cursor, b"devtmpfs", b"/dev", b"devtmpfs");

    let mut index = 0usize;
    while index < VFS_MOUNT_CAPACITY {
        if let Some(record) = mounts.record_at(index) {
            append_mount_line(
                out,
                &mut cursor,
                record.source_bytes(),
                record.target_bytes(),
                record.filesystem_bytes(),
            );
        }
        index += 1;
    }
    cursor
}

fn append_mount_line(
    out: &mut [u8],
    cursor: &mut usize,
    source: &[u8],
    target: &[u8],
    filesystem: &[u8],
) {
    append_bytes(out, cursor, source);
    append_bytes(out, cursor, b" ");
    append_bytes(out, cursor, target);
    append_bytes(out, cursor, b" ");
    append_bytes(out, cursor, filesystem);
    append_bytes(out, cursor, b" rw 0 0\n");
}

fn append_bytes(out: &mut [u8], cursor: &mut usize, bytes: &[u8]) {
    let available = out.len().saturating_sub(*cursor);
    let count = core::cmp::min(available, bytes.len());
    if count != 0 {
        out[*cursor..*cursor + count].copy_from_slice(&bytes[..count]);
    }
    *cursor += count;
}

fn read_builtin_bytes(bytes: &[u8], offset: u64, out: &mut [u8]) -> usize {
    let start = offset as usize;
    if start >= bytes.len() {
        return 0;
    }
    let count = core::cmp::min(out.len(), bytes.len() - start);
    out[..count].copy_from_slice(&bytes[start..start + count]);
    count
}

fn builtin_dir_entry(
    offset: u64,
    entries: &[(&[u8], VfsDirEntryKind)],
) -> Result<Option<VfsDirEntry>, VfsError> {
    let index = offset as usize;
    if index >= entries.len() {
        return Ok(None);
    }
    let (name, kind) = entries[index];
    VfsDirEntry::new(
        VFS_BUILTIN_PROC_DIR_INODE + index as u32 + 0x100,
        offset + 1,
        name,
        kind,
    )
    .map(Some)
}

fn path_is_child(parent: &[u8], child: &[u8]) -> bool {
    if parent == b"/" {
        return child != b"/";
    }
    child.len() > parent.len()
        && child.starts_with(parent)
        && child.get(parent.len()) == Some(&b'/')
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WritableOverlay {
    directories: [VfsPathBuffer; VFS_OVERLAY_DIR_CAPACITY],
    modes: [u16; VFS_OVERLAY_DIR_CAPACITY],
    used: [bool; VFS_OVERLAY_DIR_CAPACITY],
    files: [OverlayFile; VFS_OVERLAY_FILE_CAPACITY],
    file_used: [bool; VFS_OVERLAY_FILE_CAPACITY],
}

impl WritableOverlay {
    pub const fn new() -> Self {
        Self {
            directories: [VfsPathBuffer::root(); VFS_OVERLAY_DIR_CAPACITY],
            modes: [0; VFS_OVERLAY_DIR_CAPACITY],
            used: [false; VFS_OVERLAY_DIR_CAPACITY],
            files: [OverlayFile::empty(); VFS_OVERLAY_FILE_CAPACITY],
            file_used: [false; VFS_OVERLAY_FILE_CAPACITY],
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn mkdir(&mut self, path: VfsPath<'_>, mode: u16) -> Result<(), VfsError> {
        validate_path(path.bytes())?;
        if path.bytes() == b"/"
            || self.directory_index(path.bytes()).is_some()
            || self.file_index(path.bytes()).is_some()
        {
            return Err(VfsError::AlreadyExists);
        }

        let slot = self.free_directory_slot().ok_or(VfsError::NoSpace)?;
        self.directories[slot] = VfsPathBuffer::from_absolute(path.bytes())?;
        self.modes[slot] = VFS_OVERLAY_DIR_TYPE | (mode & 0o7777);
        self.used[slot] = true;
        Ok(())
    }

    pub fn stat(&self, path: VfsPath<'_>) -> Result<Option<VfsStat>, VfsError> {
        validate_path(path.bytes())?;
        if let Some(index) = self.directory_index(path.bytes()) {
            return Ok(Some(self.directory_stat(index)));
        }
        Ok(self
            .file_index(path.bytes())
            .map(|index| self.file_stat(index)))
    }

    pub fn open(
        &mut self,
        path: VfsPath<'_>,
        options: OpenOptions,
        mode: u16,
    ) -> Result<Option<OpenFileDescription>, VfsError> {
        validate_path(path.bytes())?;
        if let Some(index) = self.directory_index(path.bytes()) {
            if options.create() && options.exclusive() {
                return Err(VfsError::AlreadyExists);
            }
            return Ok(Some(OpenFileDescription::with_options(
                self.directory_identity(index),
                options,
            )));
        }
        if let Some(index) = self.file_index(path.bytes()) {
            if options.create() && options.exclusive() {
                return Err(VfsError::AlreadyExists);
            }
            if options.truncate() && options.writable() {
                self.files[index].truncate();
            }
            return Ok(Some(OpenFileDescription::with_options(
                self.file_identity(index),
                options,
            )));
        }
        if !options.create() {
            return Ok(None);
        }

        let slot = self.free_file_slot().ok_or(VfsError::NoSpace)?;
        self.files[slot] = OverlayFile::new(path, mode)?;
        self.file_used[slot] = true;
        Ok(Some(OpenFileDescription::with_options(
            self.file_identity(slot),
            options,
        )))
    }

    pub fn stat_open(&self, file: OpenFileDescription) -> Option<VfsStat> {
        let inode = file.identity().inode();
        if !self.owns_inode(inode) {
            return None;
        }
        Some(VfsStat::new(
            self.identity_for_inode(inode)?,
            VFS_OVERLAY_BLOCK_SIZE,
        ))
    }

    pub fn read_at(
        &self,
        file: OpenFileDescription,
        offset: u64,
        out: &mut [u8],
    ) -> Result<usize, VfsError> {
        let index = self.file_index_by_inode(file.identity().inode())?;
        let start = offset as usize;
        if start >= self.files[index].len {
            return Ok(0);
        }
        let count = core::cmp::min(out.len(), self.files[index].len - start);
        out[..count].copy_from_slice(&self.files[index].bytes[start..start + count]);
        Ok(count)
    }

    pub fn write_at(
        &mut self,
        file: OpenFileDescription,
        offset: u64,
        bytes: &[u8],
    ) -> Result<usize, VfsError> {
        let index = self.file_index_by_inode(file.identity().inode())?;
        self.files[index].write_at(offset, bytes)
    }

    pub fn file_len(&self, file: OpenFileDescription) -> Result<u64, VfsError> {
        let index = self.file_index_by_inode(file.identity().inode())?;
        Ok(self.files[index].len as u64)
    }

    pub fn unlink(&mut self, path: VfsPath<'_>) -> Result<(), VfsError> {
        validate_path(path.bytes())?;
        if self.directory_index(path.bytes()).is_some() {
            return Err(VfsError::DirectoryExpected);
        }
        let index = self
            .file_index(path.bytes())
            .ok_or(VfsError::PathNotFound)?;
        self.files[index] = OverlayFile::empty();
        self.file_used[index] = false;
        Ok(())
    }

    pub fn rmdir(&mut self, path: VfsPath<'_>) -> Result<(), VfsError> {
        validate_path(path.bytes())?;
        let index = self
            .directory_index(path.bytes())
            .ok_or(VfsError::PathNotFound)?;
        if self.directory_has_child(path.bytes()) {
            return Err(VfsError::UnsupportedPath);
        }
        self.directories[index] = VfsPathBuffer::root();
        self.modes[index] = 0;
        self.used[index] = false;
        Ok(())
    }

    pub fn rename(&mut self, old: VfsPath<'_>, new: VfsPath<'_>) -> Result<(), VfsError> {
        validate_path(old.bytes())?;
        validate_path(new.bytes())?;
        if new.bytes() == b"/"
            || self.directory_index(new.bytes()).is_some()
            || self.file_index(new.bytes()).is_some()
        {
            return Err(VfsError::AlreadyExists);
        }
        if let Some(index) = self.directory_index(old.bytes()) {
            self.directories[index] = VfsPathBuffer::from_absolute(new.bytes())?;
            return Ok(());
        }
        if let Some(index) = self.file_index(old.bytes()) {
            self.files[index].path = VfsPathBuffer::from_absolute(new.bytes())?;
            return Ok(());
        }
        Err(VfsError::PathNotFound)
    }

    pub fn dir_entry_at(
        &self,
        file: OpenFileDescription,
        _offset: u64,
    ) -> Result<Option<VfsDirEntry>, VfsError> {
        if !self.owns_inode(file.identity().inode()) {
            return Err(VfsError::PathNotFound);
        }
        if !file.is_directory() {
            return Err(VfsError::DirectoryExpected);
        }
        Ok(None)
    }

    pub const fn owns_identity(&self, identity: FileIdentity) -> bool {
        self.owns_inode(identity.inode())
    }

    const fn owns_inode(&self, inode: u32) -> bool {
        (inode >= VFS_OVERLAY_INODE_BASE
            && inode < VFS_OVERLAY_INODE_BASE + VFS_OVERLAY_DIR_CAPACITY as u32)
            || (inode >= VFS_OVERLAY_FILE_INODE_BASE
                && inode < VFS_OVERLAY_FILE_INODE_BASE + VFS_OVERLAY_FILE_CAPACITY as u32)
    }

    fn directory_index(&self, path: &[u8]) -> Option<usize> {
        let mut index = 0usize;
        while index < self.directories.len() {
            if self.used[index] && self.directories[index].bytes() == path {
                return Some(index);
            }
            index += 1;
        }
        None
    }

    fn directory_has_child(&self, path: &[u8]) -> bool {
        let mut index = 0usize;
        while index < self.directories.len() {
            if self.used[index] && path_is_child(path, self.directories[index].bytes()) {
                return true;
            }
            index += 1;
        }
        let mut file = 0usize;
        while file < self.files.len() {
            if self.file_used[file] && path_is_child(path, self.files[file].path.bytes()) {
                return true;
            }
            file += 1;
        }
        false
    }

    fn free_directory_slot(&self) -> Option<usize> {
        let mut index = 0usize;
        while index < self.used.len() {
            if !self.used[index] {
                return Some(index);
            }
            index += 1;
        }
        None
    }

    fn file_index(&self, path: &[u8]) -> Option<usize> {
        let mut index = 0usize;
        while index < self.files.len() {
            if self.file_used[index] && self.files[index].path.bytes() == path {
                return Some(index);
            }
            index += 1;
        }
        None
    }

    fn file_index_by_inode(&self, inode: u32) -> Result<usize, VfsError> {
        if inode < VFS_OVERLAY_FILE_INODE_BASE {
            return Err(VfsError::NotRegularFile);
        }
        let index = (inode - VFS_OVERLAY_FILE_INODE_BASE) as usize;
        if index >= self.files.len() || !self.file_used[index] {
            return Err(VfsError::PathNotFound);
        }
        Ok(index)
    }

    fn free_file_slot(&self) -> Option<usize> {
        let mut index = 0usize;
        while index < self.file_used.len() {
            if !self.file_used[index] {
                return Some(index);
            }
            index += 1;
        }
        None
    }

    fn directory_stat(&self, index: usize) -> VfsStat {
        VfsStat::new(self.directory_identity(index), VFS_OVERLAY_BLOCK_SIZE)
    }

    fn file_stat(&self, index: usize) -> VfsStat {
        VfsStat::new(self.file_identity(index), VFS_OVERLAY_BLOCK_SIZE)
    }

    fn directory_identity(&self, index: usize) -> FileIdentity {
        FileIdentity::new(
            VFS_OVERLAY_INODE_BASE + index as u32,
            0,
            false,
            self.modes[index],
            2,
            0,
            VfsNodeKind::Directory,
        )
    }

    fn file_identity(&self, index: usize) -> FileIdentity {
        FileIdentity::new(
            VFS_OVERLAY_FILE_INODE_BASE + index as u32,
            self.files[index].len as u64,
            false,
            self.files[index].mode,
            1,
            blocks_for_len(self.files[index].len),
            VfsNodeKind::RegularFile,
        )
    }

    fn identity_for_inode(&self, inode: u32) -> Option<FileIdentity> {
        if inode >= VFS_OVERLAY_INODE_BASE
            && inode < VFS_OVERLAY_INODE_BASE + VFS_OVERLAY_DIR_CAPACITY as u32
        {
            let index = (inode - VFS_OVERLAY_INODE_BASE) as usize;
            if self.used[index] {
                return Some(self.directory_identity(index));
            }
        }
        if inode >= VFS_OVERLAY_FILE_INODE_BASE
            && inode < VFS_OVERLAY_FILE_INODE_BASE + VFS_OVERLAY_FILE_CAPACITY as u32
        {
            let index = (inode - VFS_OVERLAY_FILE_INODE_BASE) as usize;
            if self.file_used[index] {
                return Some(self.file_identity(index));
            }
        }
        None
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VfsMountTable {
    records: [VfsMountRecord; VFS_MOUNT_CAPACITY],
    used: [bool; VFS_MOUNT_CAPACITY],
}

impl VfsMountTable {
    pub const fn new() -> Self {
        Self {
            records: [VfsMountRecord::empty(); VFS_MOUNT_CAPACITY],
            used: [false; VFS_MOUNT_CAPACITY],
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn mount(
        &mut self,
        target: VfsPath<'_>,
        source: &[u8],
        filesystem: &[u8],
        flags: usize,
    ) -> Result<(), VfsError> {
        validate_path(target.bytes())?;
        validate_mount_text(source)?;
        validate_mount_text(filesystem)?;
        if flags != 0 {
            return Err(VfsError::UnsupportedPath);
        }
        if self.target_index(target.bytes()).is_some() {
            return Err(VfsError::AlreadyExists);
        }

        let slot = self.free_slot().ok_or(VfsError::NoSpace)?;
        self.records[slot] = VfsMountRecord::new(target, source, filesystem, flags)?;
        self.used[slot] = true;
        Ok(())
    }

    pub fn unmount(&mut self, target: VfsPath<'_>) -> Result<(), VfsError> {
        validate_path(target.bytes())?;
        let index = self
            .target_index(target.bytes())
            .ok_or(VfsError::PathNotFound)?;
        self.records[index] = VfsMountRecord::empty();
        self.used[index] = false;
        Ok(())
    }

    pub fn record_at(&self, index: usize) -> Option<VfsMountRecord> {
        if index < self.records.len() && self.used[index] {
            Some(self.records[index])
        } else {
            None
        }
    }

    fn target_index(&self, target: &[u8]) -> Option<usize> {
        let mut index = 0usize;
        while index < self.records.len() {
            if self.used[index] && self.records[index].target_bytes() == target {
                return Some(index);
            }
            index += 1;
        }
        None
    }

    fn free_slot(&self) -> Option<usize> {
        let mut index = 0usize;
        while index < self.used.len() {
            if !self.used[index] {
                return Some(index);
            }
            index += 1;
        }
        None
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VfsMountRecord {
    target: VfsPathBuffer,
    source: VfsMountText,
    filesystem: VfsMountText,
    flags: usize,
}

impl VfsMountRecord {
    pub const fn empty() -> Self {
        Self {
            target: VfsPathBuffer::root(),
            source: VfsMountText::empty(),
            filesystem: VfsMountText::empty(),
            flags: 0,
        }
    }

    fn new(
        target: VfsPath<'_>,
        source: &[u8],
        filesystem: &[u8],
        flags: usize,
    ) -> Result<Self, VfsError> {
        Ok(Self {
            target: VfsPathBuffer::from_absolute(target.bytes())?,
            source: VfsMountText::from_bytes(source)?,
            filesystem: VfsMountText::from_bytes(filesystem)?,
            flags,
        })
    }

    pub fn target_bytes(&self) -> &[u8] {
        self.target.bytes()
    }

    pub fn source_bytes(&self) -> &[u8] {
        self.source.bytes()
    }

    pub fn filesystem_bytes(&self) -> &[u8] {
        self.filesystem.bytes()
    }

    pub const fn flags(self) -> usize {
        self.flags
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct VfsMountText {
    bytes: [u8; VFS_MOUNT_TEXT_MAX],
    len: usize,
}

impl VfsMountText {
    pub const fn empty() -> Self {
        Self {
            bytes: [0; VFS_MOUNT_TEXT_MAX],
            len: 0,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, VfsError> {
        validate_mount_text(bytes)?;
        let mut text = Self::empty();
        text.bytes[..bytes.len()].copy_from_slice(bytes);
        text.len = bytes.len();
        Ok(text)
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct OverlayFile {
    path: VfsPathBuffer,
    mode: u16,
    len: usize,
    bytes: [u8; VFS_OVERLAY_FILE_BYTES],
}

impl OverlayFile {
    pub const fn empty() -> Self {
        Self {
            path: VfsPathBuffer::root(),
            mode: VFS_OVERLAY_REG_TYPE | 0o666,
            len: 0,
            bytes: [0; VFS_OVERLAY_FILE_BYTES],
        }
    }

    pub fn new(path: VfsPath<'_>, mode: u16) -> Result<Self, VfsError> {
        let mut file = Self::empty();
        file.path = VfsPathBuffer::from_absolute(path.bytes())?;
        file.mode = VFS_OVERLAY_REG_TYPE | (mode & 0o7777);
        Ok(file)
    }

    pub fn truncate(&mut self) {
        self.len = 0;
    }

    pub fn write_at(&mut self, offset: u64, bytes: &[u8]) -> Result<usize, VfsError> {
        let start = offset as usize;
        if start > self.bytes.len() {
            return Err(VfsError::NoSpace);
        }
        let count = core::cmp::min(bytes.len(), self.bytes.len() - start);
        self.bytes[start..start + count].copy_from_slice(&bytes[..count]);
        self.len = core::cmp::max(self.len, start + count);
        Ok(count)
    }
}

const fn blocks_for_len(len: usize) -> u64 {
    if len == 0 {
        0
    } else {
        ((len + VFS_OVERLAY_BLOCK_SIZE as usize - 1) / VFS_OVERLAY_BLOCK_SIZE as usize) as u64
    }
}

pub trait SyscallVfs {
    fn stat_path(&self, path: VfsPath<'_>) -> Result<VfsStat, VfsError>;
    fn stat_path_at(&self, dirfd: isize, path: &[u8]) -> Result<VfsStat, VfsRuntimeError>;
    fn access_path_at(&self, dirfd: isize, path: &[u8], mode: u8) -> Result<(), VfsRuntimeError> {
        let stat = self.stat_path_at(dirfd, path)?;
        if stat.allows_access(mode) {
            Ok(())
        } else {
            Err(VfsRuntimeError::Vfs(VfsError::PermissionDenied))
        }
    }
    fn open_path(
        &self,
        path: VfsPath<'_>,
        options: OpenOptions,
        mode: u16,
    ) -> Result<usize, VfsRuntimeError>;
    fn open_path_at(
        &self,
        dirfd: isize,
        path: &[u8],
        options: OpenOptions,
        mode: u16,
    ) -> Result<usize, VfsRuntimeError>;
    fn change_dir(&self, path: &[u8]) -> Result<(), VfsRuntimeError>;
    fn mkdir_at(&self, dirfd: isize, path: &[u8], mode: u16) -> Result<(), VfsRuntimeError>;
    fn unlink_at(&self, dirfd: isize, path: &[u8], flags: usize) -> Result<(), VfsRuntimeError>;
    fn rename_at(
        &self,
        old_dirfd: isize,
        old_path: &[u8],
        new_dirfd: isize,
        new_path: &[u8],
        flags: usize,
    ) -> Result<(), VfsRuntimeError>;
    fn mount(
        &self,
        source: &[u8],
        target: &[u8],
        filesystem: &[u8],
        flags: usize,
    ) -> Result<(), VfsRuntimeError>;
    fn unmount(&self, target: &[u8], flags: usize) -> Result<(), VfsRuntimeError>;
    fn pipe(&self) -> Result<(usize, usize), VfsRuntimeError>;
    fn owns_stdio_fds(&self) -> bool;
    fn getcwd(&self, out: &mut [u8]) -> Result<usize, VfsRuntimeError>;
    fn close_fd(&self, fd: usize) -> Result<(), FdError>;
    fn fstat_fd(&self, fd: usize) -> Result<VfsStat, VfsRuntimeError>;
    fn read_fd(&self, fd: usize, out: &mut [u8]) -> Result<usize, VfsRuntimeError>;
    fn read_fd_at(&self, fd: usize, offset: u64, out: &mut [u8]) -> Result<usize, VfsRuntimeError>;
    fn write_fd(&self, fd: usize, bytes: &[u8]) -> Result<usize, VfsRuntimeError>;
    fn ioctl_fd(&self, fd: usize, request: usize) -> Result<VfsIoctl, VfsRuntimeError>;
    fn fd_readable(&self, fd: usize) -> Result<bool, VfsRuntimeError>;
    fn fd_writable(&self, fd: usize) -> Result<bool, VfsRuntimeError>;
    fn lseek_fd(&self, fd: usize, offset: i64, whence: usize) -> Result<u64, VfsRuntimeError>;
    fn dir_entry_at_fd(&self, fd: usize) -> Result<Option<VfsDirEntry>, VfsRuntimeError>;
    fn set_fd_offset(&self, fd: usize, offset: u64) -> Result<(), FdError>;
    fn set_fd_close_on_exec(&self, fd: usize, close_on_exec: bool) -> Result<(), FdError>;
    fn fd_close_on_exec(&self, fd: usize) -> Result<bool, FdError>;
    fn fd_status_flags(&self, fd: usize) -> Result<usize, FdError>;
    fn duplicate_fd_min(
        &self,
        old_fd: usize,
        min_fd: usize,
        close_on_exec: bool,
    ) -> Result<usize, FdError>;
    fn duplicate_fd_to(
        &self,
        old_fd: usize,
        new_fd: usize,
        close_on_exec: bool,
    ) -> Result<usize, FdError>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NoSyscallVfs;

impl SyscallVfs for NoSyscallVfs {
    fn stat_path(&self, _path: VfsPath<'_>) -> Result<VfsStat, VfsError> {
        Err(VfsError::RootfsSourceMissing)
    }

    fn stat_path_at(&self, _dirfd: isize, _path: &[u8]) -> Result<VfsStat, VfsRuntimeError> {
        Err(VfsRuntimeError::Vfs(VfsError::RootfsSourceMissing))
    }

    fn open_path(
        &self,
        _path: VfsPath<'_>,
        _options: OpenOptions,
        _mode: u16,
    ) -> Result<usize, VfsRuntimeError> {
        Err(VfsRuntimeError::Vfs(VfsError::RootfsSourceMissing))
    }

    fn open_path_at(
        &self,
        _dirfd: isize,
        _path: &[u8],
        _options: OpenOptions,
        _mode: u16,
    ) -> Result<usize, VfsRuntimeError> {
        Err(VfsRuntimeError::Vfs(VfsError::RootfsSourceMissing))
    }

    fn change_dir(&self, _path: &[u8]) -> Result<(), VfsRuntimeError> {
        Err(VfsRuntimeError::Vfs(VfsError::RootfsSourceMissing))
    }

    fn mkdir_at(&self, _dirfd: isize, _path: &[u8], _mode: u16) -> Result<(), VfsRuntimeError> {
        Err(VfsRuntimeError::Vfs(VfsError::RootfsSourceMissing))
    }

    fn unlink_at(&self, _dirfd: isize, _path: &[u8], _flags: usize) -> Result<(), VfsRuntimeError> {
        Err(VfsRuntimeError::Vfs(VfsError::RootfsSourceMissing))
    }

    fn rename_at(
        &self,
        _old_dirfd: isize,
        _old_path: &[u8],
        _new_dirfd: isize,
        _new_path: &[u8],
        _flags: usize,
    ) -> Result<(), VfsRuntimeError> {
        Err(VfsRuntimeError::Vfs(VfsError::RootfsSourceMissing))
    }

    fn mount(
        &self,
        _source: &[u8],
        _target: &[u8],
        _filesystem: &[u8],
        _flags: usize,
    ) -> Result<(), VfsRuntimeError> {
        Err(VfsRuntimeError::Vfs(VfsError::RootfsSourceMissing))
    }

    fn unmount(&self, _target: &[u8], _flags: usize) -> Result<(), VfsRuntimeError> {
        Err(VfsRuntimeError::Vfs(VfsError::RootfsSourceMissing))
    }

    fn pipe(&self) -> Result<(usize, usize), VfsRuntimeError> {
        Err(VfsRuntimeError::Vfs(VfsError::RootfsSourceMissing))
    }

    fn owns_stdio_fds(&self) -> bool {
        false
    }

    fn getcwd(&self, _out: &mut [u8]) -> Result<usize, VfsRuntimeError> {
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

    fn read_fd_at(
        &self,
        _fd: usize,
        _offset: u64,
        _out: &mut [u8],
    ) -> Result<usize, VfsRuntimeError> {
        Err(VfsRuntimeError::Fd(FdError::BadFileDescriptor))
    }

    fn write_fd(&self, _fd: usize, _bytes: &[u8]) -> Result<usize, VfsRuntimeError> {
        Err(VfsRuntimeError::Fd(FdError::BadFileDescriptor))
    }

    fn ioctl_fd(&self, _fd: usize, _request: usize) -> Result<VfsIoctl, VfsRuntimeError> {
        Err(VfsRuntimeError::Fd(FdError::BadFileDescriptor))
    }

    fn fd_readable(&self, _fd: usize) -> Result<bool, VfsRuntimeError> {
        Err(VfsRuntimeError::Fd(FdError::BadFileDescriptor))
    }

    fn fd_writable(&self, _fd: usize) -> Result<bool, VfsRuntimeError> {
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

    fn fd_close_on_exec(&self, _fd: usize) -> Result<bool, FdError> {
        Err(FdError::BadFileDescriptor)
    }

    fn fd_status_flags(&self, _fd: usize) -> Result<usize, FdError> {
        Err(FdError::BadFileDescriptor)
    }

    fn duplicate_fd_min(
        &self,
        _old_fd: usize,
        _min_fd: usize,
        _close_on_exec: bool,
    ) -> Result<usize, FdError> {
        Err(FdError::BadFileDescriptor)
    }

    fn duplicate_fd_to(
        &self,
        _old_fd: usize,
        _new_fd: usize,
        _close_on_exec: bool,
    ) -> Result<usize, FdError> {
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
    AlreadyExists,
    Block(BlockIoError),
    DirectoryExpected,
    EmptyFile,
    FileTooLarge,
    InvalidPath,
    MetadataCorrupt,
    NoSpace,
    NotExecutable,
    NotRegularFile,
    PathNotFound,
    PermissionDenied,
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
        if options.writable() || options.create() || options.truncate() || options.append() {
            return Err(VfsError::UnsupportedPath);
        }
        validate_path(path.bytes())?;
        let file = self
            .volume
            .lookup_path(&mut self.cache, self.provider, path.bytes())
            .map_err(map_ext4_error)?;
        let identity = file.identity();
        if options.directory_required() && !identity.is_directory() {
            return Err(VfsError::DirectoryExpected);
        }

        Ok(OpenFileDescription::with_options(identity, options))
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

fn validate_mount_text(bytes: &[u8]) -> Result<(), VfsError> {
    if bytes.is_empty() || bytes.len() > VFS_MOUNT_TEXT_MAX {
        return Err(VfsError::InvalidPath);
    }

    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] == 0 {
            return Err(VfsError::InvalidPath);
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
