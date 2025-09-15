use {
    crate::{EncodingError, JpegColorType, JpegEncoder, Result},
    image::codecs::jpeg::JpegEncoder as Encoder,
    std::io::Write,
};

impl JpegEncoder {
    pub fn encode(
        &self, writer: impl Write, bytes: &[u8], width: u32, height: u32, color_type: JpegColorType,
    ) -> Result<()> {
        #[cfg(feature = "progressive-jpeg")]
        {
            if self.progressive {
                return self.encode_progressive(writer, bytes, width, height, color_type);
            }
        }

        #[cfg(not(feature = "progressive-jpeg"))]
        if self.progressive {
            warn!(
                "progressive encoding requested, but the 'progressive-jpeg' feature is disabled; \
                 falling back to baseline JPEG."
            );
        }

        Encoder::new_with_quality(writer, self.quality)
            .encode(bytes, width, height, color_type.into())
            .map_err(EncodingError::JpegEncoding)?;

        Ok(())
    }
}
