use std::io::Write;

use crate::{Image, ImageFormat, Result};

impl Image {
    pub fn encode(&mut self, writer: impl Write) -> Result<()> {
        match self.format {
            ImageFormat::Jpeg => self.encode_jpeg(writer),
            ImageFormat::Png => self.encode_png(writer),
            ImageFormat::WebP => self.encode_webp(writer),
            ImageFormat::Avif => self.encode_avif(writer),
        }?;

        Ok(())
    }
}
