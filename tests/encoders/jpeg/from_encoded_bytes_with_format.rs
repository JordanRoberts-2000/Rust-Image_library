use {
    crate::helpers::{get_test_image_paths, load_image_as_bytes},
    razr::{ImageError, ImageFormat, JpegEncoder, ValidationError},
};

#[test]
fn test_jpeg_encoder_from_encoded_bytes_with_format_to_bytes_success() {
    for path in get_test_image_paths() {
        let bytes = load_image_as_bytes(&path);

        let format = ImageFormat::try_from_path(&path)
            .expect(&format!("could not get ImageFormat from extension: {}", path.display()));

        let result = JpegEncoder::new().from_encoded_bytes_with_format(&bytes, format).to_bytes();

        match result {
            Ok(bytes) => assert!(
                !bytes.is_empty(),
                "The result bytes are empty for image: {}",
                path.display()
            ),
            Err(e) => panic!("Test failed for image {} with error: {:?}", path.display(), e),
        }
    }
}

#[test]
fn test_progressive_jpeg_encoder_from_encoded_bytes_with_format_to_bytes_success() {
    for path in get_test_image_paths() {
        let bytes = load_image_as_bytes(&path);

        let format = ImageFormat::try_from_path(&path)
            .expect(&format!("could not get ImageFormat from extension: {}", path.display()));

        let result =
            JpegEncoder::progressive().from_encoded_bytes_with_format(&bytes, format).to_bytes();

        match result {
            Ok(bytes) => assert!(
                !bytes.is_empty(),
                "The result bytes are empty for progressive image: {}",
                path.display()
            ),
            Err(e) => {
                panic!("Test failed for progressive image {} with error: {:?}", path.display(), e)
            }
        }
    }
}

#[test]
fn test_jpeg_encoder_from_encoded_bytes_with_format_write_to_success() {
    for path in get_test_image_paths() {
        let bytes = load_image_as_bytes(&path);

        let format = ImageFormat::try_from_path(&path)
            .expect(&format!("could not get ImageFormat from extension: {}", path.display()));

        let mut buffer = Vec::new();
        let result =
            JpegEncoder::new().from_encoded_bytes_with_format(&bytes, format).write_to(&mut buffer);

        if let Err(e) = result {
            panic!("Test failed for image {} with error during write: {:?}", path.display(), e);
        }

        assert!(
            !buffer.is_empty(),
            "The buffer is empty for image {} after encoding.",
            path.display()
        );
    }
}

#[test]
fn test_progressive_jpeg_encoder_from_encoded_bytes_with_format_write_to_success() {
    for path in get_test_image_paths() {
        let bytes = load_image_as_bytes(&path);

        let format = ImageFormat::try_from_path(&path)
            .expect(&format!("could not get ImageFormat from extension: {}", path.display()));

        let mut buffer = Vec::new();
        let result = JpegEncoder::progressive()
            .from_encoded_bytes_with_format(&bytes, format)
            .write_to(&mut buffer);

        if let Err(e) = result {
            panic!(
                "Test failed for progressive image {} with error during write: {:?}",
                path.display(),
                e
            );
        }

        assert!(
            !buffer.is_empty(),
            "The buffer is empty for progressive image {} after encoding.",
            path.display()
        );
    }
}

#[test]
fn test_jpeg_encoder_errors_if_empty_bytes_array_with_format() {
    let bytes = Vec::new();
    let format = ImageFormat::Jpeg;

    let result = JpegEncoder::new().from_encoded_bytes_with_format(&bytes, format).to_bytes();

    match result {
        Ok(_) => panic!("Should return validation error for empty byte array"),
        Err(e) => {
            if let ImageError::Validation(ValidationError::EmptyByteArray) = e {
                // Expected error
            } else {
                panic!("Expected validation error (EmptyByteArray), but got: {:?}", e);
            }
        }
    }
}

#[test]
fn test_jpeg_encoder_errors_if_format_mismatch() {
    let paths = get_test_image_paths();
    let path = paths.get(0).expect("No images in 'get_test_image_paths' helper");
    let bytes = load_image_as_bytes(&path);

    let actual_format = ImageFormat::try_from_path(&path)
        .expect(&format!("could not get ImageFormat from extension: {}", path.display()));

    // Ensure the format is different from the actual format (force mismatch)
    let mismatch_format =
        if actual_format == ImageFormat::Jpeg { ImageFormat::Png } else { ImageFormat::Jpeg };

    let result =
        JpegEncoder::new().from_encoded_bytes_with_format(&bytes, mismatch_format).to_bytes();

    match result {
        Ok(_) => panic!("Should return format mismatch error"),
        Err(e) => {
            if let ImageError::Validation(ValidationError::FormatMismatch { .. }) = e {
                // Expected format mismatch error
            } else {
                panic!("Expected validation error (FormatMismatch), but got: {:?}", e);
            }
        }
    }
}
