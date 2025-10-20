use {
    crate::{
        image::{Decoded, ImageConfig},
        Format, ImageSrc,
    },
    std::cell::RefCell,
};

#[derive(Debug, Clone)]
pub struct Image {
    pub(crate) src: ImageSrc,
    pub(crate) config: ImageConfig,
    pub(crate) decoded: RefCell<Decoded>,
    pub(crate) format: Option<Format>,
}
