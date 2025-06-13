use std::path::Path;

use crate::{Img, ImgError};

impl Img {
    pub fn with_file_name<T: AsRef<Path>>(&mut self, file_name: T) -> Result<&mut Self, ImgError> {
        let new_file_name = file_name.as_ref();
        let parent = self.target_path.parent().unwrap_or_else(|| Path::new("."));

        let ext = self
            .format
            .extensions_str()
            .first()
            .ok_or(ImgError::ExtensionInvalid)?;

        self.target_path = parent.join(new_file_name).with_extension(ext);

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_img_with_file_name_updates_target_path() {
        let bytes = fs::read("tests/assets/test.png").expect("test image should exist");
        let mut img = Img::from_bytes(bytes).expect("Image should be loaded from bytes");

        let old_parent = img.target_path.parent().unwrap().to_path_buf();
        let new_name = "renamed-image.webp"; // should be corrected to .png

        img.with_file_name(new_name)
            .expect("with_file_name should succeed");

        let expected_ext = img.format.extensions_str()[0];
        let expected_file_name = format!(
            "{}.{}",
            Path::new(new_name).file_stem().unwrap().to_string_lossy(),
            expected_ext
        );

        let expected_path = old_parent.join(&expected_file_name);

        assert_eq!(
            img.target_path, expected_path,
            "Target path should be updated with correct extension"
        );

        assert_eq!(
            img.target_path.file_name().unwrap().to_string_lossy(),
            expected_file_name,
            "File name should be updated correctly"
        );
    }
}
