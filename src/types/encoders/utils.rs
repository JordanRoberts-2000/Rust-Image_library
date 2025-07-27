use {
    crate::{utils::decode, ImageError, ImageFormat, Result, ValidationError},
    image::DynamicImage,
};

pub fn decode(bytes: &[u8], format: Option<ImageFormat>) -> Result<DynamicImage> {
    if bytes.is_empty() {
        return Err(ValidationError::EmptyByteArray.into());
    }

    if let Some(fmt) = format {
        decode::from_bytes_with_format(bytes, fmt)
    } else {
        decode::from_bytes(bytes)
    }
}

pub fn validate_dimensions(width: u32, height: u32) -> Result<()> {
    if height == 0 {
        return Err(ImageError::Validation(ValidationError::InvalidHeight));
    }

    if width == 0 {
        return Err(ImageError::Validation(ValidationError::InvalidWidth));
    }

    Ok(())
}

pub fn try_strip_alpha_if_unused(img: &DynamicImage) -> Option<DynamicImage> {
    match img {
        DynamicImage::ImageRgba8(rgba) => {
            if rgba.pixels().all(|p| p.0[3] == u8::MAX) {
                Some(DynamicImage::ImageRgb8(img.to_rgb8()))
            } else {
                None
            }
        }
        DynamicImage::ImageRgba16(rgba16) => {
            if rgba16.pixels().all(|p| p.0[3] == u16::MAX) {
                Some(DynamicImage::ImageRgb16(img.to_rgb16()))
            } else {
                None
            }
        }
        DynamicImage::ImageLumaA8(la8) => {
            if la8.pixels().all(|p| p.0[1] == u8::MAX) {
                Some(DynamicImage::ImageLuma8(img.to_luma8()))
            } else {
                None
            }
        }
        DynamicImage::ImageLumaA16(la16) => {
            if la16.pixels().all(|p| p.0[1] == u16::MAX) {
                Some(DynamicImage::ImageLuma16(img.to_luma16()))
            } else {
                None
            }
        }
        DynamicImage::ImageRgba32F(rgba32) => {
            if rgba32.pixels().all(|p| p.0[3] == 1.0) {
                Some(DynamicImage::ImageRgb32F(img.to_rgb32f()))
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn quantize_image_to_8bit(img: &DynamicImage) -> Option<DynamicImage> {
    match img {
        DynamicImage::ImageRgb16(_) | DynamicImage::ImageRgb32F(_) => {
            Some(DynamicImage::ImageRgb8(img.to_rgb8()))
        }
        DynamicImage::ImageRgba16(_) | DynamicImage::ImageRgba32F(_) => {
            Some(DynamicImage::ImageRgba8(img.to_rgba8()))
        }
        DynamicImage::ImageLuma16(_) => Some(DynamicImage::ImageLuma8(img.to_luma8())),
        DynamicImage::ImageLumaA16(_) => Some(DynamicImage::ImageLumaA8(img.to_luma_alpha8())),
        _ => None,
    }
}
