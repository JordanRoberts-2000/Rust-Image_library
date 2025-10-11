use {
    crate::encoding::{
        macros::{forward_color_type_impls, forward_grayscale_impls, forward_transparency_impls},
        AlphaChannelOps, BitDepth, ColorType, ColorTypeOps, GrayscaleOps,
    },
    image::DynamicImage,
    std::borrow::Cow,
    strum_macros::EnumIter,
};

#[derive(Debug, Clone, Default, Copy, Eq, PartialEq, EnumIter)]
pub enum PngColorType {
    Rgb8,
    #[default]
    Rgba8,
    Rgb16,
    Rgba16,
    Grayscale8,
    GrayscaleAlpha8,
    Grayscale16,
    GrayscaleAlpha16,
}

impl PngColorType {
    forward_grayscale_impls!();
    forward_color_type_impls!();
    forward_transparency_impls!();

    pub(crate) fn bytes<'a>(&self, img: &'a DynamicImage) -> Cow<'a, [u8]> {
        ColorType::from(*self).bytes(img)
    }

    pub fn to_minimal_bit_depth(self) -> Self {
        match self {
            PngColorType::Rgb16 => PngColorType::Rgb8,
            PngColorType::Rgba16 => PngColorType::Rgba8,
            PngColorType::Grayscale16 => PngColorType::Grayscale8,
            PngColorType::GrayscaleAlpha16 => PngColorType::GrayscaleAlpha8,
            other => other,
        }
    }
}

impl GrayscaleOps for PngColorType {
    #[inline]
    fn is_grayscale(&self) -> bool {
        ColorType::from(*self).is_grayscale()
    }

    #[inline]
    fn to_grayscale(self) -> Self {
        match self {
            PngColorType::Rgb8 => PngColorType::Grayscale8,
            PngColorType::Rgb16 => PngColorType::Grayscale16,
            PngColorType::Rgba8 => PngColorType::GrayscaleAlpha8,
            PngColorType::Rgba16 => PngColorType::GrayscaleAlpha16,
            g @ (PngColorType::Grayscale8
            | PngColorType::Grayscale16
            | PngColorType::GrayscaleAlpha8
            | PngColorType::GrayscaleAlpha16) => g,
        }
    }

    #[inline]
    fn to_color(self) -> Self {
        match self {
            PngColorType::Grayscale8 => PngColorType::Rgb8,
            PngColorType::Grayscale16 => PngColorType::Rgb16,
            PngColorType::GrayscaleAlpha8 => PngColorType::Rgba8,
            PngColorType::GrayscaleAlpha16 => PngColorType::Rgba16,
            c @ (PngColorType::Rgb8
            | PngColorType::Rgb16
            | PngColorType::Rgba8
            | PngColorType::Rgba16) => c,
        }
    }
}

impl ColorTypeOps for PngColorType {
    #[inline]
    fn channels(&self) -> u8 {
        ColorType::from(*self).channels()
    }
    #[inline]
    fn bit_depth(&self) -> BitDepth {
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

impl AlphaChannelOps for PngColorType {
    #[inline]
    fn has_alpha(&self) -> bool {
        ColorType::from(*self).has_alpha()
    }
    #[inline]
    fn remove_alpha(self) -> Self {
        match self {
            PngColorType::Rgba8 => PngColorType::Rgb8,
            PngColorType::Rgba16 => PngColorType::Rgb16,
            PngColorType::GrayscaleAlpha8 => PngColorType::Grayscale8,
            PngColorType::GrayscaleAlpha16 => PngColorType::Grayscale16,
            other => other,
        }
    }
    #[inline]
    fn ensure_alpha(self) -> Self {
        match self {
            PngColorType::Rgb8 => PngColorType::Rgba8,
            PngColorType::Rgb16 => PngColorType::Rgba16,
            PngColorType::Grayscale8 => PngColorType::GrayscaleAlpha8,
            PngColorType::Grayscale16 => PngColorType::GrayscaleAlpha16,
            with_alpha @ (PngColorType::Rgba8
            | PngColorType::Rgba16
            | PngColorType::GrayscaleAlpha8
            | PngColorType::GrayscaleAlpha16) => with_alpha,
        }
    }
}

impl From<PngColorType> for image::ExtendedColorType {
    fn from(color_type: PngColorType) -> Self {
        match color_type {
            PngColorType::Rgb8 => image::ExtendedColorType::Rgb8,
            PngColorType::Rgba8 => image::ExtendedColorType::Rgba8,
            PngColorType::Rgb16 => image::ExtendedColorType::Rgb16,
            PngColorType::Rgba16 => image::ExtendedColorType::Rgba16,
            PngColorType::Grayscale8 => image::ExtendedColorType::L8,
            PngColorType::GrayscaleAlpha8 => image::ExtendedColorType::La8,
            PngColorType::Grayscale16 => image::ExtendedColorType::L16,
            PngColorType::GrayscaleAlpha16 => image::ExtendedColorType::La16,
        }
    }
}

impl From<PngColorType> for image::ColorType {
    fn from(color_type: PngColorType) -> Self {
        match color_type {
            PngColorType::Rgb8 => image::ColorType::Rgb8,
            PngColorType::Rgba8 => image::ColorType::Rgba8,
            PngColorType::Rgb16 => image::ColorType::Rgb16,
            PngColorType::Rgba16 => image::ColorType::Rgba16,
            PngColorType::Grayscale8 => image::ColorType::L8,
            PngColorType::GrayscaleAlpha8 => image::ColorType::La8,
            PngColorType::Grayscale16 => image::ColorType::L16,
            PngColorType::GrayscaleAlpha16 => image::ColorType::La16,
        }
    }
}

impl From<PngColorType> for ColorType {
    fn from(color_type: PngColorType) -> Self {
        match color_type {
            PngColorType::Rgb8 => ColorType::Rgb8,
            PngColorType::Rgba8 => ColorType::Rgba8,
            PngColorType::Rgb16 => ColorType::Rgb16,
            PngColorType::Rgba16 => ColorType::Rgba16,
            PngColorType::Grayscale8 => ColorType::Grayscale8,
            PngColorType::GrayscaleAlpha8 => ColorType::GrayscaleAlpha8,
            PngColorType::Grayscale16 => ColorType::Grayscale16,
            PngColorType::GrayscaleAlpha16 => ColorType::GrayscaleAlpha16,
        }
    }
}

impl From<ColorType> for PngColorType {
    fn from(color_type: ColorType) -> Self {
        match color_type {
            ColorType::Rgb8 => PngColorType::Rgb8,
            ColorType::Rgba8 => PngColorType::Rgba8,
            ColorType::Rgb16 => PngColorType::Rgb16,
            ColorType::Rgba16 => PngColorType::Rgba16,
            ColorType::Grayscale8 => PngColorType::Grayscale8,
            ColorType::GrayscaleAlpha8 => PngColorType::GrayscaleAlpha8,
            ColorType::Grayscale16 => PngColorType::Grayscale16,
            ColorType::GrayscaleAlpha16 => PngColorType::GrayscaleAlpha16,
        }
    }
}
