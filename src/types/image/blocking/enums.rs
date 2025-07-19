use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum ImageData {
    EncodedBytes(Vec<u8>),
    DynamicImage(image::DynamicImage),
    File(PathBuf),
}
