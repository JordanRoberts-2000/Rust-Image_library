use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};
use webp::{Encoder, PixelLayout};

use crate::{CompressionType, Image, ImageError, IoError, Result};

const DEFAULT_WEBP_QUALITY: f32 = 75.0;
const DEFAULT_WEBP_COMPRESSION: CompressionType = CompressionType::Lossy;

impl Image {
    pub fn encode_webp(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();

        let rgba_image = self.raw.to_rgba8();
        let (width, height) = rgba_image.dimensions();

        let webp_data = match self.config.compression {
            CompressionType::Lossless => self.encode_webp_lossless(&rgba_image, width, height)?,
            CompressionType::Lossy => {
                let quality = self
                    .config
                    .quality
                    .map(|q| (q.clamp(1, 100) as f32))
                    .unwrap_or(DEFAULT_WEBP_QUALITY);

                self.encode_webp_lossy(&rgba_image, width, height, quality)?
            }
        };

        let file = File::create(path).map_err(|e| IoError::CreateFile(e, path.to_path_buf()))?;
        let mut writer = BufWriter::new(file);

        writer
            .write_all(&webp_data)
            .map_err(|e| IoError::WriteFile(e, path.to_path_buf()))?;

        Ok(())
    }

    fn encode_webp_lossless(
        &self,
        rgba_image: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        width: u32,
        height: u32,
    ) -> Result<Vec<u8>> {
        let encoder = Encoder::new(rgba_image.as_raw(), PixelLayout::Rgba, width, height);

        let webp_data = encoder.encode_lossless();

        Ok(webp_data.to_vec())
    }

    fn encode_webp_lossy(
        &self,
        rgba_image: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        width: u32,
        height: u32,
        quality: f32,
    ) -> Result<Vec<u8>> {
        let encoder = Encoder::new(rgba_image.as_raw(), PixelLayout::Rgba, width, height);

        let webp_data =
            encoder
                .encode_simple(false, quality)
                .map_err(|e| ImageError::WebPEncoding {
                    err: e,
                    id: self.describe_source(),
                })?;

        Ok(webp_data.to_vec())
    }
}
