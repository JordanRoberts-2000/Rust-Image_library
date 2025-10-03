use {
    crate::{
        AvifColorType, BitDepth, ImageError, JpegColorType, PngColorType, ValidationError,
        WebPColorType,
    },
    strum_macros::EnumIter,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum ColorType {
    Rgb8,
    Rgb16,
    Rgba8,
    Rgba16,
    Grayscale8,
    GrayscaleAlpha8,
    Grayscale16,
    GrayscaleAlpha16,
}

impl ColorType {
    pub fn has_alpha(self) -> bool {
        use ColorType::*;
        matches!(self, Rgba8 | Rgba16 | GrayscaleAlpha8 | GrayscaleAlpha16)
    }

    pub fn is_grayscale(self) -> bool {
        use ColorType::*;
        matches!(self, Grayscale8 | GrayscaleAlpha8 | Grayscale16 | GrayscaleAlpha16)
    }

    pub fn bit_depth(self) -> BitDepth {
        match self {
            ColorType::Rgb8
            | ColorType::Rgba8
            | ColorType::Grayscale8
            | ColorType::GrayscaleAlpha8 => BitDepth::Eight,
            ColorType::Rgb16
            | ColorType::Rgba16
            | ColorType::Grayscale16
            | ColorType::GrayscaleAlpha16 => BitDepth::Sixteen,
        }
    }

    pub fn channels(self) -> u8 {
        match self {
            ColorType::Rgb8 | ColorType::Rgb16 => 3,
            ColorType::Rgba8 | ColorType::Rgba16 => 4,
            ColorType::Grayscale8 | ColorType::Grayscale16 => 1,
            ColorType::GrayscaleAlpha8 | ColorType::GrayscaleAlpha16 => 2,
        }
    }

    pub fn remove_alpha(self) -> ColorType {
        use ColorType::*;
        match self {
            Rgba8 => Rgb8,
            Rgba16 => Rgb16,
            GrayscaleAlpha8 => Grayscale8,
            GrayscaleAlpha16 => Grayscale16,
            other => other,
        }
    }
}

impl From<ColorType> for image::ColorType {
    fn from(ct: ColorType) -> Self {
        match ct {
            ColorType::Rgb8 => image::ColorType::Rgb8,
            ColorType::Rgb16 => image::ColorType::Rgb16,
            ColorType::Rgba8 => image::ColorType::Rgba8,
            ColorType::Rgba16 => image::ColorType::Rgba16,
            ColorType::Grayscale8 => image::ColorType::L8,
            ColorType::GrayscaleAlpha8 => image::ColorType::La8,
            ColorType::Grayscale16 => image::ColorType::L16,
            ColorType::GrayscaleAlpha16 => image::ColorType::La16,
        }
    }
}

impl TryFrom<image::ColorType> for ColorType {
    type Error = ImageError;

    fn try_from(ct: image::ColorType) -> Result<Self, Self::Error> {
        Ok(match ct {
            image::ColorType::L8 => ColorType::Grayscale8,
            image::ColorType::La8 => ColorType::GrayscaleAlpha8,
            image::ColorType::Rgb8 => ColorType::Rgb8,
            image::ColorType::Rgba8 => ColorType::Rgba8,
            image::ColorType::L16 => ColorType::Grayscale16,
            image::ColorType::La16 => ColorType::GrayscaleAlpha16,
            image::ColorType::Rgb16 => ColorType::Rgb16,
            image::ColorType::Rgba16 => ColorType::Rgba16,
            _ => return Err(ValidationError::UnsupportedColorType(ct).into()),
        })
    }
}

impl From<ColorType> for PngColorType {
    fn from(color_type: ColorType) -> Self {
        use ColorType::*;
        match color_type {
            Rgb8 => PngColorType::Rgb8,
            Rgba8 => PngColorType::Rgba8,
            Rgb16 => PngColorType::Rgb16,
            Rgba16 => PngColorType::Rgba16,
            Grayscale8 => PngColorType::Grayscale8,
            GrayscaleAlpha8 => PngColorType::GrayscaleAlpha8,
            Grayscale16 => PngColorType::Grayscale16,
            GrayscaleAlpha16 => PngColorType::GrayscaleAlpha16,
        }
    }
}

impl TryFrom<ColorType> for WebPColorType {
    type Error = ImageError;

    fn try_from(color_type: ColorType) -> Result<Self, Self::Error> {
        match color_type {
            ColorType::Rgb8 => Ok(WebPColorType::Rgb8),
            ColorType::Rgba8 => Ok(WebPColorType::Rgba8),
            ColorType::Grayscale8 => Ok(WebPColorType::Grayscale8),
            ColorType::GrayscaleAlpha8 => Ok(WebPColorType::GrayscaleAlpha8),
            other => Err(ValidationError::UnsupportedColorType(other.into()).into()),
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

impl TryFrom<ColorType> for AvifColorType {
    type Error = ImageError;

    fn try_from(color_type: ColorType) -> Result<Self, Self::Error> {
        match color_type {
            ColorType::Rgb8 => Ok(AvifColorType::Rgb8),
            ColorType::Rgba8 => Ok(AvifColorType::Rgba8),
            other => Err(ValidationError::UnsupportedColorType(other.into()).into()),
        }
    }
}
