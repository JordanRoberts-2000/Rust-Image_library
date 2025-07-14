use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{
    image::{
        enums::ImageSrc,
        r#async::{
            dependencies::ImageDeps,
            traits::{ImageDepsOps, MetadataOps},
            Image, ImageData, ImageState,
        },
        ImageConfig,
    },
    Result,
};

impl Image {
    pub async fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Self::from_bytes_internal(bytes, &ImageDeps::default()).await
    }

    async fn from_bytes_internal(bytes: Vec<u8>, image_deps: &impl ImageDepsOps) -> Result<Self> {
        let bytes_arc = Arc::new(bytes);
        let (format, width, height) = image_deps.metadata().from_bytes(bytes_arc.clone()).await?;

        let state = ImageState {
            config: ImageConfig::default(),
            data: ImageData::EncodedBytes(bytes_arc),
            height,
            width,
            format,
        };

        Ok(Self {
            src: ImageSrc::Bytes,
            state: Arc::new(RwLock::new(state)),
        })
    }
}
