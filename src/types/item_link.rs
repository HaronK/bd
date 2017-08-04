
use std::rc::Rc;
use std::cell::{RefCell, Ref};

/// Refcounted link to the type.
#[derive(Debug)]
pub struct ItemLink<T: ?Sized>(pub Rc<RefCell<T>>);

impl<T: ?Sized> ItemLink<T> {
    /// Get link value.
    pub fn as_ref(&self) -> Ref<T> {
        (*self.0).borrow()
    }

    /// Get link value.
    pub fn as_mut(&mut self) -> &mut T {
        Rc::get_mut(&mut self.0).unwrap().get_mut()
    }
}
