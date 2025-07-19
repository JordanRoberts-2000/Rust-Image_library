use crate::enums::JpegColorType;

pub struct JpegEncoder {
    pub(super) quality: u8,
    pub(super) color_type: Option<JpegColorType>,
    pub(super) progressive: bool,
}

impl Default for JpegEncoder {
    fn default() -> Self {
        Self::new()
    }
}
