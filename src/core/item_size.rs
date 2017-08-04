
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
    pub fn get(&self) -> Option<SizeType> {
        match *self {
            ItemSize::Static(size) => Some(size),
            ItemSize::Dynamic(size) => Some(size),
            ItemSize::Unknown => None,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_static() {
        let value = ItemSize::Static(10);
        assert!(value.is_static());
        assert!(!value.is_dynamic());
        assert!(!value.is_unknown());
        assert_eq!(10, value.get().unwrap());
    }

    #[test]
    fn is_dynamic() {
        let value = ItemSize::Dynamic(20);
        assert!(value.is_dynamic());
        assert!(!value.is_static());
        assert!(!value.is_unknown());
        assert_eq!(20, value.get().unwrap());
    }

    #[test]
    fn is_unknown() {
        let value = ItemSize::Unknown;
        assert!(value.is_unknown());
        assert!(!value.is_static());
        assert!(!value.is_dynamic());
        assert!(if let Some(_) = value.get() {false} else {true});
    }
}
