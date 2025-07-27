use {crate::helpers::get_test_image_paths, razr::JpegEncoder};

#[test]
fn test_jpeg_encoder_from_file_to_bytes_success() {
    let paths = get_test_image_paths();
    let path = paths.get(0).expect("No images in 'get_test_image_paths' helper");

    let result = JpegEncoder::new().from_file(&path).to_bytes();

    let bytes = match result {
        Ok(bytes) => bytes,
        Err(e) => panic!("Encoding failed. path: '{}', error: '{}'", path.display(), e),
    };

    let result2 = JpegEncoder::new().with_quality(10).from_file(&path).to_bytes();

    let bytes_with_lower_quality = match result2 {
        Ok(bytes) => bytes,
        Err(e) => panic!("Encoding failed. path: '{}', error: '{}'", path.display(), e),
    };

    assert!(bytes.len() > bytes_with_lower_quality.len(), "Quality should have decreased size")
}
