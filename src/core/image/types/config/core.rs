use {
    crate::{
        constants::DEFAULT_IMAGE_FILE_NAME,
        encoding::{AvifConfig, CompressionType, JpegConfig, PngConfig, WebpConfig},
        image::TransformOp,
        ImageFormat,
    },
    std::{cell::RefCell, path::PathBuf},
};

#[derive(Debug, Clone, PartialEq)]
pub struct ImageConfig {
    pub pipeline: RefCell<Vec<TransformOp>>,

    pub target_format: Option<ImageFormat>,
    pub file_name: String,
    pub output_dir: PathBuf,
    pub prefix: Option<String>,
    pub suffix: Option<String>,

    pub minimize_bit_depth: bool,
    pub remove_unused_transparency: bool,
    pub remove_source: bool,

    pub quality: Option<u8>,
    pub compression: CompressionType,
    pub jpeg: Option<JpegConfig>,
    pub png: Option<PngConfig>,
    pub avif: Option<AvifConfig>,
    pub webp: Option<WebpConfig>,
}

impl Default for ImageConfig {
    fn default() -> Self {
        Self {
            pipeline: RefCell::new(Vec::new()),

            quality: None,
            compression: CompressionType::Lossy,
            target_format: None,
            file_name: DEFAULT_IMAGE_FILE_NAME.to_string(),
            output_dir: PathBuf::from("."),
            prefix: None,
            suffix: None,

            minimize_bit_depth: false,
            remove_unused_transparency: false,
            remove_source: false,

            jpeg: None,
            png: None,
            avif: None,
            webp: None,
        }
    }
}
