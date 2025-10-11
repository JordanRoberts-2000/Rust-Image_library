use crate::encoding::{AvifColorType, ColorType, JpegColorType, PngColorType, WebpColorType};

pub enum ResolvedColorType {
    Png(PngColorType),
    Jpeg(JpegColorType),
    Webp(WebpColorType),
    Avif(AvifColorType),
}

impl From<ResolvedColorType> for ColorType {
    fn from(r: ResolvedColorType) -> Self {
        match r {
            ResolvedColorType::Png(ct) => ct.into(),
            ResolvedColorType::Jpeg(ct) => ct.into(),
            ResolvedColorType::Webp(ct) => ct.into(),
            ResolvedColorType::Avif(ct) => ct.into(),
        }
    }
}
