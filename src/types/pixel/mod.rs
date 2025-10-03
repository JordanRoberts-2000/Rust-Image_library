use {
    crate::{ColorType, PixelFormat, Result, ValidationError},
    image::{DynamicImage, ImageBuffer, Luma, LumaA, Rgb, Rgba},
};

pub struct Rgb8;
impl PixelFormat for Rgb8 {
    type Channel = u8;
    fn extract_raw_pixels(img: &DynamicImage) -> Vec<Self::Channel> {
        img.to_rgb8().into_raw()
    }

    fn create_image_from_raw(
        pixels: Vec<Self::Channel>, width: u32, height: u32,
    ) -> Result<DynamicImage> {
        let buffer =
            ImageBuffer::<Rgb<Self::Channel>, Vec<Self::Channel>>::from_raw(width, height, pixels)
                .ok_or_else(|| ValidationError::InvalidBuffer(ColorType::Rgb8))?;

        Ok(DynamicImage::ImageRgb8(buffer))
    }
}

pub struct Rgba8;
impl PixelFormat for Rgba8 {
    type Channel = u8;

    fn extract_raw_pixels(img: &DynamicImage) -> Vec<Self::Channel> {
        img.to_rgba8().into_raw()
    }

    fn create_image_from_raw(
        pixels: Vec<Self::Channel>, width: u32, height: u32,
    ) -> Result<DynamicImage> {
        let buffer =
            ImageBuffer::<Rgba<Self::Channel>, Vec<Self::Channel>>::from_raw(width, height, pixels)
                .ok_or_else(|| ValidationError::InvalidBuffer(ColorType::Rgba8))?;
        Ok(DynamicImage::ImageRgba8(buffer))
    }
}

pub struct Rgb16;
impl PixelFormat for Rgb16 {
    type Channel = u16;
    fn extract_raw_pixels(img: &DynamicImage) -> Vec<Self::Channel> {
        img.to_rgb16().into_raw()
    }

    fn create_image_from_raw(
        pixels: Vec<Self::Channel>, width: u32, height: u32,
    ) -> Result<DynamicImage> {
        let buffer =
            ImageBuffer::<Rgb<Self::Channel>, Vec<Self::Channel>>::from_raw(width, height, pixels)
                .ok_or_else(|| ValidationError::InvalidBuffer(ColorType::Rgb16))?;

        Ok(DynamicImage::ImageRgb16(buffer))
    }
}

pub struct Rgba16;
impl PixelFormat for Rgba16 {
    type Channel = u16;

    fn extract_raw_pixels(img: &DynamicImage) -> Vec<Self::Channel> {
        img.to_rgba16().into_raw()
    }

    fn create_image_from_raw(
        pixels: Vec<Self::Channel>, width: u32, height: u32,
    ) -> Result<DynamicImage> {
        let buffer =
            ImageBuffer::<Rgba<Self::Channel>, Vec<Self::Channel>>::from_raw(width, height, pixels)
                .ok_or_else(|| ValidationError::InvalidBuffer(ColorType::Rgba16))?;
        Ok(DynamicImage::ImageRgba16(buffer))
    }
}

pub struct L8;
impl PixelFormat for L8 {
    type Channel = u8;

    fn extract_raw_pixels(img: &DynamicImage) -> Vec<Self::Channel> {
        img.to_luma8().into_raw()
    }

    fn create_image_from_raw(
        pixels: Vec<Self::Channel>, width: u32, height: u32,
    ) -> Result<DynamicImage> {
        let buffer =
            ImageBuffer::<Luma<Self::Channel>, Vec<Self::Channel>>::from_raw(width, height, pixels)
                .ok_or_else(|| ValidationError::InvalidBuffer(ColorType::Grayscale8))?;
        Ok(DynamicImage::ImageLuma8(buffer))
    }
}

pub struct La8;
impl PixelFormat for La8 {
    type Channel = u8;

    fn extract_raw_pixels(img: &DynamicImage) -> Vec<Self::Channel> {
        img.to_luma_alpha8().into_raw()
    }

    fn create_image_from_raw(
        pixels: Vec<Self::Channel>, width: u32, height: u32,
    ) -> Result<DynamicImage> {
        let buffer = ImageBuffer::<LumaA<Self::Channel>, Vec<Self::Channel>>::from_raw(
            width, height, pixels,
        )
        .ok_or_else(|| ValidationError::InvalidBuffer(ColorType::GrayscaleAlpha8))?;
        Ok(DynamicImage::ImageLumaA8(buffer))
    }
}

pub struct L16;
impl PixelFormat for L16 {
    type Channel = u16;

    fn extract_raw_pixels(img: &DynamicImage) -> Vec<Self::Channel> {
        img.to_luma16().into_raw()
    }

    fn create_image_from_raw(
        pixels: Vec<Self::Channel>, width: u32, height: u32,
    ) -> Result<DynamicImage> {
        let buffer =
            ImageBuffer::<Luma<Self::Channel>, Vec<Self::Channel>>::from_raw(width, height, pixels)
                .ok_or_else(|| ValidationError::InvalidBuffer(ColorType::Grayscale16))?;
        Ok(DynamicImage::ImageLuma16(buffer))
    }
}

pub struct La16;
impl PixelFormat for La16 {
    type Channel = u16;

    fn extract_raw_pixels(img: &DynamicImage) -> Vec<Self::Channel> {
        img.to_luma_alpha16().into_raw()
    }

    fn create_image_from_raw(
        pixels: Vec<Self::Channel>, width: u32, height: u32,
    ) -> Result<DynamicImage> {
        let buffer = ImageBuffer::<LumaA<Self::Channel>, Vec<Self::Channel>>::from_raw(
            width, height, pixels,
        )
        .ok_or_else(|| ValidationError::InvalidBuffer(ColorType::GrayscaleAlpha16))?;
        Ok(DynamicImage::ImageLumaA16(buffer))
    }
}
