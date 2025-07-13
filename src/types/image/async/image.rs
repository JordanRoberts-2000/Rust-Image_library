use {
    crate::{
        image::{
            enums::{ImageData, ImageSrc},
            ImageConfig,
        },
        ImageFormat,
    },
    std::num::NonZeroU32,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub(super) src: ImageSrc,
    pub(super) config: ImageConfig,
    pub(super) data: ImageData,

    pub(super) height: NonZeroU32,
    pub(super) width: NonZeroU32,
    pub format: ImageFormat,
}
