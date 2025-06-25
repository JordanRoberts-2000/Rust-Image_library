use {
    std::io::{BufWriter, Write},
    webp::{Encoder, PixelLayout},
};

use crate::{constants::DEFAULT_WEBP_QUALITY, CompressionType, Image, ImageError, Result};

impl Image {
    pub fn encode_webp(&mut self, writer: impl Write) -> Result<()> {
        let rgba_image = &self.get_decoded()?.to_rgba8();
        let (width, height) = rgba_image.dimensions();

        let (lossless, quality) = if let Some(cfg) = &self.config.webp {
            (cfg.lossless, cfg.quality.clamp(1, 100) as f32)
        } else {
            match self.config.compression {
                CompressionType::Lossless => (true, 0.0),
                CompressionType::Lossy => (
                    false,
                    self.config
                        .quality
                        .map(|q| q.clamp(1, 100) as f32)
                        .unwrap_or(DEFAULT_WEBP_QUALITY as f32),
                ),
            }
        };

        let encoder = Encoder::new(rgba_image.as_raw(), PixelLayout::Rgba, width, height);
        let webp_data =
            encoder
                .encode_simple(lossless, quality)
                .map_err(|e| ImageError::WebPEncoding {
                    err: e,
                    id: self.describe_source(),
                })?;

        let mut buf_writer = BufWriter::new(writer);
        buf_writer
            .write_all(&webp_data)
            .map_err(|e| ImageError::WriteWebP {
                source: e,
                id: self.describe_source(),
            })?;

        Ok(())
    }
}
