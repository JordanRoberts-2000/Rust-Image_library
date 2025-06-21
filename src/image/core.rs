use crate::{CompressionType, ImageData, ImageFormat, ImageSrc, TransformOp};

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

#[derive(Default, Debug)]
pub struct ImageConfig {
    pub pipeline: Vec<TransformOp>,
    pub quality: Option<u32>,
    pub compression: CompressionType,
}
