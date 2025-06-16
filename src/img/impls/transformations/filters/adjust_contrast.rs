use crate::Img;

impl Img {
    pub fn adjust_contrast(&mut self, contrast: i32) -> &mut Self {
        let c = contrast.clamp(-100, 100) as f32;
        self.transform_pipeline.adjust_contrast(c);
        self
    }
}
