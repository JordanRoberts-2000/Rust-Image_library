use {
    crate::{
        image::{
            blocking::{
                dependencies::ImageDeps,
                traits::{ImageDepsOps, MetadataOps},
                Image,
            },
            enums::{ImageData, ImageSrc},
            ImageConfig,
        },
        ImageError, Result,
    },
    reqwest::blocking::Response,
    url::Url,
};

impl Image {
    pub fn from_http_response(response: Response) -> Result<Self> {
        Self::from_http_response_internal(response, &ImageDeps::default())
    }

    pub(crate) fn from_http_response_internal(
        response: Response,
        image_deps: &impl ImageDepsOps,
    ) -> Result<Self> {
        let url = response.url().to_owned();
        let bytes = Self::read_and_validate_response(response, &url)?;

        let (format, width, height) = image_deps.metadata().from_bytes(&bytes)?;

        Ok(Self {
            src: ImageSrc::Url(url),
            data: ImageData::EncodedBytes(bytes),
            config: ImageConfig::default(),
            height,
            width,
            format,
        })
    }

    fn read_and_validate_response(response: Response, url: &Url) -> Result<Vec<u8>> {
        if !response.status().is_success() {
            let status_code = response.status().as_u16();
            let message = response
                .text()
                .unwrap_or_else(|_| "response couldn't be read".to_string());

            return Err(ImageError::FailedRequest {
                message,
                status_code,
                url: url.to_string(),
            });
        }

        let bytes = response
            .bytes()
            .map_err(|e| ImageError::ResponseReadFailed {
                source: e,
                url: url.to_string(),
            })?
            .to_vec();

        Ok(bytes)
    }
}
