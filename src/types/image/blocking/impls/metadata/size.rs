use crate::{
    blocking::{
        dependencies::ImageService,
        traits::{FsRepoOps, ImageServiceOps},
        Image,
    },
    image::enums::ImageSrc,
    ByteSize, ImageError, Result,
};

impl Image {
    pub fn encoded_size(&mut self) -> Result<ByteSize> {
        let mut buffer = Vec::new();
        self.encode(&mut buffer, self.format)?;
        Ok(ByteSize::new(buffer.len()))
    }

    pub fn source_file_size(&mut self) -> Result<ByteSize> {
        self.source_file_size_internal(&ImageService::default())
    }

    fn source_file_size_internal(&mut self, service: &impl ImageServiceOps) -> Result<ByteSize> {
        match &self.src {
            ImageSrc::File(path) => Ok(ByteSize::from(service.fs().get_file_size(path)?)),
            _ => Err(ImageError::SourceFileSizeUnavailable),
        }
    }

    pub fn raw_size(&mut self) -> Result<ByteSize> {
        let img = self.process_image()?;

        let bytes = img.as_ref().as_bytes().len();
        Ok(ByteSize::new(bytes))
    }
}
