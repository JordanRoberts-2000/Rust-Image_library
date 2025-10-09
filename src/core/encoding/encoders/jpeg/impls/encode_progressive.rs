#[cfg(feature = "progressive-jpeg")]
use crate::encoding::error::EncodingErrorKind;
#[cfg(feature = "progressive-jpeg")]
use mozjpeg::{Compress, ScanMode};
use {
    crate::encoding::{JpegColorType, JpegEncoder},
    std::io::Write,
};

impl JpegEncoder {
    #[cfg(feature = "progressive-jpeg")]
    pub(super) fn encode_progressive(
        &self, mut writer: impl Write, bytes: &[u8], width: u32, height: u32,
        color_type: JpegColorType,
    ) -> Result<(), EncodingErrorKind> {
        let mut comp = Compress::new(color_type.into());

        comp.set_size(width as usize, height as usize);
        comp.set_quality(self.quality.into());
        comp.set_scan_optimization_mode(ScanMode::AllComponentsTogether);
        comp.set_progressive_mode();

        let mut comp_writer = comp.start_compress(Vec::new())?;

        comp_writer.write_scanlines(bytes)?;

        let jpeg_data = comp_writer.finish()?;

        writer.write_all(&jpeg_data)?;

        Ok(())
    }
}
