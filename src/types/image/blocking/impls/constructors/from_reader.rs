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
        IoError, Result, ValidationError,
    },
    std::{
        cell::RefCell,
        io::{BufRead, Seek},
        num::NonZeroU32,
        rc::Rc,
    },
};

impl Image {
    pub fn from_reader(reader: &mut (impl BufRead + Seek + 'static)) -> Result<Self> {
        Self::from_reader_internal(reader, &ImageService::default())
    }

    fn from_reader_internal(
        reader: &mut (impl BufRead + Seek + 'static), service: &impl ImageServiceOps,
    ) -> Result<Self> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).map_err(IoError::ReadStream)?;

        let metadata = service.metadata().from_bytes(&bytes)?;

        Ok(Self {
            src: ImageSrc::Reader,
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
            blocking::{dependencies::MockImageService, traits::MockMetadataOps},
            image::{
                blocking::{Image, ImageData},
                enums::ImageSrc,
            },
            ImageFormat, ImageMetadata,
        },
        std::{cell::RefCell, io::Cursor, rc::Rc},
    };

    #[test]
    fn test_from_reader_internal_success() {
        let data = vec![1, 2, 3, 4]; // dummy image data
        let mut reader = Cursor::new(data.clone());

        let mut metadata_mock = MockMetadataOps::new();
        metadata_mock
            .expect_from_bytes()
            .returning(|_| Ok(ImageMetadata::new(800, 600, ImageFormat::Jpeg)));

        let mock_deps = MockImageService { metadata: metadata_mock, ..Default::default() };

        let image = Image::from_reader_internal(&mut reader, &mock_deps).unwrap();

        assert_eq!(image.width(), 800);
        assert_eq!(image.height(), 600);
        assert_eq!(image.format, ImageFormat::Jpeg);
        assert_eq!(image.data, Rc::new(RefCell::new(ImageData::EncodedBytes(data))));
        assert_eq!(image.src, ImageSrc::Reader);
    }
}
