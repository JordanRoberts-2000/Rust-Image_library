use image::{DynamicImage, ImageBuffer, Luma, LumaA, Rgb, Rgba};

use crate::{
    BlockingImage, ColorType, ImageConfig, ImageData, ImageError, ImageFormat, ImageSrc, Result,
};

impl BlockingImage {
    pub fn from_raw_pixels(
        pixels: Vec<u8>,
        width: u32,
        height: u32,
        color_type: ColorType,
    ) -> Result<Self> {
        let img: DynamicImage = match color_type {
            ColorType::Rgb8 => ImageBuffer::<Rgb<u8>, _>::from_raw(width, height, pixels)
                .map(DynamicImage::ImageRgb8)
                .ok_or_else(|| ImageError::InvalidBuffer(color_type))?,
            ColorType::Rgba8 => ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, pixels)
                .map(DynamicImage::ImageRgba8)
                .ok_or_else(|| ImageError::InvalidBuffer(color_type))?,
            ColorType::L8 => ImageBuffer::<Luma<u8>, _>::from_raw(width, height, pixels)
                .map(DynamicImage::ImageLuma8)
                .ok_or_else(|| ImageError::InvalidBuffer(color_type))?,
            ColorType::La8 => ImageBuffer::<LumaA<u8>, _>::from_raw(width, height, pixels)
                .map(DynamicImage::ImageLumaA8)
                .ok_or_else(|| ImageError::InvalidBuffer(color_type))?,
        };

        Ok(Self {
            src: ImageSrc::RawPixels,
            data: ImageData::Decoded(img),
            config: ImageConfig::default(),
            height,
            width,
            aspect_ratio: width as f32 / height as f32,
            format: ImageFormat::default(),
        })
    }
}
