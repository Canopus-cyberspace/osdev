//! Architecture-neutral block I/O and read-only block cache.

pub const BLOCK_SECTOR_SIZE: usize = 512;
pub const BLOCK_CACHE_SLOTS: usize = 8;

pub type BlockSector = [u8; BLOCK_SECTOR_SIZE];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BlockReadRequest {
    sector: u64,
}

impl BlockReadRequest {
    pub const fn new(sector: u64) -> Self {
        Self { sector }
    }

    pub const fn sector(self) -> u64 {
        self.sector
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BlockReadCompletion {
    request: BlockReadRequest,
    status: BlockReadStatus,
}

impl BlockReadCompletion {
    pub const fn completed(request: BlockReadRequest) -> Self {
        Self {
            request,
            status: BlockReadStatus::Completed,
        }
    }

    pub const fn failed(request: BlockReadRequest, error: BlockIoError) -> Self {
        Self {
            request,
            status: BlockReadStatus::Failed(error),
        }
    }

    pub const fn request(self) -> BlockReadRequest {
        self.request
    }

    pub const fn status(self) -> BlockReadStatus {
        self.status
    }

    pub const fn error(self) -> Option<BlockIoError> {
        match self.status {
            BlockReadStatus::Completed => None,
            BlockReadStatus::Failed(error) => Some(error),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlockReadStatus {
    Completed,
    Failed(BlockIoError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlockIoError {
    CompletionTimeout,
    DeviceKindMismatch,
    DeviceStatusNonZero,
    FeatureNegotiationFailed,
    ProviderMissing,
    QueueUnavailable,
    RequestOverflow,
    SectorOutOfRange,
    UnsupportedSectorSize,
    UnsupportedTransport,
}

#[derive(Clone, Copy)]
pub struct BlockProvider {
    read_sector: fn(BlockReadRequest, &mut BlockSector) -> BlockReadCompletion,
}

impl BlockProvider {
    pub const fn new(
        read_sector: fn(BlockReadRequest, &mut BlockSector) -> BlockReadCompletion,
    ) -> Self {
        Self { read_sector }
    }

    pub fn read_sector(
        self,
        request: BlockReadRequest,
        out: &mut BlockSector,
    ) -> BlockReadCompletion {
        (self.read_sector)(request, out)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlockCacheRead {
    Hit,
    Miss,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BlockCacheStats {
    hits: usize,
    misses: usize,
}

impl BlockCacheStats {
    pub const fn hits(self) -> usize {
        self.hits
    }

    pub const fn misses(self) -> usize {
        self.misses
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BlockCacheEntry {
    valid: bool,
    sector: u64,
    data: BlockSector,
}

impl BlockCacheEntry {
    const fn empty() -> Self {
        Self {
            valid: false,
            sector: 0,
            data: [0; BLOCK_SECTOR_SIZE],
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BlockCache {
    entries: [BlockCacheEntry; BLOCK_CACHE_SLOTS],
    next: usize,
    hits: usize,
    misses: usize,
}

impl BlockCache {
    pub const fn new() -> Self {
        Self {
            entries: [BlockCacheEntry::empty(); BLOCK_CACHE_SLOTS],
            next: 0,
            hits: 0,
            misses: 0,
        }
    }

    pub const fn stats(&self) -> BlockCacheStats {
        BlockCacheStats {
            hits: self.hits,
            misses: self.misses,
        }
    }

    pub fn read_sector(
        &mut self,
        provider: BlockProvider,
        sector: u64,
        out: &mut BlockSector,
    ) -> Result<BlockCacheRead, BlockIoError> {
        if let Some(slot) = self.lookup(sector) {
            copy_sector(&self.entries[slot].data, out);
            self.hits += 1;
            return Ok(BlockCacheRead::Hit);
        }

        let mut data = [0u8; BLOCK_SECTOR_SIZE];
        let request = BlockReadRequest::new(sector);
        let completion = provider.read_sector(request, &mut data);
        if completion.request() != request {
            return Err(BlockIoError::RequestOverflow);
        }
        if let Some(error) = completion.error() {
            return Err(error);
        }

        let slot = self.next % BLOCK_CACHE_SLOTS;
        self.next = (self.next + 1) % BLOCK_CACHE_SLOTS;
        self.entries[slot] = BlockCacheEntry {
            valid: true,
            sector,
            data,
        };
        copy_sector(&data, out);
        self.misses += 1;
        Ok(BlockCacheRead::Miss)
    }

    pub fn read_bytes(
        &mut self,
        provider: BlockProvider,
        offset: u64,
        out: &mut [u8],
    ) -> Result<usize, BlockIoError> {
        let mut copied = 0usize;
        while copied < out.len() {
            let absolute = offset
                .checked_add(copied as u64)
                .ok_or(BlockIoError::RequestOverflow)?;
            let sector = absolute / BLOCK_SECTOR_SIZE as u64;
            let in_sector = (absolute % BLOCK_SECTOR_SIZE as u64) as usize;
            let mut data = [0u8; BLOCK_SECTOR_SIZE];
            self.read_sector(provider, sector, &mut data)?;

            let remaining = out.len() - copied;
            let available = BLOCK_SECTOR_SIZE - in_sector;
            let take = min_usize(remaining, available);
            out[copied..copied + take].copy_from_slice(&data[in_sector..in_sector + take]);
            copied += take;
        }

        Ok(copied)
    }

    fn lookup(&self, sector: u64) -> Option<usize> {
        let mut index = 0usize;
        while index < BLOCK_CACHE_SLOTS {
            if self.entries[index].valid && self.entries[index].sector == sector {
                return Some(index);
            }
            index += 1;
        }
        None
    }
}

fn copy_sector(src: &BlockSector, dst: &mut BlockSector) {
    dst.copy_from_slice(src);
}

const fn min_usize(lhs: usize, rhs: usize) -> usize {
    if lhs < rhs {
        lhs
    } else {
        rhs
    }
}
