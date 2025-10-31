use crate::{image::TransformOp, Image};

mod filters {
    mod adjust_contrast;
    mod blur;
    mod grayscale;
}
mod geometry {
    mod cropping;
    mod rotation;
}
mod resizing;

impl Image {
    pub fn apply_transform(&mut self, transform: TransformOp) -> &mut Self {
        self.config.pipeline.borrow_mut().push(transform);
        self
    }
}
