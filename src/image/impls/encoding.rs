use crate::{CompressionType, Image, ImageFormat};

impl Image {
    pub fn lossless(&mut self) -> &mut Self {
        self.config.compression = CompressionType::Lossless;
        self
    }

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

    pub fn quality(&mut self, quality: u32) -> &mut Self {
        self.config.quality = Some(quality.clamp(1, 100));
        self
    }
}
