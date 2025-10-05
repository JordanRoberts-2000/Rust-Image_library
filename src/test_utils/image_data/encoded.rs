use {
    crate::{test_utils::MOCK_IMAGE_DIMENSIONS, ImageFormat},
    std::{io::Cursor, path::Path},
};

pub fn encoded_bytes(format: ImageFormat) -> Vec<u8> {
    let (w, h) = MOCK_IMAGE_DIMENSIONS;
    let img = image::DynamicImage::new_rgb8(w, h);

    let mut buf = Vec::new();
    img.write_to(&mut Cursor::new(&mut buf), format.into())
        .unwrap_or_else(|e| panic!("Failed to create encoded bytes ({format:?}, {w}x{h}): {e}"));
    buf
}

pub fn unsupported_format_bytes() -> Vec<u8> {
    let unsupported_format = image::ImageFormat::Qoi;

    assert!(
        ImageFormat::try_from(unsupported_format).is_err(),
        "Test assumption invalid: {unsupported_format:?} is now supported by your ImageFormat::try_from. \
         Choose a different format for the 'unsupported' test."
    );

    let (w, h) = MOCK_IMAGE_DIMENSIONS;
    let img = image::DynamicImage::new_rgb8(w, h);

    let mut buf = Vec::new();
    img.write_to(&mut Cursor::new(&mut buf), unsupported_format).unwrap_or_else(|e| {
        panic!("Failed to create encoded bytes ({unsupported_format:?}, {w}x{h}): {e}")
    });
    buf
}

pub fn write_encoded_bytes(path: &Path, format: ImageFormat) {
    let (w, h) = MOCK_IMAGE_DIMENSIONS;
    image::DynamicImage::new_rgb8(w, h).save_with_format(path, format.into()).unwrap_or_else(|e| {
        panic!("Failed to save encoded bytes to '{}' ({format:?}, {w}x{h}): {e}", path.display())
    });
}
