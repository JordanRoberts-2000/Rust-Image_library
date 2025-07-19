use {
    crate::{blocking::traits::MetadataOps, ImageError, ImageMetadata},
    std::path::Path,
};

pub struct MetadataService;

impl MetadataOps for MetadataService {
    fn from_path(&self, path: &Path) -> Result<ImageMetadata, ImageError> {
        ImageMetadata::from_path(path)
    }

    fn from_bytes(&self, bytes: &[u8]) -> Result<ImageMetadata, ImageError> {
        ImageMetadata::from_bytes(bytes)
    }
}
