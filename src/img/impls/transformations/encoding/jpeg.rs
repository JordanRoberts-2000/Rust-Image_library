use crate::{ImageFormat, Img, ImgError, Result};

use image::{codecs::jpeg::JpegEncoder, load_from_memory, ExtendedColorType};

impl Img {
    pub fn jpeg(&mut self, quality: u8) -> Result<&mut Self> {
        if self.format == ImageFormat::Jpeg && quality == 100 {
            return Ok(self);
        }

        let mut buffer = Vec::new();
        let mut encoder = JpegEncoder::new_with_quality(&mut buffer, quality);

        encoder
            .encode(
                self.img.to_rgb8().as_raw(),
                self.width,
                self.height,
                ExtendedColorType::Rgb8,
            )
            .map_err(|e| ImgError::Conversion {
                source: e,
                id: self.describe_source(),
                format: ImageFormat::Jpeg,
            })?;

        // Note: This decoding will re-read the JPEG data, so any JPEG-specific encoding
        // (like progressive settings) will be lost.
        self.img = load_from_memory(&buffer).map_err(|e| ImgError::Decoding {
            id: self.describe_source(),
            source: e,
            format: ImageFormat::Jpeg,
        })?;

        self.format = ImageFormat::Jpeg;

        Ok(self)
    }
}
