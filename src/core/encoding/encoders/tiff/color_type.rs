use {
    crate::encoding::{
        AlphaChannelOps, BitDepthOps, ColorType, ColorTypeOps, GrayscaleOps, TiffColorType,
    },
    inherent::inherent,
};

impl TiffColorType {
    pub fn from_color_type_lossy(ct: ColorType) -> Self {
        match ct {
            ColorType::Grayscale8 => TiffColorType::Grayscale8,
            ColorType::Grayscale16 => TiffColorType::Grayscale16,
            ColorType::GrayscaleAlpha8 => TiffColorType::Grayscale8,
            ColorType::GrayscaleAlpha16 => TiffColorType::Grayscale16,
            ColorType::Rgb8 => TiffColorType::Rgb8,
            ColorType::Rgb16 => TiffColorType::Rgb16,
            ColorType::Rgba8 => TiffColorType::Rgba8,
            ColorType::Rgba16 => TiffColorType::Rgba16,
        }
    }
}

#[inherent]
impl ColorTypeOps for TiffColorType {
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
impl GrayscaleOps for TiffColorType {
    #[inline]
    pub fn is_grayscale(&self) -> bool {
        ColorType::from(*self).is_grayscale()
    }

    #[inline]
    pub fn to_grayscale(self) -> Self {
        match self {
            TiffColorType::Rgb8 => TiffColorType::Grayscale8,
            TiffColorType::Rgb16 => TiffColorType::Grayscale16,
            TiffColorType::Rgba8 => TiffColorType::Grayscale8, // drop alpha
            TiffColorType::Rgba16 => TiffColorType::Grayscale16, // drop alpha
            g @ (TiffColorType::Grayscale8 | TiffColorType::Grayscale16) => g,
        }
    }

    #[inline]
    pub fn to_color(self) -> Self {
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

#[inherent]
impl BitDepthOps for TiffColorType {
    #[inline]
    pub fn to_minimal_bit_depth(self) -> Self {
        match self {
            TiffColorType::Rgb16 => TiffColorType::Rgb8,
            TiffColorType::Rgba16 => TiffColorType::Rgba8,
            TiffColorType::Grayscale16 => TiffColorType::Grayscale8,
            other => other,
        }
    }

    #[inline]
    pub fn to_maximal_bit_depth(self) -> Self {
        match self {
            TiffColorType::Rgb8 => TiffColorType::Rgb16,
            TiffColorType::Rgba8 => TiffColorType::Rgba16,
            TiffColorType::Grayscale8 => TiffColorType::Grayscale16,
            other => other,
        }
    }
}

#[inherent]
impl AlphaChannelOps for TiffColorType {
    #[inline]
    pub fn has_alpha(&self) -> bool {
        ColorType::from(*self).has_alpha()
    }

    #[inline]
    pub fn remove_alpha(self) -> Self {
        match self {
            TiffColorType::Rgba8 => TiffColorType::Rgb8,
            TiffColorType::Rgba16 => TiffColorType::Rgb16,
            other => other,
        }
    }

    #[inline]
    pub fn ensure_alpha(self) -> Self {
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
