use crate::core::block::{BlockProvider, BlockReadCompletion, BlockReadRequest, BlockSector};

#[derive(Clone, Copy)]
pub struct BlockServices {
    provider: BlockProvider,
}

impl BlockServices {
    pub const fn new(
        read_sector: fn(BlockReadRequest, &mut BlockSector) -> BlockReadCompletion,
    ) -> Self {
        Self {
            provider: BlockProvider::new(read_sector),
        }
    }

    pub const fn provider(self) -> BlockProvider {
        self.provider
    }
}
