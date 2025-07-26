use crate::encoders::{PngColorType, PngCompression, PngEncoder, PngFilter};

impl PngEncoder {
    pub fn with_compression(mut self, compression: PngCompression) -> Self {
        self.compression = compression;
        self
    }

    pub fn with_filter(mut self, filter: PngFilter) -> Self {
        self.filter = filter;
        self
    }

    pub fn with_color_type(mut self, color_type: PngColorType) -> Self {
        self.color_type = Some(color_type);
        self
    }

    pub fn strip_unused_transparency(mut self, strip: bool) -> Self {
        self.strip_unused_transparency = strip;
        self
    }

    pub fn quantize_to_8bit(mut self, quantize: bool) -> Self {
        self.quantize_to_8bit = quantize;
        self
    }
}
