use {
    crate::{
        image::{Decoded, ImageConfig, ImageOrigin},
        Format,
    },
    std::cell::RefCell,
};

#[derive(Debug, Clone)]
pub struct Image {
    pub(crate) origin: ImageOrigin,
    pub(crate) config: ImageConfig,
    pub(crate) decoded: RefCell<Decoded>,
    pub(crate) format: Option<Format>,
}
