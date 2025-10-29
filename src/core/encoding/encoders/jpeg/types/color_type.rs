#[cfg(feature = "progressive-jpeg")]
use mozjpeg::ColorSpace;
use {
    crate::{
        encoding::{ColorType, ColorTypeOps, EncodeColorTypeOps, GrayscaleOps, JpegColorType},
        image::Decoded,
    },
    inherent::inherent,
    std::{borrow::Cow, fmt},
};

impl JpegColorType {
    pub(crate) fn bytes<'a>(&self, decoded: &'a Decoded) -> Cow<'a, [u8]> {
        ColorType::from(*self).bytes(decoded)
    }
}

impl EncodeColorTypeOps for JpegColorType {
    fn from_color_type_lossy(ct: ColorType) -> Self {
        match ct {
            ColorType::Grayscale8
            | ColorType::Grayscale16
            | ColorType::GrayscaleAlpha8
            | ColorType::GrayscaleAlpha16 => JpegColorType::Grayscale8,

            ColorType::Rgb8
            | ColorType::Rgb16
            | ColorType::Rgba8
            | ColorType::Rgba16
            | ColorType::Rgb32Float
            | ColorType::Rgba32Float => JpegColorType::Rgb8,
        }
    }

    fn to_minimal_bit_depth(self) -> Self {
        self
    }

    fn remove_alpha(self) -> Self {
        self
    }

    fn has_alpha(&self) -> bool {
        false
    }
}

#[inherent]
impl ColorTypeOps for JpegColorType {
    pub fn channels(&self) -> u8 {
        ColorType::from(*self).channels()
    }

    pub fn bit_depth(&self) -> u8 {
        ColorType::from(*self).bit_depth()
    }

    pub fn supports_grayscale() -> bool {
        true
    }

    pub fn supports_transparency() -> bool {
        false
    }
}

#[inherent]
impl GrayscaleOps for JpegColorType {
    #[inline]
    pub fn is_grayscale(&self) -> bool {
        matches!(self, JpegColorType::Grayscale8)
    }

    #[inline]
    pub fn to_grayscale(self) -> Self {
        match self {
            JpegColorType::Rgb8 => JpegColorType::Grayscale8,
            other => other,
        }
    }

    #[inline]
    pub fn to_color(self) -> Self {
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
