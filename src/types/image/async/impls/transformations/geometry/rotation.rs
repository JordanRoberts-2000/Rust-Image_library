use crate::{image::enums::TransformOp, Image, Result};

impl Image {
    pub async fn rotate_90(&self) -> Result<&Self> {
        let mut state = self.state.write().await;
        state.config.pipeline.push(TransformOp::Rotate(90));
        Ok(self)
    }

    pub async fn rotate_180(&self) -> Result<&Self> {
        let mut state = self.state.write().await;
        state.config.pipeline.push(TransformOp::Rotate(180));
        Ok(self)
    }

    pub async fn rotate_270(&self) -> Result<&Self> {
        let mut state = self.state.write().await;
        state.config.pipeline.push(TransformOp::Rotate(270));
        Ok(self)
    }
}
