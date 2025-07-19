use crate::{encoders::JpegEncoder, enums::JpegColorType};

impl JpegEncoder {
    pub fn with_quality(mut self, quality: u8) -> Self {
        self.quality = quality.clamp(1, 100);
        self
    }

    pub fn with_color_type(mut self, color_type: JpegColorType) -> Self {
        self.color_type = Some(color_type);
        self
    }

    pub fn set_progressive(mut self) -> Self {
        self.progressive = true;
        self
    }
}
