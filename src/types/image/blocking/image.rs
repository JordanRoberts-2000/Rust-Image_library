use std::num::NonZeroU32;

use crate::{
    image::{blocking::ImageData, enums::ImageSrc, ImageConfig},
    ImageFormat,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub(super) src: ImageSrc,
    pub(super) config: ImageConfig,
    pub(super) data: ImageData,

    pub(super) height: NonZeroU32,
    pub(super) width: NonZeroU32,
    pub(super) format: ImageFormat,
}
