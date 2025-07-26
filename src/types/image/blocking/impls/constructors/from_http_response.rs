use {
    crate::{
        blocking::{
            dependencies::ImageService,
            traits::{HttpClientOps, ImageServiceOps, MetadataOps},
        },
        image::{
            blocking::{Image, ImageData},
            enums::ImageSrc,
            ImageConfig,
        },
        Result, ValidationError,
    },
    reqwest::blocking::Response,
    std::{cell::RefCell, num::NonZeroU32, rc::Rc},
};

impl Image {
    pub fn from_http_response(response: Response) -> Result<Self> {
        Self::from_http_response_internal(response, &ImageService::default())
    }

    pub(crate) fn from_http_response_internal(
        response: Response, service: &impl ImageServiceOps,
    ) -> Result<Self> {
        let (bytes, url) = service.http().parse_response(response)?;
        let metadata = service.metadata().from_bytes(&bytes)?;

        Ok(Self {
            src: ImageSrc::Url(url),
            data: Rc::new(RefCell::new(ImageData::EncodedBytes(bytes))),
            config: ImageConfig::default(),
            height: NonZeroU32::new(metadata.height).ok_or(ValidationError::InvalidHeight)?,
            width: NonZeroU32::new(metadata.width).ok_or(ValidationError::InvalidWidth)?,
            format: metadata.format,
        })
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
    fn test_from_http_response_internal_success() {
        let dummy_url = Url::parse("http://example.com/image.jpg").unwrap();
        let dummy_bytes = vec![1, 2, 3];

        let mut http_mock = MockHttpClientOps::new();
        http_mock.expect_parse_response().returning(move |_| {
            Ok((vec![1, 2, 3], Url::parse("http://example.com/image.jpg").unwrap()))
        });

        let mut metadata_mock = MockMetadataOps::new();
        metadata_mock
            .expect_from_bytes()
            .returning(|_| Ok(ImageMetadata::new(800, 600, ImageFormat::Jpeg)));

        let mock_deps =
            MockImageService { metadata: metadata_mock, http: http_mock, ..Default::default() };

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
        assert_eq!(image.data, Rc::new(RefCell::new(ImageData::EncodedBytes(dummy_bytes))));
    }

    #[test]
    fn test_from_http_response_internal_parse_response_fails() {
        let mut http_mock = MockHttpClientOps::new();
        http_mock.expect_parse_response().returning(|_| {
            Err(ImageError::FailedRequest {
                message: "bad response".into(),
                status_code: 500,
                url: Url::parse("http://example.com").unwrap(),
            })
        });

        let mock_deps = MockImageService { http: http_mock, ..Default::default() };

        let response = Response::from(HttpResponse::builder().status(500).body("error").unwrap());

        let result = Image::from_http_response_internal(response, &mock_deps);
        assert!(matches!(result, Err(ImageError::FailedRequest { .. })));
    }

    #[test]
    fn test_from_http_response_internal_from_bytes_fails() {
        let dummy_url = Url::parse("http://example.com/image.jpg").unwrap();

        let mut http_mock = MockHttpClientOps::new();
        http_mock
            .expect_parse_response()
            .returning(move |_| Ok((vec![1, 2, 3], dummy_url.clone())));

        let mut metadata_mock = MockMetadataOps::new();
        metadata_mock.expect_from_bytes().returning(|_| Err(ImageError::UnknownFormat));

        let mock_deps =
            MockImageService { http: http_mock, metadata: metadata_mock, ..Default::default() };

        let response = Response::from(HttpResponse::builder().status(200).body("dummy").unwrap());

        let result = Image::from_http_response_internal(response, &mock_deps);
        assert!(matches!(result, Err(ImageError::UnknownFormat)));
    }
}
