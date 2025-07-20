use image::ColorType;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum AvifColorType {
    #[default]
    Rgba8,
    Rgb8,
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
