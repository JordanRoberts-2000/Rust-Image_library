use crate::{blocking::Image, ImageFormat};

impl Image {
    pub fn format(&self) -> ImageFormat {
        self.format
    }
}
