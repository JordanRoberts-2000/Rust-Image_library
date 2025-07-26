use crate::{blocking::Image, image::enums::TransformOp};

impl Image {
    pub fn rotate_90(&mut self) -> &mut Self {
        self.config.pipeline.push(TransformOp::Rotate90);
        self
    }

    pub fn rotate_180(&mut self) -> &mut Self {
        self.config.pipeline.push(TransformOp::Rotate180);
        self
    }

    pub fn rotate_270(&mut self) -> &mut Self {
        self.config.pipeline.push(TransformOp::Rotate270);
        self
    }
}
