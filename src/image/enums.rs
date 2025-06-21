use {std::path::PathBuf, url::Url};

#[derive(Debug)]
pub enum ImageData {
    Bytes(Vec<u8>),
    Decoded(image::DynamicImage),
    File(PathBuf),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum ImageSrc {
    File(PathBuf),
    Url(Url),
    Bytes,
    Reader,
}
