use crate::encoders::{AvifColorType, AvifEncoder};

impl AvifEncoder {
    pub fn with_color_type(mut self, color_type: AvifColorType) -> Self {
        self.color_type = Some(color_type);
        self
    }

    pub fn strip_unused_transparency(mut self, strip: bool) -> Self {
        self.strip_unused_transparency = strip;
        self
    }

    pub fn with_quality(mut self, quality: u8) -> Self {
        self.quality = quality.clamp(1, 100);
        self
    }

    pub fn with_alpha_quality(mut self, quality: u8) -> Self {
        self.alpha_quality = quality.clamp(1, 100);
        self
    }

    pub fn with_speed(mut self, speed: u8) -> Self {
        self.speed = speed.clamp(1, 10);
        self
    }
}
