use {
    crate::{
        constants::{DEFAULT_AVIF_QUALITY, DEFAULT_AVIF_SPEED},
        encoding::{AvifColorType, AvifSpeed, EncoderOps, EncodingError, Quality},
    },
    std::io::Write,
};

#[derive(Debug, Clone)]
pub struct AvifEncoder {
    pub(crate) quality: Quality,
    pub(crate) speed: AvifSpeed,
}

impl AvifEncoder {
    pub fn new() -> Self {
        Self { quality: DEFAULT_AVIF_QUALITY.into(), speed: DEFAULT_AVIF_SPEED.into() }
    }

    pub fn with_quality(mut self, quality: u8) -> Self {
        self.quality = quality.into();
        self
    }

    pub fn with_speed(mut self, speed: u8) -> Self {
        self.speed = speed.into();
        self
    }

    pub fn encode(
        &self, writer: impl Write, bytes: impl AsRef<[u8]>, w: u32, h: u32,
        ct: impl Into<AvifColorType>,
    ) -> Result<(), EncodingError> {
        <Self as EncoderOps>::encode(&self, writer, bytes, w, h, ct)
    }
}

impl Default for AvifEncoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quality_is_clamped() {
        let enc = AvifEncoder::new().with_quality(0);
        assert_eq!(enc.quality.get(), 1);
        let enc = AvifEncoder::new().with_quality(101);
        assert_eq!(enc.quality.get(), 100);
    }

    #[test]
    fn speed_is_clamped() {
        let enc = AvifEncoder::new().with_speed(0);
        assert_eq!(enc.speed.get(), 1);
        let enc = AvifEncoder::new().with_speed(101);
        assert_eq!(enc.speed.get(), 10);
    }
}
