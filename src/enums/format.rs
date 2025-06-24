use crate::ImageError;

use {
    strum::VariantNames,
    strum_macros::{Display, VariantNames},
};

#[derive(Display, Debug, Clone, Copy, VariantNames, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum ImageFormat {
    WebP,
    Png,
    Jpeg,
    Avif,
}

impl ImageFormat {
    pub fn supported() -> &'static [&'static str] {
        ImageFormat::VARIANTS
    }

    pub fn to_mime_type(&self) -> &'static str {
        match self {
            ImageFormat::WebP => "image/webp",
            ImageFormat::Png => "image/png",
            ImageFormat::Jpeg => "image/jpeg",
            ImageFormat::Avif => "image/avif",
        }
    }

    pub fn extention(&self) -> &'static str {
        match self {
            ImageFormat::WebP => "webp",
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpg",
            ImageFormat::Avif => "avif",
        }
    }
}

impl TryFrom<&str> for ImageFormat {
    type Error = ();

    fn try_from(ext: &str) -> Result<Self, Self::Error> {
        match ext.to_ascii_lowercase().as_str() {
            "webp" => Ok(ImageFormat::WebP),
            "png" => Ok(ImageFormat::Png),
            "jpg" | "jpeg" => Ok(ImageFormat::Jpeg),
            "avif" => Ok(ImageFormat::Avif),
            _ => Err(()),
        }
    }
}

impl TryFrom<image::ImageFormat> for ImageFormat {
    type Error = ImageError;

    fn try_from(fmt: image::ImageFormat) -> Result<Self, Self::Error> {
        match fmt {
            image::ImageFormat::Png => Ok(ImageFormat::Png),
            image::ImageFormat::Jpeg => Ok(ImageFormat::Jpeg),
            image::ImageFormat::WebP => Ok(ImageFormat::WebP),
            image::ImageFormat::Avif => Ok(ImageFormat::Avif),
            other => Err(ImageError::UnsupportedFormat(other)),
        }
    }
}

impl From<ImageFormat> for image::ImageFormat {
    fn from(fmt: ImageFormat) -> image::ImageFormat {
        match fmt {
            ImageFormat::WebP => image::ImageFormat::WebP,
            ImageFormat::Png => image::ImageFormat::Png,
            ImageFormat::Jpeg => image::ImageFormat::Jpeg,
            ImageFormat::Avif => image::ImageFormat::Avif,
        }
    }
}
