use {
    crate::{ErrorKind, ImageFormat, Result},
    image::{load_from_memory_with_format, DynamicImage},
};

pub fn from_bytes_with_format(
    bytes: impl AsRef<[u8]>, format: ImageFormat,
) -> Result<DynamicImage> {
    load_from_memory_with_format(bytes.as_ref(), format.into())
        .map_err(|e| ErrorKind::DecodeFromMemory(e).into())
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
    fn from_bytes_with_format_ok_for_all_supported() {
        for fmt in ImageFormat::iter() {
            let bytes = encoded_bytes(fmt);

            let img = from_bytes_with_format(&bytes, fmt)
                .unwrap_or_else(|e| panic!("from_bytes_with_format failed for {fmt:?}: {e}"));

            assert_eq!(img.dimensions(), MOCK_IMAGE_DIMENSIONS, "wrong dimensions for {fmt:?}");
        }
    }

    #[test]
    fn from_bytes_with_format_empty_err() {
        let err = from_bytes_with_format(&[], ImageFormat::Png)
            .expect_err("empty bytes should fail to decode");

        assert!(
            matches!(err.kind(), ErrorKind::DecodeFromMemory(_)),
            "expected DecodeFromMemory(_), got {:?}",
            err.kind()
        );
    }

    #[test]
    fn from_corrupted_bytes_with_format_corrupted_err() {
        let bytes = corrupted_bytes(ImageFormat::Png);

        let err = from_bytes_with_format(&bytes, ImageFormat::Png)
            .expect_err("corrupted PNG bytes should fail to decode");

        assert!(
            matches!(err.kind(), ErrorKind::DecodeFromMemory(_)),
            "expected DecodeFromMemory(_), got {:?}",
            err.kind()
        );
    }

    #[test]
    fn from_corrupted_header_bytes_with_format_err() {
        let bytes = corrupted_header_bytes(ImageFormat::Png);

        let err = from_bytes_with_format(&bytes, ImageFormat::Png)
            .expect_err("corrupted PNG bytes should fail to decode");

        assert!(
            matches!(err.kind(), ErrorKind::DecodeFromMemory(_)),
            "expected DecodeFromMemory(_), got {:?}",
            err.kind()
        );
    }

    #[test]
    fn from_bytes_with_format_mismatch_err() {
        let png_bytes = encoded_bytes(ImageFormat::Png);

        let err = from_bytes_with_format(&png_bytes, ImageFormat::Jpeg)
            .expect_err("PNG bytes forced as JPEG should fail");

        assert!(
            matches!(err.kind(), ErrorKind::DecodeFromMemory(_)),
            "expected DecodeFromMemory(_) on format mismatch, got {:?}",
            err.kind()
        );
    }
}
