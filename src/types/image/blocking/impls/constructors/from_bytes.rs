use {
    crate::{
        blocking::{
            dependencies::ImageService,
            traits::{ImageServiceOps, MetadataOps},
        },
        image::{
            blocking::{Image, ImageData},
            enums::ImageSrc,
            ImageConfig,
        },
        Result, ValidationError,
    },
    std::{cell::RefCell, num::NonZeroU32, rc::Rc},
};

impl Image {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Self::from_bytes_internal(bytes, &ImageService::default())
    }

    fn from_bytes_internal(bytes: Vec<u8>, service: &impl ImageServiceOps) -> Result<Self> {
        let metadata = service.metadata().from_bytes(&bytes)?;

        Ok(Self {
            src: ImageSrc::Bytes,
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
            blocking::{dependencies::MockImageService, traits::MockMetadataOps, Image},
            image::{blocking::ImageData, enums::ImageSrc},
            ImageError, ImageFormat, ImageMetadata,
        },
        std::{cell::RefCell, rc::Rc},
    };

    #[test]
    fn test_from_bytes_internal_success() {
        let bytes = vec![137, 80, 78, 71]; // e.g., partial PNG header

        let mut metadata_mock = MockMetadataOps::new();
        metadata_mock
            .expect_from_bytes()
            .returning(|_| Ok(ImageMetadata::new(800, 600, ImageFormat::Png)));

        let mock_deps = MockImageService { metadata: metadata_mock, ..Default::default() };

        let image = Image::from_bytes_internal(bytes.clone(), &mock_deps).unwrap();

        assert_eq!(image.format, ImageFormat::Png);
        assert_eq!(image.width(), 800);
        assert_eq!(image.height(), 600);
        assert_eq!(image.data, Rc::new(RefCell::new(ImageData::EncodedBytes(bytes))));
        assert_eq!(image.src, ImageSrc::Bytes);
    }

    #[test]
    fn test_from_bytes_internal_metadata_failure() {
        let bytes = vec![0, 1, 2, 3]; // invalid image data

        let mut metadata_mock = MockMetadataOps::new();
        metadata_mock.expect_from_bytes().returning(|_| Err(ImageError::UnknownFormat));

        let mock_deps = MockImageService { metadata: metadata_mock, ..Default::default() };

        let result = Image::from_bytes_internal(bytes, &mock_deps);

        assert!(matches!(result, Err(ImageError::UnknownFormat)));
    }
}
