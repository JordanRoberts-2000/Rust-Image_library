use {
    crate::{
        blocking::Image,
        image::{blocking::ImageData, enums::ImageSrc, ImageConfig},
        ColorType, ImageError, ImageFormat, Result, ValidationError,
    },
    image::{DynamicImage, ImageBuffer, Luma, LumaA, Rgb, Rgba},
    std::num::NonZeroU32,
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

        Ok(Self {
            src: ImageSrc::RawPixels,
            data: ImageData::Decoded(img),
            config: ImageConfig::default(),
            height,
            width,
            format: ImageFormat::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        blocking::Image,
        image::{blocking::ImageData, enums::ImageSrc, ImageConfig},
        ColorType, ImageError, ValidationError,
    };

    fn test_color_type_ok(color_type: ColorType, pixel_size: usize) {
        let width = 2;
        let height = 2;
        let num_pixels = (width * height) as usize;
        let pixels = vec![1u8; pixel_size * num_pixels];

        let img =
            Image::from_raw_pixels(pixels.clone(), width, height, color_type.clone()).unwrap();

        assert_eq!(img.width(), width);
        assert_eq!(img.height(), height);
        assert_eq!(img.src, ImageSrc::RawPixels);
        assert_eq!(img.config, ImageConfig::default());
        assert!(matches!(img.data, ImageData::Decoded(_)));
    }

    #[test]
    fn test_from_raw_pixels_valid_variants() {
        test_color_type_ok(ColorType::Rgb8, 3);
        test_color_type_ok(ColorType::Rgba8, 4);
        test_color_type_ok(ColorType::L8, 1);
        test_color_type_ok(ColorType::La8, 2);
    }

    fn test_invalid_buffer(color_type: ColorType, pixel_size: usize) {
        let width: u32 = 2;
        let height: u32 = 2;
        let too_short = vec![1u8; (width as usize * height as usize * pixel_size) - 1];

        let result = Image::from_raw_pixels(too_short, width, height, color_type.clone());
        assert!(matches!(result, Err(ImageError::InvalidBuffer(ct)) if ct == color_type));
    }

    #[test]
    fn test_from_raw_pixels_invalid_buffer_variants() {
        test_invalid_buffer(ColorType::Rgb8, 3);
        test_invalid_buffer(ColorType::Rgba8, 4);
        test_invalid_buffer(ColorType::L8, 1);
        test_invalid_buffer(ColorType::La8, 2);
    }

    #[test]
    fn test_from_raw_pixels_zero_dimensions() {
        let pixels = vec![255, 0, 0];
        let result = Image::from_raw_pixels(pixels, 0, 10, ColorType::Rgb8);

        assert!(matches!(
            result,
            Err(ImageError::Validation(ValidationError::InvalidDimensions(
                0, 10
            )))
        ));
    }
}
