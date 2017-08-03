
use core::errors::*;
use core::SizeType;

/// Item size
#[derive(Debug, Copy, Clone)]
pub enum ItemSize {
    /// Size is static.
    Static(SizeType),
    /// Size is not static and was calculated at parsing step.
    Dynamic(SizeType),
    /// Size will be calculated at parsing step as sum of sizes of subelements and set to Dynamic(size).
    Unknown,
}

impl ItemSize {
    /// Get item size
    pub fn get(&self) -> Result<SizeType> {
        match *self {
            ItemSize::Static(size) => Ok(size),
            ItemSize::Dynamic(size) => Ok(size),
            ItemSize::Unknown => bail!("Size of the item is not calculated yet"),
        }
    }

    /// Check if item has static size
    pub fn is_static(&self) -> bool {
        match *self {
            ItemSize::Static(_) => true,
            _ => false,
        }
    }

    /// Check if item has dynamic size
    pub fn is_dynamic(&self) -> bool {
        match *self {
            ItemSize::Dynamic(_) => true,
            _ => false,
        }
    }

    /// Check if item size is not calculated yet
    pub fn is_unknown(&self) -> bool {
        match *self {
            ItemSize::Unknown => true,
            _ => false,
        }
    }
}
