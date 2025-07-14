use crate::{image::enums::TransformOp, Image, Result};

impl Image {
    pub async fn grayscale(&self) -> Result<&Self> {
        let mut state = self.state.write().await;
        state.config.pipeline.push(TransformOp::Grayscale);
        Ok(self)
    }
}
