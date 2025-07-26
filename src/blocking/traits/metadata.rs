#[cfg(test)]
use mockall::automock;
use {
    crate::{ImageError, ImageMetadata},
    std::path::Path,
};

#[cfg_attr(test, automock)]

pub trait MetadataOps {
    fn from_path(&self, path: &Path) -> Result<ImageMetadata, ImageError>;
    fn from_bytes(&self, bytes: &[u8]) -> Result<ImageMetadata, ImageError>;
}
