use std::path::Path;

use crate::{Image, ImageError, ImageFormat, Result};

impl Image {
    pub fn save_as(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let mut path = path.as_ref().to_path_buf();

        if path.extension().is_none() {
            let ext = self.format.extention();
            path.set_extension(ext);
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| ImageError::ExtensionMissing(path.to_path_buf()))?;

        let format = ImageFormat::try_from(ext)
            .map_err(|_| ImageError::InvalidExtension(ext.to_string()))?;

        let original_format = self.format;
        self.format = format;

        let result = (|| {
            self.apply_transforms()?;
            self.atomic_save(&path)?;
            Ok(())
        })();

        self.format = original_format;
        result
    }
}
