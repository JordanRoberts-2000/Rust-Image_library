use {
    crate::{
        encoding::{AlphaChannelOps, BitDepthOps, ColorTypeOps, GrayscaleOps},
        image::Decoded,
        ImageError, ValidationError,
    },
    image::DynamicImage,
    inherent::inherent,
    std::{borrow::Cow, fmt},
    strum_macros::EnumIter,
    subenum::subenum,
};

#[subenum(AvifColorType, JpegColorType, PngColorType, WebpColorType, TiffColorType)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum ColorType {
    #[subenum(AvifColorType, JpegColorType, PngColorType, WebpColorType, TiffColorType)]
    Rgb8,
    #[subenum(AvifColorType, PngColorType, WebpColorType, TiffColorType)]
    Rgba8,
    #[subenum(JpegColorType, PngColorType, WebpColorType, TiffColorType)]
    Grayscale8,
    #[subenum(PngColorType, WebpColorType)]
    GrayscaleAlpha8,

    #[subenum(PngColorType, TiffColorType)]
    Rgb16,
    #[subenum(PngColorType, TiffColorType)]
    Rgba16,
    #[subenum(PngColorType, TiffColorType)]
    Grayscale16,
    #[subenum(PngColorType)]
    GrayscaleAlpha16,

    Rgb32Float,
    Rgba32Float,
}

#[inherent]
impl ColorTypeOps for ColorType {
    #[inline]
    pub fn channels(&self) -> u8 {
        match *self {
            ColorType::Rgb8 | ColorType::Rgb16 | ColorType::Rgb32Float => 3,
            ColorType::Rgba8 | ColorType::Rgba16 | ColorType::Rgba32Float => 4,
            ColorType::Grayscale8 | ColorType::Grayscale16 => 1,
            ColorType::GrayscaleAlpha8 | ColorType::GrayscaleAlpha16 => 2,
        }
    }

    #[inline]
    pub fn bit_depth(&self) -> u8 {
        match *self {
            ColorType::Rgb8
            | ColorType::Rgba8
            | ColorType::Grayscale8
            | ColorType::GrayscaleAlpha8 => 8,

            ColorType::Rgb16
            | ColorType::Rgba16
            | ColorType::Grayscale16
            | ColorType::GrayscaleAlpha16 => 16,

            ColorType::Rgb32Float | ColorType::Rgba32Float => 32,
        }
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
impl BitDepthOps for ColorType {
    #[inline]
    pub fn to_minimal_bit_depth(self) -> Self {
        match self {
            ColorType::Rgb16 => ColorType::Rgb8,
            ColorType::Rgba16 => ColorType::Rgba8,
            ColorType::Grayscale16 => ColorType::Grayscale8,
            ColorType::GrayscaleAlpha16 => ColorType::GrayscaleAlpha8,
            ColorType::Rgb32Float => ColorType::Rgb8,
            ColorType::Rgba32Float => ColorType::Rgba8,
            other => other,
        }
    }

    #[inline]
    pub fn to_maximal_bit_depth(self) -> Self {
        match self {
            ColorType::Rgb8 => ColorType::Rgb32Float,
            ColorType::Rgba8 => ColorType::Rgba32Float,
            ColorType::Rgb16 => ColorType::Rgb32Float,
            ColorType::Rgba16 => ColorType::Rgba32Float,
            ColorType::Grayscale8 => ColorType::Grayscale16,
            ColorType::GrayscaleAlpha8 => ColorType::GrayscaleAlpha16,
            other => other,
        }
    }
}

#[inherent]
impl AlphaChannelOps for ColorType {
    #[inline]
    pub fn has_alpha(&self) -> bool {
        matches!(
            *self,
            ColorType::Rgba8
                | ColorType::Rgba16
                | ColorType::Rgba32Float
                | ColorType::GrayscaleAlpha8
                | ColorType::GrayscaleAlpha16
        )
    }

    #[inline]
    pub fn remove_alpha(self) -> Self {
        match self {
            ColorType::Rgba8 => ColorType::Rgb8,
            ColorType::Rgba16 => ColorType::Rgb16,
            ColorType::Rgba32Float => ColorType::Rgb32Float,
            ColorType::GrayscaleAlpha8 => ColorType::Grayscale8,
            ColorType::GrayscaleAlpha16 => ColorType::Grayscale16,
            other => other,
        }
    }

    #[inline]
    pub fn ensure_alpha(self) -> Self {
        match self {
            ColorType::Rgb8 => ColorType::Rgba8,
            ColorType::Rgb16 => ColorType::Rgba16,
            ColorType::Rgb32Float => ColorType::Rgba32Float,
            ColorType::Grayscale8 => ColorType::GrayscaleAlpha8,
            ColorType::Grayscale16 => ColorType::GrayscaleAlpha16,
            with_alpha @ (ColorType::Rgba8
            | ColorType::Rgba16
            | ColorType::Rgba32Float
            | ColorType::GrayscaleAlpha8
            | ColorType::GrayscaleAlpha16) => with_alpha,
        }
    }
}

#[inherent]
impl GrayscaleOps for ColorType {
    #[inline]
    pub fn is_grayscale(&self) -> bool {
        matches!(
            *self,
            ColorType::Grayscale8
                | ColorType::GrayscaleAlpha8
                | ColorType::Grayscale16
                | ColorType::GrayscaleAlpha16
        )
    }

    #[inline]
    pub fn to_grayscale(self) -> Self {
        match self {
            ColorType::Rgb8 => ColorType::Grayscale8,
            ColorType::Rgb16 => ColorType::Grayscale16,
            ColorType::Rgb32Float => ColorType::Grayscale16, // No float grayscale variant
            ColorType::Rgba8 => ColorType::GrayscaleAlpha8,
            ColorType::Rgba16 => ColorType::GrayscaleAlpha16,
            ColorType::Rgba32Float => ColorType::GrayscaleAlpha16, // No float grayscale variant
            gray @ (ColorType::Grayscale8
            | ColorType::Grayscale16
            | ColorType::GrayscaleAlpha8
            | ColorType::GrayscaleAlpha16) => gray,
        }
    }

    #[inline]
    pub fn to_color(self) -> Self {
        match self {
            ColorType::Grayscale8 => ColorType::Rgb8,
            ColorType::Grayscale16 => ColorType::Rgb16,
            ColorType::GrayscaleAlpha8 => ColorType::Rgba8,
            ColorType::GrayscaleAlpha16 => ColorType::Rgba16,
            color @ (ColorType::Rgb8
            | ColorType::Rgb16
            | ColorType::Rgb32Float
            | ColorType::Rgba8
            | ColorType::Rgba16
            | ColorType::Rgba32Float) => color,
        }
    }
}

impl ColorType {
    pub(crate) fn bytes<'a>(&self, decoded: &'a Decoded) -> Cow<'a, [u8]> {
        match *self {
            ColorType::Grayscale8 => match decoded {
                Decoded::Static(img) => {
                    if let DynamicImage::ImageLuma8(b) = img {
                        Cow::Borrowed(b.as_raw())
                    } else {
                        Cow::Owned(img.to_luma8().into_raw())
                    }
                }
                Decoded::Animated { frames, .. } => Cow::Owned(
                    DynamicImage::ImageRgba8(frames.first().buffer().clone()).to_luma8().into_raw(),
                ),
            },

            ColorType::GrayscaleAlpha8 => match decoded {
                Decoded::Static(img) => {
                    if let DynamicImage::ImageLumaA8(b) = img {
                        Cow::Borrowed(b.as_raw())
                    } else {
                        Cow::Owned(img.to_luma_alpha8().into_raw())
                    }
                }
                Decoded::Animated { frames, .. } => Cow::Owned(
                    DynamicImage::ImageRgba8(frames.first().buffer().clone())
                        .to_luma_alpha8()
                        .into_raw(),
                ),
            },

            ColorType::Rgb8 => match decoded {
                Decoded::Static(img) => {
                    if let DynamicImage::ImageRgb8(b) = img {
                        Cow::Borrowed(b.as_raw())
                    } else {
                        Cow::Owned(img.to_rgb8().into_raw())
                    }
                }
                Decoded::Animated { frames, .. } => Cow::Owned(
                    DynamicImage::ImageRgba8(frames.first().buffer().clone()).to_rgb8().into_raw(),
                ),
            },

            ColorType::Rgba8 => match decoded {
                Decoded::Static(img) => {
                    if let DynamicImage::ImageRgba8(b) = img {
                        Cow::Borrowed(b.as_raw())
                    } else {
                        Cow::Owned(img.to_rgba8().into_raw())
                    }
                }
                Decoded::Animated { frames, .. } => Cow::Borrowed(frames.first().buffer().as_raw()),
            },

            ColorType::Grayscale16 => match decoded {
                Decoded::Static(img) => {
                    if let DynamicImage::ImageLuma16(b) = img {
                        Cow::Borrowed(bytemuck::cast_slice(b.as_raw()))
                    } else {
                        Cow::Owned(
                            bytemuck::cast_slice::<u16, u8>(img.to_luma16().as_raw()).to_vec(),
                        )
                    }
                }
                Decoded::Animated { frames, .. } => Cow::Owned(
                    bytemuck::cast_slice::<u16, u8>(
                        DynamicImage::ImageRgba8(frames.first().buffer().clone())
                            .to_luma16()
                            .as_raw(),
                    )
                    .to_vec(),
                ),
            },

            ColorType::GrayscaleAlpha16 => match decoded {
                Decoded::Static(img) => {
                    if let DynamicImage::ImageLumaA16(b) = img {
                        Cow::Borrowed(bytemuck::cast_slice(b.as_raw()))
                    } else {
                        Cow::Owned(
                            bytemuck::cast_slice::<u16, u8>(img.to_luma_alpha16().as_raw())
                                .to_vec(),
                        )
                    }
                }
                Decoded::Animated { frames, .. } => Cow::Owned(
                    bytemuck::cast_slice::<u16, u8>(
                        DynamicImage::ImageRgba8(frames.first().buffer().clone())
                            .to_luma_alpha16()
                            .as_raw(),
                    )
                    .to_vec(),
                ),
            },

            ColorType::Rgb16 => match decoded {
                Decoded::Static(img) => {
                    if let DynamicImage::ImageRgb16(b) = img {
                        Cow::Borrowed(bytemuck::cast_slice(b.as_raw()))
                    } else {
                        Cow::Owned(
                            bytemuck::cast_slice::<u16, u8>(img.to_rgb16().as_raw()).to_vec(),
                        )
                    }
                }
                Decoded::Animated { frames, .. } => Cow::Owned(
                    bytemuck::cast_slice::<u16, u8>(
                        DynamicImage::ImageRgba8(frames.first().buffer().clone())
                            .to_rgb16()
                            .as_raw(),
                    )
                    .to_vec(),
                ),
            },

            ColorType::Rgba16 => match decoded {
                Decoded::Static(img) => {
                    if let DynamicImage::ImageRgba16(b) = img {
                        Cow::Borrowed(bytemuck::cast_slice(b.as_raw()))
                    } else {
                        Cow::Owned(
                            bytemuck::cast_slice::<u16, u8>(img.to_rgba16().as_raw()).to_vec(),
                        )
                    }
                }
                Decoded::Animated { frames, .. } => Cow::Owned(
                    bytemuck::cast_slice::<u16, u8>(
                        DynamicImage::ImageRgba8(frames.first().buffer().clone())
                            .to_rgba16()
                            .as_raw(),
                    )
                    .to_vec(),
                ),
            },
            ColorType::Rgb32Float => match decoded {
                Decoded::Static(img) => {
                    if let DynamicImage::ImageRgb32F(b) = img {
                        Cow::Borrowed(bytemuck::cast_slice(b.as_raw()))
                    } else {
                        Cow::Owned(
                            bytemuck::cast_slice::<f32, u8>(img.to_rgb32f().as_raw()).to_vec(),
                        )
                    }
                }
                Decoded::Animated { frames, .. } => Cow::Owned(
                    bytemuck::cast_slice::<f32, u8>(
                        DynamicImage::ImageRgba8(frames.first().buffer().clone())
                            .to_rgb32f()
                            .as_raw(),
                    )
                    .to_vec(),
                ),
            },

            ColorType::Rgba32Float => match decoded {
                Decoded::Static(img) => {
                    if let DynamicImage::ImageRgba32F(b) = img {
                        Cow::Borrowed(bytemuck::cast_slice(b.as_raw()))
                    } else {
                        Cow::Owned(
                            bytemuck::cast_slice::<f32, u8>(img.to_rgba32f().as_raw()).to_vec(),
                        )
                    }
                }
                Decoded::Animated { frames, .. } => Cow::Owned(
                    bytemuck::cast_slice::<f32, u8>(
                        DynamicImage::ImageRgba8(frames.first().buffer().clone())
                            .to_rgba32f()
                            .as_raw(),
                    )
                    .to_vec(),
                ),
            },
        }
    }
}

impl fmt::Display for ColorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ColorType::Rgb8 => "rgb8",
            ColorType::Rgb16 => "rgb16",
            ColorType::Rgb32Float => "rgb32f",
            ColorType::Rgba8 => "rgba8",
            ColorType::Rgba16 => "rgba16",
            ColorType::Rgba32Float => "rgba32f",
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
            ColorType::Rgb32Float => image::ColorType::Rgb32F,
            ColorType::Rgba8 => image::ColorType::Rgba8,
            ColorType::Rgba16 => image::ColorType::Rgba16,
            ColorType::Rgba32Float => image::ColorType::Rgba32F,
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
            image::ColorType::Rgb32F => ColorType::Rgb32Float,
            image::ColorType::Rgba32F => ColorType::Rgba32Float,
            _ => return Err(ValidationError::UnsupportedColorType(ct).into()),
        })
    }
}
