
use core::errors::*;
use core::array_size::*;
use core::item_link::*;
use core::item_size::*;
use core::SizeType;
use core::data_slice::*;
use core::block_address::BlockAddress;

use types::block_attributes::*;

/// Block common data structure
pub struct BlockData {
    pub slice: DataSliceLink,
    pub offset: BlockAddress,
    pub size: SizeType,
    pub attrs: BlockAttributes,
    pub parent: Option<BlockLink>,
}

/// Type alias for block link.
pub type BlockLink = ItemLink<Block>;

/// Block structure.
pub trait Block {
    /// Get block offset.
    fn get_offset(&self) -> SizeType;
    /// Get whole block size (size = item_size * array_size).
    fn get_size(&self) -> SizeType;
    /// Check if block contains array of elements.
    fn is_array(&self) -> bool;
    /// Get item size.
    fn get_item_size(&self) -> ItemSize;
    /// Get number of items in the array.
    fn get_array_size(&self) -> ArraySize;
    /// Get parent.
    fn get_parent(&self) -> &Option<BlockLink>;
    /// Get children count.
    fn len(&self) -> ArraySize;
    /// Get child by index.
    fn get(&self, index: usize) -> Result<&BlockLink>;
}