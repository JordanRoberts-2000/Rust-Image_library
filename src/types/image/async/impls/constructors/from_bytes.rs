use std::sync::Arc;

use crate::{
    image::{
        enums::{ImageData, ImageSrc},
        r#async::{
            dependencies::ImageDeps,
            traits::{ImageDepsOps, MetadataOps},
            Image,
        },
        ImageConfig,
    },
    InternalError, Result,
};

impl Image {
    pub async fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Self::from_bytes_internal(bytes, &ImageDeps::default()).await
    }

    async fn from_bytes_internal(bytes: Vec<u8>, image_deps: &impl ImageDepsOps) -> Result<Self> {
        let bytes_arc = Arc::new(bytes);
        let (format, width, height) = image_deps.metadata().from_bytes(bytes_arc.clone()).await?;

        let bytes_vec = Arc::try_unwrap(bytes_arc).map_err(|_| {
            InternalError::ArcUnwrapFailed(
                "getting image bytes from 'from_bytes_internal'".to_string(),
            )
        })?;

        Ok(Self {
            src: ImageSrc::Bytes,
            data: ImageData::EncodedBytes(bytes_vec),
            config: ImageConfig::default(),
            height,
            width,
            format,
        })
    }
}
