
use std::rc::Rc;
use std::cell::RefCell;
use std::mem;

use core::errors::*;
use core::block_address::BlockAddress;
use core::array_size::ArraySize;
use core::block_data::*;
use core::block::*;
use core::SizeType;
use core::data_slice::*;
use core::item_link::*;
use core::item_size::*;

use types::block_attributes::*;

/// Simple block structure
pub struct BlockSimple<T: Sized> {
    pub data: BlockData,
    pub array_size: ArraySize,
    pub value: T,
}

impl<T: Sized> Block for BlockSimple<T> {
    fn get_offset(&self) -> SizeType {
        self.data.get_offset()
    }

    fn get_size(&self) -> SizeType {
        self.data.get_size()
    }

    fn get_parent(&self) -> Option<&BlockLink> {
        self.data.get_parent()
    }

    fn is_array(&self) -> bool {
        self.array_size.is_array()
    }

    fn get_item_size(&self) -> ItemSize {
        ItemSize::Static(mem::size_of::<T>())
    }

    fn get_array_size(&self) -> ArraySize {
        self.array_size
    }

    fn len(&self) -> ArraySize {
        self.array_size
    }

    fn get(&self, _index: usize) -> Result<&BlockLink> {
        bail!("Simple type doesn't have children.");
        // TODO: generate block on the fly.
    }
}

impl<T: Default + Sized + 'static> BlockSimple<T> {
    pub fn new(
        parent: Option<BlockLink>,
        slice: DataSliceLink,
        attrs: BlockAttributes,
    ) -> BlockLink {
        let block = BlockSimple::<T> {
            data: BlockData::new(
                slice,
                BlockAddress::Automatic(1),
                mem::size_of::<T>(),
                attrs,
                parent
            ),
            array_size: ArraySize::NotArray,
            value: Default::default(), // TODO: read value from the slice
        };
        // ItemLink::new(block),
        ItemLink(Rc::new(RefCell::new(block)))
    }

    pub fn arr(
        parent: Option<BlockLink>,
        size: SizeType,
        slice: DataSliceLink,
        attrs: BlockAttributes,
    ) -> BlockLink {
        let block = BlockSimple::<T> {
            data: BlockData::new(
                slice,
                BlockAddress::Automatic(1),
                mem::size_of::<T>() * size,
                attrs,
                parent
            ),
            array_size: ArraySize::Size(size),
            value: Default::default(),
        };
        // ItemLink::new(block),
        ItemLink(Rc::new(RefCell::new(block)))
    }
}
