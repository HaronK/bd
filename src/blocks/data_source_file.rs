
use std::io::{Seek, SeekFrom, Read};
use std::fs::File;
use std::path::Path;

use types::errors::*;
use types::SizeType;
use core::data_source::*;

pub struct DataSourceFile {
    file: File,
    pos: SizeType,
    size: SizeType,
}

impl DataSourceFile {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let f = File::open(path).chain_err(
            || "Unable to open contacts file",
        )?;
        let size = f.metadata().chain_err(|| "Cannot get metadata")?.len() as usize;

        Ok(DataSourceFile {
            file: f,
            pos: 0,
            size: size,
        })
    }
}

impl DataSource for DataSourceFile {
    fn get_size(&self) -> SizeType {
        self.size
    }

    fn get_position(&self) -> SizeType {
        self.pos
    }

    fn shift_position(&mut self, offset: SizeType) -> Result<()> {
        if self.pos + offset > self.size {
            bail!("Cannot shift out of the file bounds");
        }
        self.pos += offset;
        Ok(())
    }

    fn get_data(&mut self, pos: SizeType, buf: &mut [u8]) -> Result<()> {
        self.file.seek(SeekFrom::Current(pos as i64)).chain_err(
            || "Cannot shift to the specified file position",
        )?;
        self.file.read_exact(buf).chain_err(
            || "Cannot read data from the file",
        )
    }
}
