use crate::Image;

impl Image {
    pub fn apply_transforms(&mut self) -> &mut Self {
        self.config.apply_transforms(&mut self.raw);
        self
    }
}
