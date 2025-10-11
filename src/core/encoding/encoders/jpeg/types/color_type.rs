#[cfg(feature = "progressive-jpeg")]
use mozjpeg::ColorSpace;
use {
    crate::{
        encoding::{
            macros::{forward_color_type_impls, forward_grayscale_impls},
            BitDepth, ColorType, ColorTypeOps, GrayscaleOps,
        },
        ImageError, ValidationError,
    },
    image::DynamicImage,
    std::{borrow::Cow, fmt},
    strum_macros::EnumIter,
};

#[derive(Debug, Clone, Default, Copy, Eq, PartialEq, EnumIter)]
pub enum JpegColorType {
    Grayscale8,
    #[default]
    Rgb8,
}

impl JpegColorType {
    forward_grayscale_impls!();
    forward_color_type_impls!();

    pub(crate) fn bytes<'a>(&self, img: &'a DynamicImage) -> Cow<'a, [u8]> {
        ColorType::from(*self).bytes(img)
    }
}

impl ColorTypeOps for JpegColorType {
    fn channels(&self) -> u8 {
        ColorType::from(*self).channels()
    }

    fn bit_depth(&self) -> BitDepth {
        ColorType::from(*self).bit_depth()
    }

    fn supports_grayscale() -> bool {
        true
    }

    fn supports_transparency() -> bool {
        false
    }
}

impl GrayscaleOps for JpegColorType {
    #[inline]
    fn is_grayscale(&self) -> bool {
        matches!(self, JpegColorType::Grayscale8)
    }

    #[inline]
    fn to_grayscale(self) -> Self {
        match self {
            JpegColorType::Rgb8 => JpegColorType::Grayscale8,
            other => other,
        }
    }

    #[inline]
    fn to_color(self) -> Self {
        match self {
            JpegColorType::Grayscale8 => JpegColorType::Rgb8,
            other => other,
        }
    }
}

impl fmt::Display for JpegColorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", ColorType::from(*self))
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

impl TryFrom<ColorType> for JpegColorType {
    type Error = ImageError;

    fn try_from(color_type: ColorType) -> Result<Self, Self::Error> {
        match color_type {
            ColorType::Rgb8 => Ok(JpegColorType::Rgb8),
            ColorType::Grayscale8 => Ok(JpegColorType::Grayscale8),
            other => Err(ValidationError::UnsupportedColorType(other.into()).into()),
        }
    }
}
