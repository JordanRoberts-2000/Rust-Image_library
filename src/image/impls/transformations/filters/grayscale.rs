use crate::{Image, TransformOp};

impl Image {
    pub fn grayscale(&mut self) -> &mut Self {
        self.config.pipeline.push(TransformOp::Grayscale);
        self
    }
}
