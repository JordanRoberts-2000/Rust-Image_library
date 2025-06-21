use std::path::Path;

use crate::{Image, ImageError, Result};

impl Image {
    pub fn encode_png(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();

        self.get_decoded()?
            .save_with_format(path, image::ImageFormat::Png)
            .map_err(|e| ImageError::Save {
                source: e,
                path: path.to_path_buf(),
            })?;

        Ok(())
    }
}
