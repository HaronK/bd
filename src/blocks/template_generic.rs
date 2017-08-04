
use types::item_size::*;
use types::template_size::*;
use types::partial_result::*;
use types::block_attributes::*;
use types::template_attributes::*;
use core::block::*;
use core::template::*;
use core::data_slice::*;
use blocks::block_generic::*;

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
        let item_size = match self.size {
            TemplateSize::Automatic(size) => ItemSize::Dynamic(size),
            TemplateSize::Manual(size) => ItemSize::Static(size),
            TemplateSize::Dynamic => ItemSize::Unknown,
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
