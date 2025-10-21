use {
    crate::{encoding::ColorType, image::Decoded, PixelFormat, Result, ValidationError},
    image::{DynamicImage, ImageBuffer},
    std::{borrow::Cow, marker::PhantomData},
};

pub struct Rgb<T>(PhantomData<T>);

impl PixelFormat for Rgb<u8> {
    type Channel = u8;

    fn from_decoded<'a>(decoded: &'a Decoded) -> Cow<'a, [Self::Channel]> {
        match decoded {
            Decoded::Static(img) => match img {
                DynamicImage::ImageRgb8(rgb8_img) => Cow::Borrowed(rgb8_img.as_raw()),
                _ => Cow::Owned(img.to_rgb8().into_raw()),
            },
            Decoded::Animated { frames, .. } => Cow::Owned(
                DynamicImage::ImageRgba8(frames.first().buffer().clone()).to_rgb8().into_raw(),
            ),
        }
    }

    fn into_decoded(pixels: Vec<Self::Channel>, width: u32, height: u32) -> Result<Decoded> {
        let buffer = ImageBuffer::<image::Rgb<Self::Channel>, _>::from_raw(width, height, pixels)
            .ok_or_else(|| ValidationError::InvalidBuffer(ColorType::Rgb8))?;

        Ok(Decoded::Static(DynamicImage::ImageRgb8(buffer)))
    }
}

impl PixelFormat for Rgb<u16> {
    type Channel = u16;
    fn from_decoded<'a>(decoded: &'a Decoded) -> Cow<'a, [Self::Channel]> {
        match decoded {
            Decoded::Static(img) => match img {
                DynamicImage::ImageRgb16(rgb16_img) => Cow::Borrowed(rgb16_img.as_raw()),
                _ => Cow::Owned(img.to_rgb16().into_raw()),
            },
            Decoded::Animated { frames, .. } => Cow::Owned(
                DynamicImage::ImageRgba8(frames.first().buffer().clone()).to_rgb16().into_raw(),
            ),
        }
    }

    fn into_decoded(pixels: Vec<Self::Channel>, width: u32, height: u32) -> Result<Decoded> {
        let buffer = ImageBuffer::<image::Rgb<Self::Channel>, _>::from_raw(width, height, pixels)
            .ok_or_else(|| ValidationError::InvalidBuffer(ColorType::Rgb16))?;

        Ok(Decoded::Static(DynamicImage::ImageRgb16(buffer)))
    }
}
