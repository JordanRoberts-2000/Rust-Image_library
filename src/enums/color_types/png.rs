use image::{ColorType, ExtendedColorType};

#[derive(Debug, Clone, Default, PartialEq)]
pub enum PngColorType {
    Rgb8,
    #[default]
    Rgba8,
    Rgb16,
    Rgba16,
    L8,
    La8,
    L16,
    La16,
}

impl From<PngColorType> for ExtendedColorType {
    fn from(png_type: PngColorType) -> Self {
        match png_type {
            PngColorType::Rgb8 => ExtendedColorType::Rgb8,
            PngColorType::Rgba8 => ExtendedColorType::Rgba8,
            PngColorType::Rgb16 => ExtendedColorType::Rgb16,
            PngColorType::Rgba16 => ExtendedColorType::Rgba16,
            PngColorType::L8 => ExtendedColorType::L8,
            PngColorType::La8 => ExtendedColorType::La8,
            PngColorType::L16 => ExtendedColorType::L16,
            PngColorType::La16 => ExtendedColorType::La16,
        }
    }
}

impl From<PngColorType> for ColorType {
    fn from(png_type: PngColorType) -> Self {
        match png_type {
            PngColorType::Rgb8 => ColorType::Rgb8,
            PngColorType::Rgba8 => ColorType::Rgba8,
            PngColorType::L8 => ColorType::L8,
            PngColorType::La8 => ColorType::La8,
            PngColorType::L16 => ColorType::L16,
            PngColorType::La16 => ColorType::La16,
            PngColorType::Rgb16 => ColorType::Rgb16,
            PngColorType::Rgba16 => ColorType::Rgba16,
        }
    }
}

impl From<&PngColorType> for ColorType {
    fn from(png_type: &PngColorType) -> Self {
        match png_type {
            PngColorType::Rgb8 => ColorType::Rgb8,
            PngColorType::Rgba8 => ColorType::Rgba8,
            PngColorType::L8 => ColorType::L8,
            PngColorType::La8 => ColorType::La8,
            PngColorType::L16 => ColorType::L16,
            PngColorType::La16 => ColorType::La16,
            PngColorType::Rgb16 => ColorType::Rgb16,
            PngColorType::Rgba16 => ColorType::Rgba16,
        }
    }
}

impl From<ColorType> for PngColorType {
    fn from(color: ColorType) -> Self {
        match color {
            ColorType::Rgb8 => PngColorType::Rgb8,
            ColorType::Rgba8 => PngColorType::Rgba8,
            ColorType::Rgb16 => PngColorType::Rgb16,
            ColorType::Rgba16 => PngColorType::Rgba16,
            ColorType::L8 => PngColorType::L8,
            ColorType::La8 => PngColorType::La8,
            ColorType::L16 => PngColorType::L16,
            ColorType::La16 => PngColorType::La16,
            _ => PngColorType::default(),
        }
    }
}
