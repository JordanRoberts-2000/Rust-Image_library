use {
    crate::{ErrorKind, ImageMetadata, Result},
    fs_ext::file,
    image::ImageReader,
    std::path::Path,
};

impl ImageMetadata {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        file::assert_exists(&path)?;

        let reader = ImageReader::open(&path)
            .map_err(|e| ErrorKind::Open { source: e, path: path.as_ref().to_path_buf() })?; // auto-detects format

        Self::from_image_reader(reader)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            test_utils::{
                corrupted_image_file, image_file, write_encoded_bytes, MOCK_IMAGE_DIMENSIONS,
            },
            ImageFormat,
        },
        std::fs,
        strum::IntoEnumIterator,
        tempfile::TempDir,
    };

    #[test]
    fn test_from_path_all_formats() {
        let temp_dir = TempDir::new().unwrap();

        for format in ImageFormat::iter() {
            let path = image_file(&temp_dir, format);

            let metadata = ImageMetadata::from_path(&path)
                .unwrap_or_else(|e| panic!("from_path failed for {format:?}: {e}"));

            assert_eq!(metadata.format, format);
            assert_eq!(metadata.width(), MOCK_IMAGE_DIMENSIONS.0);
            assert_eq!(metadata.height(), MOCK_IMAGE_DIMENSIONS.1);
        }
    }

    #[test]
    fn test_from_path_nonexistent_file() {
        let path = Path::new("/nonexistent/path/image.png");
        let result = ImageMetadata::from_path(path);

        assert!(result.is_err(), "Expected error for nonexistent file");

        let err = result.unwrap_err();
        assert!(
            matches!(err.kind(), ErrorKind::Io { .. }),
            "Expected Io error, got {:?}",
            err.kind()
        );
    }

    #[test]
    fn test_from_path_directory_not_file() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        let result = ImageMetadata::from_path(dir_path);

        assert!(result.is_err(), "Expected error for directory path");

        let err = result.unwrap_err();
        assert!(
            matches!(err.kind(), ErrorKind::Io { .. }),
            "Expected Io error, got {:?}",
            err.kind()
        );
    }

    #[test]
    fn test_from_path_corrupted_file() {
        let temp_dir = TempDir::new().unwrap();
        let path = corrupted_image_file(&temp_dir, ImageFormat::Png);

        let result = ImageMetadata::from_path(&path);

        assert!(result.is_err(), "Expected error for corrupted file");
    }

    #[test]
    fn test_from_path_empty_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("empty.png");

        fs::File::create(&file_path).unwrap(); // Create empty file

        let result = ImageMetadata::from_path(&file_path);

        assert!(result.is_err(), "Expected error for empty file");
    }

    #[test]
    fn test_from_path_invalid_extension() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("image.txt");

        write_encoded_bytes(&file_path, ImageFormat::Png);

        let result = ImageMetadata::from_path(&file_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_path_no_extension() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("image_no_ext");

        write_encoded_bytes(&file_path, ImageFormat::Jpeg);

        let result = ImageMetadata::from_path(&file_path);
        assert!(result.is_err());
    }
}
