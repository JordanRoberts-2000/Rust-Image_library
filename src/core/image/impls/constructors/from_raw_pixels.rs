use {
    crate::{
        image::{
            enums::{ImageData, ImageSrc},
            ImageConfig,
        },
        Image, ImageFormat, RawColorType, RawColorTypeF32, RawColorTypeU16, Result,
        ValidationError,
    },
    image::{DynamicImage, ImageBuffer, Luma, LumaA, Rgb, Rgba},
    std::num::NonZeroU32,
};

impl Image {
    pub fn from_raw_pixels(
        pixels: impl AsRef<[u8]>, width: u32, height: u32, color_type: RawColorType,
    ) -> Result<Self> {
        let img: DynamicImage = match color_type {
            RawColorType::Rgb8 => {
                ImageBuffer::<Rgb<u8>, _>::from_raw(width, height, pixels.as_ref().to_vec())
                    .map(DynamicImage::ImageRgb8)
                    .ok_or_else(|| ValidationError::InvalidBuffer(color_type))?
            }
            RawColorType::Rgba8 => {
                ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, pixels.as_ref().to_vec())
                    .map(DynamicImage::ImageRgba8)
                    .ok_or_else(|| ValidationError::InvalidBuffer(color_type))?
            }
            RawColorType::L8 => {
                ImageBuffer::<Luma<u8>, _>::from_raw(width, height, pixels.as_ref().to_vec())
                    .map(DynamicImage::ImageLuma8)
                    .ok_or_else(|| ValidationError::InvalidBuffer(color_type))?
            }
            RawColorType::La8 => {
                ImageBuffer::<LumaA<u8>, _>::from_raw(width, height, pixels.as_ref().to_vec())
                    .map(DynamicImage::ImageLumaA8)
                    .ok_or_else(|| ValidationError::InvalidBuffer(color_type))?
            }
        };

        Ok(Self {
            src: ImageSrc::RawPixels,
            data: Some(ImageData::RawPixels(img)),
            config: ImageConfig::default(),
            height: NonZeroU32::new(height).ok_or(ValidationError::InvalidHeight)?,
            width: NonZeroU32::new(width).ok_or(ValidationError::InvalidWidth)?,
            format: ImageFormat::default(),
        })
    }

    pub fn from_raw_pixels_u16(
        pixels: impl AsRef<[u16]>, width: u32, height: u32, color_type: RawColorTypeU16,
    ) -> Result<Self> {
        let img: DynamicImage = match color_type {
            RawColorTypeU16::Rgb16 => {
                ImageBuffer::<Rgb<u16>, _>::from_raw(width, height, pixels.as_ref().to_vec())
                    .map(DynamicImage::ImageRgb16)
                    .ok_or_else(|| ValidationError::InvalidBufferU16(color_type))?
            }
            RawColorTypeU16::Rgba16 => {
                ImageBuffer::<Rgba<u16>, _>::from_raw(width, height, pixels.as_ref().to_vec())
                    .map(DynamicImage::ImageRgba16)
                    .ok_or_else(|| ValidationError::InvalidBufferU16(color_type))?
            }
            RawColorTypeU16::L16 => {
                ImageBuffer::<Luma<u16>, _>::from_raw(width, height, pixels.as_ref().to_vec())
                    .map(DynamicImage::ImageLuma16)
                    .ok_or_else(|| ValidationError::InvalidBufferU16(color_type))?
            }
            RawColorTypeU16::La16 => {
                ImageBuffer::<LumaA<u16>, _>::from_raw(width, height, pixels.as_ref().to_vec())
                    .map(DynamicImage::ImageLumaA16)
                    .ok_or_else(|| ValidationError::InvalidBufferU16(color_type))?
            }
        };

        Ok(Self {
            src: ImageSrc::RawPixels,
            data: Some(ImageData::RawPixels(img)),
            config: ImageConfig::default(),
            height: NonZeroU32::new(height).ok_or(ValidationError::InvalidHeight)?,
            width: NonZeroU32::new(width).ok_or(ValidationError::InvalidWidth)?,
            format: ImageFormat::default(),
        })
    }

    pub fn from_raw_pixels_f32(
        pixels: impl AsRef<[f32]>, width: u32, height: u32, color_type: RawColorTypeF32,
    ) -> Result<Self> {
        let img: DynamicImage = match color_type {
            RawColorTypeF32::Rgb32F => {
                ImageBuffer::<Rgb<f32>, _>::from_raw(width, height, pixels.as_ref().to_vec())
                    .map(DynamicImage::ImageRgb32F)
                    .ok_or_else(|| ValidationError::InvalidBufferF32(color_type))?
            }
            RawColorTypeF32::Rgba32F => {
                ImageBuffer::<Rgba<f32>, _>::from_raw(width, height, pixels.as_ref().to_vec())
                    .map(DynamicImage::ImageRgba32F)
                    .ok_or_else(|| ValidationError::InvalidBufferF32(color_type))?
            }
        };

        Ok(Self {
            src: ImageSrc::RawPixels,
            data: Some(ImageData::RawPixels(img)),
            config: ImageConfig::default(),
            height: NonZeroU32::new(height).ok_or(ValidationError::InvalidHeight)?,
            width: NonZeroU32::new(width).ok_or(ValidationError::InvalidWidth)?,
            format: ImageFormat::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ------ helpers ------

    fn seq_u8(n: usize) -> Vec<u8> {
        (0..n).map(|i| i as u8).collect()
    }
    fn seq_u16(n: usize) -> Vec<u16> {
        (0..n).map(|i| i as u16).collect()
    }
    fn seq_f32(n: usize) -> Vec<f32> {
        (0..n).map(|i| (i as f32) / 255.0).collect()
    }

    fn assert_src_rawpixels(img: &Image) {
        match img.src {
            ImageSrc::RawPixels => {}
            _ => panic!("expected ImageSrc::RawPixels"),
        }
    }

    // ------ u8 variants ------

    #[test]
    fn from_raw_pixels_rgb8_ok() -> Result<()> {
        let (w, h, c) = (2, 2, 3);
        let pixels = seq_u8((w * h * c) as usize);
        let img = Image::from_raw_pixels(&pixels, w, h, RawColorType::Rgb8)?;

        assert_src_rawpixels(&img);

        // Check pixel content via conversion
        let raw = match &img.data {
            Some(ImageData::RawPixels(di)) => di.to_rgb8().into_raw(),
            _ => panic!("expected Some(RawPixels)"),
        };
        assert_eq!(raw, pixels);
        Ok(())
    }

    #[test]
    fn from_raw_pixels_rgba8_ok() -> Result<()> {
        let (w, h, c) = (2, 2, 4);
        let pixels = seq_u8((w * h * c) as usize);
        let img = Image::from_raw_pixels(&pixels, w, h, RawColorType::Rgba8)?;
        assert_src_rawpixels(&img);
        assert_eq!(img.width.get(), w);
        assert_eq!(img.height.get(), h);

        let raw = match &img.data {
            Some(ImageData::RawPixels(di)) => di.to_rgba8().into_raw(),
            _ => panic!("expected Some(RawPixels)"),
        };
        assert_eq!(raw, pixels);
        Ok(())
    }

    #[test]
    fn from_raw_pixels_l8_ok() -> Result<()> {
        let (w, h, c) = (3, 2, 1);
        let pixels = seq_u8((w * h * c) as usize);
        let img = Image::from_raw_pixels(&pixels, w, h, RawColorType::L8)?;
        assert_src_rawpixels(&img);
        assert_eq!(img.width.get(), w);
        assert_eq!(img.height.get(), h);

        let raw = match &img.data {
            Some(ImageData::RawPixels(di)) => di.to_luma8().into_raw(),
            _ => panic!("expected Some(RawPixels)"),
        };
        assert_eq!(raw, pixels);
        Ok(())
    }

    #[test]
    fn from_raw_pixels_la8_ok() -> Result<()> {
        let (w, h, c) = (3, 2, 2);
        let pixels = seq_u8((w * h * c) as usize);
        let img = Image::from_raw_pixels(&pixels, w, h, RawColorType::La8)?;
        assert_src_rawpixels(&img);
        assert_eq!(img.width.get(), w);
        assert_eq!(img.height.get(), h);

        let raw = match &img.data {
            Some(ImageData::RawPixels(di)) => di.to_luma_alpha8().into_raw(),
            _ => panic!("expected Some(RawPixels)"),
        };
        assert_eq!(raw, pixels);
        Ok(())
    }

    #[test]
    fn from_raw_pixels_u8_zero_dims_err() {
        let pixels = seq_u8(0);
        assert!(Image::from_raw_pixels(&pixels, 0, 2, RawColorType::Rgb8).is_err());
        assert!(Image::from_raw_pixels(&pixels, 2, 0, RawColorType::Rgb8).is_err());
    }

    // ------ u16 variants ------

    #[test]
    fn from_raw_pixels_u16_rgb16_ok() -> Result<()> {
        let (w, h, c) = (2, 2, 3);
        let pixels = seq_u16((w * h * c) as usize);
        let img = Image::from_raw_pixels_u16(&pixels, w, h, RawColorTypeU16::Rgb16)?;
        assert_src_rawpixels(&img);
        assert_eq!(img.width.get(), w);
        assert_eq!(img.height.get(), h);

        let raw: Vec<u16> = match &img.data {
            Some(ImageData::RawPixels(di)) => di.to_rgb16().into_raw(),
            _ => panic!("expected Some(RawPixels)"),
        };
        assert_eq!(raw, pixels);
        Ok(())
    }

    #[test]
    fn from_raw_pixels_u16_zero_dims_err() {
        let pixels = seq_u16(0);
        assert!(Image::from_raw_pixels_u16(&pixels, 0, 2, RawColorTypeU16::Rgb16).is_err());
        assert!(Image::from_raw_pixels_u16(&pixels, 2, 0, RawColorTypeU16::Rgb16).is_err());
    }

    // ------ f32 variants ------

    #[test]
    fn from_raw_pixels_f32_rgba32f_ok() -> Result<()> {
        let (w, h, c) = (2, 2, 4);
        let pixels = seq_f32((w * h * c) as usize);
        let img = Image::from_raw_pixels_f32(&pixels, w, h, RawColorTypeF32::Rgba32F)?;
        assert_src_rawpixels(&img);
        assert_eq!(img.width.get(), w);
        assert_eq!(img.height.get(), h);

        let raw: Vec<f32> = match &img.data {
            Some(ImageData::RawPixels(di)) => di.to_rgba32f().into_raw(),
            _ => panic!("expected Some(RawPixels)"),
        };
        assert_eq!(raw, pixels);
        Ok(())
    }

    #[test]
    fn from_raw_pixels_f32_zero_dims_err() {
        let pixels = seq_f32(0);
        assert!(Image::from_raw_pixels_f32(&pixels, 0, 1, RawColorTypeF32::Rgb32F).is_err());
        assert!(Image::from_raw_pixels_f32(&pixels, 1, 0, RawColorTypeF32::Rgb32F).is_err());
    }
}
