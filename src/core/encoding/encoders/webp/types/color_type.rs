use {
    crate::encoding::{
        AlphaChannelOps, ColorType, ColorTypeOps, EncodeColorTypeOps, GrayscaleOps, WebpColorType,
    },
    inherent::inherent,
    webp::PixelLayout,
};

impl EncodeColorTypeOps for WebpColorType {
    fn from_color_type_lossy(ct: ColorType) -> Self {
        match ct {
            ColorType::Grayscale8 => WebpColorType::Grayscale8,
            ColorType::Grayscale16 => WebpColorType::Grayscale8,
            ColorType::GrayscaleAlpha8 => WebpColorType::GrayscaleAlpha8,
            ColorType::GrayscaleAlpha16 => WebpColorType::GrayscaleAlpha8,
            ColorType::Rgb8 => WebpColorType::Rgb8,
            ColorType::Rgb16 => WebpColorType::Rgb8,
            ColorType::Rgba8 => WebpColorType::Rgba8,
            ColorType::Rgba16 => WebpColorType::Rgba8,
            ColorType::Rgb32Float => WebpColorType::Rgb8,
            ColorType::Rgba32Float => WebpColorType::Rgba8,
        }
    }

    fn to_minimal_bit_depth(self) -> Self {
        self
    }

    fn remove_alpha(self) -> Self {
        <Self as AlphaChannelOps>::remove_alpha(self)
    }

    fn has_alpha(&self) -> bool {
        <Self as AlphaChannelOps>::has_alpha(&self)
    }
}

#[inherent]
impl ColorTypeOps for WebpColorType {
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
        true
    }
}

#[inherent]
impl GrayscaleOps for WebpColorType {
    pub fn is_grayscale(&self) -> bool {
        ColorType::from(*self).is_grayscale()
    }

    pub fn to_grayscale(self) -> Self {
        match self {
            WebpColorType::Rgb8 => WebpColorType::Grayscale8,
            WebpColorType::Rgba8 => WebpColorType::GrayscaleAlpha8,
            other => other,
        }
    }

    pub fn to_color(self) -> Self {
        match self {
            WebpColorType::Grayscale8 => WebpColorType::Rgb8,
            WebpColorType::GrayscaleAlpha8 => WebpColorType::Rgba8,
            other => other,
        }
    }
}

#[inherent]
impl AlphaChannelOps for WebpColorType {
    pub fn has_alpha(&self) -> bool {
        ColorType::from(*self).has_alpha()
    }

    pub fn remove_alpha(self) -> Self {
        match self {
            WebpColorType::GrayscaleAlpha8 => WebpColorType::Grayscale8,
            WebpColorType::Rgba8 => WebpColorType::Rgb8,
            other => other,
        }
    }

    pub fn ensure_alpha(self) -> Self {
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
