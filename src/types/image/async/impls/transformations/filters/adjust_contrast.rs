use crate::{image::enums::TransformOp, Image, Result};

impl Image {
    pub async fn adjust_contrast(&self, contrast: i32) -> Result<&Self> {
        let c = contrast.clamp(-100, 100) as f32;

        let mut state = self.state.write().await;
        state.config.pipeline.push(TransformOp::Contrast(c));

        Ok(self)
    }
}
