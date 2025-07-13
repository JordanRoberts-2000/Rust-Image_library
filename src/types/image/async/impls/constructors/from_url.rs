use {
    crate::{
        image::r#async::{
            dependencies::ImageDeps,
            traits::{ImageDepsOps, UrlDownloaderOp},
            Image,
        },
        ImageError, Result,
    },
    url::Url,
};

impl Image {
    pub async fn from_url(url: impl AsRef<str>) -> Result<Self> {
        let url = Url::parse(url.as_ref())
            .map_err(|e| ImageError::UrlParse(e, url.as_ref().to_string()))?;

        Self::from_url_internal(url, &ImageDeps::default()).await
    }

    async fn from_url_internal(url: Url, image_deps: &impl ImageDepsOps) -> Result<Self> {
        let response = image_deps.downloader().url(url).await?;
        Self::from_http_response_internal(response, image_deps).await
    }
}
