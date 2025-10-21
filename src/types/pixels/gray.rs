use {
    crate::{encoding::ColorType, image::Decoded, PixelFormat, Result, ValidationError},
    image::{DynamicImage, ImageBuffer},
    std::{borrow::Cow, marker::PhantomData},
};

pub struct Gray<T>(PhantomData<T>);

impl PixelFormat for Gray<u8> {
    type Channel = u8;

    fn from_decoded<'a>(decoded: &'a Decoded) -> Cow<'a, [Self::Channel]> {
        match decoded {
            Decoded::Static(img) => match img {
                DynamicImage::ImageLuma8(l8_img) => Cow::Borrowed(l8_img.as_raw()),
                _ => Cow::Owned(img.to_luma8().into_raw()),
            },
            Decoded::Animated { frames, .. } => Cow::Owned(
                DynamicImage::ImageRgba8(frames.first().buffer().clone()).to_luma8().into_raw(),
            ),
        }
    }

    fn into_decoded(pixels: Vec<Self::Channel>, width: u32, height: u32) -> Result<Decoded> {
        let buffer = ImageBuffer::<image::Luma<Self::Channel>, _>::from_raw(width, height, pixels)
            .ok_or_else(|| ValidationError::InvalidBuffer(ColorType::Grayscale8))?;

        Ok(Decoded::Static(DynamicImage::ImageLuma8(buffer)))
    }
}

impl PixelFormat for Gray<u16> {
    type Channel = u16;

    fn from_decoded<'a>(decoded: &'a Decoded) -> Cow<'a, [Self::Channel]> {
        match decoded {
            Decoded::Static(img) => match img {
                DynamicImage::ImageLuma16(l16_img) => Cow::Borrowed(l16_img.as_raw()),
                _ => Cow::Owned(img.to_luma16().into_raw()),
            },
            Decoded::Animated { frames, .. } => Cow::Owned(
                DynamicImage::ImageRgba8(frames.first().buffer().clone()).to_luma16().into_raw(),
            ),
        }
    }

    fn into_decoded(pixels: Vec<Self::Channel>, width: u32, height: u32) -> Result<Decoded> {
        let buffer = ImageBuffer::<image::Luma<Self::Channel>, _>::from_raw(width, height, pixels)
            .ok_or_else(|| ValidationError::InvalidBuffer(ColorType::Grayscale16))?;

        Ok(Decoded::Static(DynamicImage::ImageLuma16(buffer)))
    }
}
