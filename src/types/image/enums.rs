use {std::path::PathBuf, url::Url};

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
    Resize(u32, u32),
    ResizeExact(u32, u32),
    ResizeToFill(u32, u32),
    MaxSize(u32),
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
