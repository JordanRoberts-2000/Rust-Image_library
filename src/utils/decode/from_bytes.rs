use {
    crate::{ErrorKind, ImageFormat, Result},
    image::{DynamicImage, ImageReader},
    std::io::Cursor,
};

pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<(DynamicImage, ImageFormat)> {
    let b = bytes.as_ref();

    let reader = ImageReader::new(Cursor::new(b))
        .with_guessed_format()
        .map_err(ErrorKind::FormatDetectionFailed)?;

    let ext_fmt = reader.format().ok_or(ErrorKind::UnknownFormat)?;
    let format = ImageFormat::try_from(ext_fmt)?;

    let img = reader.decode().map_err(ErrorKind::DecodeFromMemory)?;

    Ok((img, format))
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            test_utils::{
                corrupted_bytes, corrupted_header_bytes, encoded_bytes, MOCK_IMAGE_DIMENSIONS,
            },
            ErrorKind, ImageFormat,
        },
        image::GenericImageView,
        strum::IntoEnumIterator,
    };

    #[test]
    fn from_bytes_all_supported_formats_ok() {
        for fmt in ImageFormat::iter() {
            let bytes = encoded_bytes(fmt);

            let (img, returned_fmt) =
                from_bytes(&bytes).unwrap_or_else(|e| panic!("from_bytes failed for {fmt:?}: {e}"));

            assert_eq!(img.dimensions(), MOCK_IMAGE_DIMENSIONS, "wrong dimensions for {fmt:?}");
            assert_eq!(fmt, returned_fmt, "wrong format for {fmt:?}");
        }
    }

    #[test]
    fn from_bytes_empty_err() {
        let err = from_bytes(&[]).expect_err("empty bytes should fail to decode");

        match err.kind() {
            ErrorKind::DecodeFromMemory(_inner) => {}
            other => panic!("expected DecodeFromMemory(_), got {:?}", other),
        }
    }

    #[test]
    fn from_corrupted_header_bytes_err() {
        let bytes = corrupted_header_bytes(ImageFormat::Png);

        let err = from_bytes(&bytes).expect_err("corrupted bytes should fail to decode");

        assert!(
            matches!(err.kind(), ErrorKind::DecodeFromMemory(_)),
            "expected DecodeFromMemory(_), got {:?}",
            err.kind()
        );
    }

    #[test]
    fn from_corrupted_bytes_err() {
        let bytes = corrupted_bytes(ImageFormat::Png);

        let err = from_bytes(&bytes).expect_err("corrupted bytes should fail to decode");

        assert!(
            matches!(err.kind(), ErrorKind::DecodeFromMemory(_)),
            "expected DecodeFromMemory(_), got {:?}",
            err.kind()
        );
    }
}
