use {
    crate::{
        constants::DEFAULT_IMAGE_FILE_NAME,
        encoding::{AvifConfig, CompressionType, GifConfig, JpegConfig, PngConfig, WebpConfig},
        image::TransformOp,
        EncodeFormat,
    },
    std::{cell::RefCell, path::PathBuf},
};

#[derive(Debug, Clone, PartialEq)]
pub struct ImageConfig {
    pub pipeline: RefCell<Vec<TransformOp>>,

    pub file_name: String,
    pub output_dir: PathBuf,
    pub prefix: Option<String>,
    pub suffix: Option<String>,

    pub minimize_bit_depth: bool,
    pub remove_unused_transparency: bool,

    pub static_only: bool,
    pub encode_format: Option<EncodeFormat>,
    pub quality: Option<u8>,
    pub compression: CompressionType,
    pub jpeg: Option<JpegConfig>,
    pub png: Option<PngConfig>,
    pub avif: Option<AvifConfig>,
    pub webp: Option<WebpConfig>,
    pub gif: Option<GifConfig>,
}

impl Default for ImageConfig {
    fn default() -> Self {
        Self {
            pipeline: RefCell::new(Vec::new()),

            file_name: DEFAULT_IMAGE_FILE_NAME.to_string(),
            output_dir: PathBuf::from("."),
            prefix: None,
            suffix: None,

            minimize_bit_depth: false,
            remove_unused_transparency: false,

            static_only: false,
            encode_format: None,
            quality: None,
            compression: CompressionType::Lossy,
            jpeg: None,
            png: None,
            avif: None,
            webp: None,
            gif: None,
        }
    }
}
