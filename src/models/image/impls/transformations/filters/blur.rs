use crate::{Image, TransformOp};

impl Image {
    pub fn blur(&mut self, intensity: u8) -> &mut Self {
        let intensity = intensity.clamp(0, 100) as f32;
        self.config.pipeline.push(TransformOp::Blur(intensity));
        self
    }
}
