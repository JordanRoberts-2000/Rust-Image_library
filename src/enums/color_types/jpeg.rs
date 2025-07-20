use image::{ColorType, ExtendedColorType};

#[cfg(feature = "progressive-jpeg")]
use mozjpeg::ColorSpace;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum JpegColorType {
    L8,
    #[default]
    Rgb8,
}

impl From<JpegColorType> for ExtendedColorType {
    fn from(jpeg_type: JpegColorType) -> Self {
        match jpeg_type {
            JpegColorType::L8 => ExtendedColorType::L8,
            JpegColorType::Rgb8 => ExtendedColorType::Rgb8,
        }
    }
}

impl From<&JpegColorType> for ExtendedColorType {
    fn from(jpeg_type: &JpegColorType) -> Self {
        match jpeg_type {
            JpegColorType::L8 => ExtendedColorType::L8,
            JpegColorType::Rgb8 => ExtendedColorType::Rgb8,
        }
    }
}

impl From<ColorType> for JpegColorType {
    fn from(color: ColorType) -> Self {
        match color {
            ColorType::L8 => JpegColorType::L8,
            ColorType::Rgb8 => JpegColorType::Rgb8,
            _ => JpegColorType::default(),
        }
    }
}

#[cfg(feature = "progressive-jpeg")]
impl From<JpegColorType> for ColorSpace {
    fn from(j: JpegColorType) -> Self {
        match j {
            JpegColorType::L8 => ColorSpace::JCS_GRAYSCALE,
            JpegColorType::Rgb8 => ColorSpace::JCS_RGB,
        }
    }
}
