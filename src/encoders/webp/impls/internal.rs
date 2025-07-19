use std::io::Write;

use image::{codecs::webp::WebPEncoder as LosslessEncoder, load_from_memory, GenericImageView};
use webp::Encoder as LossyEncoder;

use crate::{
    encoders::{
        utils::{try_strip_alpha_if_unused, validate_dimensions},
        WebPEncoder,
    },
    enums::WebPColorType,
    CompressionType, EncodingError, ImageError, IoError, Result,
};

impl WebPEncoder {
    pub(super) fn encode(&self, bytes: &[u8], mut writer: impl Write) -> Result<()> {
        let mut img = load_from_memory(bytes).map_err(ImageError::DecodeFromMemory)?;

        let (width, height) = img.dimensions();
        validate_dimensions(width, height)?;

        let mut color_type: WebPColorType = match &self.color_type {
            Some(ct) => ct.clone(),
            None => WebPColorType::try_from(img.color())?,
        };

        if self.strip_unused_transparency && color_type.has_alpha() {
            if let Some(stripped) = try_strip_alpha_if_unused(&img) {
                img = stripped;
                color_type = WebPColorType::try_from(img.color())?;
            }
        }

        match self.compression {
            CompressionType::Lossy => {
                let encoder = LossyEncoder::new(img.as_bytes(), color_type.into(), width, height);
                let encoded = encoder.encode(self.quality as f32).to_vec();
                writer
                    .write_all(&encoded)
                    .map_err(|e| IoError::WriteAll(e))?;
            }
            CompressionType::Lossless => {
                let encoder = LosslessEncoder::new_lossless(writer);
                encoder
                    .encode(img.as_bytes(), width, height, color_type.into())
                    .map_err(EncodingError::WebPLosslessEncoding)?;
            }
        };

        Ok(())
    }
}
