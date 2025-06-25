use crate::{
    constants::{DEFAULT_AVIF_QUALITY, DEFAULT_AVIF_SPEED},
    CompressionType, Image, ImageError, Result,
};

use {
    ravif::{EncodedImage, Encoder, Img, RGBA8},
    std::io::{BufWriter, Write},
};

impl Image {
    pub fn encode_avif(&mut self, writer: impl Write) -> Result<()> {
        if self.config.compression == CompressionType::Lossless {
            log::warn!("Lossless AVIF compression is not supported; falling back to lossy.");
        }

        let img_buffer = self.get_decoded()?.to_rgba8();
        let (width, height) = img_buffer.dimensions();

        let (quality, speed, alpha_quality) = if let Some(cfg) = &self.config.avif {
            (
                cfg.quality.clamp(1, 100) as f32,
                cfg.speed.clamp(1, 10),
                cfg.alpha_quality.clamp(1, 100) as f32,
            )
        } else {
            let fallback_quality = self
                .config
                .quality
                .map(|q| q.clamp(1, 100) as f32)
                .unwrap_or(DEFAULT_AVIF_QUALITY as f32);
            (fallback_quality, DEFAULT_AVIF_SPEED, fallback_quality)
        };

        let pixels: Vec<RGBA8> = img_buffer
            .pixels()
            .map(|p| RGBA8 {
                r: p[0],
                g: p[1],
                b: p[2],
                a: p[3],
            })
            .collect();

        let img_ref = Img::new(pixels.as_slice(), width as usize, height as usize);

        let encoder = Encoder::new()
            .with_quality(quality)
            .with_speed(speed)
            .with_alpha_quality(alpha_quality);

        let encoded: EncodedImage =
            encoder
                .encode_rgba(img_ref)
                .map_err(|e| ImageError::AvifEncoding {
                    err: e,
                    id: self.describe_source(),
                })?;

        let mut buf_writer = BufWriter::new(writer);
        buf_writer
            .write_all(&encoded.avif_file)
            .map_err(|e| ImageError::WriteAvif {
                source: e,
                id: self.describe_source(),
            })?;

        buf_writer.flush().map_err(|e| ImageError::WriteAvif {
            source: e,
            id: self.describe_source(),
        })?;

        Ok(())
    }
}
