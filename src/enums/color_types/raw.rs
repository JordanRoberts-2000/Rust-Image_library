#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawColorType {
    Rgb8,
    Rgba8,
    L8,
    La8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawColorTypeU16 {
    Rgb16,
    Rgba16,
    L16,
    La16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawColorTypeF32 {
    Rgb32F,
    Rgba32F,
}
