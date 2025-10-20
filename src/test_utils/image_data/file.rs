use {
    crate::{test_utils::write_encoded_bytes, ImageFormat},
    std::path::PathBuf,
    tempfile::TempDir,
};

pub fn image_file(temp_dir: &TempDir, format: ImageFormat) -> PathBuf {
    let file_path = temp_dir.path().join(format!("test.{}", format.primary_extension()));

    write_encoded_bytes(&file_path, format);

    file_path
}
