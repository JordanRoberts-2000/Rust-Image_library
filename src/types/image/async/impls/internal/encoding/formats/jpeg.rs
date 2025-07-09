use crate::{constants::DEFAULT_JPEG_QUALITY, Image, ImageError, ImageFormat, Result};

#[cfg(feature = "progressive-jpeg")]
use mozjpeg::{ColorSpace, Compress, ScanMode};

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

    #[cfg(feature = "progressive-jpeg")]
    pub fn encode_progressive_jpeg(&mut self, writer: impl Write) -> Result<()> {
        let rgb8 = self.get_decoded()?.to_rgb8();
        let (width, height) = rgb8.dimensions();
        let data = rgb8.as_raw();

        let quality = self
            .config
            .jpeg
            .as_ref()
            .map(|cfg| cfg.quality)
            .or_else(|| self.config.quality.map(|q| q as u8))
            .unwrap_or(DEFAULT_JPEG_QUALITY);

        let mut comp = Compress::new(ColorSpace::JCS_RGB);
        comp.set_size(width as usize, height as usize);
        comp.set_quality(quality as f32);
        comp.set_scan_optimization_mode(ScanMode::AllComponentsTogether);
        comp.set_progressive_mode();

        let mut comp_writer =
            comp.start_compress(Vec::new())
                .map_err(|e| ImageError::JpegCompressionStart {
                    source: e,
                    id: self.describe_source(),
                })?;

        comp_writer
            .write_scanlines(data)
            .map_err(|e| ImageError::JpegWriteScanlines {
                source: e,
                id: self.describe_source(),
            })?;

        let jpeg_data = comp_writer
            .finish()
            .map_err(|e| ImageError::JpegCompressionFinish {
                source: e,
                id: self.describe_source(),
            })?;

        let mut buf_writer = BufWriter::new(writer);
        buf_writer
            .write_all(&jpeg_data)
            .map_err(|e| ImageError::JpegWriteOutput {
                source: e,
                id: self.describe_source(),
            })?;

        buf_writer
            .flush()
            .map_err(|e| ImageError::JpegWriteOutput {
                source: e,
                id: self.describe_source(),
            })?;

        Ok(())
    }
}
