use {
    crate::{ErrorKind, ImageFormat, ImageMetadata, Result, ValidationError},
    image::ImageReader,
    std::{
        io::{BufRead, Seek},
        num::NonZeroU32,
    },
};

impl ImageMetadata {
    pub(super) fn from_image_reader<R>(reader: ImageReader<R>) -> Result<Self>
    where
        R: BufRead + Seek,
    {
        let format = reader
            .format()
            .ok_or(ErrorKind::UnknownFormat)
            .map_err(Into::into)
            .and_then(ImageFormat::try_from)?;

        let (width, height) = reader.into_dimensions().map_err(ErrorKind::PeakDimensionsFailed)?;

        Ok(Self {
            format,
            width: NonZeroU32::new(width).ok_or(ValidationError::InvalidWidth)?,
            height: NonZeroU32::new(height).ok_or(ValidationError::InvalidHeight)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::test_utils::{
            corrupted_bytes, corrupted_header_bytes, encoded_bytes, unsupported_format_bytes,
            MOCK_IMAGE_DIMENSIONS,
        },
        image::ImageReader,
        std::io::Cursor,
        strum::IntoEnumIterator,
    };

    fn create_image_reader(bytes: Vec<u8>) -> ImageReader<Cursor<Vec<u8>>> {
        let cursor = Cursor::new(bytes);
        ImageReader::new(cursor)
            .with_guessed_format()
            .unwrap_or_else(|e| panic!("{:?}", ErrorKind::FormatDetectionFailed(e)))
    }

    #[test]
    fn test_valid_images() {
        for format in ImageFormat::iter() {
            let bytes = encoded_bytes(format);
            let reader = create_image_reader(bytes);

            let metadata = ImageMetadata::from_image_reader(reader)
                .unwrap_or_else(|e| panic!("from_image_reader error for format '{format:?}': {e}"));
            let (width, height) = MOCK_IMAGE_DIMENSIONS;

            assert_eq!(metadata.format, format);
            assert_eq!(metadata.width(), width);
            assert_eq!(metadata.height(), height);
        }
    }

    #[test]
    fn test_corrupted_image_header() {
        let bytes = corrupted_header_bytes(ImageFormat::Png);
        let reader = create_image_reader(bytes);
        let result = ImageMetadata::from_image_reader(reader);

        assert!(result.is_err(), "Expected error for corrupted image header");

        let err = result.unwrap_err();
        assert!(
            matches!(err.kind(), ErrorKind::PeakDimensionsFailed(_)),
            "Expected UnknownFormat, got {:?}",
            err.kind()
        );
    }

    #[test]
    fn test_corrupted_image_data() {
        let bytes = corrupted_bytes(ImageFormat::Png);
        let reader = create_image_reader(bytes);

        let _metadata = ImageMetadata::from_image_reader(reader).unwrap_or_else(|e| {
            panic!(
                "from_image_reader unexpectedly failed on corrupted PNG: {e:?} (kind: {:?})",
                e.kind()
            )
        });
    }

    #[test]
    fn test_empty_data() {
        let empty_data = Vec::new();
        let reader = create_image_reader(empty_data);

        let err = ImageMetadata::from_image_reader(reader)
            .expect_err("empty data should not produce valid metadata");

        match err.kind() {
            ErrorKind::UnknownFormat => {}
            other => panic!("Expected UnknownFormat or FormatDetectionFailed, got {:?}", other),
        }
    }

    #[test]
    fn test_unsupported_format_conversion() {
        let bytes = unsupported_format_bytes();
        let reader = create_image_reader(bytes);

        let err = ImageMetadata::from_image_reader(reader)
            .expect_err("unsupported format should not map to your ImageFormat");

        match err.kind() {
            ErrorKind::Validation(ValidationError::UnsupportedFormat(_f)) => {}
            other => panic!("Expected an 'unsupported format' style error, got {:?}", other),
        }
    }
}
