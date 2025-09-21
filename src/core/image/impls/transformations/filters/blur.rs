use crate::{image::TransformOp, Blur, Image};

impl Image {
    pub fn blur(&mut self, intensity: Blur) -> &mut Self {
        let intensity = intensity.to_value();
        self.config.pipeline.borrow_mut().push(TransformOp::Blur(intensity));
        self
    }
}
