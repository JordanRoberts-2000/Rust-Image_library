use crate::{blocking::Image, ImageError, Result};

impl Image {
    pub fn blurhash(&mut self) -> Result<String> {
        self.apply_transforms()?;

        let pixels = self.get_decoded()?.to_rgba8().into_vec();
        blurhash::encode(4, 3, self.width(), self.height(), &pixels).map_err(ImageError::BlurHash)
    }
}
