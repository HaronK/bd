
use std::rc::Rc;
use std::cell::RefCell;

use types::errors::*;
use types::array_size::*;
use types::block_address::BlockAddress;
use types::SizeType;
use types::item_link::*;
use types::item_size::*;
use types::block_attributes::*;
use core::block::*;
use core::data_slice::*;
use blocks::block_data::*;

/// Array block structure
pub struct BlockArray {
    data: BlockData,
    item_size: ItemSize,
    children: Vec<Option<BlockLink>>,
}

impl Block for BlockArray {
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
        true
    }

    fn get_item_size(&self) -> ItemSize {
        self.item_size
    }

    fn get_array_size(&self) -> ArraySize {
        ArraySize::Size(self.children.len())
    }

    fn len(&self) -> ArraySize {
        self.get_array_size()
    }

    fn get(&self, index: usize) -> Result<&BlockLink> {
        match self.children.get(index).unwrap() {
            &Some(ref block) => Ok(block),
            &None => {
                bail!("Lazy loading is not implemented.");
            }
        }
    }
}

impl BlockArray {
    pub fn new(
        parent: Option<BlockLink>,
        array_size: SizeType,
        item_size: ItemSize,
        slice: DataSliceLink,
        attrs: BlockAttributes,
    ) -> BlockLink {
        let block = BlockArray {
            data: BlockData::new(
                slice,
                BlockAddress::Automatic(1),
                1,
                attrs,
                parent
            ),
            item_size: item_size,
            children: Self::make_empty_vec(array_size),
        };
        ItemLink(Rc::new(RefCell::new(block)))
    }

    // TODO: manage sub array size properly
    pub fn arr(
        parent: Option<BlockLink>,
        array_size: SizeType,
        item_size: ItemSize,
        slice: DataSliceLink,
        attrs: BlockAttributes,
    ) -> BlockLink {
        let block = BlockArray {
            data: BlockData::new(
                slice,
                BlockAddress::Automatic(1),
                1,
                attrs,
                parent
            ),
            item_size: item_size,
            children: Self::make_empty_vec(array_size),
        };
        ItemLink(Rc::new(RefCell::new(block)))
    }

    fn make_empty_vec(capacity: usize) -> Vec<Option<BlockLink>> {
        let mut res = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            res.push(Option::None);
        }
        res
    }
}
