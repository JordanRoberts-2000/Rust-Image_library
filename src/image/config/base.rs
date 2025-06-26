use std::path::PathBuf;

use crate::{
    constants::DEFAULT_IMAGE_FILE_NAME, AvifConfig, CompressionType, JpegConfig, TransformOp,
    WebpConfig,
};

#[derive(Debug)]
pub struct ImageConfig {
    pub pipeline: Vec<TransformOp>,
    pub remove_source: bool,

    pub quality: Option<u32>,
    pub compression: CompressionType,
    pub file_name: String,
    pub output_dir: PathBuf,
    pub prefix: Option<String>,
    pub suffix: Option<String>,

    pub jpeg: Option<JpegConfig>,
    pub avif: Option<AvifConfig>,
    pub webp: Option<WebpConfig>,
}

impl Default for ImageConfig {
    fn default() -> Self {
        Self {
            pipeline: Vec::new(),
            remove_source: false,
            quality: None,
            compression: CompressionType::Lossy,
            file_name: DEFAULT_IMAGE_FILE_NAME.to_string(),
            output_dir: PathBuf::from("."),
            prefix: None,
            suffix: None,
            jpeg: None,
            avif: None,
            webp: None,
        }
    }
}
