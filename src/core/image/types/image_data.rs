use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum ImageData {
    File(PathBuf),
    EncodedBytes(Vec<u8>),
    RawPixels(image::DynamicImage),
}
