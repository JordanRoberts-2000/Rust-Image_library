use crate::{encoders::WebPEncoder, enums::WebPColorType, CompressionType};

impl WebPEncoder {
    pub fn with_compression(mut self, compression: CompressionType) -> Self {
        self.compression = compression;
        self
    }

    pub fn with_color_type(mut self, color_type: WebPColorType) -> Self {
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
}
