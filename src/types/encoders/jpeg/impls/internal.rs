#[cfg(feature = "progressive-jpeg")]
use mozjpeg::{Compress, ScanMode};
use {
    crate::{EncodingError, JpegColorType, JpegEncoder, Result},
    image::codecs::jpeg::JpegEncoder as Encoder,
    std::io::Write,
};

impl<S> JpegEncoder<S> {
    pub(super) fn jpeg_encode(
        &self, writer: impl Write, bytes: &[u8], width: u32, height: u32, color_type: JpegColorType,
    ) -> Result<()> {
        #[cfg(feature = "progressive-jpeg")]
        {
            if self.progressive {
                return self.encode_progressive(writer, bytes, width, height, color_type);
            }
        }

        let mut encoder = Encoder::new_with_quality(writer, self.quality);
        encoder
            .encode(bytes, width, height, color_type.into())
            .map_err(EncodingError::JpegEncoding)?;

        Ok(())
    }

    #[cfg(feature = "progressive-jpeg")]
    fn encode_progressive(
        &self, mut writer: impl Write, bytes: &[u8], width: u32, height: u32,
        color_type: JpegColorType,
    ) -> Result<()> {
        use crate::IoError;

        let mut comp = Compress::new(color_type.into());

        comp.set_size(width as usize, height as usize);
        comp.set_quality(self.quality as f32);
        comp.set_scan_optimization_mode(ScanMode::AllComponentsTogether);
        comp.set_progressive_mode();

        let mut comp_writer =
            comp.start_compress(Vec::new()).map_err(EncodingError::JpegCompressionStart)?;

        comp_writer.write_scanlines(bytes).map_err(EncodingError::JpegWriteScanlines)?;

        let jpeg_data = comp_writer.finish().map_err(EncodingError::JpegCompressionFinish)?;

        writer.write_all(&jpeg_data).map_err(IoError::WriteAll)?;

        Ok(())
    }
}
