use {
    crate::{BitDepth, ColorType},
    image::{self, DynamicImage},
    std::borrow::Cow,
    strum_macros::EnumIter,
};

#[derive(Debug, Clone, Default, PartialEq, EnumIter)]
pub enum AvifColorType {
    #[default]
    Rgba8,
    Rgb8,
}

impl AvifColorType {
    pub(crate) fn bytes<'a>(&self, img: &'a DynamicImage) -> Cow<'a, [u8]> {
        match self {
            AvifColorType::Rgb8 => {
                if let DynamicImage::ImageRgb8(b) = img {
                    Cow::Borrowed(b.as_raw())
                } else {
                    Cow::Owned(img.to_rgb8().into_raw())
                }
            }
            AvifColorType::Rgba8 => {
                if let DynamicImage::ImageRgba8(b) = img {
                    Cow::Borrowed(b.as_raw())
                } else {
                    Cow::Owned(img.to_rgba8().into_raw())
                }
            }
        }
    }

    pub fn has_alpha(&self) -> bool {
        use AvifColorType::*;
        match self {
            Rgb8 => false,
            Rgba8 => true,
        }
    }

    pub fn channels(&self) -> usize {
        use AvifColorType::*;
        match self {
            Rgb8 => 3,
            Rgba8 => 4,
        }
    }

    pub fn bit_depth(&self) -> BitDepth {
        BitDepth::Eight
    }
}

impl From<AvifColorType> for image::ColorType {
    fn from(color_type: AvifColorType) -> Self {
        match color_type {
            AvifColorType::Rgb8 => image::ColorType::Rgb8,
            AvifColorType::Rgba8 => image::ColorType::Rgba8,
        }
    }
}

impl From<AvifColorType> for image::ExtendedColorType {
    fn from(color_type: AvifColorType) -> Self {
        match color_type {
            AvifColorType::Rgb8 => image::ExtendedColorType::Rgb8,
            AvifColorType::Rgba8 => image::ExtendedColorType::Rgba8,
        }
    }
}

impl From<AvifColorType> for ColorType {
    fn from(color_type: AvifColorType) -> Self {
        match color_type {
            AvifColorType::Rgb8 => ColorType::Rgb8,
            AvifColorType::Rgba8 => ColorType::Rgba8,
        }
    }
}

impl From<&AvifColorType> for ColorType {
    fn from(color_type: &AvifColorType) -> Self {
        match color_type {
            AvifColorType::Rgb8 => ColorType::Rgb8,
            AvifColorType::Rgba8 => ColorType::Rgba8,
        }
    }
}
