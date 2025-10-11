use crate::encoding::{PngCompressionType, PngEncoder};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PngConfig {
    pub compression_type: PngCompressionType,
}

impl Default for PngConfig {
    fn default() -> Self {
        Self { compression_type: PngCompressionType::Default }
    }
}

impl From<PngConfig> for PngEncoder {
    fn from(config: PngConfig) -> Self {
        Self { compression_type: config.compression_type }
    }
}
