use crate::constants::{DEFAULT_AVIF_QUALITY, DEFAULT_AVIF_SPEED};

pub struct AvifEncoder {
    pub(super) quality: u8,
    pub(super) speed: u8,
}

impl AvifEncoder {
    pub fn new() -> Self {
        Self { quality: DEFAULT_AVIF_QUALITY, speed: DEFAULT_AVIF_SPEED }
    }

    pub fn with_quality(mut self, quality: u8) -> Self {
        self.quality = quality.clamp(1, 100);
        self
    }

    pub fn with_speed(mut self, speed: u8) -> Self {
        self.speed = speed.clamp(1, 10);
        self
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
        assert_eq!(enc.quality, 1);
        let enc = AvifEncoder::new().with_quality(101);
        assert_eq!(enc.quality, 100);
    }

    #[test]
    fn speed_is_clamped() {
        let enc = AvifEncoder::new().with_speed(0);
        assert_eq!(enc.speed, 1);
        let enc = AvifEncoder::new().with_speed(101);
        assert_eq!(enc.speed, 10);
    }
}
