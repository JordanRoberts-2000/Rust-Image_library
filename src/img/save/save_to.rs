use std::path::Path;

use crate::{Img, Result};

impl Img {
    pub fn save_to<P: AsRef<Path>>(&self, folder_path: P) -> Result<()> {
        let folder_path = folder_path.as_ref();

        let file_name = self.file_name()?;

        let stem = Path::new(&file_name)
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy();

        let ext = self.extention()?;

        let mut count = 0;
        let mut output_path;

        loop {
            let candidate_name = if count == 0 {
                format!("{}.{}", stem, ext)
            } else {
                format!("{}({}).{}", stem, count, ext)
            };

            output_path = folder_path.join(&candidate_name);

            if !output_path.exists() {
                break;
            }

            count += 1;
        }

        self.atomic_save(&output_path)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_img_save_to_generates_unique_filenames() {
        // Arrange
        let bytes = fs::read("tests/assets/test.png").expect("test image should exist");
        let img = Img::from_bytes(bytes).expect("Image should load from bytes");

        // Create a temporary directory for output
        let output_dir = PathBuf::from("tests/tmp_save_to");

        if output_dir.exists() {
            fs::remove_dir_all(&output_dir).expect("Failed to clear test output folder");
        }
        fs::create_dir_all(&output_dir).expect("Failed to create test output folder");

        // Save the same image 3 times
        img.save_to(&output_dir).expect("1st save should succeed");
        img.save_to(&output_dir).expect("2nd save should succeed");
        img.save_to(&output_dir).expect("3rd save should succeed");

        // Get expected filenames
        let ext = img.extention().unwrap();
        let file_name_1 = img.file_name().unwrap();
        let stem = Path::new(&file_name_1)
            .file_stem()
            .unwrap()
            .to_string_lossy();

        let path_1 = output_dir.join(&file_name_1);
        let path_2 = output_dir.join(format!("{}(1).{}", stem, ext));
        let path_3 = output_dir.join(format!("{}(2).{}", stem, ext));

        // Assert files exist
        assert!(path_1.exists(), "Expected first image to be saved");
        assert!(
            path_2.exists(),
            "Expected second image to be saved with (1)"
        );
        assert!(path_3.exists(), "Expected third image to be saved with (2)");

        // cleanup
        fs::remove_dir_all(&output_dir).expect("Failed to clear test output folder");
    }
}
