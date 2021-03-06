
use types::block_address::BlockAddress;
use types::SizeType;
use types::block_attributes::*;
use core::data_slice::*;
use core::block::*;

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
    pub fn get_parent(&self) -> Option<&BlockLink> {
        match self.parent {
            Some(ref parent) => Some(parent),
            None => None
        }
    }
}
