use crate::{constants::DEFAULT_JPEG_QUALITY, encoding::JpegEncoder, types::Quality};

#[derive(Debug, Clone, PartialEq)]
pub struct JpegConfig {
    pub quality: Quality,
    pub progressive: bool,
}

impl Default for JpegConfig {
    fn default() -> Self {
        Self { quality: DEFAULT_JPEG_QUALITY.into(), progressive: false }
    }
}

impl From<JpegConfig> for JpegEncoder {
    fn from(config: JpegConfig) -> Self {
        Self { quality: config.quality, progressive: config.progressive }
    }
}
