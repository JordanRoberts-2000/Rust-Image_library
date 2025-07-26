use {color_thief::ColorFormat, image::ColorType};

pub trait ToColorThiefFormat {
    fn to_color_thief_format(&self) -> ColorFormat;
}

impl ToColorThiefFormat for ColorType {
    fn to_color_thief_format(&self) -> ColorFormat {
        match self {
            ColorType::Rgb8 => ColorFormat::Rgb,
            ColorType::Rgba8 => ColorFormat::Rgba,
            _ => ColorFormat::Rgb,
        }
    }
}
