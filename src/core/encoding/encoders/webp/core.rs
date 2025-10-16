use {
    crate::{
        constants::DEFAULT_WEBP_QUALITY,
        encoding::{CompressionType, Encoder, EncodingError, Quality, WebpColorType},
    },
    std::io::Write,
};

#[derive(Debug, Clone)]
pub struct WebpEncoder {
    pub(crate) quality: Quality,
    pub(crate) compression_type: CompressionType,
}

impl WebpEncoder {
    pub fn new() -> Self {
        Self::lossy(DEFAULT_WEBP_QUALITY)
    }

    pub fn lossy(quality: u8) -> Self {
        Self { quality: quality.into(), compression_type: CompressionType::Lossy }
    }

    pub fn lossless() -> Self {
        Self { quality: DEFAULT_WEBP_QUALITY.into(), compression_type: CompressionType::Lossless }
    }

    pub fn with_compression_type(mut self, compression_type: CompressionType) -> Self {
        self.compression_type = compression_type;
        self
    }

    pub fn with_quality(mut self, quality: u8) -> Self {
        self.quality = quality.into();
        self
    }

    pub fn encode(
        &self, mut writer: impl Write, bytes: impl AsRef<[u8]>, w: u32, h: u32,
        ct: impl Into<WebpColorType>,
    ) -> Result<(), EncodingError> {
        <Self as Encoder>::encode(&self, &mut writer, bytes.as_ref(), w, h, ct.into())
    }

    pub fn encode_to_vec(
        &self, bytes: impl AsRef<[u8]>, w: u32, h: u32, ct: impl Into<WebpColorType>,
    ) -> Result<Vec<u8>, EncodingError> {
        <Self as Encoder>::encode_to_vec(&self, bytes.as_ref(), w, h, ct.into())
    }
}

impl Default for WebpEncoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quality_is_clamped() {
        let enc = WebpEncoder::new().with_quality(0);
        assert_eq!(enc.quality.get(), 1);
        let enc = WebpEncoder::new().with_quality(101);
        assert_eq!(enc.quality.get(), 100);
    }
}
