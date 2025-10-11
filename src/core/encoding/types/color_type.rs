use {
    crate::{
        encoding::{
            macros::{
                forward_color_type_impls, forward_grayscale_impls, forward_transparency_impls,
            },
            AlphaChannelOps, ColorTypeOps, GrayscaleOps,
        },
        ImageError, ValidationError,
    },
    bytemuck::cast_slice,
    image::DynamicImage,
    std::{borrow::Cow, fmt},
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

impl AlphaChannelOps for ColorType {
    #[inline]
    fn has_alpha(&self) -> bool {
        matches!(
            *self,
            ColorType::Rgba8
                | ColorType::Rgba16
                | ColorType::GrayscaleAlpha8
                | ColorType::GrayscaleAlpha16
        )
    }

    #[inline]
    fn remove_alpha(self) -> Self {
        match self {
            ColorType::Rgba8 => ColorType::Rgb8,
            ColorType::Rgba16 => ColorType::Rgb16,
            ColorType::GrayscaleAlpha8 => ColorType::Grayscale8,
            ColorType::GrayscaleAlpha16 => ColorType::Grayscale16,
            other => other,
        }
    }

    #[inline]
    fn ensure_alpha(self) -> Self {
        match self {
            ColorType::Rgb8 => ColorType::Rgba8,
            ColorType::Rgb16 => ColorType::Rgba16,
            ColorType::Grayscale8 => ColorType::GrayscaleAlpha8,
            ColorType::Grayscale16 => ColorType::GrayscaleAlpha16,
            with_alpha @ (ColorType::Rgba8
            | ColorType::Rgba16
            | ColorType::GrayscaleAlpha8
            | ColorType::GrayscaleAlpha16) => with_alpha,
        }
    }
}

impl ColorTypeOps for ColorType {
    #[inline]
    fn channels(&self) -> u8 {
        match *self {
            ColorType::Rgb8 | ColorType::Rgb16 => 3,
            ColorType::Rgba8 | ColorType::Rgba16 => 4,
            ColorType::Grayscale8 | ColorType::Grayscale16 => 1,
            ColorType::GrayscaleAlpha8 | ColorType::GrayscaleAlpha16 => 2,
        }
    }

    #[inline]
    fn bit_depth(&self) -> u8 {
        match *self {
            ColorType::Rgb8
            | ColorType::Rgba8
            | ColorType::Grayscale8
            | ColorType::GrayscaleAlpha8 => 8,

            ColorType::Rgb16
            | ColorType::Rgba16
            | ColorType::Grayscale16
            | ColorType::GrayscaleAlpha16 => 16,
        }
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

impl GrayscaleOps for ColorType {
    #[inline]
    fn is_grayscale(&self) -> bool {
        matches!(
            *self,
            ColorType::Grayscale8
                | ColorType::GrayscaleAlpha8
                | ColorType::Grayscale16
                | ColorType::GrayscaleAlpha16
        )
    }

    #[inline]
    fn to_grayscale(self) -> Self {
        match self {
            ColorType::Rgb8 => ColorType::Grayscale8,
            ColorType::Rgb16 => ColorType::Grayscale16,
            ColorType::Rgba8 => ColorType::GrayscaleAlpha8,
            ColorType::Rgba16 => ColorType::GrayscaleAlpha16,
            gray @ (ColorType::Grayscale8
            | ColorType::Grayscale16
            | ColorType::GrayscaleAlpha8
            | ColorType::GrayscaleAlpha16) => gray,
        }
    }

    #[inline]
    fn to_color(self) -> Self {
        match self {
            ColorType::Grayscale8 => ColorType::Rgb8,
            ColorType::Grayscale16 => ColorType::Rgb16,
            ColorType::GrayscaleAlpha8 => ColorType::Rgba8,
            ColorType::GrayscaleAlpha16 => ColorType::Rgba16,
            color @ (ColorType::Rgb8 | ColorType::Rgb16 | ColorType::Rgba8 | ColorType::Rgba16) => {
                color
            }
        }
    }
}

impl ColorType {
    forward_grayscale_impls!();
    forward_color_type_impls!();
    forward_transparency_impls!();

    #[inline]
    pub fn to_minimal_bit_depth(self) -> Self {
        match self {
            ColorType::Rgb16 => ColorType::Rgb8,
            ColorType::Rgba16 => ColorType::Rgba8,
            ColorType::Grayscale16 => ColorType::Grayscale8,
            ColorType::GrayscaleAlpha16 => ColorType::GrayscaleAlpha8,
            other => other,
        }
    }

    pub(crate) fn bytes<'a>(&self, img: &'a DynamicImage) -> Cow<'a, [u8]> {
        match *self {
            ColorType::Grayscale8 => {
                if let DynamicImage::ImageLuma8(b) = img {
                    Cow::Borrowed(b.as_raw())
                } else {
                    Cow::Owned(img.to_luma8().into_raw())
                }
            }
            ColorType::GrayscaleAlpha8 => {
                if let DynamicImage::ImageLumaA8(b) = img {
                    Cow::Borrowed(b.as_raw())
                } else {
                    Cow::Owned(img.to_luma_alpha8().into_raw())
                }
            }
            ColorType::Rgb8 => {
                if let DynamicImage::ImageRgb8(b) = img {
                    Cow::Borrowed(b.as_raw())
                } else {
                    Cow::Owned(img.to_rgb8().into_raw())
                }
            }
            ColorType::Rgba8 => {
                if let DynamicImage::ImageRgba8(b) = img {
                    Cow::Borrowed(b.as_raw())
                } else {
                    Cow::Owned(img.to_rgba8().into_raw())
                }
            }
            ColorType::Grayscale16 => {
                if let DynamicImage::ImageLuma16(b) = img {
                    Cow::Borrowed(cast_slice(b.as_raw()))
                } else {
                    let buf = img.to_luma16();
                    Cow::Owned(cast_slice::<u16, u8>(buf.as_raw()).to_vec())
                }
            }
            ColorType::GrayscaleAlpha16 => {
                if let DynamicImage::ImageLumaA16(b) = img {
                    Cow::Borrowed(cast_slice(b.as_raw()))
                } else {
                    let buf = img.to_luma_alpha16();
                    Cow::Owned(cast_slice::<u16, u8>(buf.as_raw()).to_vec())
                }
            }
            ColorType::Rgb16 => {
                if let DynamicImage::ImageRgb16(b) = img {
                    Cow::Borrowed(cast_slice(b.as_raw()))
                } else {
                    let buf = img.to_rgb16();
                    Cow::Owned(cast_slice::<u16, u8>(buf.as_raw()).to_vec())
                }
            }
            ColorType::Rgba16 => {
                if let DynamicImage::ImageRgba16(b) = img {
                    Cow::Borrowed(cast_slice(b.as_raw()))
                } else {
                    let buf = img.to_rgba16();
                    Cow::Owned(cast_slice::<u16, u8>(buf.as_raw()).to_vec())
                }
            }
        }
    }
}

impl fmt::Display for ColorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ColorType::Rgb8 => "rgb8",
            ColorType::Rgb16 => "rgb16",
            ColorType::Rgba8 => "rgba8",
            ColorType::Rgba16 => "rgba16",
            ColorType::Grayscale8 => "grayscale8",
            ColorType::GrayscaleAlpha8 => "grayscale-alpha8",
            ColorType::Grayscale16 => "grayscale16",
            ColorType::GrayscaleAlpha16 => "grayscale-alpha16",
        };
        f.write_str(s)
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
