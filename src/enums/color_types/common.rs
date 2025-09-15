use {crate::BitDepth, image::ColorType as ImgColorType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorModel {
    Rgb,
    Rgba,
    Luma,
    LumaA,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorType {
    Rgb8,
    Rgb16,
    Rgb32F,
    Rgba8,
    Rgba16,
    Rgba32F,
    L8,
    La8,
    L16,
    La16,
}

impl ColorType {
    #[inline]
    pub fn bit_depth(self) -> BitDepth {
        match self {
            ColorType::Rgb8 | ColorType::Rgba8 | ColorType::L8 | ColorType::La8 => BitDepth::Eight,
            ColorType::Rgb16 | ColorType::Rgba16 | ColorType::L16 | ColorType::La16 => {
                BitDepth::Sixteen
            }
            ColorType::Rgb32F | ColorType::Rgba32F => BitDepth::Float32,
        }
    }

    #[inline]
    pub fn channels(self) -> u32 {
        match self {
            ColorType::Rgb8 | ColorType::Rgb16 | ColorType::Rgb32F => 3,
            ColorType::Rgba8 | ColorType::Rgba16 | ColorType::Rgba32F => 4,
            ColorType::L8 | ColorType::L16 => 1,
            ColorType::La8 | ColorType::La16 => 2,
        }
    }
}

impl From<ColorType> for ImgColorType {
    fn from(ct: ColorType) -> Self {
        match ct {
            ColorType::Rgb8 => ImgColorType::Rgb8,
            ColorType::Rgb16 => ImgColorType::Rgb16,
            ColorType::Rgb32F => ImgColorType::Rgb32F,
            ColorType::Rgba8 => ImgColorType::Rgba8,
            ColorType::Rgba16 => ImgColorType::Rgba16,
            ColorType::Rgba32F => ImgColorType::Rgba32F,
            ColorType::L8 => ImgColorType::L8,
            ColorType::La8 => ImgColorType::La8,
            ColorType::L16 => ImgColorType::L16,
            ColorType::La16 => ImgColorType::La16,
        }
    }
}
