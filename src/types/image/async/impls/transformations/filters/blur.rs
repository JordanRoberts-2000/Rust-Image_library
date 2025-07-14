use crate::{image::enums::TransformOp, Image, Result};

impl Image {
    pub async fn blur(&self, intensity: u8) -> Result<&Self> {
        let intensity = intensity.clamp(0, 100) as f32;

        let mut state = self.state.write().await;
        state.config.pipeline.push(TransformOp::Blur(intensity));

        Ok(self)
    }
}
