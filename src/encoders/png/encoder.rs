use crate::encoders::{PngColorType, PngCompression, PngFilter};

pub struct PngEncoder {
    pub(super) compression: PngCompression,
    pub(super) filter: PngFilter,
    pub(super) color_type: Option<PngColorType>,
    pub(super) strip_unused_transparency: bool,
    pub(super) quantize_to_8bit: bool,
}

impl Default for PngEncoder {
    fn default() -> Self {
        Self::new()
    }
}
