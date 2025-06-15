use crate::{ImageFormat, ImgSrc, TransformPipeline};

#[derive(Debug)]
pub struct Img {
    pub(super) img: image::DynamicImage,
    pub(super) src: ImgSrc,
    pub(super) transform_pipeline: TransformPipeline,
    pub height: u32,
    pub width: u32,
    pub aspect_ratio: f32,
    pub format: ImageFormat,
}
