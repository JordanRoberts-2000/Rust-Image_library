#[cfg(feature = "progressive-jpeg")]
use mozjpeg::ColorSpace;
use {
    crate::{ColorType, ImageError, ImageFormat},
    image::{ColorType as ImgColorType, ExtendedColorType as ImgExtendedColorType},
    strum_macros::EnumIter,
};

#[derive(Debug, Clone, Default, PartialEq, EnumIter)]
pub enum JpegColorType {
    L8,
    #[default]
    Rgb8,
}

impl From<JpegColorType> for ImgExtendedColorType {
    fn from(ct: JpegColorType) -> Self {
        match ct {
            JpegColorType::L8 => ImgExtendedColorType::L8,
            JpegColorType::Rgb8 => ImgExtendedColorType::Rgb8,
        }
    }
}

impl From<&JpegColorType> for ImgExtendedColorType {
    fn from(ct: &JpegColorType) -> Self {
        match ct {
            JpegColorType::L8 => ImgExtendedColorType::L8,
            JpegColorType::Rgb8 => ImgExtendedColorType::Rgb8,
        }
    }
}

impl TryFrom<ImgColorType> for JpegColorType {
    type Error = ImageError;

    fn try_from(ct: ImgColorType) -> Result<Self, Self::Error> {
        match ct {
            ImgColorType::L8 => Ok(JpegColorType::L8),
            ImgColorType::Rgb8 => Ok(JpegColorType::Rgb8),
            other => Err(ImageError::UnsupportedColorType(other, ImageFormat::Jpeg)),
        }
    }
}

#[cfg(feature = "progressive-jpeg")]
impl From<JpegColorType> for ColorSpace {
    fn from(ct: JpegColorType) -> Self {
        match ct {
            JpegColorType::L8 => ColorSpace::JCS_GRAYSCALE,
            JpegColorType::Rgb8 => ColorSpace::JCS_RGB,
        }
    }
}

impl TryFrom<ColorType> for JpegColorType {
    type Error = ImageError;

    fn try_from(ct: ColorType) -> Result<Self, Self::Error> {
        match ct {
            ColorType::L8 => Ok(JpegColorType::L8),
            ColorType::Rgb8 => Ok(JpegColorType::Rgb8),
            other => Err(ImageError::UnsupportedColorType(other.into(), ImageFormat::Jpeg)),
        }
    }
}
