use crate::{CompressionType, Image, ImageFormat};

impl Image {
    pub async fn lossless(&self) -> &Self {
        let mut state = self.state.write().await;
        state.config.compression = CompressionType::Lossless;
        self
    }

    pub async fn jpeg(&self) -> &Self {
        let mut state = self.state.write().await;
        state.format = ImageFormat::Jpeg;
        self
    }

    pub async fn png(&self) -> &Self {
        let mut state = self.state.write().await;
        state.format = ImageFormat::Png;
        self
    }

    pub async fn webp(&self) -> &Self {
        let mut state = self.state.write().await;
        state.format = ImageFormat::WebP;
        self
    }

    pub async fn avif(&self) -> &Self {
        let mut state = self.state.write().await;
        state.format = ImageFormat::Avif;
        self
    }

    pub async fn quality(&self, quality: u32) -> &Self {
        let mut state = self.state.write().await;
        state.config.quality = Some(quality.clamp(1, 100));
        self
    }
}
