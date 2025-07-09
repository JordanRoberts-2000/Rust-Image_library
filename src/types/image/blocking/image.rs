use crate::{
    image::{
        enums::{ImageData, ImageSrc},
        ImageConfig,
    },
    ImageFormat,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub(super) src: ImageSrc,
    pub(super) config: ImageConfig,
    pub(super) data: ImageData,

    pub height: u32,
    pub width: u32,
    pub aspect_ratio: f32,
    pub format: ImageFormat,
}
