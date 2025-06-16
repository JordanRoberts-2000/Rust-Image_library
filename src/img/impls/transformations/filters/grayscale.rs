use crate::Img;

impl Img {
    pub fn greyscale(&mut self) -> &mut Self {
        self.transform_pipeline.greyscale();
        self
    }
}
