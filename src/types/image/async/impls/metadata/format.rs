use crate::{Image, ImageFormat};

impl Image {
    pub async fn format(&self) -> ImageFormat {
        self.state.write().await.format
    }
}
