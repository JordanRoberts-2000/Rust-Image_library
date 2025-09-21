use {
    crate::{AvifConfig, CompressionType, Image, ImageFormat, JpegConfig, WebpConfig},
    std::num::NonZeroU32,
};

impl Image {
    pub fn configure_jpeg(&mut self, config: JpegConfig) -> &mut Self {
        self.config.jpeg = Some(config);
        self
    }

    pub fn configure_avif(&mut self, config: AvifConfig) -> &mut Self {
        self.config.avif = Some(config);
        self
    }

    pub fn configure_webp(&mut self, config: WebpConfig) -> &mut Self {
        self.config.webp = Some(config);
        self
    }

    pub fn remove_source_file(&mut self) -> &mut Self {
        self.config.remove_source = true;
        self
    }

    pub fn lossless(&mut self) -> &mut Self {
        self.config.compression = CompressionType::Lossless;
        self
    }

    pub fn jpeg(&mut self) -> &mut Self {
        self.metadata.format = ImageFormat::Jpeg;
        self
    }

    pub fn png(&mut self) -> &mut Self {
        self.metadata.format = ImageFormat::Png;
        self
    }

    pub fn webp(&mut self) -> &mut Self {
        self.metadata.format = ImageFormat::WebP;
        self
    }

    pub fn avif(&mut self) -> &mut Self {
        self.metadata.format = ImageFormat::Avif;
        self
    }

    pub fn to_format(&mut self, format: ImageFormat) -> &mut Self {
        self.metadata.format = format;
        self
    }

    pub fn quality(&mut self, quality: u32) -> &mut Self {
        self.config.quality = Some(quality.clamp(1, 100));
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

    pub(super) fn set_width(&mut self, width: NonZeroU32) {
        self.metadata.width = width;
    }

    pub(super) fn set_height(&mut self, height: NonZeroU32) {
        self.metadata.height = height;
    }

    pub(super) fn set_size(&mut self, width: NonZeroU32, height: NonZeroU32) {
        self.set_width(width);
        self.set_height(height);
    }
}
