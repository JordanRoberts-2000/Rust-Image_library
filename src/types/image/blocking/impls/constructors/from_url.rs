use {
    crate::{
        blocking::{
            dependencies::ImageService,
            traits::{HttpClientOps, ImageServiceOps},
        },
        image::blocking::Image,
        ImageError, Result,
    },
    url::Url,
};

impl Image {
    pub fn from_url(url: impl AsRef<str>) -> Result<Self> {
        let url = Url::parse(url.as_ref())
            .map_err(|e| ImageError::UrlParse(e, url.as_ref().to_string()))?;

        Self::from_url_internal(url, &ImageService::default())
    }

    pub fn from_url_internal(url: Url, service: &impl ImageServiceOps) -> Result<Self> {
        let response = service.http().url(url)?;
        Self::from_http_response_internal(response, service)
    }
}

#[cfg(test)]
mod tests {
    use {
        crate::{
            blocking::{
                dependencies::MockImageService,
                traits::{MockHttpClientOps, MockMetadataOps},
                Image,
            },
            image::{blocking::ImageData, enums::ImageSrc},
            ImageError, ImageFormat, ImageMetadata,
        },
        http::Response as HttpResponse,
        reqwest::blocking::Response,
        std::{cell::RefCell, rc::Rc},
        url::Url,
    };

    #[test]
    fn test_from_url_internal_success() {
        let test_url = Url::parse("http://example.com/image.jpg").unwrap();
        let dummy_bytes = vec![1, 2, 3];

        let mut http_mock = MockHttpClientOps::new();
        http_mock.expect_url().returning(move |_| {
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
        http_mock
            .expect_parse_response()
            .returning(move |_| Ok((expected_bytes.clone(), expected_url.clone())));

        let mut metadata_mock = MockMetadataOps::new();
        metadata_mock
            .expect_from_bytes()
            .returning(|_| Ok(ImageMetadata::new(800, 600, ImageFormat::Jpeg)));

        let mock_deps =
            MockImageService { http: http_mock, metadata: metadata_mock, ..Default::default() };

        let image = Image::from_url_internal(test_url.clone(), &mock_deps).unwrap();

        assert_eq!(image.format, ImageFormat::Jpeg);
        assert_eq!(image.width(), 640);
        assert_eq!(image.height(), 480);
        assert_eq!(image.src, ImageSrc::Url(test_url));
        assert_eq!(image.data, Rc::new(RefCell::new(ImageData::EncodedBytes(dummy_bytes))));
    }

    #[test]
    fn test_from_url_internal_downloader_url_failure() {
        let test_url = Url::parse("http://example.com/image.jpg").unwrap();

        let mut http_mock = MockHttpClientOps::new();
        http_mock.expect_url().returning({
            let url = test_url.clone();
            move |_| {
                Err(ImageError::DownloadFailed {
                    url: url.clone(),
                    source: reqwest::blocking::get("http://[::1]:1234").unwrap_err(),
                })
            }
        });

        let mock_deps = MockImageService { http: http_mock, ..Default::default() };

        let result = Image::from_url_internal(test_url, &mock_deps);

        assert!(matches!(result, Err(ImageError::DownloadFailed { .. })));
    }
}
