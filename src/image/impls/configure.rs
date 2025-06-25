use crate::{AvifConfig, Image, JpegConfig, WebpConfig};

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
}
