use crate::TransformPipeline;

impl TransformPipeline {
    pub fn rotate_90(&mut self) -> &mut Self {
        self.rotation_deg = (self.rotation_deg + 90) % 360;
        self
    }

    pub fn rotate_180(&mut self) -> &mut Self {
        self.rotation_deg = (self.rotation_deg + 180) % 360;
        self
    }

    pub fn rotate_270(&mut self) -> &mut Self {
        self.rotation_deg = (self.rotation_deg + 270) % 360;
        self
    }
}
