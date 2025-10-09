use crate::{constants::DEFAULT_WEBP_QUALITY, CompressionType};

pub struct WebPEncoder {
    pub(crate) quality: u8,
    pub(crate) compression_type: CompressionType,
}

impl WebPEncoder {
    pub fn new() -> Self {
        Self::lossy(DEFAULT_WEBP_QUALITY)
    }

    pub fn lossy(quality: u8) -> Self {
        Self { quality, compression_type: CompressionType::Lossy }
    }

    pub fn lossless() -> Self {
        Self { quality: DEFAULT_WEBP_QUALITY, compression_type: CompressionType::Lossless }
    }

    pub fn with_compression_type(mut self, compression_type: CompressionType) -> Self {
        self.compression_type = compression_type;
        self
    }

    pub fn with_quality(mut self, quality: u8) -> Self {
        self.quality = quality.clamp(1, 100);
        self
    }
}

impl Default for WebPEncoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quality_is_clamped() {
        let enc = WebPEncoder::new().with_quality(0);
        assert_eq!(enc.quality, 1);
        let enc = WebPEncoder::new().with_quality(101);
        assert_eq!(enc.quality, 100);
    }
}
