use {
    crate::{BitDepth, ColorType},
    image::DynamicImage,
    std::borrow::Cow,
    strum_macros::EnumIter,
    webp::PixelLayout,
};

#[derive(Debug, Clone, Default, PartialEq, EnumIter)]
pub enum WebPColorType {
    Grayscale8,
    GrayscaleAlpha8,
    Rgb8,
    #[default]
    Rgba8,
}

impl WebPColorType {
    pub(crate) fn bytes<'a>(&self, img: &'a DynamicImage) -> Cow<'a, [u8]> {
        match self {
            WebPColorType::Grayscale8 => {
                if let DynamicImage::ImageLuma8(b) = img {
                    Cow::Borrowed(b.as_raw())
                } else {
                    Cow::Owned(img.to_luma8().into_raw())
                }
            }
            WebPColorType::GrayscaleAlpha8 => {
                if let DynamicImage::ImageLumaA8(b) = img {
                    Cow::Borrowed(b.as_raw())
                } else {
                    Cow::Owned(img.to_luma_alpha8().into_raw())
                }
            }
            WebPColorType::Rgb8 => {
                if let DynamicImage::ImageRgb8(b) = img {
                    Cow::Borrowed(b.as_raw())
                } else {
                    Cow::Owned(img.to_rgb8().into_raw())
                }
            }
            WebPColorType::Rgba8 => {
                if let DynamicImage::ImageRgba8(b) = img {
                    Cow::Borrowed(b.as_raw())
                } else {
                    Cow::Owned(img.to_rgba8().into_raw())
                }
            }
        }
    }

    pub fn has_alpha(&self) -> bool {
        use WebPColorType::*;
        match self {
            Grayscale8 | Rgb8 => false,
            GrayscaleAlpha8 | Rgba8 => true,
        }
    }

    pub fn channels(&self) -> usize {
        use WebPColorType::*;
        match self {
            Grayscale8 => 1,
            GrayscaleAlpha8 => 2,
            Rgb8 => 3,
            Rgba8 => 4,
        }
    }

    pub fn bit_depth(&self) -> BitDepth {
        BitDepth::Eight
    }

    pub fn is_grayscale(&self) -> bool {
        use WebPColorType::*;
        matches!(self, Grayscale8 | GrayscaleAlpha8)
    }
}

impl From<WebPColorType> for image::ExtendedColorType {
    fn from(color_type: WebPColorType) -> Self {
        match color_type {
            WebPColorType::Rgb8 => image::ExtendedColorType::Rgb8,
            WebPColorType::Rgba8 => image::ExtendedColorType::Rgba8,
            WebPColorType::Grayscale8 => image::ExtendedColorType::L8,
            WebPColorType::GrayscaleAlpha8 => image::ExtendedColorType::La8,
        }
    }
}

impl From<WebPColorType> for image::ColorType {
    fn from(color_type: WebPColorType) -> Self {
        match color_type {
            WebPColorType::Rgb8 => image::ColorType::Rgb8,
            WebPColorType::Rgba8 => image::ColorType::Rgba8,
            WebPColorType::Grayscale8 => image::ColorType::L8,
            WebPColorType::GrayscaleAlpha8 => image::ColorType::La8,
        }
    }
}

impl From<WebPColorType> for PixelLayout {
    fn from(color_type: WebPColorType) -> Self {
        match color_type {
            WebPColorType::Rgb8 | WebPColorType::Grayscale8 => PixelLayout::Rgb,
            WebPColorType::Rgba8 | WebPColorType::GrayscaleAlpha8 => PixelLayout::Rgba,
        }
    }
}

impl From<WebPColorType> for ColorType {
    fn from(color_type: WebPColorType) -> Self {
        match color_type {
            WebPColorType::Grayscale8 => ColorType::Grayscale8,
            WebPColorType::GrayscaleAlpha8 => ColorType::GrayscaleAlpha8,
            WebPColorType::Rgb8 => ColorType::Rgb8,
            WebPColorType::Rgba8 => ColorType::Rgba8,
        }
    }
}

impl From<&WebPColorType> for ColorType {
    fn from(color_type: &WebPColorType) -> Self {
        match color_type {
            WebPColorType::Grayscale8 => ColorType::Grayscale8,
            WebPColorType::GrayscaleAlpha8 => ColorType::GrayscaleAlpha8,
            WebPColorType::Rgb8 => ColorType::Rgb8,
            WebPColorType::Rgba8 => ColorType::Rgba8,
        }
    }
}
