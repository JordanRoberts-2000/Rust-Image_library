use {
    crate::{
        encoders::{utils::validate_dimensions, JpegEncoder},
        enums::JpegColorType,
        EncodingError, ImageError, Result,
    },
    image::{codecs::jpeg::JpegEncoder as Encoder, load_from_memory, GenericImageView},
    std::io::Write,
};

use image::DynamicImage;
#[cfg(feature = "progressive-jpeg")]
use mozjpeg::{Compress, ScanMode};

impl JpegEncoder {
    pub(super) fn encode(&self, bytes: &[u8], writer: impl Write) -> Result<()> {
        #[cfg(feature = "progressive-jpeg")]
        {
            if self.progressive {
                return self.encode_progressive(bytes, writer);
            }
        }

        let (img, width, height, color_type) = self.decode(bytes)?;

        let mut encoder = Encoder::new_with_quality(writer, self.quality);
        encoder
            .encode(img.as_bytes(), width, height, color_type.into())
            .map_err(EncodingError::JpegEncoding)?;

        Ok(())
    }

    #[cfg(feature = "progressive-jpeg")]
    fn encode_progressive(&self, bytes: &[u8], mut writer: impl Write) -> Result<()> {
        use crate::IoError;

        let (img, width, height, color_type) = self.decode(bytes)?;

        let mut comp = Compress::new(color_type.into());

        comp.set_size(width as usize, height as usize);
        comp.set_quality(self.quality as f32);
        comp.set_scan_optimization_mode(ScanMode::AllComponentsTogether);
        comp.set_progressive_mode();

        let mut comp_writer = comp
            .start_compress(Vec::new())
            .map_err(EncodingError::JpegCompressionStart)?;

        comp_writer
            .write_scanlines(img.as_bytes())
            .map_err(EncodingError::JpegWriteScanlines)?;

        let jpeg_data = comp_writer
            .finish()
            .map_err(EncodingError::JpegCompressionFinish)?;

        writer.write_all(&jpeg_data).map_err(IoError::WriteAll)?;

        Ok(())
    }

    fn decode(&self, bytes: &[u8]) -> Result<(DynamicImage, u32, u32, JpegColorType)> {
        let img = load_from_memory(bytes).map_err(ImageError::DecodeFromMemory)?;

        let (width, height) = img.dimensions();
        validate_dimensions(width, height)?;

        let color_type = match &self.color_type {
            Some(ct) => ct.clone(),
            None => img.color().into(),
        };

        Ok((img, width, height, color_type))
    }
}
