use crate::Image;

impl Image {
    pub fn minimize_bit_depth(&mut self) -> &mut Self {
        self.config.minimize_bit_depth = true;
        self
    }

    pub fn remove_unused_transparency(&mut self) -> &mut Self {
        self.config.remove_unused_transparency = true;
        self
    }
}
