use crate::{enums::ImgSrc, utils, Img, Result};

impl Img {
    pub fn save(&self) -> Result<()> {
        if let ImgSrc::Local { path } = &self.src {
            if *path != self.target_path {
                utils::fs::trash_file(&path)?;
            }
        }

        self.atomic_save(&self.target_path)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_img_save_replaces_original_file() {
        let output_dir = PathBuf::from("tests/tmp_save");
        let source_path = output_dir.join("input.png");
        let target_path = output_dir.join("output.png");

        // Setup clean state
        let _ = fs::remove_dir_all(&output_dir);
        fs::create_dir_all(&output_dir).unwrap();
        fs::copy("tests/assets/test.png", &source_path).unwrap();

        // Load and redirect
        let mut img = Img::open(&source_path).unwrap();
        img.target_path = target_path.clone();

        img.save().expect("Save should succeed");

        assert!(
            !source_path.exists(),
            "Original file should be deleted when target differs"
        );

        assert!(target_path.exists(), "Image should be saved to target path");

        let _ = fs::remove_dir_all(&output_dir);
    }

    #[test]
    fn test_img_save_preserves_original_file_if_same_path() {
        let output_dir = PathBuf::from("tests/tmp_save_same");
        let source_path = output_dir.join("same.png");

        // Setup clean state
        let _ = fs::remove_dir_all(&output_dir);
        fs::create_dir_all(&output_dir).unwrap();
        fs::copy("tests/assets/test.png", &source_path).unwrap();

        let img = Img::open(&source_path).unwrap();
        let metadata_before = fs::metadata(&source_path).unwrap();
        let modified_time_before = metadata_before.modified().unwrap();

        // Save to the same path (should not delete)
        img.save().expect("Save should succeed");

        // File should still exist
        assert!(
            source_path.exists(),
            "File should still exist if path matches"
        );

        let metadata_after = fs::metadata(&source_path).unwrap();
        let modified_time_after = metadata_after.modified().unwrap();

        assert!(
            modified_time_after >= modified_time_before,
            "File should be updated or at least still present"
        );

        let _ = fs::remove_dir_all(&output_dir);
    }
}
