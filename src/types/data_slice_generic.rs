
use core::errors::*;
use core::data_source::*;
use core::data_slice::*;
use core::SizeType;

pub struct DataSliceGeneric {
    source: DataSourceLink,
    addr: SizeType,
    size: SizeType,
}

impl DataSliceGeneric {
    pub fn new(source: DataSourceLink, size: SizeType) -> Self {
        let addr = source.as_ref().get_position();

        DataSliceGeneric {
            source: source,
            addr: addr,
            size: size,
        }
    }
}

impl DataSlice for DataSliceGeneric {
    fn get_size(&self) -> SizeType {
        self.size
    }

    fn get_data(&mut self, pos: SizeType, buf: &mut [u8]) -> Result<()> {
        if pos + (buf.len() as SizeType) >= self.size {
            bail!("Slice is out of bounds");
        }
        self.source.as_mut().get_data(self.addr + pos, buf)
    }
}
