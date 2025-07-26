use crate::encoders::AvifColorType;

pub struct AvifEncoder {
    pub(super) quality: u8,
    pub(super) alpha_quality: u8,
    pub(super) speed: u8,
    pub(super) color_type: Option<AvifColorType>,
    pub(super) strip_unused_transparency: bool,
}

impl Default for AvifEncoder {
    fn default() -> Self {
        Self::new()
    }
}
