use crate::{Image, ImageFormat};

impl Image {
    pub fn format(&self) -> ImageFormat {
        self.metadata.format
    }
}
