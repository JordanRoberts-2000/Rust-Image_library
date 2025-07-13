use {
    crate::{ImageFormat, Result},
    std::{
        io::{BufRead, Seek},
        num::NonZeroU32,
        path::Path,
    },
};

pub trait MetadataOps {
    async fn from_path(&self, path: &Path) -> Result<(ImageFormat, NonZeroU32, NonZeroU32)>;
    async fn from_bytes(&self, bytes: Vec<u8>) -> Result<(ImageFormat, NonZeroU32, NonZeroU32)>;
    async fn from_reader<R>(&self, reader: R) -> Result<(ImageFormat, NonZeroU32, NonZeroU32)>
    where
        R: BufRead + Seek + Send + 'static;
}
