use {
    crate::{
        constants::DEFAULT_JPEG_QUALITY,
        encoding::{Encoder, EncodingError, JpegColorType, Quality},
    },
    std::io::Write,
};

#[derive(Debug, Clone)]
pub struct JpegEncoder {
    pub(crate) quality: Quality,
    pub(crate) progressive: bool,
}

impl JpegEncoder {
    pub fn new() -> Self {
        Self { progressive: false, quality: DEFAULT_JPEG_QUALITY.into() }
    }

    #[cfg(feature = "progressive-jpeg")]
    pub fn progressive() -> Self {
        Self { progressive: true, quality: DEFAULT_JPEG_QUALITY.into() }
    }

    pub fn with_quality(mut self, quality: u8) -> Self {
        self.quality = quality.into();
        self
    }

    #[cfg(feature = "progressive-jpeg")]
    pub fn set_progressive(mut self, progressive: bool) -> Self {
        self.progressive = progressive;
        self
    }

    pub fn encode(
        &self, mut writer: impl Write, bytes: impl AsRef<[u8]>, w: u32, h: u32,
        ct: impl Into<JpegColorType>,
    ) -> Result<(), EncodingError> {
        <Self as Encoder>::encode(&self, &mut writer, bytes.as_ref(), w, h, ct.into())
    }

    pub fn encode_to_vec(
        &self, bytes: impl AsRef<[u8]>, w: u32, h: u32, ct: impl Into<JpegColorType>,
    ) -> Result<Vec<u8>, EncodingError> {
        <Self as Encoder>::encode_to_vec(&self, bytes.as_ref(), w, h, ct.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quality_is_clamped() {
        let enc = JpegEncoder::new().with_quality(0);
        assert_eq!(enc.quality.get(), 1);
        let enc = JpegEncoder::new().with_quality(101);
        assert_eq!(enc.quality.get(), 100);
    }
}
