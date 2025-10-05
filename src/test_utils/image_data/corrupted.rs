use {
    crate::{test_utils::encoded_bytes, ImageFormat},
    std::{fs, path::PathBuf},
    tempfile::TempDir,
};

pub fn corrupted_bytes(format: ImageFormat) -> Vec<u8> {
    let mut bytes = encoded_bytes(format);

    // Can't corrupt empty or single-byte images
    assert!(bytes.len() >= 2, "Image too small to corrupt: {} bytes", bytes.len());

    // Corrupt the second half, preserving format headers in first half
    let midpoint = bytes.len() / 2;

    for byte in &mut bytes[midpoint..] {
        *byte = 0xFF;
    }

    bytes
}

pub fn corrupted_header_bytes(format: ImageFormat) -> Vec<u8> {
    let mut bytes = encoded_bytes(format);

    assert!(bytes.len() >= 20, "Image too small to corrupt header: {} bytes", bytes.len());

    // Corrupt bytes 8-48 (after most format signatures, covers dimension area)
    let corrupt_start = 8;
    let corrupt_end = 48.min(bytes.len());

    for byte in &mut bytes[corrupt_start..corrupt_end] {
        *byte = 0xFF;
    }

    bytes
}

pub fn corrupted_image_file(temp_dir: &TempDir, format: ImageFormat) -> PathBuf {
    let file_path = temp_dir.path().join(format!("corrupted.{}", format.extension()));

    let bytes = corrupted_bytes(format);
    fs::write(&file_path, bytes).unwrap_or_else(|e| {
        panic!(
            "Failed to write corrupted image file to '{}' ({format:?}): {e}",
            file_path.display()
        )
    });

    file_path
}

pub fn corrupted_header_image_file(temp_dir: &TempDir, format: ImageFormat) -> PathBuf {
    let file_path = temp_dir.path().join(format!("corrupted.{}", format.extension()));

    let bytes = corrupted_header_bytes(format);
    fs::write(&file_path, bytes).unwrap_or_else(|e| {
        panic!(
            "Failed to write corrupted image file to '{}' ({format:?}): {e}",
            file_path.display()
        )
    });

    file_path
}
