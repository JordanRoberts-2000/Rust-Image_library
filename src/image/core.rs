use crate::{ImageConfig, ImageFormat, ImageSrc};

#[derive(Debug)]
pub struct Image {
    pub(super) raw: image::DynamicImage,
    pub(super) src: ImageSrc,
    pub(super) config: ImageConfig,
    pub height: u32,
    pub width: u32,
    pub aspect_ratio: f32,
    pub format: ImageFormat,
}
