use crate::{Image, ImageFormat};

impl Image {
    pub fn width(&self) -> u32 {
        self.metadata.width()
    }

    pub fn height(&self) -> u32 {
        self.metadata.height()
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.metadata.aspect_ratio()
    }

    pub fn aspect_ratio_str(&self) -> String {
        self.metadata.aspect_ratio_str()
    }

    pub fn format(&self) -> ImageFormat {
        self.metadata.format
    }
}
