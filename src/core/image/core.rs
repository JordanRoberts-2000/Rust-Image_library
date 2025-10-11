use {
    crate::{image::ImageConfig, ImageMetadata, ImageSrc},
    image::DynamicImage,
    std::cell::RefCell,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub(crate) src: ImageSrc,
    pub(crate) config: ImageConfig,
    pub(crate) decoded: RefCell<DynamicImage>,
    pub(crate) metadata: ImageMetadata,
}
