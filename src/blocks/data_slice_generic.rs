
use types::errors::*;
use types::SizeType;
use core::data_source::*;
use core::data_slice::*;

pub struct DataSliceGeneric {
    source: DataSourceLink,
    offset: SizeType,
    size: SizeType,
}

impl DataSliceGeneric {
    pub fn new(source: DataSourceLink, size: SizeType) -> Self {
        let offset = source.as_ref().get_position();
        DataSliceGeneric { source, offset, size }
    }
}

impl DataSlice for DataSliceGeneric {
    fn get_offset(&self) -> SizeType {
        self.offset
    }

    fn get_size(&self) -> SizeType {
        self.size
    }

    fn get_data(&mut self, pos: SizeType, buf: &mut [u8]) -> Result<()> {
        if pos + (buf.len() as SizeType) >= self.size {
            bail!("Slice is out of bounds");
        }
        self.source.as_mut().get_data(self.offset + pos, buf)
    }
}
