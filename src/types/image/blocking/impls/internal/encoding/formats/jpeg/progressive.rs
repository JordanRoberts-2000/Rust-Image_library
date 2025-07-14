#[cfg(feature = "progressive-jpeg")]
use mozjpeg::{ColorSpace, Compress, ScanMode};

use crate::blocking::Image;

#[cfg(feature = "progressive-jpeg")]
use {
    crate::{constants::DEFAULT_JPEG_QUALITY, ImageError, Result},
    std::io::Write,
};

impl Image {
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

        Self::write_encoded(writer, &jpeg_data, self.describe_source())
    }
}
