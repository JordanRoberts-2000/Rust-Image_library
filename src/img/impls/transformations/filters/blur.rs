use crate::Img;

impl Img {
    pub fn blur(&mut self, intensity: u8) -> &mut Self {
        let intensity = intensity.clamp(0, 100) as f32;
        self.transform_pipeline.blur(intensity);
        self
    }
}
