use crate::{image::TransformOp, Image};

impl Image {
    pub fn grayscale(&mut self) -> &mut Self {
        self.config.pipeline.borrow_mut().push(TransformOp::Grayscale);
        self
    }
}
