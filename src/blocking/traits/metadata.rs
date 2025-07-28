use {
    crate::{ImageError, ImageMetadata},
    std::path::Path,
};

pub trait MetadataOps {
    fn from_path(&self, path: &Path) -> Result<ImageMetadata, ImageError>;
    fn from_bytes(&self, bytes: &[u8]) -> Result<ImageMetadata, ImageError>;
}
