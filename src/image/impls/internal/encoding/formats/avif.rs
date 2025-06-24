use crate::{Image, ImageError, Result};

use {
    ravif::{EncodedImage, Encoder, Img, RGBA8},
    std::io::{BufWriter, Write},
};

impl Image {
    pub fn encode_avif(&mut self, writer: impl Write) -> Result<()> {
        let img_buffer = self.get_decoded()?.to_rgba8();
        let (width, height) = img_buffer.dimensions();

        let quality = self
            .config
            .quality
            .map(|q| q.clamp(1, 100) as f32)
            .unwrap_or(75.0);

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
            .with_speed(6)
            .with_alpha_quality(quality);

        let EncodedImage { avif_file, .. } =
            encoder
                .encode_rgba(img_ref)
                .map_err(|e| ImageError::AvifEncoding {
                    err: e,
                    id: self.describe_source(),
                })?;

        let mut buf_writer = BufWriter::new(writer);
        buf_writer
            .write_all(&avif_file)
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
