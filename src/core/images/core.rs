use crate::{images::ImagesBuilder, Image};

const DEFAULT_CAPACITY: usize = 8;

pub struct Images {
    pub(crate) inner: Vec<Image>,
}

impl Images {
    pub fn new() -> Self {
        Self { inner: Vec::with_capacity(DEFAULT_CAPACITY) }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { inner: Vec::with_capacity(capacity) }
    }

    pub fn from_vec(images: Vec<Image>) -> Self {
        Self { inner: images }
    }

    pub fn builder() -> ImagesBuilder {
        ImagesBuilder::new()
    }
}
