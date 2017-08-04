
use core::SizeType;

/// Block array size
#[derive(Debug, Copy, Clone)]
pub enum ArraySize {
    /// Represents real array size
    Size(SizeType),
    /// Not an array
    NotArray,
}

impl ArraySize {
    /// Get array size
    pub fn get(&self) -> Option<SizeType> {
        match *self {
            ArraySize::Size(size) => Some(size),
            ArraySize::NotArray => None,
        }
    }

    /// Check if it is array
    pub fn is_array(&self) -> bool {
        match *self {
            ArraySize::Size(_) => true,
            ArraySize::NotArray => false,
        }
    }
}
