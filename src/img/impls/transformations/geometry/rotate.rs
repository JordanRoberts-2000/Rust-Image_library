use crate::Img;

impl Img {
    pub fn rotate_90(&mut self) -> &mut Self {
        self.transform_pipeline.rotate_90();
        self
    }

    pub fn rotate_180(&mut self) -> &mut Self {
        self.transform_pipeline.rotate_180();
        self
    }

    pub fn rotate_270(&mut self) -> &mut Self {
        self.transform_pipeline.rotate_270();
        self
    }
}
