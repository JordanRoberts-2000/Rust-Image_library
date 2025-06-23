use std::path::PathBuf;

use crate::{
    CompressionType, ImageData, ImageFormat, ImageSrc, TransformOp, DEFAULT_IMAGE_FILE_NAME,
};

#[derive(Debug)]
pub struct Image {
    pub(super) src: ImageSrc,
    pub(super) config: ImageConfig,
    pub(super) data: ImageData,

    pub height: u32,
    pub width: u32,
    pub aspect_ratio: f32,
    pub format: ImageFormat,
}

#[derive(Debug)]
pub struct ImageConfig {
    pub pipeline: Vec<TransformOp>,
    pub quality: Option<u32>,
    pub compression: CompressionType,
    pub file_name: String,
    pub output_dir: PathBuf,
}

impl Default for ImageConfig {
    fn default() -> Self {
        Self {
            pipeline: Vec::new(),
            quality: None,
            compression: CompressionType::Lossy,
            file_name: DEFAULT_IMAGE_FILE_NAME.to_string(),
            output_dir: PathBuf::from("."),
        }
    }
}
