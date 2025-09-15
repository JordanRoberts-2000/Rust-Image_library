use crate::constants::DEFAULT_JPEG_QUALITY;

#[derive(Debug, Clone)]
pub struct JpegEncoder {
    pub(super) quality: u8,
    pub(super) progressive: bool,
}

impl JpegEncoder {
    pub fn new() -> Self {
        Self { progressive: false, quality: DEFAULT_JPEG_QUALITY }
    }

    pub fn progressive() -> Self {
        Self { progressive: true, quality: DEFAULT_JPEG_QUALITY }
    }

    pub fn with_quality(mut self, quality: u8) -> Self {
        self.quality = quality.clamp(1, 100);
        self
    }

    pub fn set_progressive(mut self, progressive: bool) -> Self {
        self.progressive = progressive;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quality_is_clamped() {
        let enc = JpegEncoder::new().with_quality(0);
        assert_eq!(enc.quality, 1);
        let enc = JpegEncoder::new().with_quality(101);
        assert_eq!(enc.quality, 100);
    }
}
