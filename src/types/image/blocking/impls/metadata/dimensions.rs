use crate::blocking::Image;

impl Image {
    pub fn width(&self) -> u32 {
        self.width.get()
    }

    pub fn height(&self) -> u32 {
        self.height.get()
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.width.get() as f32 / self.height.get() as f32
    }
}
