use crate::{image::ImageConfig, images::ImagesBuilder, Image, ImageSrc};

pub struct ImageEntry {
    pub(crate) src: ImageSrc,
    pub(crate) config: ImageConfig,
}

impl ImageEntry {
    pub fn new(src: ImageSrc) -> Self {
        Self { src, config: ImageConfig::default() }
    }
}

pub struct Images {
    pub(crate) entry_vec: Vec<ImageEntry>,
    pub(crate) image_vec: Vec<Image>,
}

impl Images {
    pub fn new() -> Self {
        Self { entry_vec: Vec::new(), image_vec: Vec::new() }
    }

    pub fn from_src_vec(src: Vec<ImageSrc>) -> Self {
        Self {
            entry_vec: src.into_iter().map(|s| ImageEntry::new(s)).collect::<Vec<ImageEntry>>(),
            image_vec: Vec::new(),
        }
    }

    pub fn from_image_vec(images: Vec<Image>) -> Self {
        Self { entry_vec: Vec::new(), image_vec: images }
    }

    pub fn builder() -> ImagesBuilder {
        ImagesBuilder::new()
    }
}
