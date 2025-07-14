use {
    crate::{
        image::{
            enums::ImageSrc,
            r#async::{
                dependencies::ImageDeps,
                traits::{ImageDepsOps, MetadataOps, UrlDownloaderOp},
                Image, ImageData, ImageState,
            },
            ImageConfig,
        },
        Result,
    },
    reqwest::Response,
    std::sync::Arc,
    tokio::sync::RwLock,
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

        let state = ImageState {
            config: ImageConfig::default(),
            data: ImageData::EncodedBytes(bytes_arc),
            height,
            width,
            format,
        };

        Ok(Self {
            src: ImageSrc::Url(url),
            state: Arc::new(RwLock::new(state)),
        })
    }
}
