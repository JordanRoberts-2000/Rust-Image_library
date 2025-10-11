use crate::{
    constants::DEFAULT_WEBP_QUALITY,
    encoding::{CompressionType, Quality, WebpEncoder},
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct WebpConfig {
    pub quality: Quality,
    pub compression_type: CompressionType,
}

impl Default for WebpConfig {
    fn default() -> Self {
        Self { quality: DEFAULT_WEBP_QUALITY.into(), compression_type: CompressionType::Lossy }
    }
}

impl From<WebpConfig> for WebpEncoder {
    fn from(config: WebpConfig) -> Self {
        Self { quality: config.quality, compression_type: config.compression_type }
    }
}
