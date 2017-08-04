
use types::errors::*;
use types::array_size::*;
use types::item_link::*;
use types::item_size::*;
use types::SizeType;

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
    fn get_parent(&self) -> Option<&BlockLink>;
    /// Get children count.
    fn len(&self) -> ArraySize;
    /// Get child by index.
    fn get(&self, index: usize) -> Result<&BlockLink>;
}
