//! Minimal read-only ext4 reader for file-backed exec.

use crate::core::block::{BlockCache, BlockIoError, BlockProvider};

use super::vfs::{FileIdentity, VfsDirEntry, VfsDirEntryKind};

const EXT4_SUPER_OFFSET: u64 = 1024;
const EXT4_SUPER_SIZE: usize = 1024;
const EXT4_SUPER_MAGIC: u16 = 0xef53;
const EXT4_ROOT_INODE: u32 = 2;
const EXT4_MIN_BLOCK_SIZE: usize = 1024;
const EXT4_MAX_BLOCK_SIZE: usize = 4096;
const EXT4_MAX_INODE_SIZE: usize = 512;
const EXT4_NAME_MAX: usize = 255;
const EXT4_EXTENTS_FL: u32 = 0x0008_0000;
const EXT4_INDEX_FL: u32 = 0x0000_1000;
const EXT4_EXTENT_MAGIC: u16 = 0xf30a;
const EXT4_EXTENT_HEADER_SIZE: usize = 12;
const EXT4_EXTENT_SIZE: usize = 12;
const EXT4_EXTENT_INDEX_SIZE: usize = 12;
const EXT4_INCOMPAT_FILETYPE: u32 = 0x0002;
const EXT4_INCOMPAT_EXTENTS: u32 = 0x0040;
const EXT4_INCOMPAT_64BIT: u32 = 0x0080;
const EXT4_INCOMPAT_FLEX_BG: u32 = 0x0200;
const EXT4_ALLOWED_INCOMPAT: u32 =
    EXT4_INCOMPAT_FILETYPE | EXT4_INCOMPAT_EXTENTS | EXT4_INCOMPAT_64BIT | EXT4_INCOMPAT_FLEX_BG;
const EXT4_S_IFMT: u16 = 0xf000;
const EXT4_S_IFDIR: u16 = 0x4000;
const EXT4_S_IFREG: u16 = 0x8000;
const EXT4_EXEC_BITS: u16 = 0o111;
const EXT4_FT_REG_FILE: u8 = 1;
const EXT4_FT_DIR: u8 = 2;
const EXT4_FT_CHRDEV: u8 = 3;
const EXT4_FT_BLKDEV: u8 = 4;
const EXT4_FT_FIFO: u8 = 5;
const EXT4_FT_SOCK: u8 = 6;
const EXT4_FT_SYMLINK: u8 = 7;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Ext4Error {
    Block(BlockIoError),
    DirectoryExpected,
    FileTooLarge,
    MetadataCorrupt,
    NotRegularFile,
    PathNotFound,
    UnsupportedPath,
    UnsupportedRootfs,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Ext4Volume {
    block_size: usize,
    block_count: u64,
    blocks_per_group: u32,
    inodes_per_group: u32,
    inode_size: usize,
    descriptor_size: usize,
    descriptor_table_block: u64,
    first_data_block: u32,
}

impl Ext4Volume {
    pub fn mount(cache: &mut BlockCache, provider: BlockProvider) -> Result<Self, Ext4Error> {
        let mut superblock = [0u8; EXT4_SUPER_SIZE];
        read_bytes(cache, provider, EXT4_SUPER_OFFSET, &mut superblock)?;
        Self::from_superblock(&superblock)
    }

    pub fn lookup_path(
        self,
        cache: &mut BlockCache,
        provider: BlockProvider,
        path: &[u8],
    ) -> Result<Ext4File, Ext4Error> {
        if path.is_empty() || path[0] != b'/' {
            return Err(Ext4Error::UnsupportedPath);
        }

        let mut inode = self.read_inode(cache, provider, EXT4_ROOT_INODE)?;
        let mut cursor = 1usize;
        loop {
            cursor = skip_separators(path, cursor);
            if cursor >= path.len() {
                return Ok(Ext4File::from_inode(inode));
            }

            let start = cursor;
            while cursor < path.len() && path[cursor] != b'/' {
                cursor += 1;
            }
            let component = &path[start..cursor];
            validate_component(component)?;
            inode = self.lookup_child(cache, provider, inode, component)?;
        }
    }

    pub fn read_file(
        self,
        cache: &mut BlockCache,
        provider: BlockProvider,
        file: Ext4File,
        offset: u64,
        out: &mut [u8],
    ) -> Result<usize, Ext4Error> {
        if !file.is_regular() {
            return Err(Ext4Error::NotRegularFile);
        }
        let inode = self.read_inode(cache, provider, file.inode())?;
        self.read_inode_bytes(cache, provider, inode, offset, out)
    }

    pub fn read_dir_entry_at(
        self,
        cache: &mut BlockCache,
        provider: BlockProvider,
        directory: FileIdentity,
        offset: u64,
    ) -> Result<Option<VfsDirEntry>, Ext4Error> {
        if !directory.is_directory() {
            return Err(Ext4Error::DirectoryExpected);
        }
        let inode = self.read_inode(cache, provider, directory.inode())?;
        if !inode.is_dir() {
            return Err(Ext4Error::DirectoryExpected);
        }
        if inode.flags & EXT4_INDEX_FL != 0 {
            return Err(Ext4Error::UnsupportedRootfs);
        }
        if offset >= inode.size {
            return Ok(None);
        }

        let mut cursor = offset;
        while cursor < inode.size {
            let logical_block = (cursor / self.block_size as u64) as u32;
            let in_block = (cursor % self.block_size as u64) as usize;
            let mut block = [0u8; EXT4_MAX_BLOCK_SIZE];
            if self
                .read_inode_block(cache, provider, inode, logical_block, &mut block)?
                .is_none()
            {
                cursor = next_logical_block_offset(cursor, self.block_size)?;
                continue;
            }
            if self.block_size - in_block < 8 {
                return Err(Ext4Error::MetadataCorrupt);
            }

            let inode_number = read_u32(&block, in_block)?;
            let rec_len = read_u16(&block, in_block + 4)? as usize;
            let name_len = block[in_block + 6] as usize;
            let file_type = block[in_block + 7];
            if rec_len < 8 || rec_len % 4 != 0 || in_block + rec_len > self.block_size {
                return Err(Ext4Error::MetadataCorrupt);
            }
            if name_len > rec_len - 8 || name_len > EXT4_NAME_MAX {
                return Err(Ext4Error::MetadataCorrupt);
            }
            let next_offset = cursor
                .checked_add(rec_len as u64)
                .ok_or(Ext4Error::MetadataCorrupt)?;
            if inode_number != 0 {
                if name_len == 0 {
                    return Err(Ext4Error::MetadataCorrupt);
                }
                let name = &block[in_block + 8..in_block + 8 + name_len];
                let kind = dir_entry_kind(file_type);
                let entry = VfsDirEntry::new(inode_number, next_offset, name, kind)
                    .map_err(|_| Ext4Error::MetadataCorrupt)?;
                return Ok(Some(entry));
            }
            cursor = next_offset;
        }

        Ok(None)
    }

    pub const fn block_size(self) -> usize {
        self.block_size
    }

    fn from_superblock(superblock: &[u8; EXT4_SUPER_SIZE]) -> Result<Self, Ext4Error> {
        if read_u16(superblock, 56)? != EXT4_SUPER_MAGIC {
            return Err(Ext4Error::UnsupportedRootfs);
        }

        let log_block_size = read_u32(superblock, 24)?;
        if log_block_size > 2 {
            return Err(Ext4Error::UnsupportedRootfs);
        }
        let block_size = EXT4_MIN_BLOCK_SIZE
            .checked_shl(log_block_size)
            .ok_or(Ext4Error::UnsupportedRootfs)?;
        if !(EXT4_MIN_BLOCK_SIZE..=EXT4_MAX_BLOCK_SIZE).contains(&block_size) {
            return Err(Ext4Error::UnsupportedRootfs);
        }

        let incompat = read_u32(superblock, 96)?;
        if incompat & !EXT4_ALLOWED_INCOMPAT != 0 {
            return Err(Ext4Error::UnsupportedRootfs);
        }
        if incompat & EXT4_INCOMPAT_EXTENTS == 0 {
            return Err(Ext4Error::UnsupportedRootfs);
        }

        let blocks_lo = read_u32(superblock, 4)? as u64;
        let blocks_hi = read_u32(superblock, 0x150)? as u64;
        let block_count = blocks_lo | (blocks_hi << 32);
        let blocks_per_group = read_u32(superblock, 32)?;
        let inodes_per_group = read_u32(superblock, 40)?;
        let inode_size = read_u16(superblock, 88)? as usize;
        let first_data_block = read_u32(superblock, 20)?;
        let descriptor_size = if incompat & EXT4_INCOMPAT_64BIT != 0 {
            read_u16(superblock, 0xfe)? as usize
        } else {
            32
        };

        if block_count == 0
            || blocks_per_group == 0
            || inodes_per_group == 0
            || inode_size < 128
            || inode_size > EXT4_MAX_INODE_SIZE
            || descriptor_size < 32
            || descriptor_size > block_size
        {
            return Err(Ext4Error::MetadataCorrupt);
        }

        Ok(Self {
            block_size,
            block_count,
            blocks_per_group,
            inodes_per_group,
            inode_size,
            descriptor_size,
            descriptor_table_block: first_data_block as u64 + 1,
            first_data_block,
        })
    }

    fn read_inode(
        self,
        cache: &mut BlockCache,
        provider: BlockProvider,
        inode_number: u32,
    ) -> Result<Ext4Inode, Ext4Error> {
        if inode_number == 0 {
            return Err(Ext4Error::MetadataCorrupt);
        }

        let group = (inode_number - 1) / self.inodes_per_group;
        let index = (inode_number - 1) % self.inodes_per_group;
        let descriptor = self.read_group_descriptor(cache, provider, group)?;
        let byte_offset = (index as u64)
            .checked_mul(self.inode_size as u64)
            .ok_or(Ext4Error::MetadataCorrupt)?;
        let table_offset = descriptor
            .inode_table_block
            .checked_mul(self.block_size as u64)
            .and_then(|base| base.checked_add(byte_offset))
            .ok_or(Ext4Error::MetadataCorrupt)?;
        let mut bytes = [0u8; EXT4_MAX_INODE_SIZE];
        read_bytes(cache, provider, table_offset, &mut bytes[..self.inode_size])?;
        Ext4Inode::parse(inode_number, &bytes[..self.inode_size])
    }

    fn read_group_descriptor(
        self,
        cache: &mut BlockCache,
        provider: BlockProvider,
        group: u32,
    ) -> Result<GroupDescriptor, Ext4Error> {
        let groups = div_ceil_u64(
            self.block_count
                .saturating_sub(self.first_data_block as u64),
            self.blocks_per_group as u64,
        );
        if group as u64 >= groups {
            return Err(Ext4Error::MetadataCorrupt);
        }

        let descriptor_offset = self
            .descriptor_table_block
            .checked_mul(self.block_size as u64)
            .and_then(|base| base.checked_add(group as u64 * self.descriptor_size as u64))
            .ok_or(Ext4Error::MetadataCorrupt)?;
        let mut descriptor = [0u8; 64];
        let read_len = min_usize(self.descriptor_size, descriptor.len());
        read_bytes(
            cache,
            provider,
            descriptor_offset,
            &mut descriptor[..read_len],
        )?;

        let inode_table_lo = read_u32(&descriptor, 8)? as u64;
        let inode_table_hi = if self.descriptor_size >= 64 {
            read_u32(&descriptor, 40)? as u64
        } else {
            0
        };
        let inode_table_block = inode_table_lo | (inode_table_hi << 32);
        if inode_table_block == 0 || inode_table_block >= self.block_count {
            return Err(Ext4Error::MetadataCorrupt);
        }

        Ok(GroupDescriptor { inode_table_block })
    }

    fn lookup_child(
        self,
        cache: &mut BlockCache,
        provider: BlockProvider,
        directory: Ext4Inode,
        name: &[u8],
    ) -> Result<Ext4Inode, Ext4Error> {
        if !directory.is_dir() {
            return Err(Ext4Error::DirectoryExpected);
        }
        if directory.flags & EXT4_INDEX_FL != 0 {
            return Err(Ext4Error::UnsupportedRootfs);
        }

        let mut logical_block = 0u32;
        let block_count = div_ceil_u64(directory.size, self.block_size as u64);
        while (logical_block as u64) < block_count {
            let mut block = [0u8; EXT4_MAX_BLOCK_SIZE];
            if self
                .read_inode_block(cache, provider, directory, logical_block, &mut block)?
                .is_some()
            {
                if let Some(inode) = self.find_dir_entry(cache, provider, &block, name)? {
                    return Ok(inode);
                }
            }
            logical_block += 1;
        }

        Err(Ext4Error::PathNotFound)
    }

    fn find_dir_entry(
        self,
        cache: &mut BlockCache,
        provider: BlockProvider,
        block: &[u8; EXT4_MAX_BLOCK_SIZE],
        name: &[u8],
    ) -> Result<Option<Ext4Inode>, Ext4Error> {
        let mut offset = 0usize;
        while offset < self.block_size {
            if self.block_size - offset < 8 {
                return Err(Ext4Error::MetadataCorrupt);
            }
            let inode_number = read_u32(block, offset)?;
            let rec_len = read_u16(block, offset + 4)? as usize;
            let name_len = block[offset + 6] as usize;
            if rec_len < 8 || rec_len % 4 != 0 || offset + rec_len > self.block_size {
                return Err(Ext4Error::MetadataCorrupt);
            }
            if name_len > rec_len - 8 || name_len > EXT4_NAME_MAX {
                return Err(Ext4Error::MetadataCorrupt);
            }

            if inode_number != 0
                && name_len == name.len()
                && bytes_eq(&block[offset + 8..offset + 8 + name_len], name)
            {
                return Ok(Some(self.read_inode(cache, provider, inode_number)?));
            }
            offset += rec_len;
        }

        Ok(None)
    }

    fn read_inode_bytes(
        self,
        cache: &mut BlockCache,
        provider: BlockProvider,
        inode: Ext4Inode,
        offset: u64,
        out: &mut [u8],
    ) -> Result<usize, Ext4Error> {
        if !inode.is_regular() {
            return Err(Ext4Error::NotRegularFile);
        }
        if offset >= inode.size {
            return Ok(0);
        }

        let remaining_in_file = inode.size - offset;
        let want = min_usize(
            out.len(),
            min_u64(remaining_in_file, usize::MAX as u64) as usize,
        );
        let mut copied = 0usize;
        while copied < want {
            let absolute = offset + copied as u64;
            let logical_block = (absolute / self.block_size as u64) as u32;
            let in_block = (absolute % self.block_size as u64) as usize;
            let mut block = [0u8; EXT4_MAX_BLOCK_SIZE];
            self.read_inode_block(cache, provider, inode, logical_block, &mut block)?;
            let take = min_usize(want - copied, self.block_size - in_block);
            out[copied..copied + take].copy_from_slice(&block[in_block..in_block + take]);
            copied += take;
        }

        Ok(copied)
    }

    fn read_inode_block(
        self,
        cache: &mut BlockCache,
        provider: BlockProvider,
        inode: Ext4Inode,
        logical_block: u32,
        out: &mut [u8; EXT4_MAX_BLOCK_SIZE],
    ) -> Result<Option<u64>, Ext4Error> {
        zero_block(out);
        let physical = match self.extent_physical_block(cache, provider, inode, logical_block)? {
            Some(block) => block,
            None => return Ok(None),
        };
        self.read_block(cache, provider, physical, out)?;
        Ok(Some(physical))
    }

    fn read_block(
        self,
        cache: &mut BlockCache,
        provider: BlockProvider,
        block: u64,
        out: &mut [u8; EXT4_MAX_BLOCK_SIZE],
    ) -> Result<(), Ext4Error> {
        if block >= self.block_count {
            return Err(Ext4Error::MetadataCorrupt);
        }
        zero_block(out);
        let offset = block
            .checked_mul(self.block_size as u64)
            .ok_or(Ext4Error::MetadataCorrupt)?;
        read_bytes(cache, provider, offset, &mut out[..self.block_size])?;
        Ok(())
    }

    fn extent_physical_block(
        self,
        cache: &mut BlockCache,
        provider: BlockProvider,
        inode: Ext4Inode,
        logical_block: u32,
    ) -> Result<Option<u64>, Ext4Error> {
        if inode.flags & EXT4_EXTENTS_FL == 0 {
            return Err(Ext4Error::UnsupportedRootfs);
        }

        let mut node = [0u8; EXT4_MAX_BLOCK_SIZE];
        node[..inode.extent_root.len()].copy_from_slice(&inode.extent_root);
        let mut depth = extent_depth(&node)?;
        loop {
            if depth == 0 {
                return extent_leaf_lookup(&node, logical_block);
            }

            let child = extent_index_lookup(&node, logical_block)?;
            self.read_block(cache, provider, child, &mut node)?;
            let child_depth = extent_depth(&node)?;
            if child_depth + 1 != depth {
                return Err(Ext4Error::MetadataCorrupt);
            }
            depth = child_depth;
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GroupDescriptor {
    inode_table_block: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Ext4Inode {
    number: u32,
    mode: u16,
    size: u64,
    links: u16,
    blocks: u64,
    flags: u32,
    extent_root: [u8; 60],
}

impl Ext4Inode {
    fn parse(number: u32, bytes: &[u8]) -> Result<Self, Ext4Error> {
        if bytes.len() < 128 {
            return Err(Ext4Error::MetadataCorrupt);
        }

        let mode = read_u16(bytes, 0)?;
        let size_lo = read_u32(bytes, 4)? as u64;
        let size_hi = read_u32(bytes, 108)? as u64;
        let links = read_u16(bytes, 26)?;
        let blocks_lo = read_u32(bytes, 28)? as u64;
        let blocks_hi = if bytes.len() >= 120 {
            read_u16(bytes, 116)? as u64
        } else {
            0
        };
        let flags = read_u32(bytes, 32)?;
        let mut extent_root = [0u8; 60];
        extent_root.copy_from_slice(&bytes[40..100]);
        Ok(Self {
            number,
            mode,
            size: size_lo | (size_hi << 32),
            links,
            blocks: blocks_lo | (blocks_hi << 32),
            flags,
            extent_root,
        })
    }

    const fn is_dir(self) -> bool {
        self.mode & EXT4_S_IFMT == EXT4_S_IFDIR
    }

    const fn is_regular(self) -> bool {
        self.mode & EXT4_S_IFMT == EXT4_S_IFREG
    }

    const fn executable(self) -> bool {
        self.mode & EXT4_EXEC_BITS != 0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Ext4File {
    inode: u32,
    byte_len: u64,
    executable: bool,
    regular: bool,
    mode: u16,
    links: u16,
    blocks: u64,
}

impl Ext4File {
    pub const fn new(
        inode: u32,
        byte_len: u64,
        executable: bool,
        regular: bool,
        mode: u16,
        links: u16,
        blocks: u64,
    ) -> Self {
        Self {
            inode,
            byte_len,
            executable,
            regular,
            mode,
            links,
            blocks,
        }
    }

    fn from_inode(inode: Ext4Inode) -> Self {
        Self {
            inode: inode.number,
            byte_len: inode.size,
            executable: inode.executable(),
            regular: inode.is_regular(),
            mode: inode.mode,
            links: inode.links,
            blocks: inode.blocks,
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

    pub const fn is_regular(self) -> bool {
        self.regular
    }

    pub const fn is_directory(self) -> bool {
        self.mode & EXT4_S_IFMT == EXT4_S_IFDIR
    }

    pub const fn identity(self) -> super::vfs::FileIdentity {
        let kind = if self.is_directory() {
            super::vfs::VfsNodeKind::Directory
        } else {
            super::vfs::VfsNodeKind::RegularFile
        };
        super::vfs::FileIdentity::new(
            self.inode,
            self.byte_len,
            self.executable,
            self.mode,
            self.links,
            self.blocks,
            kind,
        )
    }
}

fn extent_depth(node: &[u8; EXT4_MAX_BLOCK_SIZE]) -> Result<u16, Ext4Error> {
    if read_u16(node, 0)? != EXT4_EXTENT_MAGIC {
        Err(Ext4Error::MetadataCorrupt)
    } else {
        Ok(read_u16(node, 6)?)
    }
}

fn extent_leaf_lookup(
    node: &[u8; EXT4_MAX_BLOCK_SIZE],
    logical_block: u32,
) -> Result<Option<u64>, Ext4Error> {
    let entries = read_u16(node, 2)? as usize;
    let mut index = 0usize;
    while index < entries {
        let offset = EXT4_EXTENT_HEADER_SIZE + index * EXT4_EXTENT_SIZE;
        if offset + EXT4_EXTENT_SIZE > node.len() {
            return Err(Ext4Error::MetadataCorrupt);
        }
        let first = read_u32(node, offset)?;
        let len_raw = read_u16(node, offset + 4)?;
        let start_hi = read_u16(node, offset + 6)? as u64;
        let start_lo = read_u32(node, offset + 8)? as u64;
        let initialized_len = (len_raw & 0x7fff) as u32;
        if initialized_len == 0 {
            return Err(Ext4Error::MetadataCorrupt);
        }
        if first <= logical_block && logical_block < first + initialized_len {
            if len_raw & 0x8000 != 0 {
                return Ok(None);
            }
            let start = start_lo | (start_hi << 32);
            return Ok(Some(start + (logical_block - first) as u64));
        }
        index += 1;
    }

    Ok(None)
}

fn extent_index_lookup(
    node: &[u8; EXT4_MAX_BLOCK_SIZE],
    logical_block: u32,
) -> Result<u64, Ext4Error> {
    let entries = read_u16(node, 2)? as usize;
    if entries == 0 {
        return Err(Ext4Error::MetadataCorrupt);
    }

    let mut selected = None;
    let mut index = 0usize;
    while index < entries {
        let offset = EXT4_EXTENT_HEADER_SIZE + index * EXT4_EXTENT_INDEX_SIZE;
        if offset + EXT4_EXTENT_INDEX_SIZE > node.len() {
            return Err(Ext4Error::MetadataCorrupt);
        }
        let first = read_u32(node, offset)?;
        if first <= logical_block {
            let leaf_lo = read_u32(node, offset + 4)? as u64;
            let leaf_hi = read_u16(node, offset + 8)? as u64;
            selected = Some(leaf_lo | (leaf_hi << 32));
        } else {
            break;
        }
        index += 1;
    }

    selected.ok_or(Ext4Error::MetadataCorrupt)
}

fn validate_component(component: &[u8]) -> Result<(), Ext4Error> {
    if component.is_empty() || component.len() > EXT4_NAME_MAX {
        return Err(Ext4Error::UnsupportedPath);
    }
    if component == b"." || component == b".." {
        return Err(Ext4Error::UnsupportedPath);
    }
    let mut index = 0usize;
    while index < component.len() {
        if component[index] == 0 {
            return Err(Ext4Error::UnsupportedPath);
        }
        index += 1;
    }
    Ok(())
}

fn read_bytes(
    cache: &mut BlockCache,
    provider: BlockProvider,
    offset: u64,
    out: &mut [u8],
) -> Result<usize, Ext4Error> {
    cache
        .read_bytes(provider, offset, out)
        .map_err(Ext4Error::Block)
}

fn read_u16(bytes: &[u8], offset: usize) -> Result<u16, Ext4Error> {
    let data = read_array::<2>(bytes, offset)?;
    Ok(u16::from_le_bytes(data))
}

fn read_u32(bytes: &[u8], offset: usize) -> Result<u32, Ext4Error> {
    let data = read_array::<4>(bytes, offset)?;
    Ok(u32::from_le_bytes(data))
}

fn read_array<const N: usize>(bytes: &[u8], offset: usize) -> Result<[u8; N], Ext4Error> {
    let end = offset.checked_add(N).ok_or(Ext4Error::MetadataCorrupt)?;
    if end > bytes.len() {
        return Err(Ext4Error::MetadataCorrupt);
    }

    let mut data = [0u8; N];
    data.copy_from_slice(&bytes[offset..end]);
    Ok(data)
}

fn skip_separators(path: &[u8], mut cursor: usize) -> usize {
    while cursor < path.len() && path[cursor] == b'/' {
        cursor += 1;
    }
    cursor
}

fn dir_entry_kind(file_type: u8) -> VfsDirEntryKind {
    match file_type {
        EXT4_FT_REG_FILE => VfsDirEntryKind::RegularFile,
        EXT4_FT_DIR => VfsDirEntryKind::Directory,
        EXT4_FT_CHRDEV => VfsDirEntryKind::CharacterDevice,
        EXT4_FT_BLKDEV => VfsDirEntryKind::BlockDevice,
        EXT4_FT_FIFO => VfsDirEntryKind::Fifo,
        EXT4_FT_SOCK => VfsDirEntryKind::Socket,
        EXT4_FT_SYMLINK => VfsDirEntryKind::Symlink,
        _ => VfsDirEntryKind::Unknown,
    }
}

fn next_logical_block_offset(offset: u64, block_size: usize) -> Result<u64, Ext4Error> {
    let block = offset / block_size as u64;
    block
        .checked_add(1)
        .and_then(|next| next.checked_mul(block_size as u64))
        .ok_or(Ext4Error::MetadataCorrupt)
}

fn zero_block(out: &mut [u8; EXT4_MAX_BLOCK_SIZE]) {
    let mut index = 0usize;
    while index < out.len() {
        out[index] = 0;
        index += 1;
    }
}

fn bytes_eq(lhs: &[u8], rhs: &[u8]) -> bool {
    if lhs.len() != rhs.len() {
        return false;
    }
    let mut index = 0usize;
    while index < lhs.len() {
        if lhs[index] != rhs[index] {
            return false;
        }
        index += 1;
    }
    true
}

const fn min_usize(lhs: usize, rhs: usize) -> usize {
    if lhs < rhs {
        lhs
    } else {
        rhs
    }
}

const fn min_u64(lhs: u64, rhs: u64) -> u64 {
    if lhs < rhs {
        lhs
    } else {
        rhs
    }
}

const fn div_ceil_u64(value: u64, divisor: u64) -> u64 {
    if value == 0 {
        0
    } else {
        1 + (value - 1) / divisor
    }
}
