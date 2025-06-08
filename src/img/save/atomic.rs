use std::path::Path;

use crate::{Img, ImgError, IoError, Result};

impl Img {
    pub fn atomic_save(&self, output: &Path) -> Result<()> {
        let file_name = output
            .file_name()
            .ok_or_else(|| ImgError::MissingFileName(output.to_path_buf()))?;
        let temp_path = output.with_file_name(format!(".{}", file_name.to_string_lossy()));

        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| IoError::CreateDir(e, parent.to_path_buf()))?;
        }

        self.img.save(&temp_path).map_err(|e| ImgError::Save {
            source: e,
            output: temp_path.clone(),
        })?;

        std::fs::rename(&temp_path, &output)
            .map_err(|e| IoError::Rename(e, temp_path.clone(), output.to_path_buf()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_atomic_save_creates_file_successfully() {
        let bytes = fs::read("tests/assets/test.png").expect("test image should exist");
        let img = Img::from_bytes(bytes).expect("Image should be loaded");

        let output_dir = PathBuf::from("tests/temp_atomic");
        let output_file = output_dir.join("saved.png");

        if output_dir.exists() {
            fs::remove_dir_all(&output_dir).expect("Failed to clear test directory");
        }

        img.atomic_save(&output_file)
            .expect("atomic save should not error");

        assert!(
            output_file.exists(),
            "Target file should exist after atomic save"
        );

        let temp_file = output_dir.join(".saved.png");
        assert!(
            !temp_file.exists(),
            "Temporary file should be renamed and not left behind"
        );

        fs::remove_dir_all(&output_dir).expect("Failed to clear test directory");
    }

    #[test]
    fn test_atomic_save_with_missing_file_name_error() {
        let bytes = fs::read("tests/assets/test.png").expect("test image should exist");
        let img = Img::from_bytes(bytes).expect("Image should be loaded");

        // Use a path that has no file name — just a directory or empty string
        let invalid_path = PathBuf::from(".");

        let result = img.atomic_save(&invalid_path);

        assert!(
            matches!(result, Err(ImgError::MissingFileName(_))),
            "Should return MissingFileName error when given a path without a filename, got: {:?}",
            result
        );
    }
}
