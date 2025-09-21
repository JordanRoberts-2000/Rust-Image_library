use {
    crate::{
        image::{ImageConfig, ImageData, ImageMetadata},
        ImageSrc,
    },
    std::cell::RefCell,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub(super) src: ImageSrc,
    pub(super) config: ImageConfig,
    pub(super) data: RefCell<ImageData>,
    pub(super) metadata: ImageMetadata,
}
