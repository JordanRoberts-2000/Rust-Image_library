use crate::{
    encoding::{AvifConfig, JpegConfig, WebpConfig},
    EncodeFormat, Image,
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

    pub fn jpeg(&mut self) -> &mut Self {
        self.config.encode_format = Some(EncodeFormat::Jpeg);
        self
    }

    pub fn png(&mut self) -> &mut Self {
        self.config.encode_format = Some(EncodeFormat::Png);
        self
    }

    pub fn webp(&mut self) -> &mut Self {
        self.config.encode_format = Some(EncodeFormat::Webp);
        self
    }

    pub fn avif(&mut self) -> &mut Self {
        self.config.encode_format = Some(EncodeFormat::Avif);
        self
    }

    pub fn to_format(&mut self, format: EncodeFormat) -> &mut Self {
        self.config.encode_format = Some(format);
        self
    }
}
