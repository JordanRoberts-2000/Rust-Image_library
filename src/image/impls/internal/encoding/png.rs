use std::path::Path;

use crate::{Image, ImageError, Result};

impl Image {
    pub fn encode_png(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();

        self.raw
            .save_with_format(path, image::ImageFormat::Png)
            .map_err(|e| ImageError::Save {
                source: e,
                path: path.to_path_buf(),
            })?;

        Ok(())
    }
}
