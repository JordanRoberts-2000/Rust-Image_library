use crate::{Image, TransformOp};

impl Image {
    pub fn rotate_90(&mut self) -> &mut Self {
        self.config.pipeline.push(TransformOp::Rotate(90));
        self
    }

    pub fn rotate_180(&mut self) -> &mut Self {
        self.config.pipeline.push(TransformOp::Rotate(180));
        self
    }

    pub fn rotate_270(&mut self) -> &mut Self {
        self.config.pipeline.push(TransformOp::Rotate(270));
        self
    }
}
