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
        IoError, Result,
    },
    std::io::{BufRead, Seek},
};

impl Image {
    pub fn from_reader(mut reader: impl BufRead + Seek + 'static) -> Result<Self> {
        Self::from_reader_internal(&mut reader, &ImageDeps::default())
    }

    fn from_reader_internal(
        reader: &mut (impl BufRead + Seek + 'static),
        image_deps: &impl ImageDepsOps,
    ) -> Result<Self> {
        reader.rewind().map_err(IoError::ReadStream)?;

        let (format, width, height) = image_deps.metadata().from_reader(reader)?;

        reader.rewind().map_err(IoError::ReadStream)?;

        let mut bytes = Vec::new();
        reader
            .read_to_end(&mut bytes)
            .map_err(IoError::ReadStream)?;

        Ok(Self {
            src: ImageSrc::Reader,
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
            image::{
                blocking::{dependencies::MockImageDeps, traits::MockMetadataOps, Image},
                enums::{ImageData, ImageSrc},
            },
            ImageError, ImageFormat,
        },
        std::{io::Cursor, num::NonZeroU32},
    };

    #[test]
    fn test_from_reader_internal_success() {
        let data = vec![1, 2, 3, 4]; // dummy image data
        let mut reader = Cursor::new(data.clone());

        let mut metadata_mock = MockMetadataOps::new();
        metadata_mock
            .expect_from_reader()
            .returning(|_: &mut Cursor<Vec<u8>>| {
                Ok((
                    ImageFormat::Jpeg,
                    NonZeroU32::new(800).unwrap(),
                    NonZeroU32::new(600).unwrap(),
                ))
            });

        let mock_deps = MockImageDeps {
            metadata: metadata_mock,
            ..Default::default()
        };

        let image = Image::from_reader_internal(&mut reader, &mock_deps).unwrap();

        assert_eq!(image.width(), 800);
        assert_eq!(image.height(), 600);
        assert_eq!(image.format, ImageFormat::Jpeg);
        assert_eq!(image.data, ImageData::EncodedBytes(data));
        assert_eq!(image.src, ImageSrc::Reader);
    }

    #[test]
    fn test_from_reader_internal_metadata_failure() {
        let data = vec![1, 2, 3];
        let mut reader = Cursor::new(data);

        let mut metadata_mock = MockMetadataOps::new();
        metadata_mock
            .expect_from_reader()
            .returning(|_: &mut Cursor<Vec<u8>>| Err(ImageError::UnknownFormat));

        let mock_deps = MockImageDeps {
            metadata: metadata_mock,
            ..Default::default()
        };

        let result = Image::from_reader_internal(&mut reader, &mock_deps);
        assert!(matches!(result, Err(ImageError::UnknownFormat)));
    }
}
