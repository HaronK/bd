
use types::errors::*;
use types::item_link::*;
use types::SizeType;

/// Type alias for data slice link.
pub type DataSliceLink = ItemLink<DataSlice>;

/// Data accessor for the slice of the source data.
pub trait DataSlice {
    /// Get offset of the slice in bytes.
    fn get_offset(&self) -> SizeType;
    /// Get size of the slice in bytes.
    fn get_size(&self) -> SizeType;
    /// Read the data starting from the *pos* position in the slice.
    fn get_data(&mut self, pos: SizeType, buf: &mut [u8]) -> Result<()>;
}
