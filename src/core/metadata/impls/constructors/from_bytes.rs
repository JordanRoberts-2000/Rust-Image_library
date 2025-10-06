use {
    crate::{ErrorKind, ImageMetadata, ImageSrc, Result, WithSrc},
    image::ImageReader,
    std::io::Cursor,
};

impl ImageMetadata {
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self> {
        let reader = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()
            .map_err(ErrorKind::FormatDetectionFailed)
            .with_src(ImageSrc::Bytes)?;

        Self::from_image_reader(reader)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            test_utils::{corrupted_header_bytes, encoded_bytes, MOCK_IMAGE_DIMENSIONS},
            ImageFormat,
        },
        strum::IntoEnumIterator,
    };

    #[test]
    fn test_from_bytes_all_formats() {
        for format in ImageFormat::iter() {
            let bytes = encoded_bytes(format);
            let metadata = ImageMetadata::from_bytes(&bytes)
                .unwrap_or_else(|e| panic!("from_bytes failed for {format:?}: {e}"));

            assert_eq!(metadata.format, format);
            assert_eq!(metadata.width(), MOCK_IMAGE_DIMENSIONS.0);
            assert_eq!(metadata.height(), MOCK_IMAGE_DIMENSIONS.1);
        }
    }

    #[test]
    fn test_from_bytes_empty_data() {
        let bytes = &[];
        let result = ImageMetadata::from_bytes(bytes);

        assert!(result.is_err(), "Expected error for empty bytes");

        let err = result.unwrap_err();
        assert!(
            matches!(err.kind(), ErrorKind::UnknownFormat),
            "Expected UnknownFormat, got {:?}",
            err.kind()
        );
    }

    #[test]
    fn test_from_bytes_invalid_data() {
        let bytes = vec![0xFF; 100]; // Random invalid data
        let result = ImageMetadata::from_bytes(&bytes);

        assert!(result.is_err(), "Expected error for invalid data");

        let err = result.unwrap_err();
        assert!(
            matches!(err.kind(), ErrorKind::UnknownFormat),
            "Expected UnknownFormat, got {:?}",
            err.kind()
        );
    }

    #[test]
    fn test_from_bytes_corrupted_header() {
        let bytes = corrupted_header_bytes(ImageFormat::Png);
        let result = ImageMetadata::from_bytes(&bytes);

        assert!(result.is_err(), "Expected error for corrupted header");
    }
}
