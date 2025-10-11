use {
    crate::{
        encoding::{
            macros::{
                forward_color_type_impls, forward_grayscale_impls, forward_transparency_impls,
            },
            AlphaChannelOps, BitDepth, ColorType, ColorTypeOps, GrayscaleOps,
        },
        ImageError, ValidationError,
    },
    image::DynamicImage,
    std::borrow::Cow,
    strum_macros::EnumIter,
    webp::PixelLayout,
};

#[derive(Debug, Clone, Default, Copy, Eq, PartialEq, EnumIter)]
pub enum WebpColorType {
    Grayscale8,
    GrayscaleAlpha8,
    Rgb8,
    #[default]
    Rgba8,
}

impl WebpColorType {
    pub(crate) fn bytes<'a>(&self, img: &'a DynamicImage) -> Cow<'a, [u8]> {
        ColorType::from(*self).bytes(img)
    }

    forward_grayscale_impls!();
    forward_color_type_impls!();
    forward_transparency_impls!();
}

impl GrayscaleOps for WebpColorType {
    fn is_grayscale(&self) -> bool {
        ColorType::from(*self).is_grayscale()
    }

    fn to_grayscale(self) -> Self {
        match self {
            WebpColorType::Rgb8 => WebpColorType::Grayscale8,
            WebpColorType::Rgba8 => WebpColorType::GrayscaleAlpha8,
            other => other,
        }
    }

    fn to_color(self) -> Self {
        match self {
            WebpColorType::Grayscale8 => WebpColorType::Rgb8,
            WebpColorType::GrayscaleAlpha8 => WebpColorType::Rgba8,
            other => other,
        }
    }
}

impl ColorTypeOps for WebpColorType {
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
        true
    }
}

impl AlphaChannelOps for WebpColorType {
    fn has_alpha(&self) -> bool {
        ColorType::from(*self).has_alpha()
    }

    fn remove_alpha(self) -> Self {
        match self {
            WebpColorType::GrayscaleAlpha8 => WebpColorType::Grayscale8,
            WebpColorType::Rgba8 => WebpColorType::Rgb8,
            other => other,
        }
    }

    fn ensure_alpha(self) -> Self {
        match self {
            WebpColorType::Grayscale8 => WebpColorType::GrayscaleAlpha8,
            WebpColorType::Rgb8 => WebpColorType::Rgba8,
            other => other,
        }
    }
}

impl From<WebpColorType> for image::ExtendedColorType {
    fn from(color_type: WebpColorType) -> Self {
        match color_type {
            WebpColorType::Rgb8 => image::ExtendedColorType::Rgb8,
            WebpColorType::Rgba8 => image::ExtendedColorType::Rgba8,
            WebpColorType::Grayscale8 => image::ExtendedColorType::L8,
            WebpColorType::GrayscaleAlpha8 => image::ExtendedColorType::La8,
        }
    }
}

impl From<WebpColorType> for image::ColorType {
    fn from(color_type: WebpColorType) -> Self {
        match color_type {
            WebpColorType::Rgb8 => image::ColorType::Rgb8,
            WebpColorType::Rgba8 => image::ColorType::Rgba8,
            WebpColorType::Grayscale8 => image::ColorType::L8,
            WebpColorType::GrayscaleAlpha8 => image::ColorType::La8,
        }
    }
}

impl From<WebpColorType> for PixelLayout {
    fn from(color_type: WebpColorType) -> Self {
        match color_type {
            WebpColorType::Rgb8 | WebpColorType::Grayscale8 => PixelLayout::Rgb,
            WebpColorType::Rgba8 | WebpColorType::GrayscaleAlpha8 => PixelLayout::Rgba,
        }
    }
}

impl From<WebpColorType> for ColorType {
    fn from(color_type: WebpColorType) -> Self {
        match color_type {
            WebpColorType::Grayscale8 => ColorType::Grayscale8,
            WebpColorType::GrayscaleAlpha8 => ColorType::GrayscaleAlpha8,
            WebpColorType::Rgb8 => ColorType::Rgb8,
            WebpColorType::Rgba8 => ColorType::Rgba8,
        }
    }
}

impl TryFrom<ColorType> for WebpColorType {
    type Error = ImageError;

    fn try_from(color_type: ColorType) -> Result<Self, Self::Error> {
        match color_type {
            ColorType::Rgb8 => Ok(WebpColorType::Rgb8),
            ColorType::Rgba8 => Ok(WebpColorType::Rgba8),
            ColorType::Grayscale8 => Ok(WebpColorType::Grayscale8),
            ColorType::GrayscaleAlpha8 => Ok(WebpColorType::GrayscaleAlpha8),
            other => Err(ValidationError::UnsupportedColorType(other.into()).into()),
        }
    }
}
