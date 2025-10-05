use {
    crate::{ErrorKind, Result},
    image::{load_from_memory, DynamicImage},
};

pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<DynamicImage> {
    load_from_memory(bytes.as_ref()).map_err(|e| ErrorKind::DecodeFromMemory(e).into())
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
        for format in ImageFormat::iter() {
            let bytes = encoded_bytes(format);

            let img = from_bytes(&bytes)
                .unwrap_or_else(|e| panic!("from_bytes failed for {format:?}: {e}"));

            assert_eq!(img.dimensions(), MOCK_IMAGE_DIMENSIONS, "wrong dimensions for {format:?}");
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
