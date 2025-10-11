use crate::{encoding::CompressionType, Image};

impl Image {
    pub fn quality(&mut self, quality: u8) -> &mut Self {
        self.config.quality = Some(quality);
        self
    }

    pub fn minimize_bit_depth(&mut self) -> &mut Self {
        self.config.minimize_bit_depth = true;
        self
    }

    pub fn remove_unused_transparency(&mut self) -> &mut Self {
        self.config.remove_unused_transparency = true;
        self
    }

    pub fn lossless(&mut self) -> &mut Self {
        self.config.compression = CompressionType::Lossless;
        self
    }
}
