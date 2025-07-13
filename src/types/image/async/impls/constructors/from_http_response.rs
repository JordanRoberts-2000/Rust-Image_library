use {
    crate::{
        image::{
            enums::{ImageData, ImageSrc},
            r#async::{
                dependencies::ImageDeps,
                traits::{ImageDepsOps, MetadataOps, UrlDownloaderOp},
                Image,
            },
            ImageConfig,
        },
        InternalError, Result,
    },
    reqwest::Response,
    std::sync::Arc,
};

impl Image {
    pub async fn from_http_response(response: Response) -> Result<Self> {
        Self::from_http_response_internal(response, &ImageDeps::default()).await
    }

    pub(crate) async fn from_http_response_internal(
        response: Response,
        image_deps: &impl ImageDepsOps,
    ) -> Result<Self> {
        let (bytes, url) = image_deps.downloader().parse_response(response).await?;

        let bytes_arc = Arc::new(bytes);
        let (format, width, height) = image_deps.metadata().from_bytes(bytes_arc.clone()).await?;

        let bytes_vec = Arc::try_unwrap(bytes_arc).map_err(|_| {
            InternalError::ArcUnwrapFailed(
                "getting image bytes from 'from_bytes_internal'".to_string(),
            )
        })?;

        Ok(Self {
            src: ImageSrc::Url(url),
            data: ImageData::EncodedBytes(bytes_vec),
            config: ImageConfig::default(),
            height,
            width,
            format,
        })
    }
}
