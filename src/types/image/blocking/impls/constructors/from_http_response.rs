use {
    crate::{
        image::{
            blocking::{
                dependencies::ImageDeps,
                traits::{ImageDepsOps, MetadataOps, UrlDownloaderOp},
                Image, ImageData,
            },
            enums::ImageSrc,
            ImageConfig,
        },
        Result,
    },
    reqwest::blocking::Response,
};

impl Image {
    pub fn from_http_response(response: Response) -> Result<Self> {
        Self::from_http_response_internal(response, &ImageDeps::default())
    }

    pub(crate) fn from_http_response_internal(
        response: Response,
        image_deps: &impl ImageDepsOps,
    ) -> Result<Self> {
        let (bytes, url) = image_deps.downloader().parse_response(response)?;
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
                    ImageData,
                },
                enums::ImageSrc,
            },
            ImageError, ImageFormat,
        },
        http::Response as HttpResponse,
        reqwest::blocking::Response,
        std::num::NonZeroU32,
        url::Url,
    };

    #[test]
    fn test_from_http_response_internal_success() {
        let dummy_url = Url::parse("http://example.com/image.jpg").unwrap();
        let dummy_bytes = vec![1, 2, 3];

        let mut downloader_mock = MockUrlDownloaderOp::new();
        downloader_mock.expect_parse_response().returning(move |_| {
            Ok((
                vec![1, 2, 3],
                Url::parse("http://example.com/image.jpg").unwrap(),
            ))
        });

        let mut metadata_mock = MockMetadataOps::new();
        metadata_mock.expect_from_bytes().returning(|_| {
            Ok((
                ImageFormat::Jpeg,
                NonZeroU32::new(800).unwrap(),
                NonZeroU32::new(600).unwrap(),
            ))
        });

        let mock_deps = MockImageDeps {
            metadata: metadata_mock,
            downloader: downloader_mock,
            ..Default::default()
        };

        let response = Response::from(
            HttpResponse::builder()
                .status(200)
                .header("content-type", "image/jpeg")
                .body("dummy body")
                .unwrap(),
        );

        let image = Image::from_http_response_internal(response, &mock_deps).unwrap();

        assert_eq!(image.format, ImageFormat::Jpeg);
        assert_eq!(image.width(), 800);
        assert_eq!(image.height(), 600);
        assert_eq!(image.src, ImageSrc::Url(dummy_url));
        assert_eq!(image.data, ImageData::EncodedBytes(dummy_bytes));
    }

    #[test]
    fn test_from_http_response_internal_parse_response_fails() {
        let mut downloader_mock = MockUrlDownloaderOp::new();
        downloader_mock.expect_parse_response().returning(|_| {
            Err(ImageError::FailedRequest {
                message: "bad response".into(),
                status_code: 500,
                url: Url::parse("http://example.com").unwrap(),
            })
        });

        let mock_deps = MockImageDeps {
            downloader: downloader_mock,
            ..Default::default()
        };

        let response = Response::from(HttpResponse::builder().status(500).body("error").unwrap());

        let result = Image::from_http_response_internal(response, &mock_deps);
        assert!(matches!(result, Err(ImageError::FailedRequest { .. })));
    }

    #[test]
    fn test_from_http_response_internal_from_bytes_fails() {
        let dummy_url = Url::parse("http://example.com/image.jpg").unwrap();

        let mut downloader_mock = MockUrlDownloaderOp::new();
        downloader_mock
            .expect_parse_response()
            .returning(move |_| Ok((vec![1, 2, 3], dummy_url.clone())));

        let mut metadata_mock = MockMetadataOps::new();
        metadata_mock
            .expect_from_bytes()
            .returning(|_| Err(ImageError::UnknownFormat));

        let mock_deps = MockImageDeps {
            downloader: downloader_mock,
            metadata: metadata_mock,
            ..Default::default()
        };

        let response = Response::from(HttpResponse::builder().status(200).body("dummy").unwrap());

        let result = Image::from_http_response_internal(response, &mock_deps);
        assert!(matches!(result, Err(ImageError::UnknownFormat)));
    }
}
