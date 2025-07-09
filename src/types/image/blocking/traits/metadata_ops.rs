use {
    crate::{ImageError, ImageFormat},
    mockall::automock,
    std::{
        io::{BufRead, Seek},
        num::NonZeroU32,
        path::Path,
    },
};

#[automock]
pub trait MetadataOps {
    fn from_path(&self, path: &Path) -> Result<(ImageFormat, NonZeroU32, NonZeroU32), ImageError>;
    fn from_bytes(
        &self,
        bytes: &Vec<u8>,
    ) -> Result<(ImageFormat, NonZeroU32, NonZeroU32), ImageError>;
    fn from_reader<R>(
        &self,
        reader: &mut R,
    ) -> Result<(ImageFormat, NonZeroU32, NonZeroU32), ImageError>
    where
        R: BufRead + Seek + 'static;
}
