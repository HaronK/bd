
use types::SizeType;

/// Represents template size. Can be Automatic, Manual and Dynamic.
#[derive(Debug, Copy, Clone)]
pub enum TemplateSize {
    /// Size is fixed and automatically calculated as sum of fixed sizes of subelements.
    Automatic(SizeType),
    /// Size is fixed and specified manually.
    Manual(SizeType),
    /// Size will be calculated at parsing step as sum of sizes of subelements.
    /// Can be different for different blocks genetrated by the same template.
    Dynamic,
}

impl TemplateSize {
    /// Get template size
    pub fn get(&self) -> Option<SizeType> {
        match *self {
            TemplateSize::Automatic(size) => Some(size),
            TemplateSize::Manual(size) => Some(size),
            TemplateSize::Dynamic => None,
        }
    }

    /// Check if template has fixed size
    pub fn is_fixed(&self) -> bool {
        match *self {
            TemplateSize::Dynamic => false,
            _ => true,
        }
    }

    /// Check if block has automatic size
    pub fn is_automatic(&self) -> bool {
        match *self {
            TemplateSize::Automatic(_) => true,
            _ => false,
        }
    }

    /// Check if block has manual size
    pub fn is_manual(&self) -> bool {
        match *self {
            TemplateSize::Manual(_) => true,
            _ => false,
        }
    }

    /// Check if block has dynamic size
    pub fn is_dynamic(&self) -> bool {
        match *self {
            TemplateSize::Dynamic => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_automatic() {
        let value = TemplateSize::Automatic(10);
        assert!(value.is_fixed());
        assert!(value.is_automatic());
        assert!(!value.is_manual());
        assert!(!value.is_dynamic());
        assert_eq!(10, value.get().unwrap());
    }

    #[test]
    fn is_manual() {
        let value = TemplateSize::Manual(20);
        assert!(value.is_fixed());
        assert!(value.is_manual());
        assert!(!value.is_automatic());
        assert!(!value.is_dynamic());
        assert_eq!(20, value.get().unwrap());
    }

    #[test]
    fn is_dynamic() {
        let value = TemplateSize::Dynamic;
        assert!(value.is_dynamic());
        assert!(!value.is_fixed());
        assert!(!value.is_automatic());
        assert!(!value.is_manual());
        assert!(if let Some(_) = value.get() {false} else {true});
    }
}
