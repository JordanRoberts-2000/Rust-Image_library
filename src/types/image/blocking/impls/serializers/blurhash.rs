use crate::{blocking::Image, ImageError, Result};

impl Image {
    pub fn blurhash(&mut self) -> Result<String> {
        let width = self.width();
        let height = self.height();

        let img = self.process_image()?;

        blurhash::encode(4, 3, width, height, img.as_ref().as_bytes()).map_err(ImageError::BlurHash)
    }
}
