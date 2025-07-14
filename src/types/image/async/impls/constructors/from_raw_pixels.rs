use {
    crate::{
        image::{
            enums::ImageSrc,
            r#async::{ImageData, ImageState},
            ImageConfig,
        },
        ColorType, Image, ImageError, ImageFormat, Result, ValidationError,
    },
    image::{DynamicImage, ImageBuffer, Luma, LumaA, Rgb, Rgba},
    std::{num::NonZeroU32, sync::Arc},
    tokio::sync::RwLock,
};

impl Image {
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

        let width =
            NonZeroU32::new(width).ok_or(ValidationError::InvalidDimensions(width, height))?;
        let height = NonZeroU32::new(height)
            .ok_or(ValidationError::InvalidDimensions(width.get(), height))?;

        let state = ImageState {
            config: ImageConfig::default(),
            data: ImageData::Decoded(img),
            height,
            width,
            format: ImageFormat::default(),
        };

        Ok(Self {
            src: ImageSrc::RawPixels,
            state: Arc::new(RwLock::new(state)),
        })
    }
}
