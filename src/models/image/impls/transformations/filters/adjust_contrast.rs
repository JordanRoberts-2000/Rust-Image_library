use crate::{Image, TransformOp};

impl Image {
    pub fn adjust_contrast(&mut self, contrast: i32) -> &mut Self {
        let c = contrast.clamp(-100, 100) as f32;
        self.config.pipeline.push(TransformOp::Contrast(c));
        self
    }
}
