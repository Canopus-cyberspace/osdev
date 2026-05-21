#![allow(dead_code)]

use core::cmp::min;

use crate::virtio_blk_pci;

const SECTOR_SIZE: usize = virtio_blk_pci::SECTOR_SIZE;
const MAX_BLOCK_SIZE: usize = 4096;
const MAX_INODE_SIZE: usize = 256;
const EXT4_SUPER_MAGIC: u16 = 0xef53;
const ROOT_INODE: u32 = 2;

pub struct LoadedFileInfo {
    pub inode: u32,
    pub mode: u16,
    pub size: usize,
}

#[derive(Copy, Clone)]
pub struct StatInfo {
    pub inode: u32,
    pub mode: u16,
    pub size: usize,
}

struct Ext4 {
    block_size: usize,
    inode_size: usize,
    inodes_per_group: u32,
    desc_size: usize,
    gdt_block: u64,
}

struct Inode {
    data: [u8; MAX_INODE_SIZE],
}

pub fn load_basic_write(out: &mut [u8]) -> Result<LoadedFileInfo, &'static str> {
    load_path("/musl/basic/write", out)
}

pub fn load_path(path: &str, out: &mut [u8]) -> Result<LoadedFileInfo, &'static str> {
    virtio_blk_pci::init()?;
    let fs = Ext4::open()?;
    let ino = fs.lookup_path_str(path)?;
    let inode = fs.read_inode(ino)?;
    let size = inode.size() as usize;
    if size > out.len() {
        return Err("ext4_file_too_large");
    }
    zero_bytes(out);
    fs.read_file_into(&inode, &mut out[..size])?;
    Ok(LoadedFileInfo {
        inode: ino,
        mode: inode.mode(),
        size,
    })
}

pub fn stat_path(path: &str) -> Result<StatInfo, &'static str> {
    virtio_blk_pci::init()?;
    let fs = Ext4::open()?;
    let ino = fs.lookup_path_str(path)?;
    let inode = fs.read_inode(ino)?;
    Ok(StatInfo {
        inode: ino,
        mode: inode.mode(),
        size: inode.size() as usize,
    })
}

impl Ext4 {
    fn open() -> Result<Self, &'static str> {
        let mut superblock = [0u8; 1024];
        let mut sector = [0u8; SECTOR_SIZE];
        virtio_blk_pci::read_sector(2, &mut sector)?;
        copy_bytes(&mut superblock[..SECTOR_SIZE], &sector);
        virtio_blk_pci::read_sector(3, &mut sector)?;
        copy_bytes(&mut superblock[SECTOR_SIZE..], &sector);

        if read_u16(&superblock, 56) != EXT4_SUPER_MAGIC {
            return Err("ext4_magic");
        }
        let log_block_size = read_u32(&superblock, 24) as usize;
        if log_block_size > 2 {
            return Err("ext4_block_size");
        }
        let block_size = 1024usize << log_block_size;
        if block_size > MAX_BLOCK_SIZE || block_size % SECTOR_SIZE != 0 {
            return Err("ext4_block_size");
        }
        let inode_size = read_u16(&superblock, 88) as usize;
        if inode_size == 0 || inode_size > MAX_INODE_SIZE {
            return Err("ext4_inode_size");
        }
        let inodes_per_group = read_u32(&superblock, 40);
        if inodes_per_group == 0 {
            return Err("ext4_inodes_per_group");
        }
        let raw_desc_size = read_u16(&superblock, 254) as usize;
        let desc_size = if raw_desc_size == 0 { 32 } else { raw_desc_size };
        if desc_size < 32 || desc_size > 64 {
            return Err("ext4_desc_size");
        }
        let gdt_block = if block_size == 1024 { 2 } else { 1 };
        Ok(Self {
            block_size,
            inode_size,
            inodes_per_group,
            desc_size,
            gdt_block,
        })
    }

    fn lookup_path(&self, parts: &[&[u8]]) -> Result<u32, &'static str> {
        let mut ino = ROOT_INODE;
        let mut i = 0usize;
        while i < parts.len() {
            let inode = self.read_inode(ino)?;
            ino = self.lookup_child(&inode, parts[i])?;
            i += 1;
        }
        Ok(ino)
    }

    fn lookup_path_str(&self, path: &str) -> Result<u32, &'static str> {
        let bytes = path.as_bytes();
        let mut ino = ROOT_INODE;
        let mut pos = 0usize;

        while pos < bytes.len() && bytes[pos] == b'/' {
            pos += 1;
        }
        if pos == bytes.len() {
            return Ok(ROOT_INODE);
        }

        while pos < bytes.len() {
            while pos < bytes.len() && bytes[pos] == b'/' {
                pos += 1;
            }
            if pos >= bytes.len() {
                break;
            }
            let start = pos;
            while pos < bytes.len() && bytes[pos] != b'/' {
                pos += 1;
            }
            let name = &bytes[start..pos];
            if bytes_eq(name, b".") {
                continue;
            }
            if bytes_eq(name, b"..") {
                ino = ROOT_INODE;
                continue;
            }
            let inode = self.read_inode(ino)?;
            ino = self.lookup_child(&inode, name)?;
        }

        Ok(ino)
    }

    fn read_inode(&self, ino: u32) -> Result<Inode, &'static str> {
        if ino == 0 {
            return Err("ext4_inode_zero");
        }
        let group = (ino - 1) / self.inodes_per_group;
        let index = ((ino - 1) % self.inodes_per_group) as usize;
        let inode_table_block = self.inode_table_block(group)?;
        let inode_offset = index
            .checked_mul(self.inode_size)
            .ok_or("ext4_inode_offset")?;
        let mut out = Inode {
            data: [0; MAX_INODE_SIZE],
        };
        let mut copied = 0usize;
        while copied < self.inode_size {
            let abs = inode_offset + copied;
            let block = inode_table_block + (abs / self.block_size) as u64;
            let in_block = abs % self.block_size;
            let mut block_buf = [0u8; MAX_BLOCK_SIZE];
            self.read_block(block, &mut block_buf)?;
            let take = min(self.inode_size - copied, self.block_size - in_block);
            copy_bytes(
                &mut out.data[copied..copied + take],
                &block_buf[in_block..in_block + take],
            );
            copied += take;
        }
        Ok(out)
    }

    fn inode_table_block(&self, group: u32) -> Result<u64, &'static str> {
        let desc_offset = group as usize * self.desc_size;
        let desc_block = self.gdt_block + (desc_offset / self.block_size) as u64;
        let in_block = desc_offset % self.block_size;
        if in_block + self.desc_size > self.block_size {
            return Err("ext4_desc_cross_block");
        }
        let mut block = [0u8; MAX_BLOCK_SIZE];
        self.read_block(desc_block, &mut block)?;
        let lo = read_u32(&block, in_block + 8) as u64;
        let hi = if self.desc_size >= 64 {
            read_u32(&block, in_block + 40) as u64
        } else {
            0
        };
        let inode_table = lo | (hi << 32);
        if inode_table == 0 {
            return Err("ext4_inode_table");
        }
        Ok(inode_table)
    }

    fn lookup_child(&self, dir: &Inode, name: &[u8]) -> Result<u32, &'static str> {
        if name.is_empty() || name.len() > 255 {
            return Err("ext4_name");
        }
        let size = dir.size() as usize;
        let mut logical = 0u32;
        let mut consumed = 0usize;
        while consumed < size {
            let mut block = [0u8; MAX_BLOCK_SIZE];
            self.read_inode_block(dir, logical, &mut block)?;
            let scan_len = min(self.block_size, size - consumed);
            let mut off = 0usize;
            while off + 8 <= scan_len {
                let ino = read_u32(&block, off);
                let rec_len = read_u16(&block, off + 4) as usize;
                let name_len = block[off + 6] as usize;
                if rec_len < 8 || off + rec_len > scan_len {
                    return Err("ext4_dirent");
                }
                if ino != 0
                    && name_len == name.len()
                    && off + 8 + name_len <= scan_len
                    && bytes_eq(&block[off + 8..off + 8 + name_len], name)
                {
                    return Ok(ino);
                }
                off += rec_len;
            }
            consumed += scan_len;
            logical += 1;
        }
        Err("ext4_lookup")
    }

    fn read_file_into(&self, inode: &Inode, out: &mut [u8]) -> Result<(), &'static str> {
        let size = inode.size() as usize;
        if size > out.len() {
            return Err("ext4_file_buffer");
        }
        let mut logical = 0u32;
        let mut consumed = 0usize;
        while consumed < size {
            let mut block = [0u8; MAX_BLOCK_SIZE];
            self.read_file_block(inode, logical, &mut block)?;
            let take = min(self.block_size, size - consumed);
            copy_bytes(&mut out[consumed..consumed + take], &block[..take]);
            consumed += take;
            logical += 1;
        }
        Ok(())
    }

    fn read_file_block(
        &self,
        inode: &Inode,
        logical: u32,
        out: &mut [u8; MAX_BLOCK_SIZE],
    ) -> Result<(), &'static str> {
        match self.read_inode_block(inode, logical, out) {
            Ok(()) => Ok(()),
            Err("ext4_extent_missing") => {
                zero_bytes(out);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn read_inode_block(
        &self,
        inode: &Inode,
        logical: u32,
        out: &mut [u8; MAX_BLOCK_SIZE],
    ) -> Result<(), &'static str> {
        let block = inode.extent_physical_block(logical)?;
        self.read_block(block, out)
    }

    fn read_block(&self, block: u64, out: &mut [u8; MAX_BLOCK_SIZE]) -> Result<(), &'static str> {
        zero_bytes(out);
        let sectors = self.block_size / SECTOR_SIZE;
        let start_sector = block.checked_mul(sectors as u64).ok_or("ext4_sector")?;
        let mut sector = [0u8; SECTOR_SIZE];
        let mut i = 0usize;
        while i < sectors {
            virtio_blk_pci::read_sector(start_sector + i as u64, &mut sector)?;
            let start = i * SECTOR_SIZE;
            copy_bytes(&mut out[start..start + SECTOR_SIZE], &sector);
            i += 1;
        }
        Ok(())
    }
}

impl Inode {
    fn mode(&self) -> u16 {
        read_u16(&self.data, 0)
    }

    fn size(&self) -> u64 {
        let lo = read_u32(&self.data, 4) as u64;
        let hi = read_u32(&self.data, 108) as u64;
        lo | (hi << 32)
    }

    fn extent_physical_block(&self, logical: u32) -> Result<u64, &'static str> {
        let base = 40usize;
        if read_u16(&self.data, base) != 0xf30a {
            return Err("ext4_extent_magic");
        }
        let entries = read_u16(&self.data, base + 2) as usize;
        let depth = read_u16(&self.data, base + 6);
        if depth != 0 {
            return Err("ext4_extent_depth");
        }
        let mut i = 0usize;
        while i < entries {
            let off = base + 12 + i * 12;
            if off + 12 > base + 60 {
                return Err("ext4_extent_bounds");
            }
            let first = read_u32(&self.data, off);
            let len = (read_u16(&self.data, off + 4) & 0x7fff) as u32;
            let start_hi = read_u16(&self.data, off + 6) as u64;
            let start_lo = read_u32(&self.data, off + 8) as u64;
            if len != 0 && logical >= first && logical < first + len {
                return Ok(((start_hi << 32) | start_lo) + (logical - first) as u64);
            }
            i += 1;
        }
        Err("ext4_extent_missing")
    }
}

fn read_u16(data: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([data[off], data[off + 1]])
}

fn read_u32(data: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]])
}

fn bytes_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0usize;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

fn copy_bytes(dst: &mut [u8], src: &[u8]) {
    let mut i = 0usize;
    while i < dst.len() {
        dst[i] = src[i];
        i += 1;
    }
}

fn zero_bytes(dst: &mut [u8]) {
    let mut i = 0usize;
    while i < dst.len() {
        dst[i] = 0;
        i += 1;
    }
}
