use {
    crate::{
        image::{blocking::ImageData, enums::ImageSrc, ImageConfig},
        ImageFormat,
    },
    std::{cell::RefCell, num::NonZeroU32, rc::Rc},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub(super) src: ImageSrc,
    pub(super) config: ImageConfig,
    pub(super) data: Rc<RefCell<ImageData>>,

    pub(super) height: NonZeroU32,
    pub(super) width: NonZeroU32,
    pub(super) format: ImageFormat,
}
