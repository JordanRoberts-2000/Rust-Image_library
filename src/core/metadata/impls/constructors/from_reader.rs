use {
    crate::{ErrorKind, ImageMetadata, ImageSrc, Result, WithSrc},
    image::ImageReader,
    std::io::{BufRead, Seek},
};

impl ImageMetadata {
    pub fn from_reader<R>(reader: R) -> Result<Self>
    where
        R: BufRead + Seek,
    {
        let reader = ImageReader::new(reader)
            .with_guessed_format()
            .map_err(ErrorKind::FormatDetectionFailed)
            .with_src(ImageSrc::Reader)?;

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
        std::io::Cursor,
        strum::IntoEnumIterator,
    };

    #[test]
    fn test_from_reader_all_formats() {
        for format in ImageFormat::iter() {
            let bytes = encoded_bytes(format);
            let cursor = Cursor::new(bytes);

            let metadata = ImageMetadata::from_reader(cursor)
                .unwrap_or_else(|e| panic!("from_reader failed for {format:?}: {e}"));

            assert_eq!(metadata.format, format);
            assert_eq!(metadata.width(), MOCK_IMAGE_DIMENSIONS.0);
            assert_eq!(metadata.height(), MOCK_IMAGE_DIMENSIONS.1);
        }
    }

    #[test]
    fn test_from_reader_empty_data() {
        let bytes: Vec<u8> = vec![];
        let cursor = Cursor::new(bytes);

        let result = ImageMetadata::from_reader(cursor);

        assert!(result.is_err(), "Expected error for empty reader");

        let err = result.unwrap_err();
        assert!(
            matches!(err.kind(), ErrorKind::UnknownFormat),
            "Expected UnknownFormat, got {:?}",
            err.kind()
        );
    }

    #[test]
    fn test_from_reader_invalid_data() {
        let bytes = vec![0xFF; 100];
        let cursor = Cursor::new(bytes);

        let result = ImageMetadata::from_reader(cursor);

        assert!(result.is_err(), "Expected error for invalid data");

        let err = result.unwrap_err();
        assert!(
            matches!(err.kind(), ErrorKind::UnknownFormat),
            "Expected UnknownFormat, got {:?}",
            err.kind()
        );
    }

    #[test]
    fn test_from_reader_corrupted_header() {
        let bytes = corrupted_header_bytes(ImageFormat::Png);
        let cursor = Cursor::new(bytes);

        let result = ImageMetadata::from_reader(cursor);

        assert!(result.is_err(), "Expected error for corrupted header");
    }
}
