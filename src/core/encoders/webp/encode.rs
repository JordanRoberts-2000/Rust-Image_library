use {
    crate::{CompressionType, EncodingError, Result, WebPColorType, WebPEncoder},
    image::codecs::webp::WebPEncoder as LosslessEncoder,
    std::io::Write,
    webp::Encoder as LossyEncoder,
};

impl WebPEncoder {
    pub fn encode(
        &self, mut writer: impl Write, bytes: &[u8], width: u32, height: u32,
        color_type: WebPColorType,
    ) -> Result<()> {
        match self.compression_type {
            CompressionType::Lossy => {
                let encoder =
                    LossyEncoder::new(bytes, WebPColorType::from(color_type).into(), width, height);
                let encoded = encoder.encode(self.quality as f32).to_vec();
                writer.write_all(&encoded)?;
            }
            CompressionType::Lossless => {
                let encoder = LosslessEncoder::new_lossless(writer);
                encoder
                    .encode(bytes, width, height, color_type.into())
                    .map_err(EncodingError::WebPLosslessEncoding)?;
            }
        };

        Ok(())
    }
}
