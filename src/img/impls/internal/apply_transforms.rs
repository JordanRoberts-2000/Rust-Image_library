use crate::Img;

impl Img {
    pub fn apply_transforms(&mut self) -> &mut Self {
        self.transform_pipeline.apply_transforms(&mut self.img);
        self
    }
}
