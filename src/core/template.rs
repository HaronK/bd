
use types::template_size::TemplateSize;
use types::partial_result::*;
use types::item_link::*;
use types::block_attributes::*;
use types::template_attributes::*;
use core::data_slice::*;
use core::block::*;

/// Type alias for block template link.
pub type TemplateLink = ItemLink<Template>;

pub type TemplateBody = FnMut(TemplateLink) -> PartialResult<BlockLink>;

/// Block template. Produces a block when applied to the data slice.
pub trait Template {
    /// Get block template name
    fn get_name(&self) -> &String;
    /// Get block template size
    fn get_size(&self) -> TemplateSize;
    /// Get template attributes
    fn get_attributes(&self) -> &TemplateAttributes;
    /// Parse block of the data from the reader.
    /// Can return partially parsed block tree and error.
    fn apply(
        &self,
        parent: Option<BlockLink>,
        slice: DataSliceLink,
        attrs: BlockAttributes,
    ) -> PartialResult<BlockLink>;
}
