use {
    crate::{BitDepth, ColorType},
    bytemuck::cast_slice,
    image::DynamicImage,
    std::borrow::Cow,
    strum_macros::EnumIter,
};

#[derive(Debug, Clone, Default, PartialEq, EnumIter)]
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
    pub(crate) fn bytes<'a>(&self, img: &'a DynamicImage) -> Cow<'a, [u8]> {
        match *self {
            PngColorType::Grayscale8 => {
                if let DynamicImage::ImageLuma8(b) = img {
                    Cow::Borrowed(b.as_raw())
                } else {
                    Cow::Owned(img.to_luma8().into_raw())
                }
            }
            PngColorType::GrayscaleAlpha8 => {
                if let DynamicImage::ImageLumaA8(b) = img {
                    Cow::Borrowed(b.as_raw())
                } else {
                    Cow::Owned(img.to_luma_alpha8().into_raw())
                }
            }
            PngColorType::Rgb8 => {
                if let DynamicImage::ImageRgb8(b) = img {
                    Cow::Borrowed(b.as_raw())
                } else {
                    Cow::Owned(img.to_rgb8().into_raw())
                }
            }
            PngColorType::Rgba8 => {
                if let DynamicImage::ImageRgba8(b) = img {
                    Cow::Borrowed(b.as_raw())
                } else {
                    Cow::Owned(img.to_rgba8().into_raw())
                }
            }
            PngColorType::Grayscale16 => {
                if let DynamicImage::ImageLuma16(b) = img {
                    Cow::Borrowed(cast_slice(b.as_raw()))
                } else {
                    let buf = img.to_luma16();
                    Cow::Owned(cast_slice::<u16, u8>(buf.as_raw()).to_vec())
                }
            }
            PngColorType::GrayscaleAlpha16 => {
                if let DynamicImage::ImageLumaA16(b) = img {
                    Cow::Borrowed(cast_slice(b.as_raw()))
                } else {
                    let buf = img.to_luma_alpha16();
                    Cow::Owned(cast_slice::<u16, u8>(buf.as_raw()).to_vec())
                }
            }
            PngColorType::Rgb16 => {
                if let DynamicImage::ImageRgb16(b) = img {
                    Cow::Borrowed(cast_slice(b.as_raw()))
                } else {
                    let buf = img.to_rgb16();
                    Cow::Owned(cast_slice::<u16, u8>(buf.as_raw()).to_vec())
                }
            }
            PngColorType::Rgba16 => {
                if let DynamicImage::ImageRgba16(b) = img {
                    Cow::Borrowed(cast_slice(b.as_raw()))
                } else {
                    let buf = img.to_rgba16();
                    Cow::Owned(cast_slice::<u16, u8>(buf.as_raw()).to_vec())
                }
            }
        }
    }

    pub fn has_alpha(&self) -> bool {
        use PngColorType::*;
        matches!(self, Rgba8 | Rgba16 | Grayscale8 | GrayscaleAlpha8)
    }

    pub fn channels(&self) -> usize {
        use PngColorType::*;
        match self {
            Rgb8 | Rgb16 => 3,
            Rgba8 | Rgba16 => 4,
            Grayscale8 | Grayscale16 => 1,
            GrayscaleAlpha8 | GrayscaleAlpha16 => 2,
        }
    }

    pub fn bit_depth(&self) -> BitDepth {
        use PngColorType::*;
        match self {
            Rgb8 | Rgba8 | Grayscale8 | GrayscaleAlpha8 => BitDepth::Eight,
            Rgb16 | Rgba16 | Grayscale16 | GrayscaleAlpha16 => BitDepth::Sixteen,
        }
    }

    pub fn is_grayscale(&self) -> bool {
        use PngColorType::*;
        matches!(self, Grayscale8 | GrayscaleAlpha8 | Grayscale16 | GrayscaleAlpha16)
    }

    pub(crate) fn to_minimal_bit_depth(self) -> Self {
        match self {
            PngColorType::Rgb16 => PngColorType::Rgb8,
            PngColorType::Rgba16 => PngColorType::Rgba8,
            PngColorType::Grayscale16 => PngColorType::Grayscale8,
            PngColorType::GrayscaleAlpha16 => PngColorType::GrayscaleAlpha8,
            other => other,
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

impl From<&PngColorType> for ColorType {
    fn from(color_type: &PngColorType) -> Self {
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
