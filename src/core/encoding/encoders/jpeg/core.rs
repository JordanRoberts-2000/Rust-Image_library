use {
    crate::{
        constants::DEFAULT_JPEG_QUALITY,
        encoding::{EncodingError, JpegColorType},
        types::Quality,
        ImageFormat,
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
        &self, writer: impl Write, bytes: impl AsRef<[u8]>, width: u32, height: u32,
        color_type: impl Into<JpegColorType>,
    ) -> Result<(), EncodingError> {
        self.encode_inner(writer, bytes.as_ref(), width, height, color_type.into())
            .map_err(|kind| EncodingError::new(ImageFormat::Jpeg, kind))
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
