use crate::{Img, ImgError, Result};

use blurhash::encode;

impl Img {
    pub fn blurhash(&mut self) -> Result<String> {
        let pixels = self.img.to_rgba8().into_vec();
        let hash = encode(4, 3, self.width, self.height, &pixels).map_err(ImgError::BlurHash)?;
        Ok(hash)
    }
}
