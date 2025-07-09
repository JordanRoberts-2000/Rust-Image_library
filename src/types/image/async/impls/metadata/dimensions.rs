use crate::Image;

impl Image {
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
