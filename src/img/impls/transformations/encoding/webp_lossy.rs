use webp::{Encoder, PixelLayout};

use crate::{ImageFormat, Img, ImgError, Result};

impl Img {
    pub fn webp_lossy(&mut self, quality: u8) -> Result<&mut Self> {
        if self.format == ImageFormat::WebP && quality == 100 {
            return Ok(self);
        }

        let rgba_image = self.img.to_rgba8();
        let encoder = Encoder::new(&rgba_image, PixelLayout::Rgba, self.width, self.height);

        let webp_data =
            encoder
                .encode_simple(false, quality as f32)
                .map_err(|e| ImgError::WebPConversion {
                    err: e,
                    id: self.describe_source(),
                })?;

        self.img = image::load_from_memory_with_format(&webp_data, (ImageFormat::WebP).into())
            .map_err(|e| ImgError::Decoding {
                id: self.describe_source(),
                source: e,
                format: ImageFormat::WebP,
            })?;

        self.format = ImageFormat::WebP;

        Ok(self)
    }
}
