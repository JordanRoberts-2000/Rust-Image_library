use {
    crate::helpers::get_test_image_paths,
    razr::{ImageError, JpegEncoder, ValidationError},
};

#[test]
fn test_jpeg_encoder_from_file_to_bytes_success() {
    for path in get_test_image_paths() {
        let result = JpegEncoder::new().from_file(&path).to_bytes();

        match result {
            Ok(bytes) => assert!(
                !bytes.is_empty(),
                "The bytes should not be empty for file: {}",
                path.display()
            ),
            Err(e) => panic!("Test failed with error for file {}: {:?}", path.display(), e),
        }
    }
}

#[test]
fn test_progressive_jpeg_encoder_from_file_to_bytes_success() {
    for path in get_test_image_paths() {
        let result = JpegEncoder::progressive().from_file(&path).to_bytes();

        match result {
            Ok(bytes) => assert!(
                !bytes.is_empty(),
                "The bytes should not be empty for file: {}",
                path.display()
            ),
            Err(e) => panic!("Test failed with error for file {}: {:?}", path.display(), e),
        }
    }
}

#[test]
fn test_jpeg_encoder_from_file_write_to_success() {
    for path in get_test_image_paths() {
        let mut buffer = Vec::new();
        let result = JpegEncoder::new().from_file(&path).write_to(&mut buffer);

        if let Err(e) = result {
            panic!("Test failed with error for file {}: {:?}", path.display(), e);
        }

        assert!(!buffer.is_empty(), "The buffer should not be empty for file: {}", path.display());
    }
}

#[test]
fn test_progressive_jpeg_encoder_from_file_write_to_success() {
    for path in get_test_image_paths() {
        let mut buffer = Vec::new();
        let result = JpegEncoder::progressive().from_file(&path).write_to(&mut buffer);

        if let Err(e) = result {
            panic!("Test failed with error for file {}: {:?}", path.display(), e);
        }

        assert!(!buffer.is_empty(), "The buffer should not be empty for file: {}", path.display());
    }
}

#[test]
fn test_jpeg_encoder_errors_if_empty_file() {
    let path = "tests/assets/non-existing.jpeg";
    let result = JpegEncoder::new().from_file(path).to_bytes();

    match result {
        Ok(_) => panic!("Should return validation error for non-existing file"),
        Err(e) => {
            if let ImageError::Validation(ValidationError::PathNotFound(_)) = e {
                // Expected error
            } else {
                panic!("Expected validation error (PathNotFound), but got: {:?}", e);
            }
        }
    }
}
