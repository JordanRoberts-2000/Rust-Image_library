use crate::{
    blocking::Image,
    image::{
        blocking::{
            dependencies::ImageDeps,
            traits::{FsOps, ImageDepsOps},
        },
        enums::ImageSrc,
    },
    ByteSize, ImageError, Result,
};

impl Image {
    pub fn encoded_size(&mut self) -> Result<ByteSize> {
        self.apply_transforms()?;

        let mut buffer = Vec::new();
        self.encode(&mut buffer)?;
        Ok(ByteSize::new(buffer.len()))
    }

    pub fn source_file_size(&mut self) -> Result<ByteSize> {
        self.source_file_size_internal(&ImageDeps::default())
    }

    fn source_file_size_internal(&mut self, image_deps: &impl ImageDepsOps) -> Result<ByteSize> {
        match &self.src {
            ImageSrc::File(path) => Ok(ByteSize::from(image_deps.fs().get_file_size(path)?)),
            _ => Err(ImageError::SourceFileSizeUnavailable),
        }
    }

    pub fn raw_size(&mut self) -> Result<ByteSize> {
        let decoded = self.get_decoded()?;
        let bytes = decoded.as_bytes().len();
        Ok(ByteSize::new(bytes))
    }
}
