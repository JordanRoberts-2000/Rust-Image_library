use crate::encoders::{PngCompression, PngEncoder, PngFilter};

impl PngEncoder {
    pub fn new() -> Self {
        Self {
            compression: PngCompression::Default,
            filter: PngFilter::Adaptive,
            color_type: None,
            strip_unused_transparency: false,
            quantize_to_8bit: false,
        }
    }

    pub fn best_compression() -> Self {
        Self {
            compression: PngCompression::Best,
            filter: PngFilter::Adaptive,
            color_type: None,
            strip_unused_transparency: true,
            quantize_to_8bit: true,
        }
    }

    pub fn fast() -> Self {
        Self {
            compression: PngCompression::Fast,
            filter: PngFilter::NoFilter,
            color_type: None,
            strip_unused_transparency: false,
            quantize_to_8bit: false,
        }
    }
}
