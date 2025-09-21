#[cfg(feature = "progressive-jpeg")]
use mozjpeg::ColorSpace;
use {
    crate::{BitDepth, ColorType},
    image::DynamicImage,
    std::borrow::Cow,
    strum_macros::EnumIter,
};

#[derive(Debug, Clone, Default, PartialEq, EnumIter)]
pub enum JpegColorType {
    Grayscale8,
    #[default]
    Rgb8,
}

impl JpegColorType {
    pub(crate) fn bytes<'a>(&self, img: &'a DynamicImage) -> Cow<'a, [u8]> {
        match self {
            JpegColorType::Grayscale8 => {
                if let DynamicImage::ImageLuma8(b) = img {
                    Cow::Borrowed(b.as_raw())
                } else {
                    Cow::Owned(img.to_luma8().into_raw())
                }
            }
            JpegColorType::Rgb8 => {
                if let DynamicImage::ImageRgb8(b) = img {
                    Cow::Borrowed(b.as_raw())
                } else {
                    Cow::Owned(img.to_rgb8().into_raw())
                }
            }
        }
    }

    pub fn channels(&self) -> usize {
        use JpegColorType::*;
        match self {
            Grayscale8 => 1,
            Rgb8 => 3,
        }
    }

    pub fn bit_depth(&self) -> BitDepth {
        BitDepth::Eight
    }

    pub fn is_grayscale(&self) -> bool {
        matches!(self, JpegColorType::Grayscale8)
    }
}

impl From<JpegColorType> for image::ExtendedColorType {
    fn from(color_type: JpegColorType) -> Self {
        match color_type {
            JpegColorType::Grayscale8 => image::ExtendedColorType::L8,
            JpegColorType::Rgb8 => image::ExtendedColorType::Rgb8,
        }
    }
}

#[cfg(feature = "progressive-jpeg")]
impl From<JpegColorType> for ColorSpace {
    fn from(color_type: JpegColorType) -> Self {
        match color_type {
            JpegColorType::Grayscale8 => ColorSpace::JCS_GRAYSCALE,
            JpegColorType::Rgb8 => ColorSpace::JCS_RGB,
        }
    }
}

impl From<JpegColorType> for ColorType {
    fn from(color_type: JpegColorType) -> Self {
        match color_type {
            JpegColorType::Grayscale8 => ColorType::Grayscale8,
            JpegColorType::Rgb8 => ColorType::Rgb8,
        }
    }
}

impl From<&JpegColorType> for ColorType {
    fn from(color_type: &JpegColorType) -> Self {
        match color_type {
            JpegColorType::Grayscale8 => ColorType::Grayscale8,
            JpegColorType::Rgb8 => ColorType::Rgb8,
        }
    }
}
