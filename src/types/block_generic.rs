
use std::rc::Rc;
use std::cell::RefCell;

use core::errors::*;
use core::array_size::*;
use core::block_address::BlockAddress;
use core::template::*;
use core::block::*;
use core::SizeType;
use core::data_slice::*;
use core::item_link::*;
use core::item_size::*;

use types::block_array::*;
use types::block_attributes::*;

/// Generic block structure
pub struct BlockGeneric {
    data: BlockData,
    item_size: ItemSize,
    template: Option<TemplateLink>,
    children: Vec<BlockLink>,
}

impl Block for BlockGeneric {
    fn get_offset(&self) -> SizeType {
        self.data.offset.get()
    }

    fn get_size(&self) -> SizeType {
        self.data.size
    }

    fn get_parent(&self) -> &Option<BlockLink> {
        &self.data.parent
    }

    fn is_array(&self) -> bool {
        false
    }

    fn get_item_size(&self) -> ItemSize {
        self.item_size
    }

    fn get_array_size(&self) -> ArraySize {
        ArraySize::NotArray
    }

    fn len(&self) -> ArraySize {
        ArraySize::Size(self.children.len())
    }

    fn get(&self, index: usize) -> Result<&BlockLink> {
        match self.children.get(index) {
            Some(block) => Ok(block),
            None => bail!("Wrong item index."),
        }
    }
}

// TODO: restore commented lines when
// [RFC 0982](https://github.com/rust-lang/rfcs/blob/master/text/0982-dst-coercion.md)
// will be implemented.

impl BlockGeneric {
    pub fn new(
        parent: Option<BlockLink>,
        item_size: ItemSize,
        slice: DataSliceLink,
        attrs: BlockAttributes,
    ) -> BlockLink {
        let block = BlockGeneric {
            data: BlockData {
                slice: slice,
                offset: BlockAddress::Automatic(1),
                size: 1,
                attrs: attrs,
                parent: parent,
            },
            item_size: item_size,
            template: None,
            children: vec![],
        };
        // ItemLink::new(block),
        ItemLink(Rc::new(RefCell::new(block)))
    }

    pub fn arr(
        parent: Option<BlockLink>,
        size: SizeType,
        item_size: ItemSize,
        slice: DataSliceLink,
        attrs: BlockAttributes,
    ) -> BlockLink {
        BlockArray::new(parent, size, item_size, slice, attrs)
    }

    pub fn add_child(&mut self, child: BlockLink) {
        self.children.push(child);
    }
}
