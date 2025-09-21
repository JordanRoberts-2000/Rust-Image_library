use image::codecs::png::{CompressionType as ImgPngCompressionType, FilterType};

#[derive(Debug, serde::Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CompressionType {
    #[default]
    Lossy,
    Lossless,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PngCompressionType {
    Default,
    Fast,
    Best,
}

impl From<PngCompressionType> for ImgPngCompressionType {
    fn from(c: PngCompressionType) -> Self {
        match c {
            PngCompressionType::Default => ImgPngCompressionType::Default,
            PngCompressionType::Fast => ImgPngCompressionType::Fast,
            PngCompressionType::Best => ImgPngCompressionType::Best,
        }
    }
}

impl PngCompressionType {
    pub(crate) fn filter(self) -> FilterType {
        match self {
            PngCompressionType::Best | PngCompressionType::Default => FilterType::Adaptive,
            PngCompressionType::Fast => FilterType::NoFilter,
        }
    }
}
