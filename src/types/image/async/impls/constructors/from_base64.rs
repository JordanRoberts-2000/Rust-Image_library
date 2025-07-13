use {
    crate::{
        image::{
            enums::{ImageData, ImageSrc},
            r#async::{
                dependencies::ImageDeps,
                traits::{ImageDepsOps, MetadataOps},
                Image,
            },
            ImageConfig,
        },
        ImageError, InternalError, Result,
    },
    base64::Engine,
    std::sync::Arc,
    tokio::task::spawn_blocking,
};

impl Image {
    pub async fn from_base64(base_64: impl AsRef<str>) -> Result<Self> {
        let base_64 = base_64.as_ref();
        Self::from_base64_internal(base_64, &ImageDeps::default()).await
    }

    async fn from_base64_internal(base_64: &str, image_deps: &impl ImageDepsOps) -> Result<Self> {
        let base_64 = base_64.to_string();

        let bytes = spawn_blocking({
            let base64_for_decode = base_64.clone();
            move || {
                base64::engine::general_purpose::STANDARD
                    .decode(&base64_for_decode)
                    .map_err(|e| ImageError::Base64DecodeFailed(e, base64_for_decode))
            }
        })
        .await
        .map_err(ImageError::TaskJoinError)??;

        let bytes_arc = Arc::new(bytes);
        let (format, width, height) = image_deps.metadata().from_bytes(bytes_arc.clone()).await?;

        let bytes_vec = Arc::try_unwrap(bytes_arc).map_err(|_| {
            InternalError::ArcUnwrapFailed(
                "getting image bytes from 'from_bytes_internal'".to_string(),
            )
        })?;

        Ok(Self {
            src: ImageSrc::Base64(base_64),
            data: ImageData::EncodedBytes(bytes_vec),
            config: ImageConfig::default(),
            height,
            width,
            format,
        })
    }
}
