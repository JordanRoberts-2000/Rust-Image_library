use webp::{Encoder, PixelLayout};

use crate::{ImageFormat, Img, ImgError, Result};

impl Img {
    pub fn webp(&mut self) -> Result<&mut Self> {
        if self.format == ImageFormat::WebP {
            return Ok(self);
        }

        let rgba_image = self.img.to_rgba8();
        let encoder = Encoder::new(&rgba_image, PixelLayout::Rgba, self.width, self.height);

        let webp_data = encoder.encode_lossless();

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
