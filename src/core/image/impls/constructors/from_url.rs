use {
    crate::{
        image::{
            enums::{ImageData, ImageSrc},
            ImageConfig,
        },
        utils::BlockingHttpClient,
        Image, ImageMetadata, Result, ValidationError,
    },
    std::num::NonZeroU32,
    url::Url,
};

impl Image {
    pub fn from_url(url: impl AsRef<str>) -> Result<Self> {
        let url = Url::parse(url.as_ref())?;
        let response = BlockingHttpClient::fetch_url(&url)?;
        let bytes = BlockingHttpClient::parse_response(response)?;
        let metadata = ImageMetadata::from_bytes(&bytes)?;

        Ok(Self {
            src: ImageSrc::Url(url),
            data: Some(ImageData::EncodedBytes(bytes)),
            config: ImageConfig::default(),
            height: NonZeroU32::new(metadata.height).ok_or(ValidationError::InvalidHeight)?,
            width: NonZeroU32::new(metadata.width).ok_or(ValidationError::InvalidWidth)?,
            format: metadata.format,
        })
    }
}

#[cfg(test)]
mod tests {
    use {super::*, crate::test_utils::png_bytes, httpmock::prelude::*, url::Url};

    #[test]
    fn from_url_happy_path_sets_fields_and_preserves_bytes() -> Result<()> {
        let server = MockServer::start();
        let bytes = png_bytes();

        let _m = server.mock(|when, then| {
            when.method(GET).path("/img.png");
            then.status(200).header("content-type", "image/png").body(bytes.clone());
        });

        let url = Url::parse(&format!("{}/img.png", server.base_url())).unwrap();
        let img = Image::from_url(&url)?;

        match &img.src {
            ImageSrc::Url(u) => assert_eq!(u, &url),
            _ => panic!("expected ImageSrc::Url"),
        }

        match &img.data {
            Some(ImageData::EncodedBytes(b)) => assert_eq!(b, &bytes),
            _ => panic!("expected Some(EncodedBytes)"),
        }

        Ok(())
    }

    #[test]
    fn from_url_rejects_non_image_payload() {
        let server = MockServer::start();
        let _m = server.mock(|when, then| {
            when.method(GET).path("/text");
            then.status(200).header("content-type", "text/plain").body("hello world");
        });

        let url = Url::parse(&format!("{}/text", server.base_url())).unwrap();
        let res = Image::from_url(&url);
        assert!(res.is_err(), "expected failure for non-image payload");
    }
}
