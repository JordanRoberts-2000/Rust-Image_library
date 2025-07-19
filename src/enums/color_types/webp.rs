use image::{ColorType, ExtendedColorType};
use webp::PixelLayout;

use crate::{EncodingError, ImageError};

#[derive(Debug, Clone, Default, PartialEq)]
pub enum WebPColorType {
    L8,
    La8,
    Rgb8,
    #[default]
    Rgba8,
}

impl WebPColorType {
    pub fn has_alpha(&self) -> bool {
        use WebPColorType::*;
        match self {
            L8 | Rgb8 => false,
            La8 | Rgba8 => true,
        }
    }
}

impl From<WebPColorType> for ExtendedColorType {
    fn from(color_type: WebPColorType) -> Self {
        match color_type {
            WebPColorType::Rgb8 => ExtendedColorType::Rgb8,
            WebPColorType::Rgba8 => ExtendedColorType::Rgba8,
            WebPColorType::L8 => ExtendedColorType::L8,
            WebPColorType::La8 => ExtendedColorType::La8,
        }
    }
}

impl From<&WebPColorType> for ExtendedColorType {
    fn from(color_type: &WebPColorType) -> Self {
        match color_type {
            WebPColorType::Rgb8 => ExtendedColorType::Rgb8,
            WebPColorType::Rgba8 => ExtendedColorType::Rgba8,
            WebPColorType::L8 => ExtendedColorType::L8,
            WebPColorType::La8 => ExtendedColorType::La8,
        }
    }
}

impl From<WebPColorType> for PixelLayout {
    fn from(color_type: WebPColorType) -> Self {
        match color_type {
            WebPColorType::Rgb8 | WebPColorType::L8 => PixelLayout::Rgb,
            WebPColorType::Rgba8 | WebPColorType::La8 => PixelLayout::Rgba,
        }
    }
}

impl TryFrom<ColorType> for WebPColorType {
    type Error = ImageError;

    fn try_from(color: ColorType) -> Result<Self, Self::Error> {
        match color {
            ColorType::L8 => Ok(WebPColorType::L8),
            ColorType::Rgb8 => Ok(WebPColorType::Rgb8),
            other => Err(ImageError::Encoding(EncodingError::UnsupportedColorType {
                format: "webp",
                color: format!("{:?}", other),
            })),
        }
    }
}
