//! Linux stat ABI encoding for VFS metadata.

use super::vfs::VfsStat;

pub const LINUX_STAT_SIZE: usize = 128;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StatEncodingError {
    ValueOutOfRange,
}

pub fn encode_linux_stat(
    stat: VfsStat,
    out: &mut [u8; LINUX_STAT_SIZE],
) -> Result<(), StatEncodingError> {
    if stat.byte_len() > i64::MAX as u64 || stat.blocks() > i64::MAX as u64 {
        return Err(StatEncodingError::ValueOutOfRange);
    }

    out.fill(0);
    put_u64(out, 0, 0);
    put_u64(out, 8, stat.inode() as u64);
    put_u32(out, 16, stat.mode() as u32);
    put_u32(out, 20, stat.links() as u32);
    put_u32(out, 24, 0);
    put_u32(out, 28, 0);
    put_u64(out, 32, 0);
    put_u64(out, 40, 0);
    put_i64(out, 48, stat.byte_len() as i64);
    put_u32(out, 56, stat.block_size());
    put_u32(out, 60, 0);
    put_i64(out, 64, stat.blocks() as i64);
    Ok(())
}

fn put_u64(out: &mut [u8; LINUX_STAT_SIZE], offset: usize, value: u64) {
    out[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

fn put_i64(out: &mut [u8; LINUX_STAT_SIZE], offset: usize, value: i64) {
    out[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

fn put_u32(out: &mut [u8; LINUX_STAT_SIZE], offset: usize, value: u32) {
    out[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}
