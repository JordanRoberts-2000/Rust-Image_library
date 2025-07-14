use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use tokio::task::spawn_blocking;

use crate::{Image, ImageError, Result};

impl Image {
    pub async fn to_base64(&self) -> Result<String> {
        self.apply_transforms().await?;
        let format = self.state.read().await.format;

        let mut buffer = Vec::new();

        self.encode(&mut buffer, format).await?;

        let encoded = spawn_blocking(move || BASE64.encode(buffer))
            .await
            .map_err(ImageError::TaskJoinError)?;

        Ok(encoded)
    }
}
