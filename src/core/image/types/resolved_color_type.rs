use crate::encoding::{
    AvifColorType, ColorType, JpegColorType, PngColorType, TiffColorType, WebpColorType,
};

pub enum ResolvedColorType {
    Png(PngColorType),
    Jpeg(JpegColorType),
    Webp(WebpColorType),
    Avif(AvifColorType),
    Tiff(TiffColorType),
}

impl From<ResolvedColorType> for ColorType {
    fn from(r: ResolvedColorType) -> Self {
        match r {
            ResolvedColorType::Png(ct) => ct.into(),
            ResolvedColorType::Jpeg(ct) => ct.into(),
            ResolvedColorType::Webp(ct) => ct.into(),
            ResolvedColorType::Avif(ct) => ct.into(),
            ResolvedColorType::Tiff(ct) => ct.into(),
        }
    }
}
