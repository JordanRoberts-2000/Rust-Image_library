use {
    crate::{encoding::ColorType, image::Decoded, PixelFormat, Result, ValidationError},
    image::{DynamicImage, ImageBuffer},
    std::{borrow::Cow, marker::PhantomData},
};

pub struct GrayAlpha<T>(PhantomData<T>);

impl PixelFormat for GrayAlpha<u8> {
    type Channel = u8;

    fn from_decoded<'a>(decoded: &'a Decoded) -> Cow<'a, [Self::Channel]> {
        match decoded {
            Decoded::Static(img) => match img {
                DynamicImage::ImageLumaA8(la8_img) => Cow::Borrowed(la8_img.as_raw()),
                _ => Cow::Owned(img.to_luma_alpha8().into_raw()),
            },
            Decoded::Animated { frames, .. } => Cow::Owned(
                DynamicImage::ImageRgba8(frames.first().buffer().clone())
                    .to_luma_alpha8()
                    .into_raw(),
            ),
        }
    }

    fn into_decoded(pixels: Vec<Self::Channel>, width: u32, height: u32) -> Result<Decoded> {
        let buffer = ImageBuffer::<image::LumaA<Self::Channel>, _>::from_raw(width, height, pixels)
            .ok_or_else(|| ValidationError::InvalidBuffer(ColorType::GrayscaleAlpha8))?;

        Ok(Decoded::Static(DynamicImage::ImageLumaA8(buffer)))
    }
}

impl PixelFormat for GrayAlpha<u16> {
    type Channel = u16;

    fn from_decoded<'a>(decoded: &'a Decoded) -> Cow<'a, [Self::Channel]> {
        match decoded {
            Decoded::Static(img) => match img {
                DynamicImage::ImageLumaA16(la16_img) => Cow::Borrowed(la16_img.as_raw()),
                _ => Cow::Owned(img.to_luma_alpha16().into_raw()),
            },
            Decoded::Animated { frames, .. } => Cow::Owned(
                DynamicImage::ImageRgba8(frames.first().buffer().clone())
                    .to_luma_alpha16()
                    .into_raw(),
            ),
        }
    }

    fn into_decoded(pixels: Vec<Self::Channel>, width: u32, height: u32) -> Result<Decoded> {
        let buffer = ImageBuffer::<image::LumaA<Self::Channel>, _>::from_raw(width, height, pixels)
            .ok_or_else(|| ValidationError::InvalidBuffer(ColorType::GrayscaleAlpha16))?;

        Ok(Decoded::Static(DynamicImage::ImageLumaA16(buffer)))
    }
}
