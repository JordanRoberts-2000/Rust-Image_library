use crate::{Image, ImageError, Result};

use blurhash::encode;

impl Image {
    pub fn blurhash(&mut self) -> Result<String> {
        let pixels = self.get_decoded()?.to_rgba8().into_vec();
        let hash = encode(4, 3, self.width, self.height, &pixels).map_err(ImageError::BlurHash)?;
        Ok(hash)
    }
}
