
use core::block::*;
use core::item_size::*;
use core::template::*;
use core::template_size::*;
use core::data_slice::*;
use core::partial_result::*;

use types::block_attributes::*;
use types::template_attributes::*;
use types::block_generic::*;

/// Block template internal data
pub struct TemplateGeneric {
    name: String,
    size: TemplateSize,
    attrs: TemplateAttributes, // children: Vec<TemplateLink>,
}

impl Template for TemplateGeneric {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_size(&self) -> TemplateSize {
        self.size
    }

    fn get_attributes(&self) -> &TemplateAttributes {
        &self.attrs
    }

    fn apply(
        &self,
        parent: Option<BlockLink>,
        slice: DataSliceLink,
        attrs: BlockAttributes,
    ) -> PartialResult<BlockLink> {
        let item_size = if self.size.is_dynamic() {
            ItemSize::Dynamic
        } else {
            ItemSize::Static(self.size.get())
        };
        Ok(BlockGeneric::new(parent, item_size, slice, attrs))
    }
}

impl TemplateGeneric {
    pub fn new(name: String, attrs: TemplateAttributes) -> Self {
        let size = if let Some(s) = attrs.size {
            TemplateSize::Manual(s)
        } else {
            TemplateSize::Dynamic
        };

        TemplateGeneric {
            name: name,
            size: size,
            attrs: attrs, // children: vec![],
        }
    }
}
