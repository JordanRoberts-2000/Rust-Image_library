use crate::{image::TransformOp, Image};

impl Image {
    pub fn adjust_contrast(&mut self, contrast: i32) -> &mut Self {
        let c = contrast.clamp(-100, 100) as f32;
        self.config.pipeline.borrow_mut().push(TransformOp::Contrast(c));
        self
    }
}
