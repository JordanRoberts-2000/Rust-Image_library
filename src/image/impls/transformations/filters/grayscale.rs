use crate::Image;

impl Image {
    pub fn grayscale(&mut self) -> &mut Self {
        self.config.grayscale();
        self
    }
}
