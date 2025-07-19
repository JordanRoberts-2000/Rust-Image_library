use crate::{
    blocking::{
        dependencies::ImageService,
        traits::{ImageServiceOps, MetadataOps},
    },
    image::{
        blocking::{Image, ImageData},
        enums::ImageSrc,
        ImageConfig,
    },
    Result,
};

impl Image {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Self::from_bytes_internal(bytes, &ImageService::default())
    }

    fn from_bytes_internal(bytes: Vec<u8>, service: &impl ImageServiceOps) -> Result<Self> {
        let (format, width, height) = service.metadata().from_bytes(&bytes)?;

        Ok(Self {
            src: ImageSrc::Bytes,
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
            blocking::{dependencies::MockImageService, traits::MockMetadataOps, Image},
            image::{blocking::ImageData, enums::ImageSrc},
            ImageError, ImageFormat,
        },
        std::num::NonZeroU32,
    };

    #[test]
    fn test_from_bytes_internal_success() {
        let bytes = vec![137, 80, 78, 71]; // e.g., partial PNG header

        let mut metadata_mock = MockMetadataOps::new();
        metadata_mock.expect_from_bytes().returning(|_| {
            Ok((
                ImageFormat::Png,
                NonZeroU32::new(800).unwrap(),
                NonZeroU32::new(600).unwrap(),
            ))
        });

        let mock_deps = MockImageService {
            metadata: metadata_mock,
            ..Default::default()
        };

        let image = Image::from_bytes_internal(bytes.clone(), &mock_deps).unwrap();

        assert_eq!(image.format, ImageFormat::Png);
        assert_eq!(image.width(), 800);
        assert_eq!(image.height(), 600);
        assert_eq!(image.data, ImageData::EncodedBytes(bytes));
        assert_eq!(image.src, ImageSrc::Bytes);
    }

    #[test]
    fn test_from_bytes_internal_metadata_failure() {
        let bytes = vec![0, 1, 2, 3]; // invalid image data

        let mut metadata_mock = MockMetadataOps::new();
        metadata_mock
            .expect_from_bytes()
            .returning(|_| Err(ImageError::UnknownFormat));

        let mock_deps = MockImageService {
            metadata: metadata_mock,
            ..Default::default()
        };

        let result = Image::from_bytes_internal(bytes, &mock_deps);

        assert!(matches!(result, Err(ImageError::UnknownFormat)));
    }
}
