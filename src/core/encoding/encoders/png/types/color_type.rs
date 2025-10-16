use {
    crate::encoding::{
        AlphaChannelOps, BitDepthOps, ColorType, ColorTypeOps, GrayscaleOps, PngColorType,
    },
    inherent::inherent,
};

impl PngColorType {
    pub fn from_color_type_lossy(ct: ColorType) -> Self {
        match ct {
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

#[inherent]
impl ColorTypeOps for PngColorType {
    #[inline]
    pub fn channels(&self) -> u8 {
        ColorType::from(*self).channels()
    }
    #[inline]
    pub fn bit_depth(&self) -> u8 {
        ColorType::from(*self).bit_depth()
    }
    #[inline]
    pub fn supports_grayscale() -> bool {
        true
    }
    #[inline]
    pub fn supports_transparency() -> bool {
        true
    }
}

#[inherent]
impl BitDepthOps for PngColorType {
    #[inline]
    pub fn to_minimal_bit_depth(self) -> Self {
        match self {
            PngColorType::Rgb16 => PngColorType::Rgb8,
            PngColorType::Rgba16 => PngColorType::Rgba8,
            PngColorType::Grayscale16 => PngColorType::Grayscale8,
            PngColorType::GrayscaleAlpha16 => PngColorType::GrayscaleAlpha8,
            other => other,
        }
    }

    #[inline]
    pub fn to_maximal_bit_depth(self) -> Self {
        match self {
            PngColorType::Rgb8 => PngColorType::Rgb16,
            PngColorType::Rgba8 => PngColorType::Rgba16,
            PngColorType::Grayscale8 => PngColorType::Grayscale16,
            PngColorType::GrayscaleAlpha8 => PngColorType::GrayscaleAlpha16,
            other => other,
        }
    }
}

#[inherent]
impl GrayscaleOps for PngColorType {
    #[inline]
    pub fn is_grayscale(&self) -> bool {
        ColorType::from(*self).is_grayscale()
    }

    #[inline]
    pub fn to_grayscale(self) -> Self {
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
    pub fn to_color(self) -> Self {
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

#[inherent]
impl AlphaChannelOps for PngColorType {
    #[inline]
    pub fn has_alpha(&self) -> bool {
        ColorType::from(*self).has_alpha()
    }
    #[inline]
    pub fn remove_alpha(self) -> Self {
        match self {
            PngColorType::Rgba8 => PngColorType::Rgb8,
            PngColorType::Rgba16 => PngColorType::Rgb16,
            PngColorType::GrayscaleAlpha8 => PngColorType::Grayscale8,
            PngColorType::GrayscaleAlpha16 => PngColorType::Grayscale16,
            other => other,
        }
    }
    #[inline]
    pub fn ensure_alpha(self) -> Self {
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
