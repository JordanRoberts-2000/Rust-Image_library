use {crate::helpers::get_test_image_paths, razr::JpegEncoder, std::fs::File};

#[test]
fn test_jpeg_encoder_from_encoded_reader_to_bytes_success() {
    for path in get_test_image_paths() {
        let file = File::open(&path).expect(&format!("Failed to open file: {}", path.display()));

        let result = JpegEncoder::new().from_encoded_reader(&file).to_bytes();

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
fn test_progressive_jpeg_encoder_from_encoded_reader_to_bytes_success() {
    for path in get_test_image_paths() {
        let file = File::open(&path).expect(&format!("Failed to open file: {}", path.display()));

        let result = JpegEncoder::progressive().from_encoded_reader(&file).to_bytes();

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
fn test_jpeg_encoder_from_encoded_reader_write_to_success() {
    for path in get_test_image_paths() {
        let file = File::open(&path).expect(&format!("Failed to open file: {}", path.display()));

        let mut buffer = Vec::new();
        let result = JpegEncoder::new().from_encoded_reader(&file).write_to(&mut buffer);

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
fn test_progressive_jpeg_encoder_from_encoded_reader_write_to_success() {
    for path in get_test_image_paths() {
        let file = File::open(&path).expect(&format!("Failed to open file: {}", path.display()));

        let mut buffer = Vec::new();
        let result = JpegEncoder::progressive().from_encoded_reader(&file).write_to(&mut buffer);

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
