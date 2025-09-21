use crate::{Image, ImageError, Result};

impl Image {
    pub fn blurhash(&self) -> Result<String> {
        let (w, h) = self.dimensions();
        let img = self.processed_image()?;

        blurhash::encode(4, 3, w, h, img.to_rgba8().as_raw()).map_err(ImageError::BlurHash)
    }
}
