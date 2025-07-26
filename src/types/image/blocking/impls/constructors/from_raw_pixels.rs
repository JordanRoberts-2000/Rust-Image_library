use {
    crate::{
        blocking::Image,
        image::{blocking::ImageData, enums::ImageSrc, ImageConfig},
        ImageFormat, RawColorType, RawColorTypeF32, RawColorTypeU16, Result, ValidationError,
    },
    image::{DynamicImage, ImageBuffer, Luma, LumaA, Rgb, Rgba},
    std::{cell::RefCell, num::NonZeroU32, rc::Rc},
};

impl Image {
    pub fn from_raw_pixels(
        pixels: Vec<u8>, width: u32, height: u32, color_type: RawColorType,
    ) -> Result<Self> {
        let img: DynamicImage = match color_type {
            RawColorType::Rgb8 => ImageBuffer::<Rgb<u8>, _>::from_raw(width, height, pixels)
                .map(DynamicImage::ImageRgb8)
                .ok_or_else(|| ValidationError::InvalidBuffer(color_type))?,
            RawColorType::Rgba8 => ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, pixels)
                .map(DynamicImage::ImageRgba8)
                .ok_or_else(|| ValidationError::InvalidBuffer(color_type))?,
            RawColorType::L8 => ImageBuffer::<Luma<u8>, _>::from_raw(width, height, pixels)
                .map(DynamicImage::ImageLuma8)
                .ok_or_else(|| ValidationError::InvalidBuffer(color_type))?,
            RawColorType::La8 => ImageBuffer::<LumaA<u8>, _>::from_raw(width, height, pixels)
                .map(DynamicImage::ImageLumaA8)
                .ok_or_else(|| ValidationError::InvalidBuffer(color_type))?,
        };

        Ok(Self {
            src: ImageSrc::RawPixels,
            data: Rc::new(RefCell::new(ImageData::DynamicImage(img))),
            config: ImageConfig::default(),
            height: NonZeroU32::new(height).ok_or(ValidationError::InvalidHeight)?,
            width: NonZeroU32::new(width).ok_or(ValidationError::InvalidWidth)?,
            format: ImageFormat::default(),
        })
    }

    pub fn from_raw_pixels_u16(
        pixels: Vec<u16>, width: u32, height: u32, color_type: RawColorTypeU16,
    ) -> Result<Self> {
        let img: DynamicImage = match color_type {
            RawColorTypeU16::Rgb16 => ImageBuffer::<Rgb<u16>, _>::from_raw(width, height, pixels)
                .map(DynamicImage::ImageRgb16)
                .ok_or_else(|| ValidationError::InvalidBufferU16(color_type))?,
            RawColorTypeU16::Rgba16 => ImageBuffer::<Rgba<u16>, _>::from_raw(width, height, pixels)
                .map(DynamicImage::ImageRgba16)
                .ok_or_else(|| ValidationError::InvalidBufferU16(color_type))?,
            RawColorTypeU16::L16 => ImageBuffer::<Luma<u16>, _>::from_raw(width, height, pixels)
                .map(DynamicImage::ImageLuma16)
                .ok_or_else(|| ValidationError::InvalidBufferU16(color_type))?,
            RawColorTypeU16::La16 => ImageBuffer::<LumaA<u16>, _>::from_raw(width, height, pixels)
                .map(DynamicImage::ImageLumaA16)
                .ok_or_else(|| ValidationError::InvalidBufferU16(color_type))?,
        };

        Ok(Self {
            src: ImageSrc::RawPixels,
            data: Rc::new(RefCell::new(ImageData::DynamicImage(img))),
            config: ImageConfig::default(),
            height: NonZeroU32::new(height).ok_or(ValidationError::InvalidHeight)?,
            width: NonZeroU32::new(width).ok_or(ValidationError::InvalidWidth)?,
            format: ImageFormat::default(),
        })
    }

    pub fn from_raw_pixels_f32(
        pixels: Vec<f32>, width: u32, height: u32, color_type: RawColorTypeF32,
    ) -> Result<Self> {
        let img: DynamicImage = match color_type {
            RawColorTypeF32::Rgb32F => ImageBuffer::<Rgb<f32>, _>::from_raw(width, height, pixels)
                .map(DynamicImage::ImageRgb32F)
                .ok_or_else(|| ValidationError::InvalidBufferF32(color_type))?,
            RawColorTypeF32::Rgba32F => {
                ImageBuffer::<Rgba<f32>, _>::from_raw(width, height, pixels)
                    .map(DynamicImage::ImageRgba32F)
                    .ok_or_else(|| ValidationError::InvalidBufferF32(color_type))?
            }
        };

        Ok(Self {
            src: ImageSrc::RawPixels,
            data: Rc::new(RefCell::new(ImageData::DynamicImage(img))),
            config: ImageConfig::default(),
            height: NonZeroU32::new(height).ok_or(ValidationError::InvalidHeight)?,
            width: NonZeroU32::new(width).ok_or(ValidationError::InvalidWidth)?,
            format: ImageFormat::default(),
        })
    }
}
