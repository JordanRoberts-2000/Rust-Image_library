use {
    crate::{encoders::JpegColorType, ImageFormat},
    std::io::Read,
};

pub struct Unset;
pub struct Bytes<'a> {
    pub bytes: &'a [u8],
    pub format: Option<ImageFormat>,
}
pub struct Reader<R: Read> {
    pub reader: R,
    pub format: Option<ImageFormat>,
}
pub struct Raw<'a> {
    pub bytes: &'a [u8],
    pub width: u32,
    pub height: u32,
}

pub struct JpegEncoder<Input> {
    pub(super) input: Input,
    pub(super) quality: u8,
    pub(super) color_type: Option<JpegColorType>,
    pub(super) progressive: bool,
}
