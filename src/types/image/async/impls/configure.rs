use crate::{AvifConfig, Image, JpegConfig, WebpConfig};

impl Image {
    pub async fn configure_jpeg(&self, config: JpegConfig) -> &Self {
        let mut state = self.state.write().await;
        state.config.jpeg = Some(config);
        self
    }

    pub async fn configure_avif(&self, config: AvifConfig) -> &Self {
        let mut state = self.state.write().await;
        state.config.avif = Some(config);
        self
    }

    pub async fn configure_webp(&self, config: WebpConfig) -> &Self {
        let mut state = self.state.write().await;
        state.config.webp = Some(config);
        self
    }
}
