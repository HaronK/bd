
use core::block_address::BlockAddress;
use core::data_slice::*;
use core::block::*;
use core::SizeType;
use types::block_attributes::*;

/// Block common data structure
pub struct BlockData {
    slice: DataSliceLink,
    offset: BlockAddress,
    size: SizeType,
    attrs: BlockAttributes,
    parent: Option<BlockLink>,
}

impl BlockData {
    pub fn new(slice: DataSliceLink,
        offset: BlockAddress,
        size: SizeType,
        attrs: BlockAttributes,
        parent: Option<BlockLink>) -> Self {
        BlockData { slice, offset, size, attrs, parent }
    }

    /// Get offset of the block data in bytes.
    pub fn get_offset(&self) -> SizeType {
        self.offset.get()
    }

    /// Get size of the slice in bytes.
    pub fn get_size(&self) -> SizeType {
        self.size
    }

    /// Get parent.
    pub fn get_parent(&self) -> Option<BlockLink> {
        match self.parent {
            Some(parent) => Some(parent),
            None => None
        }
    }
}
