use crate::{enums::WebPColorType, CompressionType};

pub struct WebPEncoder {
    pub(super) quality: u8,
    pub(super) compression: CompressionType,
    pub(super) color_type: Option<WebPColorType>,
    pub(super) strip_unused_transparency: bool,
}

impl Default for WebPEncoder {
    fn default() -> Self {
        Self::new()
    }
}
