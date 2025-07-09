use {
    crate::{
        image::blocking::{
            dependencies::ImageDeps,
            traits::{ImageDepsOps, UrlDownloaderOp},
            Image,
        },
        ImageError, Result,
    },
    url::Url,
};

impl Image {
    pub fn from_url(url: impl AsRef<str>) -> Result<Self> {
        let url = Url::parse(url.as_ref())
            .map_err(|e| ImageError::UrlParse(e, url.as_ref().to_string()))?;

        Self::from_url_internal(url, &ImageDeps::default())
    }

    pub fn from_url_internal(url: Url, image_deps: &impl ImageDepsOps) -> Result<Self> {
        let response = image_deps.downloader().url(url)?;
        Self::from_http_response_internal(response, image_deps)
    }
}

#[cfg(test)]
mod tests {
    use {
        crate::{
            blocking::Image,
            image::{
                blocking::{
                    dependencies::MockImageDeps,
                    traits::{MockMetadataOps, MockUrlDownloaderOp},
                },
                enums::{ImageData, ImageSrc},
            },
            ImageError, ImageFormat,
        },
        http::Response as HttpResponse,
        reqwest::blocking::Response,
        std::num::NonZeroU32,
        url::Url,
    };

    #[test]
    fn test_from_url_internal_success() {
        let test_url = Url::parse("http://example.com/image.jpg").unwrap();
        let dummy_bytes = vec![1, 2, 3];

        let mut downloader_mock = MockUrlDownloaderOp::new();
        downloader_mock.expect_url().returning(move |_| {
            Ok(Response::from(
                HttpResponse::builder()
                    .status(200)
                    .header("content-type", "image/jpeg")
                    .body("dummy body")
                    .unwrap(),
            ))
        });

        let expected_url = test_url.clone();
        let expected_bytes = dummy_bytes.clone();
        downloader_mock
            .expect_parse_response()
            .returning(move |_| Ok((expected_bytes.clone(), expected_url.clone())));

        let mut metadata_mock = MockMetadataOps::new();
        metadata_mock.expect_from_bytes().returning(|_| {
            Ok((
                ImageFormat::Jpeg,
                NonZeroU32::new(640).unwrap(),
                NonZeroU32::new(480).unwrap(),
            ))
        });

        let mock_deps = MockImageDeps {
            downloader: downloader_mock,
            metadata: metadata_mock,
            ..Default::default()
        };

        let image = Image::from_url_internal(test_url.clone(), &mock_deps).unwrap();

        assert_eq!(image.format, ImageFormat::Jpeg);
        assert_eq!(image.width(), 640);
        assert_eq!(image.height(), 480);
        assert_eq!(image.src, ImageSrc::Url(test_url));
        assert_eq!(image.data, ImageData::EncodedBytes(dummy_bytes));
    }

    #[test]
    fn test_from_url_internal_downloader_url_failure() {
        let test_url = Url::parse("http://example.com/image.jpg").unwrap();

        let mut downloader_mock = MockUrlDownloaderOp::new();
        downloader_mock.expect_url().returning({
            let url = test_url.clone();
            move |_| {
                Err(ImageError::DownloadFailed {
                    url: url.clone(),
                    source: reqwest::blocking::get("http://[::1]:1234").unwrap_err(),
                })
            }
        });

        let mock_deps = MockImageDeps {
            downloader: downloader_mock,
            ..Default::default()
        };

        let result = Image::from_url_internal(test_url, &mock_deps);

        assert!(matches!(result, Err(ImageError::DownloadFailed { .. })));
    }
}
