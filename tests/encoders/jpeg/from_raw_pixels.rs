use {
    crate::helpers::get_test_image_paths,
    razr::{ImageError, JpegEncoder, ValidationError},
};

#[test]
fn test_jpeg_encoder_from_raw_pixels_to_bytes_success() {
    for path in get_test_image_paths() {
        let pixels = vec![255, 0, 128, 128, 255, 0, 128, 255, 0];
        let width = 3;
        let height = 1;

        let result = JpegEncoder::new().from_raw_pixels(&pixels, width, height).to_bytes();

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
fn test_progressive_jpeg_encoder_from_raw_pixels_to_bytes_success() {
    for path in get_test_image_paths() {
        let pixels = vec![255, 0, 128, 128, 255, 0, 128, 255, 0];
        let width = 3;
        let height = 1;

        let result = JpegEncoder::progressive().from_raw_pixels(&pixels, width, height).to_bytes();

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
fn test_jpeg_encoder_from_raw_pixels_write_to_success() {
    for path in get_test_image_paths() {
        let pixels = vec![255, 0, 128, 128, 255, 0, 128, 255, 0];
        let width = 3;
        let height = 1;

        let mut buffer = Vec::new();
        let result =
            JpegEncoder::new().from_raw_pixels(&pixels, width, height).write_to(&mut buffer);

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
fn test_progressive_jpeg_encoder_from_raw_pixels_write_to_success() {
    for path in get_test_image_paths() {
        let pixels = vec![255, 0, 128, 128, 255, 0, 128, 255, 0];
        let width = 3;
        let height = 1;

        let mut buffer = Vec::new();
        let result = JpegEncoder::progressive()
            .from_raw_pixels(&pixels, width, height)
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
fn test_jpeg_encoder_errors_if_empty_raw_pixels_array() {
    let pixels = Vec::new();
    let width = 3;
    let height = 1;

    let result = JpegEncoder::new().from_raw_pixels(&pixels, width, height).to_bytes();

    match result {
        Ok(_) => panic!("Should return validation error for empty raw pixel array"),
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
fn test_jpeg_encoder_errors_if_invalid_width() {
    let pixels = vec![255, 0, 128, 128, 255, 0, 128, 255, 0];
    let width = 0;
    let height = 1;

    let result = JpegEncoder::new().from_raw_pixels(&pixels, width, height).to_bytes();

    match result {
        Ok(_) => panic!("Should return validation error for invalid width"),
        Err(e) => {
            if let ImageError::Validation(ValidationError::InvalidWidth) = e {
                // Expected error
            } else {
                panic!("Expected validation error (InvalidWidth), but got: {:?}", e);
            }
        }
    }
}

#[test]
fn test_jpeg_encoder_errors_if_invalid_height() {
    let pixels = vec![255, 0, 128, 128, 255, 0, 128, 255, 0];
    let width = 1;
    let height = 0;

    let result = JpegEncoder::new().from_raw_pixels(&pixels, width, height).to_bytes();

    match result {
        Ok(_) => panic!("Should return validation error for invalid height"),
        Err(e) => {
            if let ImageError::Validation(ValidationError::InvalidHeight) = e {
                // Expected error
            } else {
                panic!("Expected validation error (InvalidHeight), but got: {:?}", e);
            }
        }
    }
}
