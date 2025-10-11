use {
    crate::{
        encoding::{
            macros::{
                forward_color_type_impls, forward_grayscale_impls, forward_transparency_impls,
            },
            AlphaChannelOps, ColorType, ColorTypeOps, GrayscaleOps,
        },
        ImageError, ValidationError,
    },
    image::DynamicImage,
    std::borrow::Cow,
    strum_macros::EnumIter,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, EnumIter, Default)]
pub enum TiffColorType {
    Grayscale8,
    Grayscale16,
    Rgb8,
    Rgb16,
    #[default]
    Rgba8,
    Rgba16,
}

impl TiffColorType {
    forward_grayscale_impls!();
    forward_color_type_impls!();
    forward_transparency_impls!();

    #[inline]
    pub(crate) fn bytes<'a>(&self, img: &'a DynamicImage) -> Cow<'a, [u8]> {
        ColorType::from(*self).bytes(img)
    }

    #[inline]
    pub fn to_minimal_bit_depth(self) -> Self {
        match self {
            TiffColorType::Rgb16 => TiffColorType::Rgb8,
            TiffColorType::Rgba16 => TiffColorType::Rgba8,
            TiffColorType::Grayscale16 => TiffColorType::Grayscale8,
            other => other,
        }
    }
}

impl GrayscaleOps for TiffColorType {
    #[inline]
    fn is_grayscale(&self) -> bool {
        ColorType::from(*self).is_grayscale()
    }

    #[inline]
    fn to_grayscale(self) -> Self {
        match self {
            TiffColorType::Rgb8 => TiffColorType::Grayscale8,
            TiffColorType::Rgb16 => TiffColorType::Grayscale16,
            TiffColorType::Rgba8 => TiffColorType::Grayscale8, // drop alpha
            TiffColorType::Rgba16 => TiffColorType::Grayscale16, // drop alpha
            g @ (TiffColorType::Grayscale8 | TiffColorType::Grayscale16) => g,
        }
    }

    #[inline]
    fn to_color(self) -> Self {
        match self {
            TiffColorType::Grayscale8 => TiffColorType::Rgb8,
            TiffColorType::Grayscale16 => TiffColorType::Rgb16,
            c @ (TiffColorType::Rgb8
            | TiffColorType::Rgb16
            | TiffColorType::Rgba8
            | TiffColorType::Rgba16) => c,
        }
    }
}

impl ColorTypeOps for TiffColorType {
    #[inline]
    fn channels(&self) -> u8 {
        ColorType::from(*self).channels()
    }

    #[inline]
    fn bit_depth(&self) -> u8 {
        ColorType::from(*self).bit_depth()
    }

    #[inline]
    fn supports_grayscale() -> bool {
        true
    }

    #[inline]
    fn supports_transparency() -> bool {
        true
    }
}

impl AlphaChannelOps for TiffColorType {
    #[inline]
    fn has_alpha(&self) -> bool {
        ColorType::from(*self).has_alpha()
    }

    #[inline]
    fn remove_alpha(self) -> Self {
        match self {
            TiffColorType::Rgba8 => TiffColorType::Rgb8,
            TiffColorType::Rgba16 => TiffColorType::Rgb16,
            other => other,
        }
    }

    #[inline]
    fn ensure_alpha(self) -> Self {
        match self {
            TiffColorType::Rgb8 => TiffColorType::Rgba8,
            TiffColorType::Rgb16 => TiffColorType::Rgba16,
            TiffColorType::Grayscale8 => TiffColorType::Rgba8,
            TiffColorType::Grayscale16 => TiffColorType::Rgba16,
            with_alpha @ (TiffColorType::Rgba8 | TiffColorType::Rgba16) => with_alpha,
        }
    }
}

impl From<TiffColorType> for image::ExtendedColorType {
    #[inline]
    fn from(ct: TiffColorType) -> Self {
        match ct {
            TiffColorType::Grayscale8 => image::ExtendedColorType::L8,
            TiffColorType::Grayscale16 => image::ExtendedColorType::L16,
            TiffColorType::Rgb8 => image::ExtendedColorType::Rgb8,
            TiffColorType::Rgb16 => image::ExtendedColorType::Rgb16,
            TiffColorType::Rgba8 => image::ExtendedColorType::Rgba8,
            TiffColorType::Rgba16 => image::ExtendedColorType::Rgba16,
        }
    }
}

impl From<TiffColorType> for image::ColorType {
    #[inline]
    fn from(ct: TiffColorType) -> Self {
        match ct {
            TiffColorType::Grayscale8 => image::ColorType::L8,
            TiffColorType::Grayscale16 => image::ColorType::L16,
            TiffColorType::Rgb8 => image::ColorType::Rgb8,
            TiffColorType::Rgb16 => image::ColorType::Rgb16,
            TiffColorType::Rgba8 => image::ColorType::Rgba8,
            TiffColorType::Rgba16 => image::ColorType::Rgba16,
        }
    }
}

impl From<TiffColorType> for ColorType {
    #[inline]
    fn from(ct: TiffColorType) -> Self {
        match ct {
            TiffColorType::Grayscale8 => ColorType::Grayscale8,
            TiffColorType::Grayscale16 => ColorType::Grayscale16,
            TiffColorType::Rgb8 => ColorType::Rgb8,
            TiffColorType::Rgb16 => ColorType::Rgb16,
            TiffColorType::Rgba8 => ColorType::Rgba8,
            TiffColorType::Rgba16 => ColorType::Rgba16,
        }
    }
}

impl TryFrom<ColorType> for TiffColorType {
    type Error = ImageError;

    #[inline]
    fn try_from(ct: ColorType) -> Result<Self, Self::Error> {
        match ct {
            ColorType::Grayscale8 => Ok(TiffColorType::Grayscale8),
            ColorType::Grayscale16 => Ok(TiffColorType::Grayscale16),
            ColorType::Rgb8 => Ok(TiffColorType::Rgb8),
            ColorType::Rgb16 => Ok(TiffColorType::Rgb16),
            ColorType::Rgba8 => Ok(TiffColorType::Rgba8),
            ColorType::Rgba16 => Ok(TiffColorType::Rgba16),
            other => Err(ValidationError::UnsupportedColorType(other.into()).into()),
        }
    }
}
