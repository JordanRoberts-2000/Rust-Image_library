use crate::{blocking::Image, constants::DEFAULT_JPEG_QUALITY, ImageError, ImageFormat, Result};

use {
    image::{codecs::jpeg::JpegEncoder, ExtendedColorType},
    std::io::{BufWriter, Write},
};

impl Image {
    pub fn encode_jpeg(&mut self, writer: impl Write) -> Result<()> {
        #[cfg(feature = "progressive-jpeg")]
        {
            if self
                .config
                .jpeg
                .as_ref()
                .map_or(false, |cfg| cfg.progressive)
            {
                return self.encode_progressive_jpeg(writer);
            }
        }

        let rgb8 = &self.get_decoded()?.to_rgb8();
        let (width, height) = rgb8.dimensions();

        let quality = self
            .config
            .jpeg
            .as_ref()
            .map(|cfg| cfg.quality)
            .or_else(|| self.config.quality.map(|q| q as u8))
            .unwrap_or(DEFAULT_JPEG_QUALITY);

        let mut buf_writer = BufWriter::new(writer);
        let mut encoder = JpegEncoder::new_with_quality(&mut buf_writer, quality);

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
