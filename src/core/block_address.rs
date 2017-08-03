
use core::SizeType;

/// Block address
#[derive(Debug, Copy, Clone)]
pub enum BlockAddress {
    /// Address is taken automatically from data block
    Automatic(SizeType),
    /// Address is set manually
    Manual(SizeType),
}

impl BlockAddress {
    /// Get address value
    pub fn get(&self) -> SizeType {
        match *self {
            BlockAddress::Automatic(addr) => addr,
            BlockAddress::Manual(addr) => addr,
        }
    }

    /// Check if it is automatic address
    pub fn is_automatic(&self) -> bool {
        match *self {
            BlockAddress::Automatic(_) => true,
            BlockAddress::Manual(_) => false,
        }
    }

    /// Check if it is manual address
    pub fn is_manual(&self) -> bool {
        match *self {
            BlockAddress::Automatic(_) => false,
            BlockAddress::Manual(_) => true,
        }
    }
}
