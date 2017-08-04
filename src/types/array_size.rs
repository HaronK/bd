
use types::SizeType;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_array() {
        let value = ArraySize::Size(10);
        assert!(value.is_array());
        assert_eq!(10, value.get().unwrap());
    }

    #[test]
    fn is_not_array() {
        let value = ArraySize::NotArray;
        assert!(!value.is_array());
        assert!(if let Some(_) = value.get() {false} else {true});
    }
}
