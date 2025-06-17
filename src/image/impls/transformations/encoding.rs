use crate::{Image, ImageFormat};

impl Image {
    pub fn jpeg(&mut self) -> &mut Self {
        self.format = ImageFormat::Jpeg;
        self
    }

    pub fn png(&mut self) -> &mut Self {
        self.format = ImageFormat::Jpeg;
        self
    }

    pub fn webp(&mut self) -> &mut Self {
        self.format = ImageFormat::WebP;
        self
    }
}
