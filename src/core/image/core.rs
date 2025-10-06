use {
    crate::{image::ImageConfig, ImageMetadata, ImageSrc},
    image::DynamicImage,
    std::cell::RefCell,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub(super) src: ImageSrc,
    pub(super) config: ImageConfig,
    pub(super) decoded: RefCell<DynamicImage>,
    pub(super) metadata: ImageMetadata,
}
