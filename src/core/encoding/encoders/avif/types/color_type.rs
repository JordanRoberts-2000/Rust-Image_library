use {
    crate::encoding::{
        AlphaChannelOps, AvifColorType, ColorType, ColorTypeOps, EncodeColorTypeOps,
    },
    inherent::inherent,
};

impl EncodeColorTypeOps for AvifColorType {
    fn from_color_type_lossy(ct: ColorType) -> Self {
        match ct {
            ColorType::Rgb8 => AvifColorType::Rgb8,
            ColorType::Rgba8 => AvifColorType::Rgba8,
            ColorType::Rgb16 => AvifColorType::Rgb8,
            ColorType::Rgba16 => AvifColorType::Rgba8,
            ColorType::Grayscale8 => AvifColorType::Rgb8,
            ColorType::GrayscaleAlpha8 => AvifColorType::Rgba8,
            ColorType::Grayscale16 => AvifColorType::Rgb8,
            ColorType::GrayscaleAlpha16 => AvifColorType::Rgba8,
            ColorType::Rgb32Float => AvifColorType::Rgb8,
            ColorType::Rgba32Float => AvifColorType::Rgba8,
        }
    }

    fn to_minimal_bit_depth(self) -> Self {
        self
    }

    fn remove_alpha(self) -> Self {
        <Self as AlphaChannelOps>::remove_alpha(self)
    }

    fn has_alpha(&self) -> bool {
        <Self as AlphaChannelOps>::has_alpha(&self)
    }
}

#[inherent]
impl ColorTypeOps for AvifColorType {
    pub fn channels(&self) -> u8 {
        ColorType::from(*self).channels()
    }

    pub fn bit_depth(&self) -> u8 {
        ColorType::from(*self).bit_depth()
    }

    pub fn supports_grayscale() -> bool {
        false
    }

    pub fn supports_transparency() -> bool {
        true
    }
}

#[inherent]
impl AlphaChannelOps for AvifColorType {
    pub fn has_alpha(&self) -> bool {
        ColorType::from(*self).has_alpha()
    }

    pub fn remove_alpha(self) -> Self {
        match self {
            AvifColorType::Rgba8 => AvifColorType::Rgb8,
            other => other,
        }
    }

    pub fn ensure_alpha(self) -> Self {
        match self {
            AvifColorType::Rgb8 => AvifColorType::Rgba8,
            other => other,
        }
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
