use std::{path::PathBuf, sync::Arc};

#[derive(Debug, Clone, PartialEq)]
pub enum ImageData {
    EncodedBytes(Arc<Vec<u8>>),
    Decoded(image::DynamicImage),
    File(PathBuf),
}
