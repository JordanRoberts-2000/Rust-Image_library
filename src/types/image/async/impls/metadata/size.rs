use crate::{
    image::{
        enums::ImageSrc,
        r#async::{
            dependencies::ImageDeps,
            traits::{FsRepoOps, ImageDepsOps},
            ImageData,
        },
    },
    ByteSize, Image, ImageError, InternalError, Result,
};

impl Image {
    pub async fn encoded_size(&mut self) -> Result<ByteSize> {
        let bytes = self.to_bytes().await?;
        Ok(ByteSize::new(bytes.len()))
    }

    pub async fn source_file_size(&mut self) -> Result<ByteSize> {
        self.source_file_size_internal(&ImageDeps::default()).await
    }

    async fn source_file_size_internal(
        &mut self,
        image_deps: &impl ImageDepsOps,
    ) -> Result<ByteSize> {
        match &self.src {
            ImageSrc::File(path) => Ok(ByteSize::from(image_deps.fs().get_file_size(path).await?)),
            _ => Err(ImageError::SourceFileSizeUnavailable),
        }
    }

    pub async fn raw_size(&mut self) -> Result<ByteSize> {
        self.decode().await?;
        let state = self.state.read().await;

        let img = match &state.data {
            ImageData::Decoded(ref img) => img,
            _ => return Err(InternalError::DecodingInvariantViolatedAfterDecodeAssignment.into()),
        };

        let bytes = img.as_bytes().len();
        Ok(ByteSize::new(bytes))
    }
}
