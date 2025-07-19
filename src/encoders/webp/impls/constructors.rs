use crate::{constants::DEFAULT_WEBP_QUALITY, encoders::WebPEncoder, CompressionType};

impl WebPEncoder {
    pub fn new() -> Self {
        Self::lossy(DEFAULT_WEBP_QUALITY)
    }

    pub fn lossy(quality: u8) -> Self {
        Self {
            color_type: None,
            quality,
            strip_unused_transparency: false,
            compression: CompressionType::Lossy,
        }
    }

    pub fn lossless() -> Self {
        Self {
            color_type: None,
            quality: DEFAULT_WEBP_QUALITY,
            strip_unused_transparency: false,
            compression: CompressionType::Lossless,
        }
    }
}
