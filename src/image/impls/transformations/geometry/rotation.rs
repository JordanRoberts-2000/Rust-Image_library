use crate::Image;

impl Image {
    pub fn rotate_90(&mut self) -> &mut Self {
        self.config.rotate_90();
        self
    }

    pub fn rotate_180(&mut self) -> &mut Self {
        self.config.rotate_180();
        self
    }

    pub fn rotate_270(&mut self) -> &mut Self {
        self.config.rotate_270();
        self
    }
}
