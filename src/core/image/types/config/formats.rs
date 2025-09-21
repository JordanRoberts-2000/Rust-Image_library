use crate::{
    constants::{
        DEFAULT_AVIF_QUALITY, DEFAULT_AVIF_SPEED, DEFAULT_JPEG_QUALITY, DEFAULT_WEBP_QUALITY,
    },
    PngCompressionType,
};

#[derive(Debug, Clone, PartialEq)]
pub struct JpegConfig {
    pub quality: u8,
    pub progressive: bool,
}

impl Default for JpegConfig {
    fn default() -> Self {
        Self { quality: DEFAULT_JPEG_QUALITY, progressive: false }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PngConfig {
    pub compression_type: PngCompressionType,
}

impl Default for PngConfig {
    fn default() -> Self {
        Self { compression_type: PngCompressionType::Default }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AvifConfig {
    pub quality: u8,
    pub speed: u8,
}

impl Default for AvifConfig {
    fn default() -> Self {
        Self { quality: DEFAULT_AVIF_QUALITY, speed: DEFAULT_AVIF_SPEED }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WebpConfig {
    pub quality: u8,
    pub lossless: bool,
}

impl Default for WebpConfig {
    fn default() -> Self {
        Self { quality: DEFAULT_WEBP_QUALITY, lossless: false }
    }
}
