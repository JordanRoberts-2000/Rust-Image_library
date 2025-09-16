use crate::{
    constants::{DEFAULT_AVIF_QUALITY, DEFAULT_AVIF_SPEED},
    AvifColorType,
};

pub struct AvifEncoder {
    pub(super) quality: u8,
    pub(super) speed: u8,
    pub(super) color_type: Option<AvifColorType>,
}

impl AvifEncoder {
    pub fn new() -> Self {
        Self { quality: DEFAULT_AVIF_QUALITY, speed: DEFAULT_AVIF_SPEED, color_type: None }
    }

    pub fn with_color_type(mut self, color_type: AvifColorType) -> Self {
        self.color_type = Some(color_type);
        self
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
