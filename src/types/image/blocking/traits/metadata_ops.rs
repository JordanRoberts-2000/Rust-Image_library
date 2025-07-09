use {
    mockall::automock,
    std::{
        io::{BufRead, Seek},
        path::Path,
    },
};

use crate::{ImageError, ImageFormat};

#[automock]
pub trait MetadataOps {
    fn from_path(&self, path: &Path) -> Result<(ImageFormat, u32, u32), ImageError>;
    fn from_bytes(&self, bytes: &Vec<u8>) -> Result<(ImageFormat, u32, u32), ImageError>;
    fn from_reader<R>(&self, reader: &mut R) -> Result<(ImageFormat, u32, u32), ImageError>
    where
        R: BufRead + Seek + 'static;
}
