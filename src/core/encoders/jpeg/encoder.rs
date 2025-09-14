use crate::{constants::DEFAULT_JPEG_QUALITY, JpegColorType};

#[derive(Debug, Clone)]
pub struct JpegEncoder {
    pub(super) quality: u8,
    pub(super) color_type: Option<JpegColorType>,
    pub(super) progressive: bool,
}

impl JpegEncoder {
    pub fn new() -> Self {
        Self { color_type: None, progressive: false, quality: DEFAULT_JPEG_QUALITY }
    }

    pub fn progressive() -> Self {
        Self { color_type: None, progressive: true, quality: DEFAULT_JPEG_QUALITY }
    }

    pub fn with_quality(mut self, quality: u8) -> Self {
        self.quality = quality.clamp(1, 100);
        self
    }

    pub fn with_color_type(mut self, color_type: JpegColorType) -> Self {
        self.color_type = Some(color_type);
        self
    }

    pub fn set_progressive(mut self, progressive: bool) -> Self {
        self.progressive = progressive;
        self
    }

    pub(super) fn color_type(&self) -> JpegColorType {
        self.color_type.as_ref().cloned().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use {super::*, crate::JpegColorType};

    #[test]
    fn quality_is_clamped() {
        let enc = JpegEncoder::new().with_quality(0);
        assert_eq!(enc.quality, 1);
        let enc = JpegEncoder::new().with_quality(101);
        assert_eq!(enc.quality, 100);
    }

    #[test]
    fn color_type_default_fallback() {
        let enc = JpegEncoder::new();
        assert_eq!(enc.color_type(), JpegColorType::default());
    }

    #[test]
    fn color_type_setter() {
        let enc = JpegEncoder::new().with_color_type(JpegColorType::Rgb8);
        assert_eq!(enc.color_type(), JpegColorType::Rgb8);
    }
}
