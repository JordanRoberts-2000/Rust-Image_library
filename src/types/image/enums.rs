use {
    std::{num::NonZeroU32, path::PathBuf},
    url::Url,
};

#[derive(Debug, Clone, PartialEq)]
pub enum ImageData {
    EncodedBytes(Vec<u8>),
    Decoded(image::DynamicImage),
    File(PathBuf),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransformOp {
    Crop(u32, u32, u32, u32),
    Rotate(u32),
    Resize(NonZeroU32, NonZeroU32),
    ResizeExact(NonZeroU32, NonZeroU32),
    ResizeToFill(NonZeroU32, NonZeroU32),
    MaxSize(NonZeroU32),
    Grayscale,
    Contrast(f32),
    Blur(f32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImageSrc {
    File(PathBuf),
    Url(Url),
    Base64(String),
    Bytes,
    RawPixels,
    Reader,
}
