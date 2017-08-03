
use core::SizeType;
use core::errors::*;
use core::item_link::*;

/// Type alias for data source link.
pub type DataSourceLink = ItemLink<DataSource>;

/// Source data accessor.
pub trait DataSource {
    /// Get size of the slice in bytes.
    fn get_size(&self) -> SizeType;
    /// Get current read position in the slice.
    fn get_position(&self) -> SizeType;
    /// Shift current read position in the slice on *offset* bytes.
    fn shift_position(&mut self, offset: SizeType) -> Result<()>;
    /// Read the data starting from the *pos* position.
    fn get_data(&mut self, pos: SizeType, buf: &mut [u8]) -> Result<()>;
}
