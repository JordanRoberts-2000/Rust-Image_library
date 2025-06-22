use crate::{Image, ImageError, ImageFormat, Result};

use {
    image::{codecs::jpeg::JpegEncoder, ExtendedColorType},
    std::io::{BufWriter, Write},
};

impl Image {
    pub fn encode_jpeg(&mut self, writer: impl Write) -> Result<()> {
        let rgb8 = &self.get_decoded()?.to_rgb8();
        let (width, height) = rgb8.dimensions();

        let mut buf_writer = BufWriter::new(writer);

        let mut encoder = match self.config.quality {
            None => JpegEncoder::new(&mut buf_writer),
            Some(q) => JpegEncoder::new_with_quality(&mut buf_writer, q as u8),
        };

        encoder
            .encode(rgb8.as_raw(), width, height, ExtendedColorType::Rgb8)
            .map_err(|e| ImageError::Encoding {
                source: e,
                id: self.describe_source(),
                format: ImageFormat::Jpeg,
            })?;

        Ok(())
    }
}
