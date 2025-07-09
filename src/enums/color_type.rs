#[derive(Debug, Clone, Default, PartialEq)]
pub enum ColorType {
    #[default]
    Rgb8,
    Rgba8,
    L8,
    La8,
}

impl ColorType {
    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            ColorType::L8 => 1,
            ColorType::La8 => 2,
            ColorType::Rgb8 => 3,
            ColorType::Rgba8 => 4,
        }
    }
}
