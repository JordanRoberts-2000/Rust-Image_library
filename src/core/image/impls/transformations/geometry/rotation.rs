use crate::{image::TransformOp, Image};

impl Image {
    pub fn rotate_90(&mut self) -> &mut Self {
        self.config.pipeline.borrow_mut().push(TransformOp::Rotate90);
        self
    }

    pub fn rotate_180(&mut self) -> &mut Self {
        self.config.pipeline.borrow_mut().push(TransformOp::Rotate180);
        self
    }

    pub fn rotate_270(&mut self) -> &mut Self {
        self.config.pipeline.borrow_mut().push(TransformOp::Rotate270);
        self
    }
}
