use crate::Image;

impl Image {
    pub fn greyscale(&mut self) -> &mut Self {
        self.config.greyscale();
        self
    }
}
