use {
    crate::{
        encoding::{
            macros::{forward_color_type_impls, forward_transparency_impls},
            AlphaChannelOps, BitDepth, ColorType, ColorTypeOps,
        },
        ImageError, ValidationError,
    },
    image::{self, DynamicImage},
    std::borrow::Cow,
    strum_macros::EnumIter,
};

#[derive(Debug, Clone, Default, Copy, Eq, PartialEq, EnumIter)]
pub enum AvifColorType {
    #[default]
    Rgba8,
    Rgb8,
}

impl AlphaChannelOps for AvifColorType {
    fn has_alpha(&self) -> bool {
        ColorType::from(*self).has_alpha()
    }

    fn remove_alpha(self) -> Self {
        match self {
            AvifColorType::Rgba8 => AvifColorType::Rgb8,
            other => other,
        }
    }

    fn ensure_alpha(self) -> Self {
        match self {
            AvifColorType::Rgb8 => AvifColorType::Rgba8,
            other => other,
        }
    }
}

impl ColorTypeOps for AvifColorType {
    fn channels(&self) -> u8 {
        ColorType::from(*self).channels()
    }

    fn bit_depth(&self) -> BitDepth {
        ColorType::from(*self).bit_depth()
    }

    fn supports_grayscale() -> bool {
        false
    }

    fn supports_transparency() -> bool {
        true
    }
}

impl AvifColorType {
    pub(crate) fn bytes<'a>(&self, img: &'a DynamicImage) -> Cow<'a, [u8]> {
        ColorType::from(*self).bytes(img)
    }

    forward_color_type_impls!();
    forward_transparency_impls!();
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

impl TryFrom<ColorType> for AvifColorType {
    type Error = ImageError;

    fn try_from(color_type: ColorType) -> Result<Self, Self::Error> {
        match color_type {
            ColorType::Rgb8 => Ok(AvifColorType::Rgb8),
            ColorType::Rgba8 => Ok(AvifColorType::Rgba8),
            other => Err(ValidationError::UnsupportedColorType(other.into()).into()),
        }
    }
}
