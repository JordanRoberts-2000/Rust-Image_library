use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum ImageData {
    EncodedBytes(Vec<u8>),
    Decoded(image::DynamicImage),
    File(PathBuf),
}
