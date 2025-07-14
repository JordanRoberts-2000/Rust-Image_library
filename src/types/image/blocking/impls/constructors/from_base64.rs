use {
    crate::{
        image::{
            blocking::{
                dependencies::ImageDeps,
                traits::{ImageDepsOps, MetadataOps},
                Image, ImageData,
            },
            enums::ImageSrc,
            ImageConfig,
        },
        ImageError, Result,
    },
    base64::Engine,
};

impl Image {
    pub fn from_base64(base_64: impl AsRef<str>) -> Result<Self> {
        let base_64 = base_64.as_ref();
        Self::from_base64_internal(base_64, &ImageDeps::default())
    }

    fn from_base64_internal(base_64: &str, image_deps: &impl ImageDepsOps) -> Result<Self> {
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(base_64)
            .map_err(|e| ImageError::Base64DecodeFailed(e, base_64.to_string()))?;

        let (format, width, height) = image_deps.metadata().from_bytes(&bytes)?;

        Ok(Self {
            src: ImageSrc::Base64(base_64.to_string()),
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
                blocking::{dependencies::MockImageDeps, traits::MockMetadataOps, ImageData},
                enums::ImageSrc,
            },
            ImageError, ImageFormat,
        },
        base64::Engine,
        std::num::NonZeroU32,
    };

    #[test]
    fn test_from_base64_internal_success() {
        let dummy_bytes = vec![1, 2, 3];
        let valid_base64 = base64::engine::general_purpose::STANDARD.encode(&dummy_bytes);

        let mut metadata_mock = MockMetadataOps::new();
        metadata_mock.expect_from_bytes().returning(|_| {
            Ok((
                ImageFormat::Png,
                NonZeroU32::new(800).unwrap(),
                NonZeroU32::new(600).unwrap(),
            ))
        });

        let mock_deps = MockImageDeps {
            metadata: metadata_mock,
            ..Default::default()
        };

        let image = Image::from_base64_internal(&valid_base64, &mock_deps).unwrap();

        assert_eq!(image.format, ImageFormat::Png);
        assert_eq!(image.width(), 800);
        assert_eq!(image.height(), 600);
        assert_eq!(image.data, ImageData::EncodedBytes(dummy_bytes));
        assert_eq!(image.src, ImageSrc::Base64(valid_base64));
    }

    #[test]
    fn test_from_base64_internal_invalid_base64() {
        let invalid_base64 = "!!!invalid_base64%%%";
        let result = Image::from_base64_internal(invalid_base64, &MockImageDeps::default());

        assert!(matches!(result, Err(ImageError::Base64DecodeFailed(_, s)) if s == invalid_base64));
    }

    #[test]
    fn test_from_base64_internal_metadata_failure() {
        let dummy_bytes = vec![1, 2, 3];
        let valid_base64 = base64::engine::general_purpose::STANDARD.encode(&dummy_bytes);

        let mut metadata_mock = MockMetadataOps::new();
        metadata_mock
            .expect_from_bytes()
            .returning(|_| Err(ImageError::UnknownFormat));

        let mock_deps = MockImageDeps {
            metadata: metadata_mock,
            ..Default::default()
        };

        let result = Image::from_base64_internal(&valid_base64, &mock_deps);

        assert!(matches!(result, Err(ImageError::UnknownFormat)));
    }
}
