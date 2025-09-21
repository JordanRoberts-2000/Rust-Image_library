use {
    crate::{
        image::{ImageConfig, ImageData, ImageMetadata},
        ColorModel, ErrorKind, Image, ImageFormat, ImageSrc, Result, ResultCtx, ValidationError,
    },
    image::{DynamicImage, ImageBuffer, Luma, LumaA, Rgb, Rgba},
    std::{borrow::Cow, cell::RefCell},
};

#[derive(Debug, Clone)]
pub enum PixelDataCow<'a> {
    U8(Cow<'a, [u8]>),
    U16(Cow<'a, [u16]>),
}

impl<'a> From<&'a [u8]> for PixelDataCow<'a> {
    fn from(s: &'a [u8]) -> Self {
        Self::U8(Cow::Borrowed(s))
    }
}
impl From<Vec<u8>> for PixelDataCow<'_> {
    fn from(v: Vec<u8>) -> Self {
        Self::U8(Cow::Owned(v))
    }
}
impl<'a> From<&'a [u16]> for PixelDataCow<'a> {
    fn from(s: &'a [u16]) -> Self {
        Self::U16(Cow::Borrowed(s))
    }
}
impl From<Vec<u16>> for PixelDataCow<'_> {
    fn from(v: Vec<u16>) -> Self {
        Self::U16(Cow::Owned(v))
    }
}
impl<'a> From<&'a Vec<u8>> for PixelDataCow<'a> {
    fn from(v: &'a Vec<u8>) -> Self {
        Self::U8(Cow::Borrowed(v.as_slice()))
    }
}
impl<'a> From<&'a Vec<u16>> for PixelDataCow<'a> {
    fn from(v: &'a Vec<u16>) -> Self {
        Self::U16(Cow::Borrowed(v.as_slice()))
    }
}

impl Image {
    pub fn from_raw_pixels<'a>(
        pixels: impl Into<PixelDataCow<'a>>, width: u32, height: u32, color_model: ColorModel,
    ) -> Result<Self> {
        let pixels = pixels.into();

        use PixelDataCow::*;
        let img: DynamicImage = match (pixels, color_model) {
            (U8(b), ColorModel::Rgb) => {
                ImageBuffer::<Rgb<u8>, _>::from_raw(width, height, b.into_owned())
                    .map(DynamicImage::ImageRgb8)
                    .ok_or_else(|| ValidationError::InvalidBuffer(ColorModel::Rgb))
                    .ctx(ErrorKind::Validate, Some(&ImageSrc::RawPixels))?
            }
            (U8(b), ColorModel::Rgba) => {
                ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, b.into_owned())
                    .map(DynamicImage::ImageRgba8)
                    .ok_or_else(|| ValidationError::InvalidBuffer(ColorModel::Rgba))
                    .ctx(ErrorKind::Validate, Some(&ImageSrc::RawPixels))?
            }
            (U8(b), ColorModel::Grayscale) => {
                ImageBuffer::<Luma<u8>, _>::from_raw(width, height, b.into_owned())
                    .map(DynamicImage::ImageLuma8)
                    .ok_or_else(|| ValidationError::InvalidBuffer(ColorModel::Grayscale))
                    .ctx(ErrorKind::Validate, Some(&ImageSrc::RawPixels))?
            }
            (U8(b), ColorModel::GrayscaleAlpha) => {
                ImageBuffer::<LumaA<u8>, _>::from_raw(width, height, b.into_owned())
                    .map(DynamicImage::ImageLumaA8)
                    .ok_or_else(|| ValidationError::InvalidBuffer(ColorModel::GrayscaleAlpha))
                    .ctx(ErrorKind::Validate, Some(&ImageSrc::RawPixels))?
            }

            (U16(b), ColorModel::Rgb) => {
                ImageBuffer::<Rgb<u16>, _>::from_raw(width, height, b.into_owned())
                    .map(DynamicImage::ImageRgb16)
                    .ok_or_else(|| ValidationError::InvalidBuffer(ColorModel::Rgb))
                    .ctx(ErrorKind::Validate, Some(&ImageSrc::RawPixels))?
            }
            (U16(b), ColorModel::Rgba) => {
                ImageBuffer::<Rgba<u16>, _>::from_raw(width, height, b.into_owned())
                    .map(DynamicImage::ImageRgba16)
                    .ok_or_else(|| ValidationError::InvalidBuffer(ColorModel::Rgba))
                    .ctx(ErrorKind::Validate, Some(&ImageSrc::RawPixels))?
            }
            (U16(b), ColorModel::Grayscale) => {
                ImageBuffer::<Luma<u16>, _>::from_raw(width, height, b.into_owned())
                    .map(DynamicImage::ImageLuma16)
                    .ok_or_else(|| ValidationError::InvalidBuffer(ColorModel::Grayscale))
                    .ctx(ErrorKind::Validate, Some(&ImageSrc::RawPixels))?
            }
            (U16(b), ColorModel::GrayscaleAlpha) => {
                ImageBuffer::<LumaA<u16>, _>::from_raw(width, height, b.into_owned())
                    .map(DynamicImage::ImageLumaA16)
                    .ok_or_else(|| ValidationError::InvalidBuffer(ColorModel::GrayscaleAlpha))
                    .ctx(ErrorKind::Validate, Some(&ImageSrc::RawPixels))?
            }
        };

        Ok(Self {
            src: ImageSrc::RawPixels,
            data: RefCell::new(ImageData::RawPixels(img)),
            config: ImageConfig::default(),
            metadata: ImageMetadata::new(width, height, ImageFormat::default())
                .ctx(ErrorKind::ReadMetadata, Some(&ImageSrc::RawPixels))?,
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
        let img = Image::from_raw_pixels(&pixels, w, h, ColorModel::Rgb)?;
        assert_src_rawpixels(&img);
        assert_eq!(img.width(), w);
        assert_eq!(img.height(), h);

        let raw = {
            let data = img.data.borrow();
            match &*data {
                ImageData::RawPixels(di) => di.to_rgb8().into_raw(),
                _ => panic!("expected RawPixels"),
            }
        };
        assert_eq!(raw, pixels);
        Ok(())
    }

    #[test]
    fn from_raw_pixels_rgba8_ok() -> Result<()> {
        let (w, h, c) = (2, 2, 4);
        let pixels = seq_u8((w * h * c) as usize);
        let img = Image::from_raw_pixels(&pixels, w, h, ColorModel::Rgba)?;
        assert_src_rawpixels(&img);
        assert_eq!(img.width(), w);
        assert_eq!(img.height(), h);

        let raw = {
            let data = img.data.borrow();
            match &*data {
                ImageData::RawPixels(di) => di.to_rgba8().into_raw(),
                _ => panic!("expected RawPixels"),
            }
        };
        assert_eq!(raw, pixels);
        Ok(())
    }

    #[test]
    fn from_raw_pixels_l8_ok() -> Result<()> {
        let (w, h, c) = (3, 2, 1);
        let pixels = seq_u8((w * h * c) as usize);
        let img = Image::from_raw_pixels(&pixels, w, h, ColorModel::Grayscale)?;
        assert_src_rawpixels(&img);
        assert_eq!(img.width(), w);
        assert_eq!(img.height(), h);

        let raw = {
            let data = img.data.borrow();
            match &*data {
                ImageData::RawPixels(di) => di.to_luma8().into_raw(),
                _ => panic!("expected RawPixels"),
            }
        };
        assert_eq!(raw, pixels);
        Ok(())
    }

    #[test]
    fn from_raw_pixels_la8_ok() -> Result<()> {
        let (w, h, c) = (3, 2, 2);
        let pixels = seq_u8((w * h * c) as usize);
        let img = Image::from_raw_pixels(&pixels, w, h, ColorModel::GrayscaleAlpha)?;
        assert_src_rawpixels(&img);
        assert_eq!(img.width(), w);
        assert_eq!(img.height(), h);

        let raw = {
            let data = img.data.borrow();
            match &*data {
                ImageData::RawPixels(di) => di.to_luma_alpha8().into_raw(),
                _ => panic!("expected RawPixels"),
            }
        };
        assert_eq!(raw, pixels);
        Ok(())
    }

    #[test]
    fn from_raw_pixels_u8_zero_dims_err() {
        let pixels = seq_u8(0);
        assert!(Image::from_raw_pixels(&pixels, 0, 2, ColorModel::Rgb).is_err());
        assert!(Image::from_raw_pixels(&pixels, 2, 0, ColorModel::Rgb).is_err());
    }

    // ------ u16 variants ------

    #[test]
    fn from_raw_pixels_u16_rgb16_ok() -> Result<()> {
        let (w, h, c) = (2, 2, 3);
        let pixels = seq_u16((w * h * c) as usize);
        let img = Image::from_raw_pixels(&pixels, w, h, ColorModel::Rgb)?;
        assert_src_rawpixels(&img);
        assert_eq!(img.width(), w);
        assert_eq!(img.height(), h);

        let raw = {
            let data = img.data.borrow();
            match &*data {
                ImageData::RawPixels(di) => di.to_rgb16().into_raw(),
                _ => panic!("expected RawPixels"),
            }
        };
        assert_eq!(raw, pixels);
        Ok(())
    }

    #[test]
    fn from_raw_pixels_u16_rgba16_ok() -> Result<()> {
        let (w, h, c) = (2, 2, 4);
        let pixels = seq_u16((w * h * c) as usize);
        let img = Image::from_raw_pixels(&pixels, w, h, ColorModel::Rgba)?;
        assert_src_rawpixels(&img);

        let raw = {
            let data = img.data.borrow();
            match &*data {
                ImageData::RawPixels(di) => di.to_rgba16().into_raw(),
                _ => panic!("expected RawPixels"),
            }
        };
        assert_eq!(raw, pixels);
        Ok(())
    }

    #[test]
    fn from_raw_pixels_u16_zero_dims_err() {
        let pixels = seq_u16(0);
        assert!(Image::from_raw_pixels(&pixels, 0, 2, ColorModel::Rgb).is_err());
        assert!(Image::from_raw_pixels(&pixels, 2, 0, ColorModel::Rgb).is_err());
    }
}
