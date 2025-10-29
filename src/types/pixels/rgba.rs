use {
    crate::{encoding::ColorType, image::Decoded, PixelFormat, Result, ValidationError},
    image::{DynamicImage, ImageBuffer},
    std::{borrow::Cow, marker::PhantomData},
};

pub struct Rgba<T>(PhantomData<T>);

impl PixelFormat for Rgba<u8> {
    type Channel = u8;

    fn from_decoded<'a>(decoded: &'a Decoded) -> Cow<'a, [Self::Channel]> {
        match decoded {
            Decoded::Static(img) => match img {
                DynamicImage::ImageRgba8(rgba8_img) => Cow::Borrowed(rgba8_img.as_raw()),
                _ => Cow::Owned(img.to_rgba8().into_raw()),
            },
            Decoded::Animated { frames, .. } => Cow::Borrowed(frames.first().buffer().as_raw()),
        }
    }

    fn into_decoded(pixels: Vec<Self::Channel>, width: u32, height: u32) -> Result<Decoded> {
        let buffer = ImageBuffer::<image::Rgba<Self::Channel>, _>::from_raw(width, height, pixels)
            .ok_or_else(|| ValidationError::InvalidBuffer(ColorType::Rgba8))?;

        Ok(Decoded::Static(DynamicImage::ImageRgba8(buffer)))
    }
}

impl PixelFormat for Rgba<u16> {
    type Channel = u16;

    fn from_decoded<'a>(decoded: &'a Decoded) -> Cow<'a, [Self::Channel]> {
        match decoded {
            Decoded::Static(img) => match img {
                DynamicImage::ImageRgba16(rgba16_img) => Cow::Borrowed(rgba16_img.as_raw()),
                _ => Cow::Owned(img.to_rgba16().into_raw()),
            },
            Decoded::Animated { frames, .. } => Cow::Owned(
                DynamicImage::ImageRgba8(frames.first().buffer().clone()).to_rgba16().into_raw(),
            ),
        }
    }

    fn into_decoded(pixels: Vec<Self::Channel>, width: u32, height: u32) -> Result<Decoded> {
        let buffer = ImageBuffer::<image::Rgba<Self::Channel>, _>::from_raw(width, height, pixels)
            .ok_or_else(|| ValidationError::InvalidBuffer(ColorType::Rgba16))?;

        Ok(Decoded::Static(DynamicImage::ImageRgba16(buffer)))
    }
}

impl PixelFormat for Rgba<f32> {
    type Channel = f32;

    fn from_decoded<'a>(decoded: &'a Decoded) -> Cow<'a, [Self::Channel]> {
        match decoded {
            Decoded::Static(img) => match img {
                DynamicImage::ImageRgba32F(rgba32f_img) => Cow::Borrowed(rgba32f_img.as_raw()),
                _ => Cow::Owned(img.to_rgba32f().into_raw()),
            },
            Decoded::Animated { frames, .. } => Cow::Owned(
                DynamicImage::ImageRgba8(frames.first().buffer().clone()).to_rgba32f().into_raw(),
            ),
        }
    }

    fn into_decoded(pixels: Vec<Self::Channel>, width: u32, height: u32) -> Result<Decoded> {
        let buffer = ImageBuffer::<image::Rgba<Self::Channel>, _>::from_raw(width, height, pixels)
            .ok_or_else(|| ValidationError::InvalidBuffer(ColorType::Rgba32Float))?;

        Ok(Decoded::Static(DynamicImage::ImageRgba32F(buffer)))
    }
}
